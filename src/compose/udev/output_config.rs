// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    backend::{
        drm::{output::DrmOutputRenderElements, DrmNode, VrrSupport},
        session::Session,
    },
    output::{Mode as WlMode, Output, Scale as OutputScale},
    reexports::drm::control::{crtc, Mode as DrmMode},
    utils::Point,
};
use tracing::warn;

use super::{AnvilState, SurfaceData, UdevData, UdevOutputId, UdevRenderer};
use crate::{
    protocols::screens::ScreenPlan,
    render::OutputRenderElements,
    shell::WindowRenderElement,
    state::Backend,
};

type Planned<'a> = (DrmNode, crtc::Handle, &'a ScreenPlan, Option<DrmMode>);

pub(super) fn apply(state: &mut AnvilState<UdevData>, targets: &[ScreenPlan], test_only: bool) -> Result<(), String> {

    let plan = check(&state.backend_data, targets)?;
    if test_only {
        return Ok(());
    }

    change_modes(&mut state.backend_data, &plan)?;

    for &(node, crtc, target, _) in &plan {
        if !target.on {
            state.disable_surface(node, crtc);
        }
    }
    for &(node, crtc, target, mode) in &plan {
        if target.on {
            state.arrange_surface(node, crtc, target, mode);
        }
    }
    crate::shell::fixup_positions(&mut state.space, state.pointer.current_location());
    Ok(())
}

fn check<'a>(data: &UdevData, targets: &'a [ScreenPlan]) -> Result<Vec<Planned<'a>>, String> {
    let mut plan = Vec::with_capacity(targets.len());
    for target in targets {
        let Some((node, crtc)) = data.find_by_name(&target.name) else {
            return Err(format!("{} is no longer plugged in", target.name));
        };
        let surface = &data.backends[&node].surfaces[&crtc];
        let mode = match target.mode {
            Some(i) => Some(
                *surface
                    .drm_modes
                    .get(i)
                    .ok_or_else(|| format!("{} has no mode {i}", target.name))?,
            ),
            None => None,
        };
        if target.on && target.varies_rate {
            check_adaptive_sync(surface, &target.name)?;
        }
        plan.push((node, crtc, target, mode));
    }
    Ok(plan)
}

fn check_adaptive_sync(surface: &SurfaceData, name: &str) -> Result<(), String> {
    let support = surface
        .drm_output
        .with_compositor(|c| c.vrr_supported(surface.connector));
    if !matches!(support, Ok(VrrSupport::Supported | VrrSupport::RequiresModeset)) {
        return Err(format!("{name} cannot vary its refresh rate"));
    }
    Ok(())
}

fn change_modes(data: &mut UdevData, plan: &[Planned<'_>]) -> Result<(), String> {
    let mut changed: Vec<(DrmNode, crtc::Handle, DrmMode)> = Vec::new();
    for &(node, crtc, target, mode) in plan {
        let Some(mode) = mode.filter(|_| target.on) else {
            continue;
        };
        let old = data.backends[&node].surfaces[&crtc]
            .drm_output
            .with_compositor(|c| c.pending_mode());
        if mode == old {
            continue;
        }
        if let Err(err) = data.use_drm_mode(node, crtc, mode) {
            put_modes_back(data, changed);
            let (w, h) = mode.size();
            return Err(format!("{} refused {w}x{h}: {err}", target.name));
        }
        changed.push((node, crtc, old));
    }
    Ok(())
}

fn put_modes_back(data: &mut UdevData, changed: Vec<(DrmNode, crtc::Handle, DrmMode)>) {
    for (node, crtc, old) in changed.into_iter().rev() {
        if let Err(err) = data.use_drm_mode(node, crtc, old) {
            warn!("could not put a screen's mode back: {err}");
        }
    }
}

impl UdevData {
    pub(super) fn surface_of(&self, output: &Output) -> Option<&SurfaceData> {
        let id = output.user_data().get::<UdevOutputId>()?;
        self.backends.get(&id.device_id)?.surfaces.get(&id.crtc)
    }

    fn find_by_name(&self, name: &str) -> Option<(DrmNode, crtc::Handle)> {
        self.backends.iter().find_map(|(node, device)| {
            device
                .surfaces
                .iter()
                .find(|(_, surface)| surface.output.name() == name)
                .map(|(crtc, _)| (*node, *crtc))
        })
    }

    fn use_drm_mode(&mut self, node: DrmNode, crtc: crtc::Handle, mode: DrmMode) -> Result<(), String> {
        let primary_gpu = self.primary_gpu;
        let device = self.backends.get_mut(&node).ok_or("the graphics card is gone")?;
        let render_node = device.render_node.unwrap_or(primary_gpu);
        let mut renderer = self
            .gpus
            .single_renderer(&render_node)
            .map_err(|err| err.to_string())?;
        let surface = device.surfaces.get_mut(&crtc).ok_or("the screen is gone")?;
        surface
            .drm_output
            .use_mode::<_, OutputRenderElements<UdevRenderer<'_>, WindowRenderElement<UdevRenderer<'_>>>>(
                mode,
                &mut renderer,
                &DrmOutputRenderElements::default(),
            )
            .map_err(|err| err.to_string())?;

        // fit; with it settled, they are tried again.
        let _ = device.drm_output_manager.try_to_restore_modifiers::<_, OutputRenderElements<
            UdevRenderer<'_>,
            WindowRenderElement<UdevRenderer<'_>>,
        >>(&mut renderer, &DrmOutputRenderElements::default());
        Ok(())
    }
}

impl AnvilState<UdevData> {

    pub(super) fn disable_surface(&mut self, node: DrmNode, crtc: crtc::Handle) {
        let active = self.backend_data.session.is_active();
        let Some(surface) = self
            .backend_data
            .backends
            .get_mut(&node)
            .and_then(|device| device.surfaces.get_mut(&crtc))
        else {
            return;
        };
        if !surface.enabled {
            return;
        }
        surface.enabled = false;
        if active {
            if let Err(err) = surface.drm_output.with_compositor(|c| c.clear()) {
                warn!(?err, "could not switch {} off", surface.output.name());
            }
        }
        if let Some(timer) = surface.vblank_throttle_timer.take() {
            self.handle.remove(timer);
        }
        if let Some(global) = surface.global.take() {
            self.display_handle.remove_global::<AnvilState<UdevData>>(global);
        }
        let output = surface.output.clone();
        self.space.unmap_output(&output);
        self.output_power_state.output_removed(&output);
        self.night_state.output_removed(&output);
        self.backend_data.set_gamma(&output, None);
    }

    pub(super) fn arrange_surface(
        &mut self,
        node: DrmNode,
        crtc: crtc::Handle,
        target: &ScreenPlan,
        mode: Option<DrmMode>,
    ) {
        let Some(surface) = self
            .backend_data
            .backends
            .get_mut(&node)
            .and_then(|device| device.surfaces.get_mut(&crtc))
        else {
            return;
        };
        let output = surface.output.clone();
        let position = Point::from(target.position);
        apply_plan(&output, mode, target);
        let vrr = surface.drm_output.with_compositor(|c| c.vrr_enabled());
        if vrr != target.varies_rate {
            if let Err(err) = surface
                .drm_output
                .with_compositor(|c| c.use_vrr(target.varies_rate))
            {
                warn!(?err, "could not change adaptive sync on {}", output.name());
            }
        }
        let newly_enabled = !surface.enabled;
        if newly_enabled {
            surface.enabled = true;
            surface.powered = true;
            surface.global = Some(output.create_global::<AnvilState<UdevData>>(&self.display_handle));
        }

        surface.drm_output.reset_buffers();
        self.space.map_output(&output, position);
        smithay::desktop::layer_map_for_output(&output).arrange();
        if newly_enabled {

            self.handle
                .insert_idle(move |state| state.render_surface(node, crtc, state.clock.now()));
        }
    }
}

fn apply_plan(output: &Output, mode: Option<DrmMode>, target: &ScreenPlan) {
    output.change_current_state(
        mode.map(WlMode::from),
        Some(target.turn),
        Some(OutputScale::Fractional(target.scale)),
        Some(Point::from(target.position)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use smithay::output::PhysicalProperties;
    use smithay::utils::Transform;

    fn plan(turn: Transform) -> ScreenPlan {
        ScreenPlan { name: "eDP-1".into(), on: true, mode: None, position: (0, 0), turn, scale: 1.0, varies_rate: false }
    }

    #[test]
    fn a_saved_rotation_beats_the_panels_orientation() {
        let output = Output::new(
            "eDP-1".into(),
            PhysicalProperties { size: (0, 0).into(), subpixel: smithay::output::Subpixel::Unknown, make: String::new(), model: String::new() },
        );
        output.change_current_state(None, Some(Transform::_90), None, None);
        assert_eq!(output.current_transform(), Transform::_90);
        apply_plan(&output, None, &plan(Transform::Normal));
        assert_eq!(output.current_transform(), Transform::Normal);
        apply_plan(&output, None, &plan(Transform::_180));
        assert_eq!(output.current_transform(), Transform::_180);
    }
}

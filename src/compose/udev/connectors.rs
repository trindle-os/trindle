// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    backend::{
        drm::{
            compositor::FrameFlags,
            output::DrmOutputRenderElements,
            DrmDevice, DrmNode, Planes,
        },
    },
    desktop::space::Space,
    output::{Mode as WlMode, Output, PhysicalProperties, Scale as OutputScale},
    reexports::{
        drm::{
            control::{connector, crtc, Device, Mode as DrmMode, ModeTypeFlags},
            Device as _,
            Driver,
        },
        wayland_server::{backend::GlobalId, DisplayHandle},
    },
    utils::Transform,
};
use tracing::{debug, info, warn};

use super::{
    dmabuf::get_surface_dmabuf_feedback, AnvilState, BackendData, CardOutput, OutputManager, SurfaceData, UdevData,
    UdevOutputId, UdevRenderer,
};
use crate::{
    drm_extras::{display_info, leftovers},
    protocols::screens::{unique_modes, ScreenMode},
    render::OutputRenderElements,
    shell::{WindowElement, WindowRenderElement},
    state::SurfaceDmabufFeedback,
};

struct ScreenNames {
    make: String,
    model: String,
    serial: Option<String>,
}

impl ScreenNames {
    fn read(drm_device: &DrmDevice, connector: connector::Handle) -> Self {
        let display_info = display_info::for_connector(drm_device, connector);

        let make = display_info
            .as_ref()
            .and_then(|info| info.make())
            .unwrap_or_else(|| "Unknown".into());

        let model = display_info
            .as_ref()
            .and_then(|info| info.model())
            .unwrap_or_else(|| "Unknown".into());

        let serial = display_info.as_ref().and_then(|info| info.serial());
        ScreenNames { make, model, serial }
    }
}

struct ModeChoice {
    infos: Vec<ScreenMode>,
    modes: Vec<DrmMode>,
    chosen: DrmMode,
}

impl ModeChoice {
    fn of(connector: &connector::Info) -> Option<Self> {
        let listed: Vec<ScreenMode> = connector.modes().iter().map(|m| mode_info(*m)).collect();
        let (infos, from) = unique_modes(&listed);
        let modes: Vec<DrmMode> = from.iter().map(|&i| connector.modes()[i]).collect();
        let mode_id = infos
            .iter()
            .position(|m| m.preferred)
            .or((!modes.is_empty()).then_some(0))?;
        let chosen = modes[mode_id];
        Some(ModeChoice { infos, modes, chosen })
    }
}

struct NewScreen {
    node: DrmNode,
    crtc: crtc::Handle,
    connector: connector::Handle,
    render_node: Option<DrmNode>,
    name: String,
    names: ScreenNames,
    modes: ModeChoice,
    size_mm: (i32, i32),
}

impl NewScreen {
    fn of(
        node: DrmNode,
        crtc: crtc::Handle,
        connector: &connector::Info,
        render_node: Option<DrmNode>,
        name: String,
        names: ScreenNames,
    ) -> Option<Self> {
        let Some(modes) = ModeChoice::of(connector) else {
            warn!("Connector {} lists no modes", name);
            return None;
        };
        let (phys_w, phys_h) = connector.size().unwrap_or((0, 0));
        Some(NewScreen {
            node,
            crtc,
            connector: connector.handle(),
            render_node,
            name,
            names,
            modes,
            size_mm: (phys_w as i32, phys_h as i32),
        })
    }

    fn login_scale(&self) -> Option<f64> {
        let login_scale = crate::session::launch().greeter.then(|| {
            let (w, h) = self.modes.chosen.size();
            crate::screen_scale::for_login(&crate::screen_scale::Screen {
                connector: self.name.clone(),
                make: self.names.make.clone(),
                model: self.names.model.clone(),
                serial: self.names.serial.clone().unwrap_or_default(),
                pixels: (w as i32, h as i32),
                millimetres: self.size_mm,
            })
        });
        if let Some(scale) = login_scale {
            info!("login screen: {} at {}%", self.name, (scale * 100.0).round());
        }
        login_scale
    }
}

struct Started {
    output: Output,
    global: GlobalId,
    drm_output: CardOutput,
    frame_flags: FrameFlags,
    dmabuf_feedback: Option<SurfaceDmabufFeedback>,
}

impl AnvilState<UdevData> {
    pub(super) fn connector_connected(&mut self, node: DrmNode, connector: connector::Info, crtc: crtc::Handle) {
        let device = if let Some(device) = self.backend_data.backends.get_mut(&node) {
            device
        } else {
            return;
        };

        let render_node = device.render_node.unwrap_or(self.backend_data.primary_gpu);
        let mut renderer = self.backend_data.gpus.single_renderer(&render_node).unwrap();

        let output_name = format!("{}-{}", connector.interface().as_str(), connector.interface_id());
        info!(?crtc, "Trying to setup connector {}", output_name,);

        let drm_device = device.drm_output_manager.device();
        let non_desktop = is_non_desktop(drm_device, connector.handle());
        let names = ScreenNames::read(drm_device, connector.handle());

        if non_desktop {
            lend_connector(device, connector.handle(), crtc, output_name, &names);
            return;
        }
        let Some(spec) = NewScreen::of(node, crtc, &connector, device.render_node, output_name, names) else {
            return;
        };
        let (output, global) = add_output(&mut self.space, &self.display_handle, drm_device, &connector, &spec);

        let Some((driver, planes)) = query_planes(drm_device, &connector, crtc, &output) else {
            abandon(&mut self.space, &self.display_handle, &output, global);
            return;
        };
        let Some(drm_output) =
            start_output(&mut device.drm_output_manager, &mut renderer, &spec, &output, planes)
        else {
            abandon(&mut self.space, &self.display_handle, &output, global);
            return;
        };

        let dmabuf_feedback = drm_output.with_compositor(|compositor| {
            get_surface_dmabuf_feedback(
                self.backend_data.primary_gpu,
                device.render_node,
                node,
                &mut self.backend_data.gpus,
                compositor.surface(),
            )
        });
        let started = Started { output, global, drm_output, frame_flags: scanout_flags(&driver), dmabuf_feedback };
        device.surfaces.insert(crtc, SurfaceData::new(self.display_handle.clone(), spec, started));

        self.handle.insert_idle(move |state| {
            state.render_surface(node, crtc, state.clock.now());
        });
    }

    pub(super) fn connector_disconnected(&mut self, node: DrmNode, connector: connector::Info, crtc: crtc::Handle) {
        let device = if let Some(device) = self.backend_data.backends.get_mut(&node) {
            device
        } else {
            return;
        };

        if let Some(pos) = device
            .non_desktop_connectors
            .iter()
            .position(|(handle, _)| *handle == connector.handle())
        {
            let _ = device.non_desktop_connectors.remove(pos);
            if let Some(leasing_state) = device.leasing_global.as_mut() {
                leasing_state.withdraw_connector(connector.handle());
            }
        } else if let Some(surface) = device.surfaces.remove(&crtc) {

            self.output_power_state.output_removed(&surface.output);
            self.night_state.output_removed(&surface.output);
            self.image_capture_state.remove_output(&surface.output);
            self.space.unmap_output(&surface.output);
        }

        let render_node = device.render_node.unwrap_or(self.backend_data.primary_gpu);
        let mut renderer = self.backend_data.gpus.single_renderer(&render_node).unwrap();
        let _ = device.drm_output_manager.try_to_restore_modifiers::<_, OutputRenderElements<
            UdevRenderer<'_>,
            WindowRenderElement<UdevRenderer<'_>>,
        >>(
            &mut renderer,

            &DrmOutputRenderElements::default(),
        );
    }
}

fn lend_connector(
    device: &mut BackendData,
    connector: connector::Handle,
    crtc: crtc::Handle,
    output_name: String,
    names: &ScreenNames,
) {
    info!("Connector {} is non-desktop, setting up for leasing", output_name);
    device.non_desktop_connectors.push((connector, crtc));
    if let Some(lease_state) = device.leasing_global.as_mut() {
        lease_state.add_connector::<AnvilState<UdevData>>(
            connector,
            output_name,
            format!("{} {}", names.make, names.model),
        );
    }
}

fn is_non_desktop(drm_device: &DrmDevice, connector: connector::Handle) -> bool {
    drm_device
        .get_properties(connector)
        .ok()
        .and_then(|props| {
            let (info, value) = props
                .into_iter()
                .filter_map(|(handle, value)| {
                    let info = drm_device.get_property(handle).ok()?;

                    Some((info, value))
                })
                .find(|(info, _)| info.name().to_str() == Ok("non-desktop"))?;

            info.value_type().convert_value(value).as_boolean()
        })
        .unwrap_or(false)
}

fn add_output(
    space: &mut Space<WindowElement>,
    display_handle: &DisplayHandle,
    drm_device: &DrmDevice,
    connector: &connector::Info,
    spec: &NewScreen,
) -> (Output, GlobalId) {
    let login_scale = spec.login_scale();
    let output = Output::new(
        spec.name.clone(),
        PhysicalProperties {
            size: spec.size_mm.into(),
            subpixel: connector.subpixel().into(),
            make: spec.names.make.clone(),
            model: spec.names.model.clone(),
        },
    );
    let global = output.create_global::<AnvilState<UdevData>>(display_handle);

    let x = space
        .outputs()
        .fold(0, |acc, o| acc + space.output_geometry(o).unwrap().size.w);
    let position = (x, 0).into();

    let turn = panel_turn(drm_device, connector.handle());
    let wl_mode = WlMode::from(spec.modes.chosen);
    output.set_preferred(wl_mode);
    output.change_current_state(Some(wl_mode), turn, login_scale.map(OutputScale::Fractional), Some(position));
    space.map_output(&output, position);

    output.user_data().insert_if_missing(|| UdevOutputId {
        crtc: spec.crtc,
        device_id: spec.node,
    });
    (output, global)
}

fn abandon(space: &mut Space<WindowElement>, display_handle: &DisplayHandle, output: &Output, global: GlobalId) {
    space.unmap_output(output);
    display_handle.remove_global::<AnvilState<UdevData>>(global);
}

fn query_planes(
    drm_device: &DrmDevice,
    connector: &connector::Info,
    crtc: crtc::Handle,
    output: &Output,
) -> Option<(Driver, Planes)> {
    let driver = match drm_device.get_driver() {
        Ok(driver) => driver,
        Err(err) => {
            warn!("Failed to query drm driver: {}", err);
            return None;
        }
    };

    if let Err(err) = leftovers::cap_depth(drm_device, connector.handle()) {
        debug!("could not set the colour depth for {}: {}", output.name(), err);
    }

    let mut planes = match drm_device.planes(&crtc) {
        Ok(planes) => planes,
        Err(err) => {
            warn!("Failed to query crtc planes: {}", err);
            return None;
        }
    };

    if driver.name().to_string_lossy().to_lowercase().contains("nvidia")
        || driver
            .description()
            .to_string_lossy()
            .to_lowercase()
            .contains("nvidia")
    {
        planes.overlay = vec![];
    }
    Some((driver, planes))
}

fn start_output(
    manager: &mut OutputManager,
    renderer: &mut UdevRenderer<'_>,
    spec: &NewScreen,
    output: &Output,
    planes: Planes,
) -> Option<CardOutput> {
    for mode in fallback_modes(&spec.modes.modes, spec.modes.chosen) {
        match manager
            .initialize_output::<_, OutputRenderElements<UdevRenderer<'_>, WindowRenderElement<UdevRenderer<'_>>>>(
                spec.crtc,
                mode,
                &[spec.connector],
                output,
                Some(planes.clone()),
                renderer,
                &DrmOutputRenderElements::default(),
            ) {
            Ok(made) => {
                if mode != spec.modes.chosen {
                    let (w, h) = mode.size();
                    warn!("{} could not have its best mode, so it runs at {w}x{h}", output.name());
                    output.change_current_state(Some(WlMode::from(mode)), None, None, None);
                }
                return Some(made);
            }
            Err(err) => warn!("Failed to initialize drm output: {}", err),
        }
    }
    None
}

fn scanout_flags(driver: &Driver) -> FrameFlags {
    if std::env::var("ANVIL_DISABLE_DIRECT_SCANOUT").is_ok() {
        FrameFlags::empty()
    } else if driver.name().to_string_lossy().contains("evdi") {

        FrameFlags::DEFAULT - FrameFlags::ALLOW_CURSOR_PLANE_SCANOUT
    } else {
        FrameFlags::DEFAULT
    }
}

impl SurfaceData {
    fn new(dh: DisplayHandle, spec: NewScreen, started: Started) -> Self {
        let Started { output, global, drm_output, frame_flags, dmabuf_feedback } = started;
        SurfaceData {
            dh,
            device_id: spec.node,
            render_node: spec.render_node,
            global: Some(global),
            drm_output,
            frame_flags,
            dmabuf_feedback,
            last_presentation_time: None,
            vblank_throttle_timer: None,
            output,
            connector: spec.connector,
            serial: spec.names.serial,
            drm_modes: spec.modes.modes,
            mode_infos: spec.modes.infos,
            enabled: true,
            powered: true,
            gamma: None,
            original_gamma: None,
            locked_frame_queued: false,
            refused_flips: 0,
            failed_frames: 0,
            lit: false,
            recovery: Default::default(),
            still_since: None,
            resting: None,
            rest_timer: false,
            rest_looks: (0, 0),
            rest_told: None,
        }
    }
}

fn mode_info(mode: DrmMode) -> ScreenMode {
    let wl = WlMode::from(mode);
    ScreenMode {
        width: wl.size.w,
        height: wl.size.h,
        millihertz: wl.refresh,
        preferred: mode.mode_type().contains(ModeTypeFlags::PREFERRED),
    }
}

const MODE_TRIES: usize = 4;

fn fallback_modes(modes: &[DrmMode], best: DrmMode) -> Vec<DrmMode> {
    fallback_order(modes, best, |m| { let (w, h) = m.size(); (w as u32 * h as u32, m.vrefresh()) })
}

fn fallback_order<T: Copy + PartialEq>(modes: &[T], best: T, rank: impl Fn(&T) -> (u32, u32)) -> Vec<T> {
    let mut rest: Vec<T> = modes.iter().copied().filter(|m| *m != best && rank(m).0 <= rank(&best).0).collect();
    rest.sort_by_key(|m| std::cmp::Reverse(rank(m)));
    std::iter::once(best).chain(rest).take(MODE_TRIES).collect()
}

fn panel_turn(drm: &DrmDevice, conn: connector::Handle) -> Option<Transform> {
    let props = drm.get_properties(conn).ok()?;
    let (handles, values) = props.as_props_and_values();
    handles.iter().zip(values).find_map(|(&h, &v)| {
        let info = drm.get_property(h).ok()?;
        if info.name().to_str() != Ok("panel orientation") {
            return None;
        }
        match info.value_type().convert_value(v) {
            smithay::reexports::drm::control::property::Value::Enum(Some(e)) => turn_named(e.name().to_str().ok()?),
            _ => None,
        }
    })
}

fn turn_named(name: &str) -> Option<Transform> {
    match name {
        "Normal" => Some(Transform::Normal),
        "Upside Down" => Some(Transform::_180),
        "Left Side Up" => Some(Transform::_90),
        "Right Side Up" => Some(Transform::_270),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_refused_screen_tries_smaller_modes() {
        let rank = |m: &(u32, u32, u32)| (m.0 * m.1, m.2);
        let best = (2560, 1440, 144);
        let modes = [(1920, 1080, 60), best, (3840, 2160, 60), (2560, 1440, 60), (1280, 720, 60), (1024, 768, 60)];
        let tries = fallback_order(&modes, best, rank);
        assert_eq!(tries, [best, (2560, 1440, 60), (1920, 1080, 60), (1280, 720, 60)]);
        assert_eq!(fallback_order(&[best], best, rank), [best]);
    }

    #[test]
    fn a_turned_panel_is_drawn_turned_back() {
        assert_eq!(turn_named("Normal"), Some(Transform::Normal));
        assert_eq!(turn_named("Upside Down"), Some(Transform::_180));
        assert_eq!(turn_named("Left Side Up"), Some(Transform::_90));
        assert_eq!(turn_named("Right Side Up"), Some(Transform::_270));
        assert_eq!(turn_named("Sideways"), None);
    }
}

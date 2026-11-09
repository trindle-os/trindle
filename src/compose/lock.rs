// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::time::{Duration, Instant};

use smithay::{
    backend::renderer::{
        element::{
            surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
            AsRenderElements, Kind,
        },
        Color32F, ImportAll, Renderer,
    },
    desktop::{layer_map_for_output, LayerSurface, WindowSurfaceType},
    input::{keyboard::Keysym, pointer::MotionEvent},
    output::Output,
    reexports::{
        wayland_protocols::ext::session_lock::v1::server::{
            ext_session_lock_manager_v1::ExtSessionLockManagerV1,
            ext_session_lock_surface_v1::ExtSessionLockSurfaceV1,
            ext_session_lock_v1::{self, ExtSessionLockV1},
        },
        wayland_server::{
            protocol::{wl_output::WlOutput, wl_surface::WlSurface},
            Client, DataInit, Dispatch, DisplayHandle, Resource,
        },
    },
    utils::{IsAlive, Logical, Point, Scale, Size, SERIAL_COUNTER},
    wayland::{
        session_lock::{
            ExtLockSurfaceUserData, LockSurface, SessionLockHandler, SessionLockManagerGlobalData,
            SessionLockManagerState, SessionLockState, SessionLocker,
        },
        shell::wlr_layer::Layer,
    },
};
use tracing::info;

use crate::{
    focus::{KeyboardFocusTarget, PointerFocusTarget},
    state::{AnvilState, Backend},
};

pub const LOCK_COLOR: Color32F = Color32F::new(0.0, 0.0, 0.0, 1.0);

pub const UNLOCK_QUESTION: &str = "trindle-pocketkey-unlock";

pub const SETTLE: Duration = Duration::from_millis(600);

#[derive(Debug, Default)]
pub struct Plugged {
    pub(crate) now: bool,
    pub(crate) at: Option<Point<f64, Logical>>,

    pub(crate) kept: Vec<u32>,
}

pub fn is_unlock_question(layer: &LayerSurface) -> bool {
    layer.layer() == Layer::Overlay && layer.namespace() == UNLOCK_QUESTION && crate::trusted::counts(layer)
}

pub fn reaches_question(plugged: bool, placed: bool, shown_for: Duration) -> bool {
    plugged && placed && shown_for >= SETTLE
}

fn root_of(surface: &WlSurface) -> WlSurface {
    let mut root = surface.clone();
    while let Some(parent) = smithay::wayland::compositor::get_parent(&root) {
        root = parent;
    }
    root
}

#[derive(Debug, Default)]
pub struct SessionLock {

    active: bool,

    lock: Option<ExtSessionLockV1>,

    pending: Option<SessionLocker>,

    blanked: Vec<Output>,

    surfaces: Vec<(Output, LockSurface)>,

    focus_before: Option<KeyboardFocusTarget>,

    pub(crate) locked_at: Option<std::time::Instant>,

    pub(crate) flow_tries: u32,

    questions: Vec<(WlSurface, Instant)>,
}

impl SessionLock {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn surface_on(&self, output: &Output) -> Option<&WlSurface> {
        self.surfaces
            .iter()
            .find(|(o, s)| o == output && s.alive())
            .map(|(_, s)| s.wl_surface())
    }

    fn is_lock_surface(&self, surface: &WlSurface) -> bool {
        self.surfaces.iter().any(|(_, s)| s.wl_surface() == surface)
    }

    pub fn render_elements<R>(&self, renderer: &mut R, output: &Output) -> Vec<WaylandSurfaceRenderElement<R>>
    where
        R: Renderer + ImportAll,
        R::TextureId: Clone + 'static,
    {
        let scale: Scale<f64> = output.current_scale().fractional_scale().into();
        let mut shown = Vec::new();
        {
            let map = layer_map_for_output(output);
            for layer in map.layers().rev().filter(|l| is_unlock_question(l)) {
                let Some(geo) = map.layer_geometry(layer) else { continue };
                let loc = geo.loc.to_physical_precise_round(scale);
                shown.extend(AsRenderElements::<R>::render_elements::<WaylandSurfaceRenderElement<R>>(layer, renderer, loc, scale, 1.0));
            }
        }
        if let Some(surface) = self.surface_on(output) {
            shown.extend(render_elements_from_surface_tree(renderer, surface, (0, 0), scale, 1.0, Kind::Unspecified));
        }
        shown
    }
}

fn all_blank<'a>(
    screens: impl IntoIterator<Item = &'a Output>,
    blanked: &[Output],
    is_off: impl Fn(&Output) -> bool,
) -> bool {
    screens.into_iter().all(|o| blanked.contains(o) || is_off(o))
}

pub fn works_while_locked(keysym: Keysym) -> bool {
    (Keysym::XF86_Switch_VT_1.raw()..=Keysym::XF86_Switch_VT_12.raw()).contains(&keysym.raw())
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn is_locked(&self) -> bool {
        self.session_lock.active
    }

    #[cfg(test)]
    pub(crate) fn lock_for_rig(&mut self) {
        self.session_lock.active = true;
    }

    #[cfg(test)]
    pub(crate) fn unlock_for_rig(&mut self) {
        self.session_lock.active = false;
    }

    fn lock_size(&self, output: &Output) -> Size<u32, Logical> {
        let size = self.space.output_geometry(output).map(|g| g.size).unwrap_or_default();
        Size::from((size.w.max(1) as u32, size.h.max(1) as u32))
    }

    pub fn lock_frame_shown(&mut self, output: &Output) {
        let lock = &mut self.session_lock;
        if lock.active && lock.pending.is_some() && !lock.blanked.contains(output) {
            lock.blanked.push(output.clone());
            self.confirm_lock_if_blank();
        }
    }

    fn confirm_lock_if_blank(&mut self) {
        let lock = &self.session_lock;
        if lock.pending.is_none() {
            return;
        }
        let backend = &self.backend_data;
        if all_blank(self.space.outputs(), &lock.blanked, |o| backend.output_power(o) == Some(false)) {
            if let Some(locker) = self.session_lock.pending.take() {
                locker.lock();
                info!("Screen locked");
            }
        }
    }

    pub fn focus_lock_at(&mut self, location: Point<f64, Logical>) {
        let lock = &self.session_lock;
        let under = self.space.output_under(location).next();
        let surface = under
            .and_then(|o| lock.surface_on(o))
            .or_else(|| lock.surfaces.iter().find(|(_, s)| s.alive()).map(|(_, s)| s.wl_surface()))
            .cloned();
        let target = surface.map(KeyboardFocusTarget::LockSurface);
        let keyboard = self.seat.get_keyboard().unwrap();
        if keyboard.current_focus() != target {
            self.give_keyboard(target, SERIAL_COUNTER.next_serial());
        }
    }

    pub fn lock_surface_under(
        &self,
        location: Point<f64, Logical>,
    ) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        self.question_under(location).or_else(|| self.lock_only_under(location))
    }

    pub fn lock_only_under(
        &self,
        location: Point<f64, Logical>,
    ) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        let output = self.space.output_under(location).next()?;
        let origin = self.space.output_geometry(output)?.loc.to_f64();
        let surface = self.session_lock.surface_on(output)?;
        Some((PointerFocusTarget::WlSurface(surface.clone()), origin))
    }

    fn question_under(&self, location: Point<f64, Logical>) -> Option<(PointerFocusTarget, Point<f64, Logical>)> {
        if !self.session_lock.active {
            return None;
        }
        let output = self.space.output_under(location).next()?;
        let origin = self.space.output_geometry(output)?.loc;
        let map = layer_map_for_output(output);
        let at = location - origin.to_f64();
        let layer = crate::shell::layer_taking_input(&map, &[Layer::Overlay], at, is_unlock_question)?;
        let loc = map.layer_geometry(layer)?.loc;
        let (surface, inner) = layer.surface_under(at - loc.to_f64(), WindowSurfaceType::ALL)?;
        Some((PointerFocusTarget::from(surface), (inner + loc + origin).to_f64()))
    }

    fn note_questions(&mut self) {
        if !self.session_lock.active {
            return;
        }
        let out: Vec<WlSurface> = self
            .space
            .outputs()
            .flat_map(|o| layer_map_for_output(o).layers().filter(|l| is_unlock_question(l)).map(|l| l.wl_surface().clone()).collect::<Vec<_>>())
            .collect();
        let now = Instant::now();
        let seen = &mut self.session_lock.questions;
        seen.retain(|(s, _)| out.contains(s));
        for surface in out {
            if !seen.iter().any(|(s, _)| *s == surface) {
                seen.push((surface, now));
            }
        }
    }

    pub(crate) fn press_on_question(&mut self, targets: &[Option<PointerFocusTarget>], mouse: bool) -> Option<bool> {
        if !self.session_lock.active {
            return None;
        }
        self.note_questions();
        let shown_for = targets.iter().flatten().find_map(|t| match t {
            PointerFocusTarget::WlSurface(s) => {
                let root = root_of(s);
                self.session_lock.questions.iter().find(|(q, _)| *q == root).map(|(_, at)| at.elapsed())
            }
            _ => None,
        })?;
        let placed = !mouse || self.plugged.at == Some(self.pointer.current_location());
        Some(reaches_question(self.plugged.now, placed, shown_for))
    }

    pub(crate) fn through_door(&mut self, plugged: bool, moves: bool, handle: impl FnOnce(&mut Self)) {
        let before = std::mem::replace(&mut self.plugged.now, plugged);
        handle(self);
        self.plugged.now = before;
        if plugged && moves {
            self.plugged.at = Some(self.pointer.current_location());
        }
    }

    pub fn refresh_session_lock(&mut self) {
        if !self.session_lock.active {
            return;
        }
        self.note_questions();
        let lock = &mut self.session_lock;
        lock.surfaces.retain(|(_, s)| s.alive());

        if lock.pending.as_ref().is_some_and(|p| !p.ext_session_lock().is_alive()) {
            lock.pending = None;
        }

        let surfaces: Vec<(Output, LockSurface)> = self.session_lock.surfaces.clone();
        for (output, surface) in surfaces {
            let size = self.lock_size(&output);
            surface.with_pending_state(|state| state.size = Some(size));
            surface.send_configure();
        }

        self.confirm_lock_if_blank();

        let keyboard = self.seat.get_keyboard().unwrap();
        let on_lock = keyboard.current_focus().is_some_and(|f| match f {
            KeyboardFocusTarget::LockSurface(s) => s.alive() && self.session_lock.is_lock_surface(&s),
            _ => false,
        });
        let location = self.pointer.current_location();
        if !on_lock {
            self.focus_lock_at(location);
        }
        self.repoint(location);
    }

    fn repoint(&mut self, location: Point<f64, Logical>) {
        let under = self.surface_under(location);
        let pointer = self.pointer.clone();
        if pointer.current_focus() == under.as_ref().map(|(target, _)| target.clone()) {
            return;
        }
        pointer.motion(
            self,
            under,
            &MotionEvent {
                location,
                serial: SERIAL_COUNTER.next_serial(),
                time: self.clock.now().as_millis(),
            },
        );
        pointer.frame(self);
    }

    pub fn focus_after_unlock(&mut self, target: KeyboardFocusTarget) {
        self.session_lock.focus_before = Some(target);
    }
}

pub fn is_flow_client(client: &Client) -> bool {
    crate::virtual_input::from_box(client)
        && crate::virtual_input::may_control(client)
        && crate::virtual_input::app_id_of(client).is_some_and(|id| crate::own_hands::is_flow(&id))
}

impl<BackendData: Backend + 'static> crate::protocols::unlock::UnlockHandler for AnvilState<BackendData> {
    fn unlock_with_pin(&mut self, client: &Client, pin: &str) -> Result<(), String> {
        use crate::flow_unlock::{judge, Verdict};
        if !is_flow_client(client) {
            info!("An unlock was asked for by a program that is not Flow; refused");
            return Err(String::new());
        }
        let lock = &self.session_lock;
        let holds = lock.active && lock.pending.is_none() && lock.lock.as_ref().is_some_and(|l| l.is_alive());
        let since = lock.locked_at.map_or(std::time::Duration::MAX, |at| at.elapsed());
        match judge(holds, crate::pin::is_set(), since, crate::pin::minutes(), lock.flow_tries, pin, crate::pin::check) {
            Verdict::No(words, tried) => {
                if tried {
                    self.session_lock.flow_tries += 1;
                }
                info!("Flow asked to unlock; refused: {words}");
                Err(words)
            }
            Verdict::Unlock => {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
                let _ = crate::flow_unlock::note(&crate::flow_unlock::notes(), now);
                info!("Flow unlocked the screen with the PIN");
                if let Some(lock) = self.session_lock.lock.clone() {
                    lock.finished();
                }
                self.unlock();
                Ok(())
            }
        }
    }
}

crate::delegate_unlock!(@<BackendData: Backend + 'static> AnvilState<BackendData>);

impl<BackendData: Backend + 'static> SessionLockHandler for AnvilState<BackendData> {
    fn lock_state(&mut self) -> &mut SessionLockManagerState {
        &mut self.session_lock_state
    }

    fn lock(&mut self, confirmation: SessionLocker) {

        if self.session_lock.lock.as_ref().is_some_and(|l| l.is_alive()) {
            info!("A lock was asked for while one holds; refused");
            return;
        }
        info!("Locking the screen");
        if !self.session_lock.active {
            self.session_lock.locked_at = Some(std::time::Instant::now());

            self.end_capture(Some(crate::protocols::input_capture::capture::DeactivatedReason::Locked), None);

            let keyboard = self.seat.get_keyboard().unwrap();
            self.session_lock.focus_before = keyboard.current_focus();
            keyboard.unset_grab(self);
            let pointer = self.pointer.clone();
            pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), self.clock.now().as_millis());
            self.dnd_icon = None;
            self.cursor_status = smithay::input::pointer::CursorImageStatus::default_named();
            self.release_key = None;
            self.released_bind = None;
        }

        crate::locked::mark(true);
        let lock = &mut self.session_lock;
        lock.active = true;
        lock.lock = Some(confirmation.ext_session_lock().clone());
        lock.pending = Some(confirmation);
        lock.blanked.clear();
        lock.surfaces.retain(|(_, s)| s.alive());

        self.note_questions();

        self.confirm_lock_if_blank();
        let location = self.pointer.current_location();
        self.focus_lock_at(location);
        self.repoint(location);
    }

    fn unlock(&mut self) {

        info!("Unlocking the screen");
        let lock = std::mem::take(&mut self.session_lock);
        crate::locked::mark(false);
        self.backend_data.unlocked();
        let keyboard = self.seat.get_keyboard().unwrap();
        match lock.focus_before.filter(|f| f.alive()) {
            Some(target) => keyboard.set_focus(self, Some(target), SERIAL_COUNTER.next_serial()),
            None => self.focus_topmost(),
        }
        self.repoint(self.pointer.current_location());
    }

    fn new_surface(&mut self, surface: LockSurface, wl_output: WlOutput) {
        let Some(output) = Output::from_resource(&wl_output) else {
            return;
        };

        let from_holder = self.session_lock.lock.as_ref().is_some_and(|lock| {
            let owner = lock.client();
            owner.is_some() && owner == surface.wl_surface().client()
        });
        if !self.session_lock.active || !from_holder {
            return;
        }
        let size = self.lock_size(&output);
        surface.with_pending_state(|state| state.size = Some(size));
        output.enter(surface.wl_surface());
        let lock = &mut self.session_lock;
        lock.surfaces.retain(|(o, s)| s.alive() && o != &output);
        lock.surfaces.push((output, surface));
        let location = self.pointer.current_location();
        self.focus_lock_at(location);
        self.repoint(location);
    }
}

impl<BackendData: Backend + 'static> Dispatch<ExtSessionLockV1, SessionLockState> for AnvilState<BackendData> {
    fn request(
        state: &mut Self,
        client: &Client,
        resource: &ExtSessionLockV1,
        request: ext_session_lock_v1::Request,
        data: &SessionLockState,
        dh: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        if let ext_session_lock_v1::Request::UnlockAndDestroy = request {
            let lock = &state.session_lock;
            let holds = lock.active && lock.pending.is_none() && lock.lock.as_ref() == Some(resource);
            if !holds {
                resource.post_error(ext_session_lock_v1::Error::InvalidUnlock, "This lock does not hold the session.");
                return;
            }
        }
        <SessionLockManagerState as Dispatch<ExtSessionLockV1, SessionLockState, Self>>::request(
            state, client, resource, request, data, dh, data_init,
        );
    }
}

smithay::reexports::wayland_server::delegate_global_dispatch!(@<BackendData: Backend + 'static> AnvilState<BackendData>: [
    ExtSessionLockManagerV1: SessionLockManagerGlobalData
] => SessionLockManagerState);
smithay::reexports::wayland_server::delegate_dispatch!(@<BackendData: Backend + 'static> AnvilState<BackendData>: [
    ExtSessionLockManagerV1: ()
] => SessionLockManagerState);
smithay::reexports::wayland_server::delegate_dispatch!(@<BackendData: Backend + 'static> AnvilState<BackendData>: [
    ExtSessionLockSurfaceV1: ExtLockSurfaceUserData
] => SessionLockManagerState);

#[cfg(test)]
mod tests {
    use super::*;
    use smithay::output::{PhysicalProperties, Subpixel};

    fn screen(name: &str) -> Output {
        Output::new(
            name.into(),
            PhysicalProperties { size: (0, 0).into(), subpixel: Subpixel::Unknown, make: "".into(), model: "".into() },
        )
    }

    #[test]
    fn locked_only_once_every_screen_is_blank_or_off() {
        let (a, b) = (screen("A"), screen("B"));
        let screens = [a.clone(), b.clone()];
        let on = |_: &Output| false;
        assert!(!all_blank(&screens, &[], on));
        assert!(!all_blank(&screens, std::slice::from_ref(&a), on));
        assert!(all_blank(&screens, &[a.clone(), b.clone()], on));

        let b_off = |o: &Output| o.name() == "B";
        assert!(all_blank(&screens, std::slice::from_ref(&a), b_off));

        assert!(all_blank(&[], &[], on));
    }

    #[test]
    fn a_press_reaches_the_question_only_when_plugged_in_placed_and_settled() {
        let long = SETTLE + Duration::from_millis(1);
        assert!(reaches_question(true, true, long));
        assert!(reaches_question(true, true, SETTLE));
        assert!(!reaches_question(false, true, long), "a press no plugged-in device made");
        assert!(!reaches_question(true, false, long), "a click after a made-up move");
        assert!(!reaches_question(true, true, SETTLE - Duration::from_millis(1)), "a click before it settled");
    }

    mod live {
        use std::time::Duration;

        use crate::rig::{alone, Rig, KEY_A};
        use crate::state::ClientState;

        use super::super::{SETTLE, UNLOCK_QUESTION};

        const BTN_LEFT: u32 = 0x110;
        const KEY_ENTER: u32 = 28;
        const KEY_ESC: u32 = 1;
        const SIZE: (i32, i32) = (400, 300);
        const AREA: [i32; 4] = [100, 200, 200, 60];

        const YES: (f64, f64) = (0.5, 0.6);
        const ABOVE: (f64, f64) = (0.5, 0.375);
        const NONE: [u32; 0] = [];

        fn root(rig: &mut Rig) -> usize {
            rig.join(ClientState { from_root: true, ..ClientState::default() })
        }

        fn locked(rig: &mut Rig) -> usize {
            let locker = rig.join(ClientState::default());
            rig.lock_by(locker);
            rig.lock_surfaces(locker);
            locker
        }

        fn turn(rig: &mut Rig) {
            rig.state.refresh_session_lock();
            rig.pump();
        }

        fn settle(rig: &mut Rig) {
            std::thread::sleep(SETTLE + Duration::from_millis(100));
            turn(rig);
        }

        #[test]
        fn a_plugged_in_press_reaches_the_unlock_question_once_it_has_settled() {
            alone("lock::tests::live::a_plugged_in_press_reaches_the_unlock_question_once_it_has_settled", |dir| {
                let mut rig = Rig::new(dir);
                let question = root(&mut rig);
                let locker = locked(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 0, SIZE, AREA);
                turn(&mut rig);
                rig.move_by_hand(YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), NONE, "a click reached the question before it had settled");
                assert_eq!(rig.clicked(locker), NONE, "a click on the question went to the lock instead");
                settle(&mut rig);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), [BTN_LEFT], "a real click on the settled question did not reach it");
                assert_eq!(rig.clicked(locker), NONE);

                for key in [KEY_A, KEY_ENTER, KEY_ESC] {
                    rig.key_by_hand(key);
                }
                assert_eq!(rig.pressed(locker), [KEY_A, KEY_ENTER, KEY_ESC]);
                assert_eq!(rig.pressed(question), NONE, "a key reached the question under the lock");
                assert!(rig.state.is_locked(), "the question's click unlocked the screen");
            });
        }

        #[test]
        fn no_made_up_or_unplugged_press_reaches_the_unlock_question() {
            alone("lock::tests::live::no_made_up_or_unplugged_press_reaches_the_unlock_question", |dir| {
                let mut rig = Rig::new(dir);
                let question = root(&mut rig);
                let locker = locked(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 0, SIZE, AREA);
                turn(&mut rig);
                settle(&mut rig);
                rig.move_by_hand(YES.0, YES.1);
                rig.made_up_click();
                rig.click_by_a_program();
                rig.click_through_no_door();
                assert_eq!(rig.clicked(question), NONE, "a press no plugged-in device made reached the question");

                rig.made_up_move(YES.0 + 0.01, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), NONE, "a real click after a made-up move reached the question");
                assert_eq!(rig.clicked(locker), NONE, "a press kept from the question went to the lock");

                rig.move_by_hand(YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), [BTN_LEFT]);
                assert!(rig.state.plugged.kept.is_empty(), "a release was left waiting");
            });
        }

        #[test]
        fn without_an_unlock_question_a_click_goes_to_the_lock_as_before() {
            alone("lock::tests::live::without_an_unlock_question_a_click_goes_to_the_lock_as_before", |dir| {
                let mut rig = Rig::new(dir);
                let root_other = root(&mut rig);
                let stranger = rig.join(ClientState::default());
                let locker = locked(&mut rig);
                rig.move_by_hand(YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(locker), [BTN_LEFT], "with no question the lock lost its click");

                rig.panel_with_area(root_other, "trindle-pocketkey", 0, SIZE, AREA);
                rig.panel_with_area(stranger, UNLOCK_QUESTION, 0, SIZE, AREA);
                turn(&mut rig);
                settle(&mut rig);
                rig.move_by_hand(YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(locker), [BTN_LEFT, BTN_LEFT]);
                assert_eq!(rig.clicked(root_other), NONE, "root's question of another name was pressed under the lock");
                assert_eq!(rig.clicked(stranger), NONE, "a stranger took the name and was pressed under the lock");
                let map = smithay::desktop::layer_map_for_output(&rig.output);
                assert_eq!(map.layers().filter(|l| super::super::is_unlock_question(l)).count(), 0);
            });
        }

        #[test]
        fn a_click_on_the_question_outside_its_area_goes_to_the_lock() {
            alone("lock::tests::live::a_click_on_the_question_outside_its_area_goes_to_the_lock", |dir| {
                let mut rig = Rig::new(dir);
                let question = root(&mut rig);
                let locker = locked(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 0, SIZE, AREA);
                turn(&mut rig);
                settle(&mut rig);
                for (x, y) in [ABOVE, (0.1, 0.1)] {
                    rig.move_by_hand(x, y);
                    rig.click_by_hand();
                }
                assert_eq!(rig.clicked(question), NONE, "a click outside the yes/no area reached the question");
                assert_eq!(rig.clicked(locker), [BTN_LEFT, BTN_LEFT]);
            });
        }

        #[test]
        fn unlocked_the_question_is_an_ordinary_trusted_panel() {
            alone("lock::tests::live::unlocked_the_question_is_an_ordinary_trusted_panel", |dir| {
                let mut rig = Rig::new(dir);
                let question = root(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 0, SIZE, AREA);
                rig.move_by_hand(YES.0, YES.1);
                let under = [rig.state.surface_under(rig.state.pointer.current_location()).map(|u| u.0)];
                assert_eq!(rig.state.press_on_question(&under, true), None, "the lock's path judged a press while unlocked");

                rig.click_by_hand();
                assert_eq!(rig.clicked(question), [BTN_LEFT]);
                assert!(rig.state.plugged.kept.is_empty());
            });
        }

        #[test]
        fn with_two_screens_the_question_takes_presses_only_on_its_own() {
            alone("lock::tests::live::with_two_screens_the_question_takes_presses_only_on_its_own", |dir| {
                let mut rig = Rig::new(dir);
                rig.second_screen("rig2");
                let question = root(&mut rig);
                let locker = locked(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 1, SIZE, AREA);
                turn(&mut rig);
                settle(&mut rig);

                rig.move_by_hand(YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), NONE, "the question took a click on the other screen");
                assert_eq!(rig.clicked(locker), [BTN_LEFT]);
                rig.move_by_hand_on("rig2", YES.0, YES.1);
                rig.click_by_hand();
                assert_eq!(rig.clicked(question), [BTN_LEFT], "the question on the second screen was not reached");
                assert_eq!(rig.clicked(locker), [BTN_LEFT]);
            });
        }

        #[test]
        fn a_crashed_question_gives_its_screen_back_to_the_lock() {
            alone("lock::tests::live::a_crashed_question_gives_its_screen_back_to_the_lock", |dir| {
                let mut rig = Rig::new(dir);
                let question = root(&mut rig);
                let locker = locked(&mut rig);
                rig.panel_with_area(question, UNLOCK_QUESTION, 0, SIZE, AREA);
                turn(&mut rig);
                settle(&mut rig);
                rig.move_by_hand(YES.0, YES.1);
                rig.crash(question);
                turn(&mut rig);
                rig.click_by_hand();
                assert_eq!(rig.clicked(locker), [BTN_LEFT], "the lock did not have its screen back");
                assert!(rig.state.is_locked());
            });
        }

        #[test]
        fn a_device_a_program_made_is_never_plugged_in_even_when_allowed() {
            alone("lock::tests::live::a_device_a_program_made_is_never_plugged_in_even_when_allowed", |dir| {
                let mut rig = Rig::new(dir);
                let devices = &mut rig.state.keys_changed;
                devices.arrived("event3".into(), "USB mouse".into(), false);
                devices.arrived("event9".into(), "Kernel typer".into(), true);
                assert!(devices.plugged_in("event3"));
                assert!(!devices.plugged_in("event9") && devices.not_allowed("event9"));
                assert!(!devices.plugged_in("event12"), "a device never seen counted as plugged in");
                let file = dir.join("home/.local/state/trindle/own-helpers");
                std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                use crate::own_helpers_set::allow_device as allow_by_name;
                crate::own_helpers::kept_at(&file, || {
                    allow_by_name("Kernel typer", true, true).unwrap();
                    rig.state.keys_reread();
                });
                assert!(!rig.state.keys_changed.not_allowed("event9"), "the device was not allowed by name");
                assert!(!rig.state.keys_changed.plugged_in("event9"), "allowing a made device by name made it plugged in");
            });
        }
    }

    #[test]
    fn only_console_switching_works_while_locked() {
        assert!(works_while_locked(Keysym::XF86_Switch_VT_1));
        assert!(works_while_locked(Keysym::XF86_Switch_VT_12));
        assert!(!works_while_locked(Keysym::Super_L));
        assert!(!works_while_locked(Keysym::Return));
        assert!(!works_while_locked(Keysym::Escape));
    }
}

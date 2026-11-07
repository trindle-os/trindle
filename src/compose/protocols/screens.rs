//! trindle_screens_v1 (screens.xml).

use std::sync::Mutex;

use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::protocol::wl_output::Transform as WlTurn;
use smithay::reexports::wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource};
use smithay::utils::Transform;

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/screens.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/screens.xml");
}

pub use server::trindle_screen_plan_v1::{self as plan, TrindleScreenPlanV1};
pub use server::trindle_screen_v1::{self as screen, TrindleScreenV1};
pub use server::trindle_screens_manager_v1::{self as manager, TrindleScreensManagerV1};

const VERSION: u32 = 1;

const RATE_SLACK: i32 = 1000;
const SCALE_FLOOR: u32 = 30;
const SCALE_CEILING: u32 = 1200;
const MOST_SCREENS: usize = 64;
const NO_MODE: u32 = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenMode {
    pub width: i32,
    pub height: i32,
    pub millihertz: i32,
    pub preferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenFacts {
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: Option<String>,
    pub size_mm: Option<(i32, i32)>,
    pub modes: Vec<ScreenMode>,
    pub on: bool,
    pub mode: Option<usize>,
    pub position: (i32, i32),
    pub turn: Transform,
    pub scale: f64,
    pub varies_rate: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wish {
    pub name: String,
    pub on: bool,
    pub width: i32,
    pub height: i32,
    pub millihertz: i32,
    pub position: (i32, i32),
    pub turn: u32,
    pub scale_120ths: u32,
    pub varies_rate: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenPlan {
    pub name: String,
    pub on: bool,
    pub mode: Option<usize>,
    pub position: (i32, i32),
    pub turn: Transform,
    pub scale: f64,
    pub varies_rate: bool,
}

pub trait ScreensHandler {
    fn screens_state(&mut self) -> &mut ScreensState;
    fn screens_now(&mut self) -> Vec<ScreenFacts>;
    fn arrange(&mut self, plan: &[ScreenPlan], test_only: bool) -> Result<(), String>;
}

pub fn closest_mode(modes: &[ScreenMode], width: i32, height: i32, millihertz: i32) -> Option<usize> {
    let same = modes.iter().enumerate().filter(|(_, m)| (m.width, m.height) == (width, height));
    if millihertz <= 0 {
        return same.max_by_key(|(i, m)| (m.preferred, m.millihertz, std::cmp::Reverse(*i))).map(|(i, _)| i);
    }
    same.map(|(i, m)| (i, (m.millihertz - millihertz).abs()))
        .filter(|&(_, off)| off <= RATE_SLACK)
        .min_by_key(|&(i, off)| (off, i))
        .map(|(i, _)| i)
}

pub fn unique_modes(listed: &[ScreenMode]) -> (Vec<ScreenMode>, Vec<usize>) {
    let mut kept: Vec<ScreenMode> = Vec::new();
    let mut at: Vec<usize> = Vec::new();
    for (i, mode) in listed.iter().enumerate() {
        match kept.iter().position(|k| (k.width, k.height, k.millihertz) == (mode.width, mode.height, mode.millihertz)) {
            Some(twin) if mode.preferred && !kept[twin].preferred => {
                kept[twin].preferred = true;
                at[twin] = i;
            }
            Some(_) => {}
            None => {
                kept.push(*mode);
                at.push(i);
            }
        }
    }
    (kept, at)
}

fn usual_mode(modes: &[ScreenMode]) -> Option<usize> {
    modes.iter().position(|m| m.preferred).or((!modes.is_empty()).then_some(0))
}

fn turn_from(raw: u32) -> Option<Transform> {
    Some(match raw {
        0 => Transform::Normal,
        1 => Transform::_90,
        2 => Transform::_180,
        3 => Transform::_270,
        4 => Transform::Flipped,
        5 => Transform::Flipped90,
        6 => Transform::Flipped180,
        7 => Transform::Flipped270,
        _ => return None,
    })
}

fn turn_number(turn: Transform) -> u32 {
    WlTurn::from(turn) as u32
}

pub fn resolve(now: &[ScreenFacts], wishes: &[Wish]) -> Result<Vec<ScreenPlan>, String> {
    for (i, wish) in wishes.iter().enumerate() {
        if wishes[..i].iter().any(|w| w.name == wish.name) {
            return Err(format!("{} is described twice", wish.name));
        }
    }
    let mut out = Vec::with_capacity(now.len());
    for facts in now {
        let Some(wish) = wishes.iter().find(|w| w.name == facts.name) else {
            return Err(format!("{} was left out", facts.name));
        };
        if !wish.on {
            out.push(ScreenPlan {
                name: facts.name.clone(),
                on: false,
                mode: None,
                position: facts.position,
                turn: facts.turn,
                scale: facts.scale,
                varies_rate: facts.varies_rate,
            });
            continue;
        }
        let no = |why: String| format!("{}: {why}", facts.name);
        let mode = if wish.width > 0 && wish.height > 0 {
            closest_mode(&facts.modes, wish.width, wish.height, wish.millihertz).ok_or_else(|| {
                no(format!("it cannot run at {}x{}@{}mHz", wish.width, wish.height, wish.millihertz))
            })?
        } else {
            facts
                .mode
                .filter(|&i| facts.on && i < facts.modes.len())
                .or_else(|| usual_mode(&facts.modes))
                .ok_or_else(|| no("it lists no modes".into()))?
        };
        if !(SCALE_FLOOR..=SCALE_CEILING).contains(&wish.scale_120ths) {
            return Err(no(format!("a scale of {}/120 is out of reach", wish.scale_120ths)));
        }
        let turn = turn_from(wish.turn).ok_or_else(|| no(format!("there is no way up numbered {}", wish.turn)))?;
        out.push(ScreenPlan {
            name: facts.name.clone(),
            on: true,
            mode: Some(mode),
            position: wish.position,
            turn,
            scale: f64::from(wish.scale_120ths) / 120.0,
            varies_rate: wish.varies_rate,
        });
    }
    if let Some(stray) = wishes.iter().find(|w| !now.iter().any(|f| f.name == w.name)) {
        return Err(format!("{} is not plugged in", stray.name));
    }
    if !out.iter().any(|p| p.on) {
        return Err("that would leave every screen off".into());
    }
    Ok(out)
}

pub struct ScreenKey(String);

pub struct PlanData {
    serial: u32,
    wishes: Mutex<Vec<Wish>>,
    used: Mutex<bool>,
}

pub struct ScreensGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Debug)]
struct Shown {
    object: TrindleScreenV1,
    told: ScreenFacts,
}

#[derive(Debug)]
struct Listener {
    manager: TrindleScreensManagerV1,
    shown: Vec<Shown>,
}

#[derive(Debug)]
pub struct ScreensState {
    display: DisplayHandle,
    serial: u32,
    now: Vec<ScreenFacts>,
    listeners: Vec<Listener>,
}

impl ScreensState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> ScreensState
    where
        D: GlobalDispatch<TrindleScreensManagerV1, ScreensGlobalData>
            + Dispatch<TrindleScreensManagerV1, ()>
            + Dispatch<TrindleScreenV1, ScreenKey>
            + Dispatch<TrindleScreenPlanV1, PlanData>
            + ScreensHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleScreensManagerV1, _>(VERSION, ScreensGlobalData { filter: Box::new(filter) });
        ScreensState { display: display.clone(), serial: 1, now: Vec::new(), listeners: Vec::new() }
    }

    fn tell<D>(&mut self, at: usize)
    where
        D: Dispatch<TrindleScreenV1, ScreenKey> + 'static,
    {
        let Self { display, serial, now, listeners } = self;
        let listener = &mut listeners[at];
        let Some(client) = listener.manager.client() else { return };
        let (still, gone): (Vec<Shown>, Vec<Shown>) =
            std::mem::take(&mut listener.shown).into_iter().partition(|s| now.iter().any(|f| f.name == s.told.name));
        listener.shown = still;
        for unplugged in gone.iter().filter(|s| s.object.is_alive()) {
            unplugged.object.unplugged();
        }
        for facts in now.iter() {
            match listener.shown.iter_mut().find(|s| s.told.name == facts.name) {
                Some(shown) if shown.told == *facts => {}
                Some(shown) => {
                    if shown.object.is_alive() {
                        shown.object.clear();
                        describe(&shown.object, facts);
                    }
                    shown.told = facts.clone();
                }
                None => {
                    let Ok(object) = client.create_resource::<TrindleScreenV1, ScreenKey, D>(
                        display,
                        listener.manager.version(),
                        ScreenKey(facts.name.clone()),
                    ) else {
                        continue;
                    };
                    listener.manager.screen(&object);
                    describe(&object, facts);
                    listener.shown.push(Shown { object, told: facts.clone() });
                }
            }
        }
        listener.manager.settled(*serial);
    }
}

fn describe(object: &TrindleScreenV1, f: &ScreenFacts) {
    object.identity(
        f.name.clone(),
        f.description.clone(),
        f.make.clone(),
        f.model.clone(),
        f.serial.clone().unwrap_or_default(),
    );
    let (w, h) = f.size_mm.unwrap_or((0, 0));
    object.size_mm(w, h);
    for m in &f.modes {
        object.mode(m.width, m.height, m.millihertz, u32::from(m.preferred));
    }
    object.layout(
        u32::from(f.on),
        f.mode.filter(|_| f.on).map_or(NO_MODE, |i| i as u32),
        f.position.0,
        f.position.1,
        turn_number(f.turn),
        (f.scale * 120.0).round() as u32,
        u32::from(f.varies_rate),
    );
}

pub fn refresh<D>(state: &mut D)
where
    D: ScreensHandler + Dispatch<TrindleScreenV1, ScreenKey> + 'static,
{
    let facts = state.screens_now();
    let st = state.screens_state();
    st.listeners.retain(|l| l.manager.is_alive());
    if st.now == facts {
        return;
    }
    st.now = facts;
    st.serial = st.serial.wrapping_add(1);
    for at in 0..st.listeners.len() {
        st.tell::<D>(at);
    }
}

impl<D> GlobalDispatch<TrindleScreensManagerV1, ScreensGlobalData, D> for ScreensState
where
    D: GlobalDispatch<TrindleScreensManagerV1, ScreensGlobalData>
        + Dispatch<TrindleScreensManagerV1, ()>
        + Dispatch<TrindleScreenV1, ScreenKey>
        + Dispatch<TrindleScreenPlanV1, PlanData>
        + ScreensHandler
        + 'static,
{
    fn bind(
        state: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleScreensManagerV1>,
        _: &ScreensGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        let manager = data_init.init(resource, ());
        let facts = state.screens_now();
        let st = state.screens_state();
        if st.now != facts {
            st.now = facts;
            st.serial = st.serial.wrapping_add(1);
            for at in 0..st.listeners.len() {
                st.tell::<D>(at);
            }
        }
        st.listeners.push(Listener { manager, shown: Vec::new() });
        let last = st.listeners.len() - 1;
        st.tell::<D>(last);
    }

    fn can_view(client: Client, data: &ScreensGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleScreensManagerV1, (), D> for ScreensState
where
    D: Dispatch<TrindleScreensManagerV1, ()>
        + Dispatch<TrindleScreenV1, ScreenKey>
        + Dispatch<TrindleScreenPlanV1, PlanData>
        + ScreensHandler
        + 'static,
{
    fn request(
        _: &mut D,
        _: &Client,
        _: &TrindleScreensManagerV1,
        request: manager::Request,
        _: &(),
        _: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            manager::Request::Destroy => {}
            manager::Request::Plan { id, serial } => {
                data_init.init(id, PlanData { serial, wishes: Mutex::new(Vec::new()), used: Mutex::new(false) });
            }
        }
    }

    fn destroyed(state: &mut D, _: ClientId, manager: &TrindleScreensManagerV1, _: &()) {
        state.screens_state().listeners.retain(|l| &l.manager != manager);
    }
}

impl<D> Dispatch<TrindleScreenV1, ScreenKey, D> for ScreensState
where
    D: Dispatch<TrindleScreenV1, ScreenKey> + ScreensHandler + 'static,
{
    fn request(
        _: &mut D,
        _: &Client,
        _: &TrindleScreenV1,
        request: screen::Request,
        _: &ScreenKey,
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            screen::Request::Destroy => {}
        }
    }
}

impl<D> Dispatch<TrindleScreenPlanV1, PlanData, D> for ScreensState
where
    D: Dispatch<TrindleScreenPlanV1, PlanData> + ScreensHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        object: &TrindleScreenPlanV1,
        request: plan::Request,
        data: &PlanData,
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            plan::Request::Destroy => {}
            plan::Request::Screen { screen, on, width, height, millihertz, x, y, turn, scale_120ths, varies_rate } => {
                let Some(ScreenKey(name)) = screen.data::<ScreenKey>() else { return };
                let mut wishes = data.wishes.lock().unwrap_or_else(|e| e.into_inner());
                if wishes.len() < MOST_SCREENS {
                    wishes.push(Wish {
                        name: name.clone(),
                        on: on != 0,
                        width,
                        height,
                        millihertz,
                        position: (x, y),
                        turn,
                        scale_120ths,
                        varies_rate: varies_rate != 0,
                    });
                }
            }
            plan::Request::Apply | plan::Request::Check => {
                let test_only = matches!(request, plan::Request::Check);
                if !test_only {
                    let mut used = data.used.lock().unwrap_or_else(|e| e.into_inner());
                    if std::mem::replace(&mut *used, true) {
                        object.post_error(0u32, "a plan can be applied only once");
                        return;
                    }
                }
                let (outcome, why) = answer(state, data, test_only);
                object.outcome(outcome, why);
            }
        }
    }
}

fn answer<D: ScreensHandler>(state: &mut D, data: &PlanData, test_only: bool) -> (plan::Outcome, String) {
    if state.screens_state().serial != data.serial {
        return (plan::Outcome::Stale, "the screens changed first".into());
    }
    let wishes = data.wishes.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let now = state.screens_state().now.clone();
    match resolve(&now, &wishes).and_then(|exact| state.arrange(&exact, test_only)) {
        Ok(()) => (plan::Outcome::Applied, String::new()),
        Err(why) => (plan::Outcome::Refused, why),
    }
}

#[macro_export]
macro_rules! delegate_screens {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::screens::TrindleScreensManagerV1: $crate::protocols::screens::ScreensGlobalData
        ] => $crate::protocols::screens::ScreensState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::screens::TrindleScreensManagerV1: ()
        ] => $crate::protocols::screens::ScreensState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::screens::TrindleScreenV1: $crate::protocols::screens::ScreenKey
        ] => $crate::protocols::screens::ScreensState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::screens::TrindleScreenPlanV1: $crate::protocols::screens::PlanData
        ] => $crate::protocols::screens::ScreensState);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(width: i32, height: i32, millihertz: i32, preferred: bool) -> ScreenMode {
        ScreenMode { width, height, millihertz, preferred }
    }

    fn screen(name: &str, on: bool) -> ScreenFacts {
        ScreenFacts {
            name: name.into(),
            description: String::new(),
            make: String::new(),
            model: String::new(),
            serial: None,
            size_mm: None,
            modes: vec![
                mode(1920, 1080, 60000, true),
                mode(1920, 1080, 59940, false),
                mode(1920, 1080, 50000, false),
                mode(1280, 720, 60000, false),
            ],
            on,
            mode: on.then_some(0),
            position: (0, 0),
            turn: Transform::Normal,
            scale: 1.0,
            varies_rate: false,
        }
    }

    fn keep(f: &ScreenFacts) -> Wish {
        Wish {
            name: f.name.clone(),
            on: true,
            width: 0,
            height: 0,
            millihertz: 0,
            position: f.position,
            turn: turn_number(f.turn),
            scale_120ths: (f.scale * 120.0).round() as u32,
            varies_rate: f.varies_rate,
        }
    }

    fn off(f: &ScreenFacts) -> Wish {
        Wish { on: false, ..keep(f) }
    }

    #[test]
    fn a_rate_picks_the_nearest_listed_mode_within_a_hertz() {
        let modes = screen("A", true).modes;
        assert_eq!(closest_mode(&modes, 1920, 1080, 60000), Some(0));
        assert_eq!(closest_mode(&modes, 1920, 1080, 59940), Some(1));
        assert_eq!(closest_mode(&modes, 1920, 1080, 59950), Some(1));
        assert_eq!(closest_mode(&modes, 1920, 1080, 50000), Some(2));
        assert_eq!(closest_mode(&modes, 1920, 1080, 75000), None);
        assert_eq!(closest_mode(&modes, 1280, 720, 60000), Some(3));
        assert_eq!(closest_mode(&modes, 800, 600, 60000), None);
    }

    #[test]
    fn a_saved_rate_finds_its_mode() {
        let modes = [mode(1920, 1080, 60000, true), mode(1920, 1080, 59940, false), mode(1920, 1080, 144000, false),
                     mode(1280, 720, 60000, false)];
        let at = |rate| closest_mode(&modes, 1920, 1080, rate);
        assert_eq!(at(59940), Some(1));
        assert_eq!(at(60000), Some(0));
        assert_eq!(at(143800), Some(2));
        assert_eq!(at(50000), None);
        assert_eq!(at(0), Some(0), "no rate: the one it prefers");
        assert_eq!(closest_mode(&modes, 1280, 720, 0), Some(3));
    }

    #[test]
    fn no_rate_means_the_preferred_mode_or_else_the_fastest() {
        let modes = screen("A", true).modes;
        assert_eq!(closest_mode(&modes, 1920, 1080, 0), Some(0));
        assert_eq!(closest_mode(&modes, 1280, 720, 0), Some(3));
        let plain = [mode(800, 600, 56000, false), mode(800, 600, 72000, false)];
        assert_eq!(closest_mode(&plain, 800, 600, 0), Some(1));
    }

    #[test]
    fn modes_listed_twice_are_kept_once() {
        let listed = [
            mode(1920, 1080, 60000, false),
            mode(1920, 1080, 60000, true),
            mode(1280, 720, 60000, false),
            mode(1920, 1080, 60000, false),
        ];
        let (kept, at) = unique_modes(&listed);
        assert_eq!(kept, vec![mode(1920, 1080, 60000, true), mode(1280, 720, 60000, false)]);
        assert_eq!(at, vec![1, 2]);
    }

    #[test]
    fn a_screen_kept_as_it_is_keeps_its_mode_and_place() {
        let mut a = screen("A", true);
        a.position = (10, 20);
        a.scale = 1.5;
        a.mode = Some(1);
        let got = resolve(&[a.clone()], &[keep(&a)]).unwrap();
        assert_eq!(got[0].mode, Some(1));
        assert_eq!(got[0].position, (10, 20));
        assert_eq!(got[0].scale, 1.5);
    }

    #[test]
    fn a_screen_switched_on_gets_its_preferred_mode() {
        let (a, b) = (screen("A", true), screen("B", false));
        let got = resolve(&[a.clone(), b.clone()], &[keep(&a), keep(&b)]).unwrap();
        assert_eq!(got[1].mode, Some(0));
        assert!(got[1].on);
    }

    #[test]
    fn a_size_picks_a_listed_mode_and_the_scale_is_exact() {
        let (a, b) = (screen("A", true), screen("B", true));
        let wish = Wish { width: 1280, height: 720, millihertz: 60000, scale_120ths: 240, ..keep(&a) };
        let got = resolve(&[a, b.clone()], &[wish, keep(&b)]).unwrap();
        assert_eq!(got[0].mode, Some(3));
        assert_eq!(got[0].scale, 2.0);
    }

    #[test]
    fn a_plan_that_cannot_be_used_says_why() {
        let (a, b) = (screen("A", true), screen("B", true));
        let now = [a.clone(), b.clone()];
        assert!(resolve(&now, &[keep(&a)]).unwrap_err().contains("B was left out"));
        assert!(resolve(&now, &[keep(&a), keep(&a), keep(&b)]).unwrap_err().contains("twice"));
        assert!(resolve(&now, &[off(&a), off(&b)]).unwrap_err().contains("every screen off"));
        assert!(resolve(&now, &[keep(&a), off(&b)]).is_ok());
        let stray = Wish { name: "C".into(), ..keep(&a) };
        assert!(resolve(&now, &[keep(&a), keep(&b), stray]).unwrap_err().contains("not plugged in"));
        let missing_mode = Wish { width: 800, height: 600, millihertz: 60000, ..keep(&a) };
        assert!(resolve(&now, &[missing_mode, keep(&b)]).unwrap_err().contains("cannot run at 800x600"));
        for scale_120ths in [0, 29, 1201, u32::MAX] {
            let wild = Wish { scale_120ths, ..keep(&a) };
            assert!(resolve(&now, &[wild, keep(&b)]).unwrap_err().contains("out of reach"), "{scale_120ths}");
        }
        let sideways = Wish { turn: 8, ..keep(&a) };
        assert!(resolve(&now, &[sideways, keep(&b)]).unwrap_err().contains("no way up"));
    }

    #[test]
    fn every_turn_has_a_number_and_comes_back() {
        for n in 0..8 {
            assert_eq!(turn_number(turn_from(n).unwrap()), n);
        }
        assert_eq!(turn_from(8), None);
    }
}

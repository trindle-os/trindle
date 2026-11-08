//! Not a back door: Pocketkey is an optional plugin the person downloads that lets a phone they paired say yes instead of typing the password. This does nothing unless that plugin is installed and switched on, only root can use it, and the typed password always works.

use std::cell::Cell;
use std::os::{fd::AsRawFd, unix::net::UnixStream};
use std::time::{Duration, Instant};

use smithay::{
    desktop::{LayerMap, LayerSurface},
    output::Output,
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    wayland::compositor::with_states,
    utils::{Logical, Point},
    wayland::shell::wlr_layer::Layer,
};

use crate::state::ClientState;

pub fn from_root(stream: &UnixStream) -> bool {
    let mut cred = libc::ucred { pid: 0, uid: u32::MAX, gid: u32::MAX };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: the fd is an open socket for the call, and cred and len are

    let rc = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut cred as *mut libc::ucred).cast(),
            &mut len,
        )
    };
    rc == 0 && len as usize == std::mem::size_of::<libc::ucred>() && cred.uid == 0
}

pub fn is_trusted(surface: &WlSurface) -> bool {
    surface
        .client()
        .is_some_and(|c| c.get_data::<ClientState>().is_some_and(|s| s.from_root))
}

pub fn overlays(map: &LayerMap) -> impl DoubleEndedIterator<Item = &LayerSurface> {
    map.layers_on(Layer::Overlay).filter(|l| is_trusted(l.wl_surface()))
}

pub fn overlay_under(map: &LayerMap, at: Point<f64, Logical>) -> Option<&LayerSurface> {
    crate::shell::layer_taking_input(map, &[Layer::Overlay], at, |l| is_trusted(l.wl_surface()))
}

pub fn is_ours(surface: &WlSurface) -> bool {
    surface
        .client()
        .is_some_and(|c| c.get_data::<ClientState>().is_some_and(|s| s.ours))
}

const RING: &str = crate::person_only::RING;

pub const ASKED_FOR: Duration = Duration::from_secs(4);

#[derive(Default)]
struct Out {
    showing: Cell<bool>,
    asked: Cell<bool>,
}

fn asked_after(showing: bool, asked: bool, shows: bool, just_asked: bool) -> bool {
    match (showing, shows) {
        (_, false) => false,
        (true, true) => asked,
        (false, true) => just_asked,
    }
}

pub fn ours_committed(surface: &WlSurface, shows: bool, wished: Option<Instant>, now: Instant) -> bool {
    if !is_ours(surface) {
        return false;
    }
    let just_asked = wished.is_some_and(|w| now.saturating_duration_since(w) < ASKED_FOR);
    with_states(surface, |states| {
        let out = states.data_map.get_or_insert(Out::default);
        let was = out.showing.get();
        out.asked.set(asked_after(was, out.asked.get(), shows, just_asked));
        out.showing.set(shows);
        was != shows
    })
}

fn asked_for(surface: &WlSurface) -> bool {
    with_states(surface, |states| states.data_map.get::<Out>().is_some_and(|o| o.asked.get()))
}

pub fn bring_out_unseen(output: &Output, kept: impl Fn(&WlSurface) -> bool) -> bool {
    let full = output.user_data().get::<crate::shell::FullscreenSurface>().and_then(|f| f.get()).is_some();
    if !full {
        return false;
    }
    let map = smithay::desktop::layer_map_for_output(output);
    let mut brought = false;
    for layer in map.layers().filter(|l| l.namespace() == RING && is_ours(l.wl_surface()) && kept(l.wl_surface())) {
        with_states(layer.wl_surface(), |states| {
            if let Some(out) = states.data_map.get::<Out>().filter(|o| o.showing.get() && !o.asked.get()) {
                out.asked.set(true);
                brought = true;
            }
        });
    }
    brought
}

fn is_ring(layer: &LayerSurface) -> bool {
    layer.namespace() == RING && is_ours(layer.wl_surface()) && asked_for(layer.wl_surface())
}

const SHOT: &str = "trindle-screenshot";

fn is_shot(layer: &LayerSurface) -> bool {
    layer.namespace() == SHOT
        && layer.wl_surface().client().is_some_and(|c| c.get_data::<ClientState>().is_some_and(|s| s.asked))
}

fn is_asked_for(layer: &LayerSurface) -> bool {
    is_ring(layer) || is_shot(layer)
}

fn shows_over_full_screen(layer: &LayerSurface) -> bool {
    is_trusted(layer.wl_surface()) || is_asked_for(layer)
}

pub fn hidden_by_full_screen(output: &Output, layer: &LayerSurface) -> bool {
    let full = output.user_data().get::<crate::shell::FullscreenSurface>().and_then(|f| f.get()).is_some();
    full && !shows_over_full_screen(layer)
}

pub fn over_full_screen(map: &LayerMap) -> impl DoubleEndedIterator<Item = &LayerSurface> {
    map.layers_on(Layer::Overlay).filter(|l| is_asked_for(l)).chain(overlays(map))
}

pub fn over_full_screen_under(map: &LayerMap, at: Point<f64, Logical>, full: bool) -> Option<&LayerSurface> {
    overlay_under(map, at).or_else(|| {
        if !full {
            return None;
        }
        crate::shell::layer_taking_input(map, &[Layer::Overlay], at, is_asked_for)
    })
}

const NEWS: &str = "trindle-notes";

pub const QUESTION: &str = crate::person_only::PASSWORD;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {

    Ordinary,

    Question,

    News,

    Trusted,
}

pub(crate) fn kind_of(layer: &LayerSurface) -> Kind {
    let surface = layer.wl_surface();
    if is_trusted(surface) {
        Kind::Trusted
    } else if layer.namespace() == NEWS && is_ours(surface) {
        Kind::News
    } else if layer.namespace() == QUESTION && is_ours(surface) {
        Kind::Question
    } else {
        Kind::Ordinary
    }
}

pub fn standing(layer: &LayerSurface) -> Kind {
    kind_of(layer)
}

pub const MODAL: &str = "trindle-modal";

pub const SMALLEST: i32 = 16;

pub fn too_small(layer: &LayerSurface) -> bool {
    let size = layer.bbox().size;
    is_trusted(layer.wl_surface()) && (size.w < SMALLEST || size.h < SMALLEST)
}

pub fn counts(layer: &LayerSurface) -> bool {
    is_trusted(layer.wl_surface()) && !too_small(layer)
}

pub fn asks(layer: &LayerSurface) -> bool {
    let surface = layer.wl_surface();
    let name = layer.namespace();
    counts(layer)
        || crate::person_only::ASKED_BY_NAME.contains(&name)
        || (crate::person_only::ASKED_BY_TRINDLE.contains(&name) && is_ours(surface)
            && with_states(surface, |states| states.data_map.get::<Out>().is_some_and(|o| o.showing.get())))
}

pub fn panel_kept_for_the_person(layer: &LayerSurface) -> bool {
    asks(layer) || (crate::person_only::PANELS_BY_NAME.contains(&layer.namespace()) && !is_ours(layer.wl_surface()))
}

pub fn window_kept_for_the_person(toplevel: &WlSurface) -> bool {
    use smithay::wayland::shell::xdg::XdgToplevelSurfaceData;
    outside_every_box(toplevel)
        && with_states(toplevel, |states| {
            states.data_map.get::<XdgToplevelSurfaceData>().and_then(|data| data.lock().ok().and_then(|d| d.app_id.clone()))
        })
        .is_some_and(|id| crate::person_only::WINDOWS.contains(&id.as_str()))
}

pub fn outside_every_box(surface: &WlSurface) -> bool {
    surface.client().is_some_and(|c| c.get_data::<ClientState>().is_some_and(|s| s.security_context.is_none()))
}

pub fn is_ring_panel(layer: &LayerSurface) -> bool {
    layer.namespace() == RING && is_ours(layer.wl_surface())
}

fn in_order(panels: &[Kind]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..panels.len()).collect();
    order.sort_by_key(|i| panels[*i]);
    order
}

pub fn keep_on_top(map: &mut LayerMap) {
    let layers: Vec<LayerSurface> = map.layers().cloned().collect();
    let panels: Vec<Kind> = layers.iter().map(kind_of).collect();
    let order = in_order(&panels);
    let Some(first) = (0..order.len()).find(|i| order[*i] != *i) else { return };
    for i in &order[first..] {
        map.unmap_layer(&layers[*i]);
    }
    for i in &order[first..] {
        let _ = map.map_layer(&layers[*i]);
    }
}

#[cfg(test)]
mod tests {
    use super::Kind::{News, Ordinary, Trusted};
    use super::*;

    fn after(panels: &[Kind]) -> Vec<Kind> {
        in_order(panels).into_iter().map(|i| panels[i]).collect()
    }

    #[test]
    fn news_goes_above_a_panel_opened_after_it() {

        assert_eq!(after(&[News, Ordinary]), [Ordinary, News]);

        assert_eq!(after(&[News, News, News, Ordinary, Ordinary]), [Ordinary, Ordinary, News, News, News]);

        assert_eq!(in_order(&[News, Ordinary, News, Ordinary]), [1, 3, 0, 2]);
    }

    #[test]
    fn the_password_question_goes_above_the_ring_and_under_the_news_whichever_opened_first() {
        use super::Kind::Question;
        assert_eq!(after(&[Question, Ordinary]), [Ordinary, Question]);
        assert_eq!(after(&[Ordinary, Question]), [Ordinary, Question]);
        assert_eq!(after(&[News, Question, Trusted, Ordinary, Question]), [Ordinary, Question, Question, News, Trusted]);
        assert!(Ordinary < Question && Question < News && News < Trusted);

        assert!(crate::source::code(include_str!("../polkit/mod.rs")).contains(&format!("name: \"{QUESTION}\".into()")));
    }

    #[test]
    fn trusted_panels_stay_above_everything() {
        assert_eq!(after(&[Trusted, News, Ordinary]), [Ordinary, News, Trusted]);
        assert_eq!(after(&[Ordinary, Trusted, News]), [Ordinary, News, Trusted]);

        assert_eq!(in_order(&[Trusted, Ordinary, Trusted]), [1, 0, 2]);
    }

    #[test]
    fn panels_already_in_order_are_left_alone() {
        assert!(in_order(&[]).is_empty());
        assert_eq!(in_order(&[Ordinary, Ordinary, News, Trusted]), [0, 1, 2, 3]);
        assert_eq!(in_order(&[Ordinary, Ordinary]), [0, 1]);
    }

    #[test]
    fn every_arrangement_is_sorted_once_and_for_all() {

        let kinds = [Ordinary, News, Trusted];
        for n in 1..=7u32 {
            for code in 0..3u32.pow(n) {
                let now: Vec<Kind> = (0..n).map(|i| kinds[((code / 3u32.pow(i)) % 3) as usize]).collect();
                let order = in_order(&now);
                let mut seen = order.clone();
                seen.sort_unstable();
                assert_eq!(seen, (0..now.len()).collect::<Vec<_>>(), "{now:?}: every panel once");
                let got = after(&now);
                assert!(got.windows(2).all(|w| w[0] <= w[1]), "{now:?} became {got:?}");

                for kind in kinds {
                    let places: Vec<usize> = order.iter().copied().filter(|i| now[*i] == kind).collect();
                    assert!(places.windows(2).all(|w| w[0] < w[1]), "{now:?}: {kind:?} out of order");
                }

                assert_eq!(in_order(&got), (0..got.len()).collect::<Vec<_>>(), "{now:?} became {got:?}");
            }
        }
    }

    mod live {
        use std::os::unix::net::UnixStream;
        use std::sync::Arc;

        use smithay::desktop::{layer_map_for_output, LayerSurface};
        use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
        use smithay::reexports::wayland_server::backend::ClientId;
        use smithay::reexports::wayland_server::protocol::{wl_output::WlOutput, wl_surface::WlSurface};
        use smithay::reexports::wayland_server::{Client, Display, Resource};
        use smithay::wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState};
        use smithay::wayland::output::OutputHandler;
        use smithay::wayland::shell::wlr_layer::{Layer, LayerSurface as WlrLayerSurface, WlrLayerShellHandler, WlrLayerShellState};
        use smithay::{delegate_compositor, delegate_layer_shell, delegate_output};
        use wayland_client::protocol::{wl_compositor, wl_registry, wl_surface};
        use wayland_client::{delegate_noop, Connection, Dispatch, EventQueue, QueueHandle};
        use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::{Layer as On, ZwlrLayerShellV1};
        use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;

        use crate::state::ClientState;

        struct Server {
            compositor: CompositorState,
            shell: WlrLayerShellState,
            output: Output,

            wished: Option<std::time::Instant>,
        }

        impl CompositorHandler for Server {
            fn compositor_state(&mut self) -> &mut CompositorState {
                &mut self.compositor
            }
            fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
                &client.get_data::<ClientState>().unwrap().compositor_state
            }

            fn commit(&mut self, surface: &WlSurface) {
                super::super::ours_committed(surface, true, self.wished, std::time::Instant::now());
            }
        }
        delegate_compositor!(Server);

        impl OutputHandler for Server {}
        delegate_output!(Server);

        impl WlrLayerShellHandler for Server {
            fn shell_state(&mut self) -> &mut WlrLayerShellState {
                &mut self.shell
            }

            fn new_layer_surface(&mut self, surface: WlrLayerSurface, _: Option<WlOutput>, _: Layer, namespace: String) {
                let mut map = layer_map_for_output(&self.output);
                map.map_layer(&LayerSurface::new(surface, namespace)).unwrap();
                super::super::keep_on_top(&mut map);
            }
        }
        delegate_layer_shell!(Server);

        #[derive(Default)]
        struct Program {
            compositor: Option<wl_compositor::WlCompositor>,
            shell: Option<ZwlrLayerShellV1>,
        }

        impl Dispatch<wl_registry::WlRegistry, ()> for Program {
            fn event(p: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
                if let wl_registry::Event::Global { name, interface, version } = event {
                    match interface.as_str() {
                        "wl_compositor" => p.compositor = Some(registry.bind(name, version, qh, ())),
                        "zwlr_layer_shell_v1" => p.shell = Some(registry.bind(name, version, qh, ())),
                        _ => {}
                    }
                }
            }
        }
        delegate_noop!(Program: ignore wl_compositor::WlCompositor);
        delegate_noop!(Program: ignore wl_surface::WlSurface);
        delegate_noop!(Program: ignore ZwlrLayerShellV1);
        delegate_noop!(Program: ignore ZwlrLayerSurfaceV1);

        struct Joined {
            id: ClientId,
            conn: Connection,
            queue: EventQueue<Program>,
            program: Program,
        }

        struct Desk {
            display: Display<Server>,
            server: Server,
            programs: Vec<Joined>,
        }

        impl Desk {
            fn new() -> Desk {
                let display = Display::<Server>::new().unwrap();
                let dh = display.handle();
                let output = Output::new(
                    "test".into(),
                    PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
                );
                output.change_current_state(Some(Mode { size: (1280, 800).into(), refresh: crate::render::ASSUMED_REFRESH }), None, None, None);
                output.create_global::<Server>(&dh);
                let server = Server { compositor: CompositorState::new::<Server>(&dh), shell: WlrLayerShellState::new::<Server>(&dh), output, wished: None };
                Desk { display, server, programs: Vec::new() }
            }

            fn wish(&mut self) {
                self.server.wished = Some(std::time::Instant::now());
            }

            fn wish_long_ago(&mut self) {
                self.server.wished = std::time::Instant::now().checked_sub(super::super::ASKED_FOR * 2);
                assert!(self.server.wished.is_some());
            }

            fn join(&mut self, from_root: bool) -> usize {
                self.join_as(from_root, false)
            }

            fn join_ours(&mut self) -> usize {
                self.join_as(false, true)
            }

            fn join_asked(&mut self) -> usize {
                self.join_with(ClientState { asked: true, ..ClientState::default() })
            }

            fn join_as(&mut self, from_root: bool, ours: bool) -> usize {
                self.join_with(ClientState { from_root, ours, ..ClientState::default() })
            }

            fn join_with(&mut self, state: ClientState) -> usize {
                let (server_end, client_end) = UnixStream::pair().unwrap();
                let id = self.display.handle().insert_client(server_end, Arc::new(state)).unwrap().id();
                let conn = Connection::from_socket(client_end).unwrap();
                let queue = conn.new_event_queue();
                conn.display().get_registry(&queue.handle(), ());
                self.programs.push(Joined { id, conn, queue, program: Program::default() });
                self.pump();
                self.pump();
                self.programs.len() - 1
            }

            fn pump(&mut self) {
                for _ in 0..4 {
                    for p in &mut self.programs {
                        p.conn.flush().unwrap();
                    }
                    self.display.dispatch_clients(&mut self.server).unwrap();
                    self.display.flush_clients().unwrap();
                    for p in &mut self.programs {
                        if let Some(guard) = p.conn.prepare_read() {
                            // Nothing waiting is not an error here.
                            let _ = guard.read();
                        }
                        p.queue.dispatch_pending(&mut p.program).unwrap();
                    }
                }
            }

            fn panel(&mut self, who: usize, name: &str, layer: On) {
                let p = &self.programs[who];
                let qh = p.queue.handle();
                let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
                let made = p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, None, layer, name.into(), &qh, ());

                made.set_size(200, 40);
                surface.commit();
                self.pump();
            }

            fn order(&self) -> Vec<(usize, String)> {
                let map = layer_map_for_output(&self.server.output);
                let whose = |l: &LayerSurface| {
                    let id = l.wl_surface().client().map(|c| c.id());
                    self.programs.iter().position(|p| Some(&p.id) == id.as_ref()).unwrap()
                };
                map.layers().map(|l| (whose(l), l.namespace().to_string())).collect()
            }
        }

        fn named(order: &[(usize, &str)]) -> Vec<(usize, String)> {
            order.iter().map(|(who, name)| (*who, name.to_string())).collect()
        }

        #[test]
        fn the_rings_news_stays_above_the_rings_own_panels() {
            let mut desk = Desk::new();
            let ring = desk.join_ours();

            desk.panel(ring, super::super::NEWS, On::Overlay);
            desk.panel(ring, "trindle-notice", On::Overlay);
            desk.panel(ring, "trindle-ring", On::Overlay);
            assert_eq!(desk.order(), named(&[(ring, "trindle-notice"), (ring, "trindle-ring"), (ring, super::super::NEWS)]));

            desk.panel(ring, "trindle-battery", On::Top);
            desk.panel(ring, super::super::NEWS, On::Overlay);
            let got = desk.order();
            assert_eq!(got.iter().map(|p| p.1.as_str()).collect::<Vec<_>>(),
                       ["trindle-notice", "trindle-ring", "trindle-battery", super::super::NEWS, super::super::NEWS]);
        }

        #[test]
        fn a_stranger_taking_the_name_is_put_above_nobody_elses_panel() {
            let mut desk = Desk::new();
            let ring = desk.join_ours();
            let stranger = desk.join(false);
            desk.panel(ring, "trindle-notice", On::Overlay);

            desk.panel(stranger, super::super::NEWS, On::Overlay);
            assert_eq!(desk.order(), named(&[(ring, "trindle-notice"), (stranger, super::super::NEWS)]));

            desk.panel(ring, "trindle-ring", On::Overlay);
            let want = named(&[(ring, "trindle-notice"), (stranger, super::super::NEWS), (ring, "trindle-ring")]);
            assert_eq!(desk.order(), want);

            desk.panel(stranger, "anything", On::Overlay);
            desk.panel(ring, "trindle-label", On::Overlay);
            let got = desk.order();
            assert_eq!(got.iter().map(|p| p.0).collect::<Vec<_>>(), [ring, stranger, ring, stranger, ring], "each place stays with its program");
            assert_eq!(got.last(), Some(&(ring, "trindle-label".to_string())));

            desk.panel(ring, super::super::NEWS, On::Overlay);
            desk.panel(ring, "trindle-magnify", On::Overlay);
            let got = desk.order();
            assert_eq!(got.iter().map(|p| p.0).collect::<Vec<_>>(), [ring, stranger, ring, stranger, ring, ring, ring]);
            assert_eq!(got.last(), Some(&(ring, super::super::NEWS.to_string())));
        }

        #[test]
        fn the_screenshot_the_person_asked_for_by_its_key_is_over_a_full_screen_window_and_no_other() {
            let mut desk = Desk::new();
            let over = |desk: &Desk| -> Vec<(usize, String)> {
                let map = layer_map_for_output(&desk.server.output);
                let whose = |l: &LayerSurface| {
                    let id = l.wl_surface().client().map(|c| c.id());
                    desk.programs.iter().position(|p| Some(&p.id) == id.as_ref()).unwrap()
                };
                let over: Vec<_> = super::super::over_full_screen(&map).map(|l| (whose(l), l.namespace().to_string())).collect();
                assert_eq!(map.layers().filter(|l| super::super::shows_over_full_screen(l)).count(), over.len());
                over
            };

            let by_command = desk.join(false);
            desk.panel(by_command, "trindle-screenshot", On::Overlay);
            assert_eq!(over(&desk), []);

            let ring = desk.join_ours();
            desk.wish();
            desk.panel(ring, "trindle-screenshot", On::Overlay);
            assert_eq!(over(&desk), []);

            let tool = desk.join_asked();
            desk.panel(tool, "trindle-screenshot-pin", On::Overlay);
            desk.panel(tool, "trindle-ring", On::Overlay);
            desk.panel(tool, "anything", On::Overlay);
            assert_eq!(over(&desk), []);
            desk.panel(tool, "trindle-screenshot", On::Overlay);
            assert_eq!(over(&desk), named(&[(tool, "trindle-screenshot")]));

            let root = desk.join(true);
            desk.panel(root, "pocketkey", On::Overlay);
            assert_eq!(over(&desk), named(&[(tool, "trindle-screenshot"), (root, "pocketkey")]));
        }

        #[test]
        fn the_ring_opened_by_a_command_or_the_bus_is_not_over_a_full_screen_window() {
            let mut desk = Desk::new();
            let ring = desk.join_ours();
            let over = |desk: &Desk| {
                let map = layer_map_for_output(&desk.server.output);
                let over = super::super::over_full_screen(&map).count();
                assert_eq!(map.layers().filter(|l| super::super::shows_over_full_screen(l)).count(), over);
                over
            };

            desk.panel(ring, "trindle-ring", On::Overlay);
            assert_eq!(over(&desk), 0, "the ring was put over a full-screen window with nobody asking for it");

            desk.wish_long_ago();
            desk.panel(ring, "trindle-ring", On::Overlay);
            assert_eq!(over(&desk), 0);

            desk.wish();
            desk.panel(ring, "trindle-ring", On::Overlay);
            assert_eq!(over(&desk), 1);

            desk.panel(ring, "trindle-notice", On::Overlay);
            let stranger = desk.join(false);
            desk.panel(stranger, "trindle-ring", On::Overlay);
            assert_eq!(over(&desk), 1);
        }

        #[test]
        fn a_panel_is_asked_for_as_it_comes_out_and_no_longer_once_put_away() {
            use super::super::asked_after;

            assert!(asked_after(false, false, true, true));
            assert!(!asked_after(false, false, true, false));

            assert!(!asked_after(true, false, true, true));
            assert!(asked_after(true, true, true, false));

            assert!(!asked_after(true, true, false, true));
            assert!(!asked_after(false, true, false, true));
            assert!(!asked_after(false, true, true, false));
        }

        #[test]
        fn a_stranger_calling_its_panel_the_ring_is_not_over_a_full_screen_window() {
            let mut desk = Desk::new();
            let stranger = desk.join(false);
            desk.panel(stranger, "trindle-ring", On::Overlay);
            {
                let map = layer_map_for_output(&desk.server.output);
                let its = map.layers().next().expect("the stranger's panel is there");
                assert_eq!(super::super::over_full_screen(&map).count(), 0, "a name put a stranger's panel over a full-screen window");

                assert!(!super::super::shows_over_full_screen(its));
            }

            desk.wish();
            let ring = desk.join_ours();
            desk.panel(ring, "trindle-notice", On::Overlay);
            desk.panel(ring, super::super::NEWS, On::Overlay);
            desk.panel(ring, "trindle-ring", On::Overlay);
            let map = layer_map_for_output(&desk.server.output);
            let over: Vec<_> = super::super::over_full_screen(&map).map(|l| (l.wl_surface().client().map(|c| c.id()), l.namespace().to_string())).collect();
            assert_eq!(over, [(Some(desk.programs[ring].id.clone()), "trindle-ring".to_string())]);
            let shown: Vec<bool> = map.layers().map(super::super::shows_over_full_screen).collect();
            assert_eq!(shown.iter().filter(|s| **s).count(), 1);

            drop(map);
            let root = desk.join(true);
            desk.panel(root, "anything", On::Overlay);
            let map = layer_map_for_output(&desk.server.output);
            assert_eq!(super::super::over_full_screen(&map).count(), 2);
            assert!(super::super::shows_over_full_screen(map.layers().last().unwrap()));
        }

        #[test]
        fn trindles_own_news_is_above_every_other_programs_panel() {
            let mut desk = Desk::new();
            let ring = desk.join_ours();
            let stranger = desk.join(false);

            desk.panel(ring, super::super::NEWS, On::Overlay);
            desk.panel(stranger, "anything", On::Overlay);
            desk.panel(stranger, super::super::NEWS, On::Overlay);
            let got = desk.order();
            assert_eq!(got.last(), Some(&(ring, super::super::NEWS.to_string())), "a stranger's panel covers a notification: {got:?}");

            desk.panel(ring, "trindle-ring", On::Overlay);
            desk.panel(stranger, "more", On::Overlay);
            let got = desk.order();
            assert_eq!(got.last(), Some(&(ring, super::super::NEWS.to_string())), "{got:?}");

            let under: Vec<(usize, &str)> = got[..got.len() - 1].iter().map(|p| (p.0, p.1.as_str())).collect();
            assert_eq!(under, [(stranger, "anything"), (stranger, super::super::NEWS), (ring, "trindle-ring"), (stranger, "more")]);
        }

        #[test]
        fn only_a_program_that_connected_as_root_is_above_everything() {
            let mut desk = Desk::new();
            let root = desk.join(true);
            let ring = desk.join_ours();
            let stranger = desk.join(false);
            desk.panel(root, "pocketkey", On::Overlay);
            desk.panel(ring, super::super::NEWS, On::Overlay);
            desk.panel(stranger, super::super::NEWS, On::Overlay);
            desk.panel(stranger, "pocketkey", On::Overlay);
            desk.panel(ring, "trindle-ring", On::Overlay);
            let got = desk.order();
            assert_eq!(got.last(), Some(&(root, "pocketkey".to_string())), "{got:?}");

            let want = named(&[(stranger, super::super::NEWS), (stranger, "pocketkey"), (ring, "trindle-ring"), (ring, super::super::NEWS), (root, "pocketkey")]);
            assert_eq!(got, want);
            let map = layer_map_for_output(&desk.server.output);
            assert_eq!(super::super::overlays(&map).count(), 1);

            let (a, _b) = UnixStream::pair().unwrap();
            // SAFETY: geteuid only reads this process's own user.
            let me_root = unsafe { libc::geteuid() } == 0;
            assert_eq!(super::super::from_root(&a), me_root);
        }
    }
}

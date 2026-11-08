//! Trindle's own protocols driven end to end, in memory: a small server
//! holding only these globals, and a client standing in for the ring, the
//! panel, Display settings and night light, joined by a socket pair. Nothing
//! is drawn and no screen is needed.

use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::sync::Arc;

use smithay::output::{Output, PhysicalProperties, Subpixel};
use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::{Client, Display};
use smithay::utils::Transform;
use smithay::wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState};
use smithay::wayland::output::OutputHandler;
use smithay::{delegate_compositor, delegate_output};
use wayland_client::protocol::{wl_compositor, wl_output::WlOutput, wl_registry, wl_surface};
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};

use super::night::{self, NightHandler, NightState};
use super::screens::{ScreenFacts, ScreenMode, ScreenPlan, ScreensHandler, ScreensState};
use super::windows::{Change, WindowInfo, WindowsHandler, WindowsState};
use super::workspaces::{WorkspacesHandler, WorkspacesState};
use crate::wire::night::{
    trindle_night_manager_v1::TrindleNightManagerV1,
    trindle_night_v1::{self, TrindleNightV1},
};
use crate::wire::screens::{
    trindle_screen_plan_v1::{self, TrindleScreenPlanV1},
    trindle_screen_v1::{self, TrindleScreenV1},
    trindle_screens_manager_v1::{self, TrindleScreensManagerV1},
};
use crate::wire::windows::{
    trindle_window_v1::{self, TrindleWindowV1},
    trindle_windows_manager_v1::{self, TrindleWindowsManagerV1},
};
use crate::wire::workspaces::trindle_workspaces_manager_v1::{self, TrindleWorkspacesManagerV1};

struct Server {
    compositor: CompositorState,
    windows: WindowsState,
    workspaces: WorkspacesState,
    night: NightState,
    screens: ScreensState,

    surfaces: Vec<WlSurface>,

    asked: Vec<String>,

    table_sizes: HashMap<String, u32>,

    tables: Vec<(String, Option<Vec<u16>>)>,

    facts: Vec<ScreenFacts>,

    arranged: Vec<Vec<ScreenPlan>>,
    backend_refuses: Option<String>,
}

#[derive(Default)]
struct ServerClient {
    compositor: CompositorClientState,
}

impl ClientData for ServerClient {
    fn initialized(&self, _: ClientId) {}
    fn disconnected(&self, _: ClientId, _: DisconnectReason) {}
}

impl CompositorHandler for Server {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor
    }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ServerClient>().unwrap().compositor
    }
    fn new_surface(&mut self, surface: &WlSurface) {
        self.surfaces.push(surface.clone());
    }
    fn commit(&mut self, _: &WlSurface) {}
}
delegate_compositor!(Server);

impl OutputHandler for Server {}
delegate_output!(Server);

impl Server {
    fn number(&self, surface: &WlSurface) -> usize {
        self.surfaces.iter().position(|s| s == surface).unwrap()
    }
}

impl WindowsHandler for Server {
    fn windows_state(&mut self) -> &mut WindowsState {
        &mut self.windows
    }
    fn raise_window(&mut self, s: WlSurface) {
        let n = self.number(&s);
        self.asked.push(format!("raise {n}"));
    }
    fn close_window(&mut self, s: WlSurface) {
        let n = self.number(&s);
        self.asked.push(format!("close {n}"));
    }
    fn change_window(&mut self, s: WlSurface, change: Change, on: bool) {
        let n = self.number(&s);
        self.asked.push(format!("{change:?} {n} {on}"));
    }
}
crate::delegate_windows!(Server);

impl WorkspacesHandler for Server {
    fn workspaces_state(&mut self) -> &mut WorkspacesState {
        &mut self.workspaces
    }
    fn workspace_asked(&mut self, index: usize) {
        self.asked.push(format!("workspace {index}"));
    }
}
crate::delegate_workspaces!(Server);

impl NightHandler for Server {
    fn night_state(&mut self) -> &mut NightState {
        &mut self.night
    }
    fn colour_table_size(&mut self, output: &Output) -> Option<u32> {
        self.table_sizes.get(&output.name()).copied()
    }
    fn set_colour_table(&mut self, output: &Output, table: Option<Vec<u16>>) -> bool {
        self.tables.push((output.name(), table));
        true
    }
}
crate::delegate_night!(Server);

impl ScreensHandler for Server {
    fn screens_state(&mut self) -> &mut ScreensState {
        &mut self.screens
    }
    fn screens_now(&mut self) -> Vec<ScreenFacts> {
        self.facts.clone()
    }
    fn arrange(&mut self, plan: &[ScreenPlan], test_only: bool) -> Result<(), String> {
        if let Some(why) = &self.backend_refuses {
            return Err(why.clone());
        }
        if !test_only {
            self.arranged.push(plan.to_vec());
        }
        Ok(())
    }
}
crate::delegate_screens!(Server);

#[derive(Debug)]
struct Win {
    handle: TrindleWindowV1,
    title: String,
    app_id: String,
    flags: u32,
    outputs: Vec<WlOutput>,
    parent: Option<TrindleWindowV1>,
    dones: usize,
    gone: bool,
}

#[derive(Debug)]
struct Tint {
    object: TrindleNightV1,
    refused: Option<u32>,
}

#[derive(Debug)]
struct Scr {
    object: TrindleScreenV1,
    name: String,
    description: String,
    size_mm: (i32, i32),
    modes: Vec<(i32, i32, i32, u32)>,
    layout: Option<(u32, u32, i32, i32, u32, u32, u32)>,
    clears: usize,
    unplugged: bool,
}

#[derive(Default)]
struct Panel {
    compositor: Option<wl_compositor::WlCompositor>,
    output: Option<WlOutput>,
    windows_manager: Option<TrindleWindowsManagerV1>,
    wins: Vec<Win>,
    workspace_manager: Option<TrindleWorkspacesManagerV1>,
    rows: Vec<(u32, u32)>,
    night_manager: Option<TrindleNightManagerV1>,
    tints: Vec<Tint>,
    screens_manager: Option<TrindleScreensManagerV1>,
    screens: Vec<Scr>,
    settled: Vec<u32>,
    results: Vec<(u32, String)>,
}

impl Panel {
    fn win(&self, title: &str) -> &Win {
        self.wins.iter().find(|w| w.title == title).unwrap()
    }

    fn screen(&self, name: &str) -> &Scr {
        self.screens.iter().find(|s| s.name == name).unwrap()
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for Panel {
    fn event(panel: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        let wl_registry::Event::Global { name, interface, .. } = event else { return };
        match interface.as_str() {
            "wl_compositor" => panel.compositor = Some(registry.bind(name, 1, qh, ())),
            "wl_output" => panel.output = Some(registry.bind(name, 1, qh, ())),
            "trindle_windows_manager_v1" => panel.windows_manager = Some(registry.bind(name, 1, qh, ())),
            "trindle_workspaces_manager_v1" => panel.workspace_manager = Some(registry.bind(name, 1, qh, ())),
            "trindle_night_manager_v1" => panel.night_manager = Some(registry.bind(name, 1, qh, ())),
            "trindle_screens_manager_v1" => panel.screens_manager = Some(registry.bind(name, 1, qh, ())),
            _ => {}
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for Panel {
    fn event(_: &mut Self, _: &wl_compositor::WlCompositor, _: wl_compositor::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}
impl Dispatch<wl_surface::WlSurface, ()> for Panel {
    fn event(_: &mut Self, _: &wl_surface::WlSurface, _: wl_surface::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}
impl Dispatch<WlOutput, ()> for Panel {
    fn event(_: &mut Self, _: &WlOutput, _: wayland_client::protocol::wl_output::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}
impl Dispatch<TrindleNightManagerV1, ()> for Panel {
    fn event(_: &mut Self, _: &TrindleNightManagerV1, _: <TrindleNightManagerV1 as Proxy>::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {}
}

impl Dispatch<TrindleWindowsManagerV1, ()> for Panel {
    fn event(panel: &mut Self, _: &TrindleWindowsManagerV1, event: trindle_windows_manager_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        let trindle_windows_manager_v1::Event::Window { window } = event;
        panel.wins.push(Win {
            handle: window,
            title: String::new(),
            app_id: String::new(),
            flags: 0,
            outputs: Vec::new(),
            parent: None,
            dones: 0,
            gone: false,
        });
    }

    event_created_child!(Panel, TrindleWindowsManagerV1, [
        trindle_windows_manager_v1::EVT_WINDOW_OPCODE => (TrindleWindowV1, ()),
    ]);
}

impl Dispatch<TrindleWindowV1, ()> for Panel {
    fn event(panel: &mut Self, handle: &TrindleWindowV1, event: trindle_window_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        use trindle_window_v1::Event as E;
        let w = panel.wins.iter_mut().find(|w| &w.handle == handle).unwrap();
        match event {
            E::Title { title } => w.title = title,
            E::AppId { app_id } => w.app_id = app_id,
            E::Flags { flags } => w.flags = flags,
            E::Enter { output } => w.outputs.push(output),
            E::Leave { output } => w.outputs.retain(|o| o != &output),
            E::Parent { parent } => w.parent = parent,
            E::Done => w.dones += 1,
            E::Gone => w.gone = true,
        }
    }
}

impl Dispatch<TrindleWorkspacesManagerV1, ()> for Panel {
    fn event(panel: &mut Self, _: &TrindleWorkspacesManagerV1, event: trindle_workspaces_manager_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        let trindle_workspaces_manager_v1::Event::Row { count, showing } = event;
        panel.rows.push((count, showing));
    }
}

impl Dispatch<TrindleNightV1, ()> for Panel {
    fn event(panel: &mut Self, object: &TrindleNightV1, event: trindle_night_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        let trindle_night_v1::Event::Refused { reason } = event;
        let reason = match reason {
            WEnum::Value(r) => r as u32,
            WEnum::Unknown(n) => n,
        };
        panel.tints.iter_mut().find(|t| &t.object == object).unwrap().refused = Some(reason);
    }
}

impl Dispatch<TrindleScreensManagerV1, ()> for Panel {
    fn event(panel: &mut Self, _: &TrindleScreensManagerV1, event: trindle_screens_manager_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        match event {
            trindle_screens_manager_v1::Event::Screen { screen } => panel.screens.push(Scr {
                object: screen,
                name: String::new(),
                description: String::new(),
                size_mm: (0, 0),
                modes: Vec::new(),
                layout: None,
                clears: 0,
                unplugged: false,
            }),
            trindle_screens_manager_v1::Event::Settled { serial } => panel.settled.push(serial),
        }
    }

    event_created_child!(Panel, TrindleScreensManagerV1, [
        trindle_screens_manager_v1::EVT_SCREEN_OPCODE => (TrindleScreenV1, ()),
    ]);
}

impl Dispatch<TrindleScreenV1, ()> for Panel {
    fn event(panel: &mut Self, object: &TrindleScreenV1, event: trindle_screen_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        use trindle_screen_v1::Event as E;
        let s = panel.screens.iter_mut().find(|s| &s.object == object).unwrap();
        match event {
            E::Clear => {
                s.modes.clear();
                s.clears += 1;
            }
            E::Identity { name, description, .. } => {
                s.name = name;
                s.description = description;
            }
            E::SizeMm { width, height } => s.size_mm = (width, height),
            E::Mode { width, height, millihertz, flags } => s.modes.push((width, height, millihertz, flags)),
            E::Layout { on, mode, x, y, turn, scale_120ths, varies_rate } => {
                s.layout = Some((on, mode, x, y, turn, scale_120ths, varies_rate))
            }
            E::Unplugged => s.unplugged = true,
        }
    }
}

impl Dispatch<TrindleScreenPlanV1, ()> for Panel {
    fn event(panel: &mut Self, _: &TrindleScreenPlanV1, event: trindle_screen_plan_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        let trindle_screen_plan_v1::Event::Outcome { outcome, why } = event;
        let code = match outcome {
            WEnum::Value(r) => r as u32,
            WEnum::Unknown(n) => n,
        };
        panel.results.push((code, why));
    }
}

struct PanelClient {
    conn: Connection,
    queue: EventQueue<Panel>,
    panel: Panel,
}

struct Harness {
    display: Display<Server>,
    server: Server,
    output: Output,
    clients: Vec<PanelClient>,
}

fn mode(width: i32, height: i32, millihertz: i32, preferred: bool) -> ScreenMode {
    ScreenMode { width, height, millihertz, preferred }
}

fn screen(name: &str, on: bool) -> ScreenFacts {
    ScreenFacts {
        name: name.into(),
        description: format!("Test {name}"),
        make: "test".into(),
        model: "test".into(),
        serial: None,
        size_mm: Some((300, 200)),
        modes: vec![mode(1920, 1080, 60000, true), mode(1920, 1080, 50000, false), mode(1280, 720, 60000, false)],
        on,
        mode: on.then_some(0),
        position: (0, 0),
        turn: Transform::Normal,
        scale: 1.0,
        varies_rate: false,
    }
}

impl Harness {
    fn new() -> Self {
        let display = Display::<Server>::new().unwrap();
        let dh = display.handle();
        let output = Output::new(
            "test-1".into(),
            PhysicalProperties {
                size: (300, 200).into(),
                subpixel: Subpixel::Unknown,
                make: "test".into(),
                model: "test".into(),
            },
        );
        output.create_global::<Server>(&dh);
        let server = Server {
            compositor: CompositorState::new::<Server>(&dh),
            windows: WindowsState::new::<Server, _>(&dh, |_| true),
            workspaces: WorkspacesState::new::<Server, _>(&dh, |_| true),
            night: NightState::new::<Server, _>(&dh, |_| true),
            screens: ScreensState::new::<Server, _>(&dh, |_| true),
            surfaces: Vec::new(),
            asked: Vec::new(),
            table_sizes: HashMap::from([("test-1".to_string(), 256)]),
            tables: Vec::new(),
            facts: vec![screen("A", true), screen("B", true)],
            arranged: Vec::new(),
            backend_refuses: None,
        };
        Harness { display, server, output, clients: Vec::new() }
    }

    fn connect(&mut self) -> usize {
        let (server_end, client_end) = UnixStream::pair().unwrap();
        self.display.handle().insert_client(server_end, Arc::new(ServerClient::default())).unwrap();
        let conn = Connection::from_socket(client_end).unwrap();
        let queue = conn.new_event_queue();
        conn.display().get_registry(&queue.handle(), ());
        self.clients.push(PanelClient { conn, queue, panel: Panel::default() });

        self.pump();
        self.pump();
        self.clients.len() - 1
    }

    fn pump(&mut self) {
        for _ in 0..4 {
            for c in &mut self.clients {
                c.conn.flush().unwrap();
            }
            self.display.dispatch_clients(&mut self.server).unwrap();
            self.display.flush_clients().unwrap();
            for c in &mut self.clients {
                if let Some(guard) = c.conn.prepare_read() {
                    // Nothing waiting is not an error here.
                    let _ = guard.read();
                }
                c.queue.dispatch_pending(&mut c.panel).unwrap();
            }
        }
    }

    fn surface(&mut self, c: usize) -> WlSurface {
        let client = &mut self.clients[c];
        client.panel.compositor.as_ref().unwrap().create_surface(&client.queue.handle(), ());
        self.pump();
        self.server.surfaces.last().unwrap().clone()
    }

    fn panel(&self, c: usize) -> &Panel {
        &self.clients[c].panel
    }

    fn qh(&self, c: usize) -> QueueHandle<Panel> {
        self.clients[c].queue.handle()
    }

    fn windows(&mut self, windows: Vec<WindowInfo>) {
        super::windows::refresh(&mut self.server, windows);
        self.pump();
    }

    fn row(&mut self, count: usize, showing: usize) {
        super::workspaces::refresh(&mut self.server, count, showing);
        self.pump();
    }

    fn rescreen(&mut self) {
        super::screens::refresh(&mut self.server);
        self.pump();
    }
}

fn info(surface: &WlSurface, title: &str) -> WindowInfo {
    WindowInfo {
        surface: surface.clone(),
        title: title.into(),
        app_id: format!("org.test.{title}"),
        active: false,
        minimized: false,
        maximized: false,
        fullscreen: false,
        outputs: Vec::new(),
        parent: None,
    }
}

const ACTIVE: u32 = 1;
const MINIMIZED: u32 = 2;
const MAXIMIZED: u32 = 4;
const FULLSCREEN: u32 = 8;

fn the_windows_are_announced_in_the_order_given_and_a_repeat_sends_nothing(h: &mut Harness, p: usize) -> WindowInfo {
    let a = h.surface(p);
    let b = h.surface(p);

    let mut files = info(&a, "Files");
    files.active = true;
    files.outputs = vec![h.output.clone()];
    let mut dialog = info(&b, "Dialog");
    dialog.minimized = true;
    dialog.parent = Some(a.clone());
    h.windows(vec![files.clone(), dialog.clone()]);

    let panel = h.panel(p);
    assert_eq!(panel.wins.len(), 2);
    assert_eq!(panel.wins[0].title, "Files", "announced in the order given");
    let f = panel.win("Files");
    assert_eq!(f.app_id, "org.test.Files");
    assert_eq!(f.flags, ACTIVE);
    assert_eq!(f.outputs.len(), 1);
    assert_eq!(f.dones, 1);
    let d = panel.win("Dialog");
    assert_eq!(d.flags, MINIMIZED);
    assert_eq!(d.parent.as_ref(), Some(&f.handle));
    assert_eq!(d.dones, 1);

    h.windows(vec![files.clone(), dialog.clone()]);
    assert_eq!(h.panel(p).win("Files").dones, 1);
    assert_eq!(h.panel(p).win("Dialog").dones, 1);
    files
}

fn the_program_asks_for_things(h: &mut Harness, p: usize) -> TrindleWindowV1 {

    let f = h.panel(p).win("Files").handle.clone();
    let d = h.panel(p).win("Dialog").handle.clone();
    d.change(0, MINIMIZED);
    f.change(MAXIMIZED | FULLSCREEN, 0);
    f.change(0, FULLSCREEN);
    f.change(ACTIVE, 0);
    f.change(MAXIMIZED, MAXIMIZED);
    f.raise();
    f.close();
    h.pump();
    assert_eq!(
        h.server.asked,
        vec![
            "Minimized 1 false",
            "Maximized 0 true",
            "Fullscreen 0 true",
            "Fullscreen 0 false",
            "raise 0",
            "close 0"
        ],
        "active cannot be asked for, and a flag asked on and off at once is left alone"
    );
    d
}

#[test]
fn the_window_list_follows_the_windows_and_acts_on_them() {
    let mut h = Harness::new();
    let p = h.connect();
    assert!(h.panel(p).windows_manager.is_some(), "the global is offered");
    let mut files = the_windows_are_announced_in_the_order_given_and_a_repeat_sends_nothing(&mut h, p);
    let d = the_program_asks_for_things(&mut h, p);

    files.title = "Files - Home".into();
    files.active = false;
    files.fullscreen = true;
    h.windows(vec![files.clone()]);
    let panel = h.panel(p);
    assert_eq!(panel.wins[0].title, "Files - Home");
    assert_eq!(panel.wins[0].flags, FULLSCREEN);
    assert_eq!(panel.wins[0].dones, 2);
    assert!(panel.win("Dialog").gone);

    files.outputs.clear();
    h.windows(vec![files.clone()]);
    assert!(h.panel(p).wins[0].outputs.is_empty());
    assert_eq!(h.panel(p).wins[0].dones, 3);

    d.close();
    h.pump();
    assert_eq!(h.server.asked.len(), 6);
}

#[test]
fn a_late_listener_sees_the_windows_already_open_with_objects_of_its_own() {
    let mut h = Harness::new();
    let app = h.connect();
    let a = h.surface(app);
    let b = h.surface(app);
    let mut child = info(&b, "Child");
    child.parent = Some(a.clone());
    h.windows(vec![info(&a, "Parent"), child]);

    let late = h.connect();
    let panel = h.panel(late);
    assert_eq!(panel.wins.len(), 2);
    let parent = panel.win("Parent");
    let child = panel.win("Child");
    assert_eq!((parent.dones, child.dones), (1, 1));
    assert_eq!(child.parent.as_ref(), Some(&parent.handle));
    assert_ne!(h.panel(app).win("Parent").handle.id(), parent.handle.id());
}

#[test]
fn a_program_that_forgets_a_window_hears_no_more_of_it() {
    let mut h = Harness::new();
    let p = h.connect();
    let a = h.surface(p);
    let mut one = info(&a, "One");
    h.windows(vec![one.clone()]);
    h.panel(p).win("One").handle.destroy();
    h.pump();
    one.title = "Renamed".into();
    h.windows(vec![one]);
    assert_eq!(h.panel(p).wins[0].title, "One", "nothing more is sent for a destroyed window");
}

#[test]
fn the_row_of_workspaces_is_told_whole_and_only_when_it_changes() {
    let mut h = Harness::new();
    let p = h.connect();
    h.row(2, 0);
    assert_eq!(h.panel(p).rows, vec![(2, 0)]);

    h.row(2, 0);
    assert_eq!(h.panel(p).rows.len(), 1);

    h.panel(p).workspace_manager.as_ref().unwrap().go(1);
    h.pump();
    assert_eq!(h.server.asked, vec!["workspace 1"]);
    h.row(2, 1);
    assert_eq!(h.panel(p).rows.last(), Some(&(2, 1)));

    h.row(3, 1);
    h.row(1, 0);
    assert_eq!(h.panel(p).rows, vec![(2, 0), (2, 1), (3, 1), (1, 0)]);

    let late = h.connect();
    assert_eq!(h.panel(late).rows, vec![(1, 0)]);
}

#[test]
fn a_request_past_the_end_of_the_row_reaches_the_compositor_to_judge() {
    let mut h = Harness::new();
    let p = h.connect();
    h.row(2, 0);
    h.panel(p).workspace_manager.as_ref().unwrap().go(40);
    h.pump();
    assert_eq!(h.server.asked, vec!["workspace 40"]);
}

fn tint(h: &mut Harness, c: usize) -> usize {
    let qh = h.qh(c);
    let panel = &mut h.clients[c].panel;
    let output = panel.output.clone().unwrap();
    let object = panel.night_manager.as_ref().unwrap().tint(&output, &qh, ());
    panel.tints.push(Tint { object, refused: None });
    let at = panel.tints.len() - 1;
    h.pump();
    at
}

#[test]
fn night_light_tints_a_screen_and_the_screen_comes_back_when_it_lets_go() {
    let mut h = Harness::new();
    let p = h.connect();
    let t = tint(&mut h, p);
    assert_eq!(h.panel(p).tints[t].refused, None);
    assert!(h.server.tables.is_empty(), "nothing changes until the tint is set");

    h.panel(p).tints[t].object.warm(65535, 40_000, 20_000);
    h.pump();
    let (name, table) = h.server.tables.last().unwrap();
    assert_eq!(name, "test-1");
    assert_eq!(table.as_ref().unwrap(), &night::table(256, [65535, 40_000, 20_000]));

    h.panel(p).tints[t].object.warm(65535, 50_000, 30_000);
    h.pump();
    assert_eq!(h.server.tables.len(), 2);

    h.panel(p).tints[t].object.destroy();
    h.pump();
    assert_eq!(h.server.tables.last().unwrap(), &("test-1".to_string(), None), "the screen's own colours are back");
}

#[test]
fn a_tint_is_taken_down_when_its_program_ends() {
    let mut h = Harness::new();
    let p = h.connect();
    let t = tint(&mut h, p);
    h.panel(p).tints[t].object.warm(65535, 40_000, 20_000);
    h.pump();

    let gone = h.clients.remove(p);
    drop(gone);
    h.display.dispatch_clients(&mut h.server).unwrap();
    assert_eq!(h.server.tables.last().unwrap(), &("test-1".to_string(), None));
}

#[test]
fn one_program_at_a_time_holds_a_screen() {
    let mut h = Harness::new();
    let first = h.connect();
    let second = h.connect();
    let a = tint(&mut h, first);
    let b = tint(&mut h, second);
    assert_eq!(h.panel(first).tints[a].refused, None);
    assert_eq!(h.panel(second).tints[b].refused, Some(1), "taken");

    h.panel(second).tints[b].object.warm(0, 0, 0);
    h.panel(second).tints[b].object.destroy();
    h.pump();
    assert!(h.server.tables.is_empty());

    h.panel(first).tints[a].object.destroy();
    h.pump();
    let c = tint(&mut h, second);
    assert_eq!(h.panel(second).tints[c].refused, None);
}

#[test]
fn a_screen_with_no_colour_tables_is_refused() {
    let mut h = Harness::new();
    h.server.table_sizes.clear();
    let p = h.connect();
    let t = tint(&mut h, p);
    assert_eq!(h.panel(p).tints[t].refused, Some(2), "unsupported");
}

#[test]
fn a_tint_cannot_turn_a_screen_black() {
    let mut h = Harness::new();
    let p = h.connect();
    let t = tint(&mut h, p);
    h.panel(p).tints[t].object.warm(0, 0, 0);
    h.pump();
    let table = h.server.tables.last().unwrap().1.clone().unwrap();
    let floor = u16::MAX / 10;
    assert_eq!((table[255], table[256 + 255], table[512 + 255]), (floor, floor, floor));
}

fn plan_for(h: &mut Harness, c: usize, serial: u32) -> TrindleScreenPlanV1 {
    let qh = h.qh(c);
    h.clients[c].panel.screens_manager.as_ref().unwrap().plan(serial, &qh, ())
}

fn keep(plan: &TrindleScreenPlanV1, s: &Scr, on: bool) {
    let (_, _, x, y, turn, scale, varies) = s.layout.unwrap();
    plan.screen(&s.object, u32::from(on), 0, 0, 0, x, y, turn, scale, varies);
}

#[test]
fn the_screens_are_described_in_full_and_changes_arrive_whole() {
    let mut h = Harness::new();
    let p = h.connect();
    let panel = h.panel(p);
    assert_eq!(panel.screens.len(), 2);
    let a = panel.screen("A");
    assert_eq!(a.description, "Test A");
    assert_eq!(a.size_mm, (300, 200));
    assert_eq!(a.modes, vec![(1920, 1080, 60000, 1), (1920, 1080, 50000, 0), (1280, 720, 60000, 0)]);
    assert_eq!(a.layout, Some((1, 0, 0, 0, 0, 120, 0)));
    let first = *panel.settled.last().unwrap();

    h.rescreen();
    assert_eq!(h.panel(p).settled.len(), 1);

    h.server.facts[1].position = (1920, 0);
    h.server.facts[1].scale = 1.25;
    h.rescreen();
    let panel = h.panel(p);
    assert_eq!(panel.screen("B").clears, 1);
    assert_eq!(panel.screen("B").layout, Some((1, 0, 1920, 0, 0, 150, 0)));
    assert_eq!(panel.screen("B").modes.len(), 3, "the whole description arrives again");
    assert_eq!(panel.screen("A").clears, 0);
    assert!(*panel.settled.last().unwrap() > first, "the serial goes up");

    h.server.facts[0].on = false;
    h.server.facts[0].mode = None;
    h.rescreen();
    assert_eq!(h.panel(p).screen("A").layout.unwrap().0, 0);
    assert_eq!(h.panel(p).screen("A").layout.unwrap().1, u32::MAX);
    h.server.facts.remove(0);
    h.rescreen();
    assert!(h.panel(p).screen("A").unplugged);

    let late = h.connect();
    assert_eq!(h.panel(late).screens.len(), 1);
    assert_eq!(h.panel(late).screens[0].name, "B");
}

#[test]
fn a_plan_made_from_the_latest_picture_is_carried_out() {
    let mut h = Harness::new();
    let p = h.connect();
    let serial = *h.panel(p).settled.last().unwrap();
    let plan = plan_for(&mut h, p, serial);
    let (a, b) = (h.panel(p).screen("A"), h.panel(p).screen("B"));

    let (_, _, x, y, turn, _, varies) = a.layout.unwrap();
    plan.screen(&a.object, 1, 1280, 720, 60000, x, y, turn, 180, varies);
    keep(&plan, b, false);
    plan.apply();
    h.pump();
    assert_eq!(h.panel(p).results, vec![(0, String::new())]);
    let done = &h.server.arranged;
    assert_eq!(done.len(), 1);
    let (a, b) = (&done[0][0], &done[0][1]);
    assert_eq!((a.name.as_str(), a.on, a.mode, a.scale), ("A", true, Some(2), 1.5));
    assert_eq!((b.name.as_str(), b.on, b.mode), ("B", false, None));
}

#[test]
fn a_check_asks_without_changing_anything() {
    let mut h = Harness::new();
    let p = h.connect();
    let serial = *h.panel(p).settled.last().unwrap();
    let plan = plan_for(&mut h, p, serial);
    for s in ["A", "B"] {
        keep(&plan, h.panel(p).screen(s), true);
    }
    plan.check();
    h.pump();
    assert_eq!(h.panel(p).results, vec![(0, String::new())]);
    assert!(h.server.arranged.is_empty());

    plan.check();
    plan.apply();
    h.pump();
    assert_eq!(h.panel(p).results.len(), 3);
    assert_eq!(h.server.arranged.len(), 1);
}

#[test]
fn a_plan_from_an_old_picture_is_turned_away() {
    let mut h = Harness::new();
    let p = h.connect();
    let serial = *h.panel(p).settled.last().unwrap();
    let plan = plan_for(&mut h, p, serial);
    for s in ["A", "B"] {
        keep(&plan, h.panel(p).screen(s), true);
    }

    h.server.facts[0].scale = 2.0;
    h.rescreen();
    plan.apply();
    h.pump();
    assert_eq!(h.panel(p).results.len(), 1);
    assert_eq!(h.panel(p).results[0].0, 2, "stale");
    assert!(h.server.arranged.is_empty());
}

#[test]
fn a_plan_that_cannot_be_used_says_why() {
    let mut h = Harness::new();
    let p = h.connect();
    let serial = *h.panel(p).settled.last().unwrap();

    let plan = plan_for(&mut h, p, serial);
    for s in ["A", "B"] {
        keep(&plan, h.panel(p).screen(s), false);
    }
    plan.apply();

    let plan = plan_for(&mut h, p, serial);
    keep(&plan, h.panel(p).screen("A"), true);
    plan.apply();

    let plan = plan_for(&mut h, p, serial);
    let (_, _, x, y, turn, scale, varies) = h.panel(p).screen("A").layout.unwrap();
    plan.screen(&h.panel(p).screen("A").object, 1, 800, 600, 60000, x, y, turn, scale, varies);
    keep(&plan, h.panel(p).screen("B"), true);
    plan.apply();
    h.pump();
    let results = &h.panel(p).results;
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|(code, _)| *code == 1));
    assert!(results[0].1.contains("every screen off"), "{}", results[0].1);
    assert!(results[1].1.contains("B was left out"), "{}", results[1].1);
    assert!(results[2].1.contains("cannot run at 800x600"), "{}", results[2].1);
    assert!(h.server.arranged.is_empty());
}

#[test]
fn a_refusal_from_the_screens_themselves_is_passed_on_in_its_words() {
    let mut h = Harness::new();
    h.server.backend_refuses = Some("A refused 1280x720".into());
    let p = h.connect();
    let serial = *h.panel(p).settled.last().unwrap();
    let plan = plan_for(&mut h, p, serial);
    for s in ["A", "B"] {
        keep(&plan, h.panel(p).screen(s), true);
    }
    plan.apply();
    h.pump();
    assert_eq!(h.panel(p).results, vec![(1, "A refused 1280x720".to_string())]);
}

//! End-to-end tests of panels on the layer shell, in memory.

use std::os::fd::{AsFd, FromRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::sync::Arc;

use smithay::desktop::{layer_map_for_output, LayerSurface, PopupManager, Space};
use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use smithay::reexports::wayland_server::protocol::{wl_buffer::WlBuffer, wl_output::WlOutput, wl_surface::WlSurface};
use smithay::reexports::wayland_server::{Client, Display};
use smithay::utils::{Logical, Point};
use smithay::wayland::buffer::BufferHandler;
use smithay::wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState};
use smithay::wayland::shell::wlr_layer::{Layer, LayerSurface as WlrLayerSurface, WlrLayerShellHandler, WlrLayerShellState};
use smithay::wayland::shm::{ShmHandler, ShmState};
use smithay::{delegate_compositor, delegate_layer_shell, delegate_shm};
use wayland_client::protocol::{wl_buffer, wl_compositor, wl_region, wl_registry, wl_shm, wl_shm_pool, wl_surface};
use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
};

use super::WindowElement;

struct Server {
    compositor: CompositorState,
    shm: ShmState,
    layers: WlrLayerShellState,
    output: Output,
    space: Space<WindowElement>,
    popups: PopupManager,

    surfaces: Vec<WlSurface>,
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

    fn commit(&mut self, surface: &WlSurface) {
        smithay::backend::renderer::utils::on_commit_buffer_handler::<Self>(surface);
        super::compositor::ensure_initial_configure(surface, &self.space, &mut self.popups);
    }
}
delegate_compositor!(Server);

impl BufferHandler for Server {
    fn buffer_destroyed(&mut self, _: &WlBuffer) {}
}
impl ShmHandler for Server {
    fn shm_state(&self) -> &ShmState {
        &self.shm
    }
}
delegate_shm!(Server);

impl WlrLayerShellHandler for Server {
    fn shell_state(&mut self) -> &mut WlrLayerShellState {
        &mut self.layers
    }
    fn new_layer_surface(&mut self, surface: WlrLayerSurface, _: Option<WlOutput>, _: Layer, namespace: String) {
        let mut map = layer_map_for_output(&self.output);
        map.map_layer(&LayerSurface::new(surface, namespace)).unwrap();
        crate::trusted::keep_on_top(&mut map);
    }
    fn layer_destroyed(&mut self, surface: WlrLayerSurface) {
        let mut map = layer_map_for_output(&self.output);
        let layer = map.layers().find(|l| l.layer_surface() == &surface).cloned();
        if let Some(layer) = layer {
            map.unmap_layer(&layer);
        }
    }
}
delegate_layer_shell!(Server);

#[derive(Default)]
struct Panels {
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,
    shell: Option<ZwlrLayerShellV1>,

    configures: Vec<(ZwlrLayerSurfaceV1, Option<u32>)>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for Panels {
    fn event(p: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection,
             qh: &QueueHandle<Self>) {
        let wl_registry::Event::Global { name, interface, version } = event else { return };
        match interface.as_str() {
            "wl_compositor" => p.compositor = Some(registry.bind(name, version.min(4), qh, ())),
            "wl_shm" => p.shm = Some(registry.bind(name, 1, qh, ())),
            "zwlr_layer_shell_v1" => p.shell = Some(registry.bind(name, version.min(4), qh, ())),
            _ => {}
        }
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for Panels {
            fn event(_: &mut Self, _: &$t, _: <$t as wayland_client::Proxy>::Event, _: &(), _: &Connection,
                     _: &QueueHandle<Self>) {}
        }
    )*};
}
ignore!(wl_compositor::WlCompositor, wl_surface::WlSurface, wl_shm::WlShm, wl_shm_pool::WlShmPool,
        wl_buffer::WlBuffer, wl_region::WlRegion, ZwlrLayerShellV1);

impl Dispatch<ZwlrLayerSurfaceV1, ()> for Panels {
    fn event(p: &mut Self, layer: &ZwlrLayerSurfaceV1, event: zwlr_layer_surface_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        if let zwlr_layer_surface_v1::Event::Configure { serial, .. } = event {
            if let Some(c) = p.configures.iter_mut().find(|(l, _)| l == layer) {
                c.1 = Some(serial);
            }
        }
    }
}

struct Shape {
    layer: zwlr_layer_shell_v1::Layer,
    anchor: Anchor,
    size: (i32, i32),
    margin_top: i32,

    keep: i32,
}

struct Harness {
    display: Display<Server>,
    server: Server,
    conn: Connection,
    queue: EventQueue<Panels>,
    panels: Panels,

    made: Vec<(wl_surface::WlSurface, ZwlrLayerSurfaceV1, WlSurface, (i32, i32))>,
}

impl Harness {
    fn new() -> Self {
        let display = Display::<Server>::new().unwrap();
        let dh = display.handle();
        let output = Output::new("test-1".into(), PhysicalProperties {
            size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into(),
        });
        let mode = Mode { size: (800, 600).into(), refresh: 60_000 };
        output.change_current_state(Some(mode), None, None, Some((0, 0).into()));
        output.set_preferred(mode);
        let mut space = Space::default();
        space.map_output(&output, (0, 0));
        let server = Server {
            compositor: CompositorState::new::<Server>(&dh),
            shm: ShmState::new::<Server>(&dh, vec![]),
            layers: WlrLayerShellState::new::<Server>(&dh),
            output,
            space,
            popups: PopupManager::default(),
            surfaces: Vec::new(),
        };
        let (server_end, client_end) = UnixStream::pair().unwrap();
        display.handle().insert_client(server_end, Arc::new(ServerClient::default())).unwrap();
        let conn = Connection::from_socket(client_end).unwrap();
        let queue = conn.new_event_queue();
        conn.display().get_registry(&queue.handle(), ());
        let mut h = Harness { display, server, conn, queue, panels: Panels::default(), made: Vec::new() };
        h.pump();
        h.pump();
        h
    }

    fn pump(&mut self) {
        for _ in 0..4 {
            self.conn.flush().unwrap();
            self.display.dispatch_clients(&mut self.server).unwrap();
            self.display.flush_clients().unwrap();
            if let Some(guard) = self.conn.prepare_read() {
                let _ = guard.read();
            }
            self.queue.dispatch_pending(&mut self.panels).unwrap();
        }
    }

    fn buffer(&mut self, (w, h): (i32, i32)) -> wl_buffer::WlBuffer {
        let len = (w * h * 4) as usize;
        // SAFETY: memfd_create is given a valid name and no flags that matter.
        let fd = unsafe { libc::memfd_create(c"trindle-test".as_ptr(), 0) };
        assert!(fd >= 0);
        // SAFETY: fd is a new descriptor owned by nothing else.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        std::fs::File::from(fd.try_clone().unwrap()).set_len(len as u64).unwrap();
        let qh = self.queue.handle();
        let pool = self.panels.shm.as_ref().unwrap().create_pool(fd.as_fd(), len as i32, &qh, ());
        let buffer = pool.create_buffer(0, w, h, w * 4, wl_shm::Format::Argb8888, &qh, ());
        pool.destroy();
        buffer
    }

    fn open(&mut self, shape: Shape, namespace: &str) -> usize {
        let qh = self.queue.handle();
        let surface = self.panels.compositor.as_ref().unwrap().create_surface(&qh, ());
        let layer = self.panels.shell.as_ref().unwrap().get_layer_surface(&surface, None, shape.layer,
                                                                          namespace.into(), &qh, ());
        layer.set_size(shape.size.0 as u32, shape.size.1 as u32);
        layer.set_anchor(shape.anchor);
        layer.set_margin(shape.margin_top, 0, 0, 0);
        layer.set_exclusive_zone(shape.keep);
        self.panels.configures.push((layer.clone(), None));
        surface.commit();
        self.pump();
        let server_side = self.server.surfaces.last().unwrap().clone();
        self.made.push((surface, layer, server_side, shape.size));
        let n = self.made.len() - 1;
        self.show(n);
        n
    }

    fn show(&mut self, n: usize) {
        let size = self.made[n].3;
        let (surface, layer) = (self.made[n].0.clone(), self.made[n].1.clone());
        let serial = self.panels.configures.iter_mut().find(|(l, _)| l == &layer).and_then(|c| c.1.take())
            .expect("the panel was asked to show");
        layer.ack_configure(serial);
        let buffer = self.buffer(size);
        surface.attach(Some(&buffer), 0, 0);
        surface.damage_buffer(0, 0, size.0, size.1);
        surface.commit();
        self.pump();
    }

    fn hide(&mut self, n: usize) {
        let surface = self.made[n].0.clone();
        surface.attach(None, 0, 0);
        surface.commit();
        self.pump();
    }

    fn bring_back(&mut self, n: usize) {
        self.made[n].0.commit();
        self.pump();
        self.show(n);
    }

    fn see_through(&mut self, n: usize) {
        let region = self.panels.compositor.as_ref().unwrap().create_region(&self.queue.handle(), ());
        self.made[n].0.set_input_region(Some(&region));
        self.made[n].0.commit();
        self.pump();
    }

    fn under(&self, at: (f64, f64)) -> Option<usize> {
        let map = layer_map_for_output(&self.server.output);
        let at: Point<f64, Logical> = at.into();
        let layer =
            super::layer_taking_input(&map, &[Layer::Overlay, Layer::Top, Layer::Bottom, Layer::Background], at, |_| true)?;
        self.made.iter().position(|m| &m.2 == layer.wl_surface())
    }
}

use zwlr_layer_shell_v1::Layer as L;

fn list() -> Shape {
    Shape { layer: L::Overlay, anchor: Anchor::Left | Anchor::Top, size: (186, 300), margin_top: 150, keep: 0 }
}

fn whole(layer: L) -> Shape {
    Shape { layer, anchor: Anchor::all(), size: (800, 600), margin_top: 0, keep: 0 }
}

#[test]
fn a_panel_letting_the_pointer_through_gives_it_to_the_one_under() {
    let mut h = Harness::new();
    let list = h.open(list(), "trindle-notice");

    let note = h.open(Shape { layer: L::Overlay, anchor: Anchor::Left | Anchor::Top, size: (300, 200), margin_top: 200, keep: 0 },
                      "trindle-notes");
    assert_eq!(h.under((50.0, 250.0)), Some(note));
    h.see_through(note);
    assert_eq!(h.under((50.0, 250.0)), Some(list));

    assert_eq!(h.under((700.0, 50.0)), None);
}

#[test]
fn a_panel_shown_again_comes_to_the_top() {
    let mut h = Harness::new();
    let list = h.open(list(), "trindle-notice");

    let ring = h.open(whole(L::Overlay), "trindle-ring");
    assert_eq!(h.under((50.0, 250.0)), Some(ring));

    h.hide(list);
    assert_eq!(h.under((50.0, 250.0)), Some(ring));
    h.bring_back(list);
    assert_eq!(h.under((50.0, 250.0)), Some(list));
    assert_eq!(h.under((500.0, 250.0)), Some(ring));

    h.hide(ring);
    h.bring_back(ring);
    assert_eq!(h.under((50.0, 250.0)), Some(ring));
}

#[test]
fn the_overlay_comes_before_the_top_layer_whatever_the_order() {
    let mut h = Harness::new();
    let list = h.open(list(), "trindle-notice");

    let catcher = h.open(whole(L::Top), "trindle-battery");
    assert_eq!(h.under((50.0, 250.0)), Some(list));
    assert_eq!(h.under((500.0, 250.0)), Some(catcher));
    h.hide(list);
    h.bring_back(list);
    assert_eq!(h.under((50.0, 250.0)), Some(list));

    let wall = h.open(whole(L::Background), "wallpaper");
    h.see_through(catcher);
    assert_eq!(h.under((500.0, 250.0)), Some(wall));
    assert_eq!(h.under((50.0, 250.0)), Some(list));
}

fn keyboard() -> Shape {
    Shape { layer: L::Top, anchor: Anchor::Bottom | Anchor::Left | Anchor::Right, size: (800, 240), margin_top: 0, keep: 240 }
}

impl Harness {

    fn place(&self, n: usize) -> smithay::utils::Rectangle<i32, Logical> {
        let map = layer_map_for_output(&self.server.output);
        let layer = map.layers().find(|l| l.wl_surface() == &self.made[n].2).expect("the panel is mapped");
        let size = layer.layer_surface().with_pending_state(|s| s.size).expect("it was told a size");
        smithay::utils::Rectangle::new(map.layer_geometry(layer).unwrap().loc, size)
    }

    fn room(&self) -> smithay::utils::Rectangle<i32, Logical> {
        layer_map_for_output(&self.server.output).non_exclusive_zone()
    }
}

#[test]
fn the_ring_takes_the_whole_screen_back_when_the_keyboard_goes() {
    let mut h = Harness::new();
    let ring = h.open(whole(L::Overlay), "trindle-ring");
    assert_eq!(h.place(ring).size, (800, 600).into());
    let keys = h.open(keyboard(), "trindle-keyboard");
    assert_eq!(h.room().size, (800, 360).into());
    assert_eq!(h.place(ring).size, (800, 360).into(), "the ring gives way to the keys");
    assert_eq!(h.place(keys).loc.y, 360);
    h.hide(keys);
    assert_eq!(h.room().size, (800, 600).into(), "the keyboard put away keeps no room");
    assert_eq!(h.place(ring).size, (800, 600).into());
    assert!(h.panels.configures.iter().any(|(l, s)| l == &h.made[ring].1 && s.is_some()), "the ring was told its new size");
    h.bring_back(keys);
    assert_eq!(h.room().size, (800, 360).into(), "back out, it keeps its room again");
    assert_eq!(h.place(ring).size, (800, 360).into());

    h.made[keys].1.destroy();
    h.made[keys].0.destroy();
    h.pump();
    assert_eq!(h.room().size, (800, 600).into());
    assert_eq!(h.place(ring).size, (800, 600).into());
}

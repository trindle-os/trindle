// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::io::Write;
use std::os::fd::{AsFd, FromRawFd, OwnedFd};

use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use wayland_client::protocol::{wl_buffer, wl_shm, wl_surface};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::Layer as On;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::{self, KeyboardInteractivity};

use crate::state::{AnvilState, ClientState};
use crate::virtual_input::Maker;

use super::*;

impl Rig {

    pub(super) fn picture(&self, who: usize, w: i32, h: i32) -> wl_buffer::WlBuffer {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let size = w * h * 4;
        // SAFETY: memfd_create is handed a name that ends in NUL and gives

        let fd = unsafe { libc::memfd_create(c"trindle-rig".as_ptr(), libc::MFD_CLOEXEC) };
        assert!(fd >= 0);
        // SAFETY: the number was just made by memfd_create and is owned here alone.
        let file = std::fs::File::from(unsafe { OwnedFd::from_raw_fd(fd) });
        file.set_len(size as u64).unwrap();
        let pool = p.program.shm.as_ref().expect("shared memory").create_pool(file.as_fd(), size, &qh, ());
        pool.create_buffer(0, w, h, w * 4, wl_shm::Format::Argb8888, &qh, ())
    }

    pub fn panel(&mut self, who: usize, name: &str) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let made = p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, None, On::Overlay, name.into(), &qh, ());
        made.set_size(200, 40);
        made.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        surface.commit();
        self.pump();
        self.show(who, &surface, 200, 40);
        surface
    }

    pub fn panel_sized(&mut self, who: usize, name: &str, on: On, w: i32, h: i32, keys: KeyboardInteractivity) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let made = p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, None, on, name.into(), &qh, ());
        made.set_size(w as u32, h as u32);
        made.set_keyboard_interactivity(keys);
        surface.commit();
        self.pump();
        self.show(who, &surface, w, h);
        surface
    }

    pub fn keyboard_panel(&mut self, who: usize, tall: i32) -> wl_surface::WlSurface {
        use zwlr_layer_surface_v1::Anchor;
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let made = p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, None, On::Top, "trindle-keyboard".into(), &qh, ());
        made.set_size(0, tall as u32);
        made.set_anchor(Anchor::Bottom | Anchor::Left | Anchor::Right);
        made.set_exclusive_zone(tall);
        surface.commit();
        self.pump();
        let wide = self.state.space.output_geometry(&self.output).unwrap().size.w;
        self.show(who, &surface, wide, tall);
        surface
    }

    pub fn closed(&self, who: usize) -> usize {
        self.programs[who].program.closed
    }

    pub fn clicked(&self, who: usize) -> &[u32] {
        &self.programs[who].program.clicked
    }

    pub fn crash(&mut self, who: usize) {
        let client = self.programs[who].client.clone();
        self.state.display_handle.backend_handle().kill_client(client.id(), smithay::reexports::wayland_server::backend::DisconnectReason::ConnectionClosed);
        self.pump();
    }

    pub fn show(&mut self, who: usize, surface: &wl_surface::WlSurface, w: i32, h: i32) {
        let picture = self.picture(who, w, h);
        surface.attach(Some(&picture), 0, 0);
        surface.commit();
        self.pump();
    }

    pub fn put_away(&mut self, surface: &wl_surface::WlSurface) {
        surface.attach(None, 0, 0);
        surface.commit();
        self.pump();
    }

    pub fn window_called(&mut self, who: usize, id: &str) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let xdg = p.program.wm.as_ref().expect("the windows' global").get_xdg_surface(&surface, &qh, ());
        let top = xdg.get_toplevel(&qh, ());
        top.set_app_id(id.into());
        surface.commit();
        self.pump();
        self.show(who, &surface, 300, 200);
        surface
    }

    pub fn boxed(&mut self) -> usize {
        let outside = self.join(ClientState::default());
        let context = smithay::wayland::security_context::SecurityContext {
            sandbox_engine: Some(crate::sandbox::BOXED.into()),
            app_id: Some("org.example.boxed".into()),
            instance_id: None,
            creator_client_id: self.programs[outside].id.clone(),
        };
        self.join(ClientState { security_context: Some(context), ..ClientState::default() })
    }

    pub fn offered(&self, who: usize) -> bool {
        self.programs[who].program.named.is_some()
    }

    pub fn named_first(&mut self, who: usize) -> Vec<bool> {
        std::mem::take(&mut self.programs[who].program.named_first)
    }

    pub fn on_screen_key(&mut self, who: usize, code: u32) {
        use smithay::input::keyboard::xkb;
        let map = xkb::Keymap::new_from_names(&xkb::Context::new(xkb::CONTEXT_NO_FLAGS), "", "", "", "", None, xkb::KEYMAP_COMPILE_NO_FLAGS)
            .expect("the usual keymap").get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
        // SAFETY: the name is a valid C string, and the flag is memfd_create's own.
        let made = unsafe { libc::memfd_create(c"rig-keymap".as_ptr(), libc::MFD_CLOEXEC) };
        assert!(made >= 0);
        // SAFETY: a fresh memfd, owned by nothing else, taken over by the File.
        let mut file = unsafe { std::fs::File::from_raw_fd(made) };
        file.write_all(map.as_bytes()).unwrap();
        file.write_all(&[0]).unwrap();
        let times = [self.next_time(), self.next_time()];
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let keyboard = p.program.typer.as_ref().expect("the made-up keyboards' global")
            .create_virtual_keyboard(p.program.seat.as_ref().expect("a seat"), &qh, ());
        keyboard.keymap(1, file.as_fd(), map.len() as u32 + 1);
        self.pump();
        for (time, pressed) in times.into_iter().zip([1, 0]) {
            keyboard.key(time, code, pressed);
            self.pump();
        }
    }

    pub fn onto_the_window_in_front(&mut self) {
        let window = self.state.space.elements().last().cloned().expect("a window is open");
        let at = self.state.space.element_geometry(&window).expect("the window has a place");
        let size = self.state.space.output_geometry(&self.output).unwrap().size;
        let (x, y) = (f64::from(at.loc.x + at.size.w / 2) / f64::from(size.w), f64::from(at.loc.y + at.size.h / 2) / f64::from(size.h));
        self.move_by_hand(x, y);
        self.click_by_hand();
        self.middle = (x, y);
    }

    pub fn window(&mut self, who: usize, full: bool) -> wl_surface::WlSurface {
        self.window_first_drawn(who, full, (300, 200))
    }

    pub fn window_first_drawn(&mut self, who: usize, full: bool, first: (i32, i32)) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let xdg = p.program.wm.as_ref().expect("the windows' global").get_xdg_surface(&surface, &qh, ());
        let top = xdg.get_toplevel(&qh, ());
        surface.commit();
        self.pump();
        self.show(who, &surface, first.0, first.1);
        if full {
            top.set_fullscreen(None);
            self.pump();
            self.show(who, &surface, 1280, 800);
        }
        surface
    }

    pub fn menu_without_opener(&mut self, who: usize) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let surface = p.program.compositor.as_ref().expect("the compositor's global").create_surface(&qh, ());
        let xdg = p.program.wm.as_ref().expect("the windows' global").get_xdg_surface(&surface, &qh, ());
        let place = p.program.wm.as_ref().expect("the windows' global").create_positioner(&qh, ());
        place.set_size(60, 40);
        place.set_anchor_rect(0, 0, 1, 1);
        let _menu = xdg.get_popup(None, &place, &qh, ());
        self.pump();
        surface
    }

    pub fn server_side(&self, who: usize, surface: &wl_surface::WlSurface) -> smithay::reexports::wayland_server::protocol::wl_surface::WlSurface {
        use wayland_client::Proxy;
        self.programs[who].client.object_from_protocol_id(&self.state.display_handle, surface.id().protocol_id()).expect("the surface on the compositor's side")
    }

    pub fn maker(&mut self, engine: Option<&str>, id: Option<&str>) -> Maker {
        let who = self.boxed_as(engine, id);
        crate::virtual_input::maker(&self.programs[who].client)
    }

    pub fn boxed_as(&mut self, engine: Option<&str>, id: Option<&str>) -> usize {
        let outside = self.join(ClientState::default());
        let context = engine.map(|engine| smithay::wayland::security_context::SecurityContext {
            sandbox_engine: Some(engine.into()),
            app_id: id.map(String::from),
            instance_id: None,
            creator_client_id: self.programs[outside].id.clone(),
        });
        self.join(ClientState { security_context: context, ..ClientState::default() })
    }

    pub fn second_screen(&mut self, name: &str) -> Output {
        let output = Output::new(
            name.into(),
            PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
        );
        let mode = Mode { size: (1280, 800).into(), refresh: 60_000 };
        output.change_current_state(Some(mode), None, None, Some((1280, 0).into()));
        output.set_preferred(mode);
        output.create_global::<AnvilState<RigData>>(&self.state.display_handle);
        self.state.space.map_output(&output, (1280, 0));
        self.pump();
        output
    }

    pub fn panel_with_area(&mut self, who: usize, name: &str, screen: usize, size: (i32, i32), area: [i32; 4]) -> wl_surface::WlSurface {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let compositor = p.program.compositor.as_ref().expect("the compositor's global");
        let surface = compositor.create_surface(&qh, ());
        let region = compositor.create_region(&qh, ());
        region.add(area[0], area[1], area[2], area[3]);
        surface.set_input_region(Some(&region));
        let on = p.program.outputs.get(screen);
        assert!(on.is_some(), "no screen {screen}");
        let made = p.program.shell.as_ref().expect("the panels' global").get_layer_surface(&surface, on, On::Overlay, name.into(), &qh, ());
        made.set_size(size.0 as u32, size.1 as u32);
        made.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        surface.commit();
        self.pump();
        self.show(who, &surface, size.0, size.1);
        surface
    }
}

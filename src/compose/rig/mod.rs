// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use smithay::backend::input::InputEvent;
use smithay::input::keyboard::LedState;
use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface as ServerSurface;
use smithay::reexports::wayland_server::Display;
use wayland_client::protocol::wl_surface;
use wayland_client::{Connection, EventQueue};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::Layer as On;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::KeyboardInteractivity;

use crate::protocols::screens::{ScreenFacts, ScreenPlan};
use crate::state::{AnvilState, Backend, ClientState};
use crate::virtual_input::{Absolute, Maker, Virtual};

const INNER: &str = "TRINDLE_RIG";

const SCREEN: &str = "rig";

const LONGEST: Duration = Duration::from_secs(90);

pub const KEY_A: u32 = 30;
pub const KEY_F5: u32 = 63;
pub const KEY_F6: u32 = 64;
pub const KEY_F7: u32 = 65;
const BTN_LEFT: u32 = 0x110;

pub fn alone(name: &str, test: impl FnOnce(&Path)) {
    if let Some(dir) = std::env::var_os(INNER) {
        test(Path::new(&dir));
        return;
    }
    let dir = std::env::temp_dir().join(format!("trindle-rig-{}-{}", std::process::id(), name.replace("::", "-")));
    let _ = std::fs::remove_dir_all(&dir);
    let (run, home, bin) = (dir.join("run"), dir.join("home"), dir.join("bin"));
    for d in [&run, &home, &bin] {
        std::fs::create_dir_all(d).unwrap();
    }
    let runs = |file: &Path, script: String| {
        std::fs::write(file, script).unwrap();
        std::fs::set_permissions(file, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
    };
    std::fs::set_permissions(&run, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();

    runs(&bin.join("trindle-ring"), format!("#!/bin/sh\necho \"ring $*\" >> {}\n", dir.join("ran").display()));
    runs(&bin.join("trindle-screenshot"), format!("#!/bin/sh\necho \"shot $*\" >> {}\n", dir.join("ran").display()));
    runs(&bin.join("wpctl"), format!("#!/bin/sh\necho \"sound $*\" >> {}\n", dir.join("ran").display()));
    runs(&bin.join("trindle-brightness"), format!("#!/bin/sh\necho \"light $*\" >> {}\n", dir.join("ran").display()));

    runs(&bin.join("trindle-settings"), format!("#!/bin/sh\necho \"settings $* own=${{TRINDLE_OWN_KEY-unsaid}}\" >> {}\n", dir.join("ran").display()));

    runs(&dir.join("trindle"), "#!/bin/sh\nwhile kill -0 $PPID 2>/dev/null; do sleep 0.2; done\n".to_string());
    let out = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env(INNER, &dir)
        .env("XDG_RUNTIME_DIR", &run)
        .env("HOME", &home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_STATE_HOME", home.join(".local/state"))
        .env("XDG_DATA_HOME", home.join(".local/share"))
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}", run.join("no-bus").display()))
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .stdin(Stdio::null())
        .stdout(std::fs::File::create(dir.join("out")).unwrap())
        .stderr(std::fs::File::create(dir.join("err")).unwrap())
        .spawn()
        .unwrap();

    let mut out = out;
    let since = Instant::now();
    let status = loop {
        match out.try_wait().unwrap() {
            Some(status) => break Some(status),
            None if since.elapsed() > LONGEST => {
                let _ = out.kill();
                let _ = out.wait();
                break None;
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    };
    let read = |file: &str| std::fs::read_to_string(dir.join(file)).unwrap_or_default();
    let said = format!("{}\n{}", read("out"), read("err"));
    let _ = std::fs::remove_dir_all(&dir);
    let Some(status) = status else { panic!("{name} had not ended in its own program after {} seconds:\n{said}", LONGEST.as_secs()) };
    assert!(status.success(), "{name} failed in its own program:\n{said}");
    assert!(said.contains("test result: ok. 1 passed"), "{name} was not run in its own program:\n{said}");
}

pub struct RigData;

impl Backend for RigData {

    const HAS_RELATIVE_MOTION: bool = true;
    fn seat_name(&self) -> String {
        "rig".into()
    }
    fn reset_buffers(&mut self, _output: &Output) {}
    fn early_import(&mut self, _surface: &ServerSurface) {}
    fn update_led_state(&mut self, _led_state: LedState) {}
    fn output_heads(_state: &AnvilState<Self>) -> Vec<ScreenFacts> {
        Vec::new()
    }
    fn apply_output_config(_state: &mut AnvilState<Self>, _targets: &[ScreenPlan], _test_only: bool) -> Result<(), String> {
        Err("the rig has no screens to arrange".into())
    }
}

mod hands;
pub mod kept_ring;
mod locks;
mod observing;
mod panels;
mod program;
mod tests;

use program::Program;

struct Joined {
    id: ClientId,
    client: smithay::reexports::wayland_server::Client,
    conn: Connection,
    queue: EventQueue<Program>,
    program: Program,
}

pub struct Rig {
    events: EventLoop<'static, AnvilState<RigData>>,
    pub state: AnvilState<RigData>,
    pub output: Output,
    programs: Vec<Joined>,
    dir: PathBuf,
    time: u32,

    pub middle: (f64, f64),
}

impl Rig {

    pub fn new(dir: &Path) -> Rig {
        assert!(std::env::var_os(INNER).is_some(), "the rig is only built in a program of its own (alone)");
        let _ = crate::session::LAUNCH.set(crate::session::Launch { ring: Some(dir.join("trindle")), ..Default::default() });
        let events = EventLoop::try_new().unwrap();
        let display = Display::new().unwrap();
        let mut state = AnvilState::init(display, events.handle(), RigData, false);
        let output = Output::new(
            SCREEN.into(),
            PhysicalProperties { size: (300, 200).into(), subpixel: Subpixel::Unknown, make: "test".into(), model: "test".into() },
        );
        let mode = Mode { size: (1280, 800).into(), refresh: 60_000 };
        output.change_current_state(Some(mode), None, None, Some((0, 0).into()));
        output.set_preferred(mode);
        output.create_global::<AnvilState<RigData>>(&state.display_handle);
        state.space.map_output(&output, (0, 0));
        Rig { events, state, output, programs: Vec::new(), dir: dir.to_path_buf(), time: 1000, middle: (0.5, 0.5) }
    }

    pub fn join(&mut self, known: ClientState) -> usize {
        let (server_end, client_end) = UnixStream::pair().unwrap();
        let client = self.state.display_handle.insert_client(server_end, Arc::new(known)).unwrap();
        let id = client.id();
        let conn = Connection::from_socket(client_end).unwrap();
        let queue = conn.new_event_queue();
        conn.display().get_registry(&queue.handle(), ());
        self.programs.push(Joined { id, client, conn, queue, program: Program::default() });
        self.pump();
        self.programs.len() - 1
    }

    pub fn pump(&mut self) {
        for _ in 0..6 {
            for p in &mut self.programs {
                // A program the compositor has shut out can send nothing.
                let _ = p.conn.flush();
            }
            self.events.dispatch(Some(Duration::from_millis(2)), &mut self.state).unwrap();
            self.state.space.refresh();
            self.state.popups.cleanup();
            let _ = self.state.display_handle.flush_clients();
            for p in &mut self.programs {
                if let Some(guard) = p.conn.prepare_read() {
                    // Nothing waiting is not an error here.
                    let _ = guard.read();
                }
                let _ = p.queue.dispatch_pending(&mut p.program);
            }
        }
    }

    pub fn until(&mut self, most: Duration, mut done: impl FnMut(&mut Rig) -> bool) -> bool {
        let since = Instant::now();
        loop {
            self.pump();
            if done(self) {
                return true;
            }
            if since.elapsed() > most {
                return false;
            }
        }
    }
}

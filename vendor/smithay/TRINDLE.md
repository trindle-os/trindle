# Smithay 0.7.0, as Trindle uses it

A copy of smithay 0.7.0 from crates.io (MIT, see LICENSE.txt), with its
examples and benchmarks left out and the thirteen changes below.
The crate it was taken from, smithay-0.7.0.crate, has the SHA-256
740cea6927892bc182d5bf70c8f79806c8bc9f68f2fb96e55a30be171b63af98, the same
as crates.io's index gives. Its dependency cgmath 0.18.0 is unmaintained and
has an unsound `swap_columns` (RUSTSEC-2026-0196 and 0197); nothing here calls
it, and deny.toml says why both are let through.

Change 1, all in `src/backend/renderer/utils/wayland.rs`:

- `set_early_shm_release(bool)`: when on, `import_surface` hands a shared
  memory buffer back to its client (wl_buffer.release) as soon as its texture
  is uploaded, as wlroots and mutter do. `InnerBuffer` remembers it went back,
  so it is not released twice, and a client attaching it again gets a fresh
  hold.

Why: GTK 4.18's cairo drawing (gdkcairocontext-wayland.c) forgets every buffer
still held when a window changes size, so the later release finds nothing and
the buffer is never freed, about 4 MB a time on a panel. Fixed upstream in GTK
commit 8a39f8e6; Debian's 4.18.6 does not have it. Smithay's own version of
this change (pull request 2104) was turned down because a second renderer may
need to read the buffer again, so the compositor turns it on only while a
single GPU draws everything.

Change 2, in `src/wayland/selection/data_device/dnd_grab.rs`
(`DnDGrab::drop`): a drag whose data source is no longer alive is never
"validated", so the surface under the pointer is sent `leave` and not `drop`.

Why: when the program something is dragged from ends, or destroys its source
to give the drag up (Escape in Trindle's own programs), smithay still told the
program under the pointer that it had been dropped there, with nothing behind
it to read. The compositor also ends such a drag at once (`dragging.rs`),
which goes through the same function.

Change 3, for Xwayland:

- `src/xwayland/xserver.rs`, `Instance::take_socket`: the display pipe
  closing with no number on it is an error, so `XWaylandEvent::Error` is
  sent. Smithay took it as "not ready yet", so an Xwayland that ended before
  it was ready was never reported, its display was left with nobody
  listening, and the closed pipe woke the event loop again and again.
- `src/xwayland/xserver.rs`, `XWaylandClientData::pid`: the Xwayland
  process's id, so the compositor can tell a stand-in still waiting from one
  that has become the real Xwayland and is starting.
- `src/xwayland/x11_sockets.rs`, `open_socket`: the X socket's queue holds
  64 connections, not 1, so several older programs started together at login
  all wait their turn while Xwayland starts.

To move to a newer smithay: copy the new release here and apply the changes
again, or drop this folder and the [patch.crates-io] entry once GTK is fixed
and smithay no longer drops for a dead source.

Change 4, for drag and drop between older (X11) programs and
Wayland ones (spec, Older programs). Smithay has no bridge between XDND and
the data device.

- `src/wayland/selection/data_device`: `DataDeviceHandler` gains `foreign_target`,
  `foreign_enter`, `foreign_motion`, `foreign_leave` and `foreign_drop`, which
  both drag grabs call in place of any data device when the surface under the
  pointer belongs to a program with none; `ServerDndGrabHandler` gains
  `drop_allowed` and `released`; `start_dnd_following` starts a compositor
  drag that keeps the pointer going to the program it began in and is ended
  by the handler, not by the last button coming up.
- `src/xwayland/xwm`: `dnd.rs` speaks XDND (both ways); the drag selection is
  a third selection beside the clipboard and the primary (`Slot`), sharing
  their transfers; transfers are capped at `MAX_TRANSFER_BYTES` (64 MiB) for
  all three, and a refused conversion (reply with no property) now ends the
  transfer instead of leaving it waiting.
- `src/xwayland/xwm/mod.rs` also makes the write end of the pipe handed to a
  Wayland program for an X11 program's request (clipboard and drag alike)
  blocking: it was non-blocking, so a program writing more than 64 KB at once
  was told to try again part way and the copy ended early.

Change 5, for older (X11) programs:

- `src/xwayland/xwm/surface.rs` (`X11Surface::configure`): a window that is
  not yet reparented (not yet shown) is sent the synthetic ConfigureNotify too
  (ICCCM 4.1.5), as one that is shown always was.
- `src/xwayland/xwm/mod.rs`: an outgoing selection transfer remembers it has
  been answered (`answered`). When the same window asks again, a transfer
  still waiting for its answer is refused as before, but one already answered
  is only cleared away: Smithay sent it a refusal, which the program took for
  the answer to its new request, so every paste after the first from one
  window came back empty.

Change 6, for typing expansion:

- `src/wayland/virtual_keyboard/virtual_keyboard_handle.rs`, `type_with_keymap`:
  the compositor types text with a keymap of its own made for those letters,
  as an on-screen keyboard does, then sends the seat's keymap back at once.
  Smithay's `send_keymap` and `for_each_focused_kbds` are crate-only.
- `src/wayland/text_input/text_input_handle.rs`, `TextInputHandle::active_purpose`:
  whether each text input is enabled and its purpose are kept as committed,
  with or without an input method running (Smithay dropped every request
  when none ran, and only passed the purpose on to one), so the compositor
  knows a password box.
  `active_sensitive` is the same for the sensitive-data hint.

Change 7, in `src/input/keyboard/keymap_file.rs`: the temporary-file path used for
  wl_keyboard versions below 7 now rewinds the file after writing (the offset is shared with the
  client's descriptor, so a client reading from the current offset got nothing),
  and also writes the closing NUL and counts it in `size`, as the sealed path does.

Change 8, in `src/backend/drm`: a virtual graphics card
(QEMU virtio, VMware, VirtualBox) hides its cursor plane from an atomic
program that does not ask for `DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT` (kernel
6.6 and later), so the pointer was drawn into every frame by the processor.
`DrmDevice::new` now asks for the capability once atomic mode is on, and
`set_cursor_hotspot(x, y)` (in `mod.rs`) says where the pointer's point is in
the picture; the atomic surface writes it to the plane's `HOTSPOT_X` and
`HOTSPOT_Y` where they exist, which is only on those cards.

Change 9, for games, in `src/xwayland/xwm`: `X11Surface::emulated_screens()`
reads `_XWAYLAND_RANDR_EMU_MONITOR_RECTS`, the screens as a program sees them
after it changed the resolution through RandR (Xwayland only pretends to, and
scales the window up with wp_viewporter when the window manager gives it that
size), with `WmWindowProperty::EmulatedScreens` when it changes; and
`X11Surface::set_iconic(bool)` writes `WM_STATE` (and `_NET_WM_STATE_HIDDEN`)
without unmapping the frame, for a program that asked to be minimised and
waits to see the answer there (Wine's full-screen games do).

Change 10, so the copy builds without warnings on newer Rust: the
unused `tracing::error` import is dropped from six files (backend/drm/surface/gbm.rs,
backend/session/libseat.rs, backend/winit/mod.rs, input/keyboard/mod.rs,
wayland/seat/keyboard.rs, xwayland/xwm/mod.rs), and wayland/shm/pool.rs casts
the SIGBUS handler through a pointer (`as *const () as libc::sighandler_t`)
rather than straight from the function to a number.

Change 11: the cursor hotspot was written only by the atomic
request builder that debug builds use (`#[cfg(debug_assertions)]`); release
builds have a second builder, which now writes `HOTSPOT_X` and `HOTSPOT_Y`
too, so the pointer's point is right on a virtual card in what ships.

Change 12, in `src/wayland/xwayland_shell.rs` and `src/xwayland/xwm/mod.rs`:
the surfaces Xwayland has committed with a `WL_SURFACE_SERIAL` are kept per
window manager (`XwmId`), taken out when an X11 window is matched with one, and
dead ones dropped; `surface_for_serial` is now `take_surface_for_serial`.

Why: every Xwayland numbers its windows from 1, and Smithay kept one map of
number to surface for all of them, never emptied. A box's Xwayland that died
and started again had its first window matched with the dead server's surface
of the same number, so the window showed nothing but its frame; with two
running at once, a window could be matched with another Xwayland's surface.

Change 13, in `src/desktop/space/mod.rs`: `Space::changes()`, a count that
goes up whenever an element is mapped, moved, raised or unmapped, an output
is mapped or unmapped, or an element enters or leaves an output in `refresh`.

Why: the panel's window list was compared with the last one on every turn of
the event loop, a pass over every window even while only the pointer moved.
With the count the compositor looks at its windows only on a turn when the
space, or a window itself, changed.

Besides the code, `README.md` lacks one line of upstream's list of compositors
built with Smithay (a desktop whose name Trindle's shipped files may not carry).

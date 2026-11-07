//! One connection to PipeWire.

use std::ffi::{c_int, c_void};
use std::os::fd::BorrowedFd;
use std::rc::Rc;

use super::native::{lib, properties, Lib, LoopControlMethods, PwLoop};

pub struct PipeWire {
    pub(super) lib: &'static Lib,
    main: *mut PwLoop,
    context: *mut c_void,
    pub(super) core: *mut c_void,
    pub(super) fd: c_int,
}

impl PipeWire {
    pub fn connect(app: &str) -> Result<Rc<PipeWire>, String> {
        let lib = lib().ok_or("PipeWire is not installed")?;
        // SAFETY: pw_init may be called more than once and takes no
        // arguments here; the loop is made, entered once on this thread (as
        // its iterate needs) and left and destroyed in Drop.
        let main = unsafe {
            (lib.init)(std::ptr::null_mut(), std::ptr::null_mut());
            (lib.loop_new)(std::ptr::null())
        };
        if main.is_null() {
            return Err("PipeWire's loop could not be made".into());
        }
        let mut pw = PipeWire { lib, main, context: std::ptr::null_mut(), core: std::ptr::null_mut(), fd: -1 };
        pw.control(|m, o| {
            if let Some(enter) = m.enter {
                // SAFETY: the loop's own enter, with its own object.
                unsafe { enter(o) };
            }
        });
        pw.fd = pw.control(|m, o| match m.get_fd {
            // SAFETY: the loop's own get_fd, with its own object.
            Some(get_fd) => unsafe { get_fd(o) },
            None => -1,
        });
        // SAFETY: a context on the loop just made, with no extra properties.
        pw.context = unsafe { (lib.context_new)(main, std::ptr::null_mut(), 0) };
        if pw.context.is_null() || pw.fd < 0 {
            return Err("PipeWire could not be started here".into());
        }
        let props = properties(lib, &[("application.name", app)]);
        // SAFETY: connects the context just made; the properties are handed over.
        pw.core = unsafe { (lib.context_connect)(pw.context, props, 0) };
        if pw.core.is_null() {
            return Err("PipeWire is not running".into());
        }
        Ok(Rc::new(pw))
    }

    fn control<T>(&self, f: impl FnOnce(&LoopControlMethods, *mut c_void) -> T) -> T {
        // SAFETY: `main` is a live pw_loop made by pw_loop_new, whose control
        // is a spa_interface whose callbacks are spa_loop_control_methods,
        // as loop.h lays them out; nothing here keeps a reference past `f`.
        unsafe {
            let iface = &*(*self.main).control;
            let methods = &*(iface.cb.funcs as *const LoopControlMethods);
            f(methods, iface.cb.data)
        }
    }

    pub fn fd(&self) -> BorrowedFd<'_> {
        // SAFETY: the loop's own descriptor, open until the loop is destroyed
        // in Drop, which the borrow of self outlives.
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }

    pub fn dispatch(&self) {
        self.control(|m, o| {
            if let Some(iterate) = m.iterate {
                // SAFETY: the loop's own iterate, entered on this thread in
                // `connect`, with a timeout of nothing.
                unsafe { iterate(o, 0) };
            }
        });
    }
}

impl Drop for PipeWire {
    fn drop(&mut self) {
        // SAFETY: each was made in `connect` and is undone once, in the
        // reverse order; every stream holds an Rc of this, so all are gone.
        unsafe {
            if !self.core.is_null() {
                (self.lib.core_disconnect)(self.core);
            }
            if !self.context.is_null() {
                (self.lib.context_destroy)(self.context);
            }
        }
        self.control(|m, o| {
            if let Some(leave) = m.leave {
                // SAFETY: leaves the loop entered in `connect`.
                unsafe { leave(o) };
            }
        });
        // SAFETY: made in `connect`, destroyed once.
        unsafe { (self.lib.loop_destroy)(self.main) };
    }
}

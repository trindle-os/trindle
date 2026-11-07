use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use tracing::{debug, info, warn};

use rustix::{io::Errno, net::SocketAddrUnix};

pub(crate) fn prepare_x11_sockets(
    display: Option<u32>,
    open_abstract_socket: bool,
) -> Result<(X11Lock, Vec<UnixStream>), std::io::Error> {
    match display {
        Some(d) => {
            if let Ok(lock) = X11Lock::grab(d) {
                match open_x11_sockets_for_display(d, open_abstract_socket) {
                    Ok(sockets) => return Ok((lock, sockets)),
                    Err(err) => return Err(std::io::Error::from(err)),
                };
            }
        }
        None => {
            for d in 0..33 {
                if let Ok(lock) = X11Lock::grab(d) {
                    match open_x11_sockets_for_display(d, open_abstract_socket) {
                        Ok(sockets) => return Ok((lock, sockets)),
                        Err(err) => warn!(display = d, "Failed to create sockets: {}", err),
                    }
                }
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrInUse,
        "Could not find a free socket for the XServer.",
    ))
}

#[derive(Debug)]
pub(crate) struct X11Lock {
    display: u32,
}

impl X11Lock {
    fn grab(number: u32) -> Result<X11Lock, ()> {
        debug!(display = number, "Attempting to aquire an X11 display lock");
        let filename = format!("/tmp/.X{}-lock", number);
        let lockfile = ::std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename);
        match lockfile {
            Ok(mut file) => {
                let ret = file.write_fmt(format_args!(
                    "{:>10}\n",
                    rustix::process::Pid::as_raw(Some(rustix::process::getpid()))
                ));
                if ret.is_err() {
                    ::std::mem::drop(file);
                    let _ = ::std::fs::remove_file(&filename);
                    Err(())
                } else {
                    debug!(display = number, "X11 lock acquired");
                    Ok(X11Lock { display: number })
                }
            }
            Err(_) => {
                debug!(display = number, "Failed to acquire lock");
                let mut file = ::std::fs::File::open(&filename).map_err(|_| ())?;
                let mut spid = [0u8; 11];
                file.read_exact(&mut spid).map_err(|_| ())?;
                ::std::mem::drop(file);
                let pid = rustix::process::Pid::from_raw(
                    ::std::str::from_utf8(&spid)
                        .map_err(|_| ())?
                        .trim()
                        .parse::<i32>()
                        .map_err(|_| ())?,
                )
                .ok_or(())?;
                if let Err(Errno::SRCH) = rustix::process::test_kill_process(pid) {
                    if let Ok(()) = ::std::fs::remove_file(filename) {
                        debug!(
                            display = number,
                            "Lock was blocked by a defunct X11 server, trying again"
                        );
                        return X11Lock::grab(number);
                    } else {
                        return Err(());
                    }
                }
                Err(())
            }
        }
    }

    pub(crate) fn display_number(&self) -> u32 {
        self.display
    }
}

impl Drop for X11Lock {
    fn drop(&mut self) {
        info!("Cleaning up X11 lock.");
        if let Err(e) = ::std::fs::remove_file(format!("/tmp/.X11-unix/X{}", self.display)) {
            warn!(error = ?e, "Failed to remove X11 socket");
        }
        if let Err(e) = ::std::fs::remove_file(format!("/tmp/.X{}-lock", self.display)) {
            warn!(error = ?e, "Failed to remove X11 lockfile");
        }
    }
}

#[cfg(target_os = "linux")]
fn open_x11_sockets_for_display(
    display: u32,
    open_abstract_socket: bool,
) -> rustix::io::Result<Vec<UnixStream>> {
    let path = format!("/tmp/.X11-unix/X{}", display);
    let _ = ::std::fs::remove_file(&path);
    let fs_addr = SocketAddrUnix::new(path.as_bytes()).unwrap();
    let mut sockets = vec![open_socket(fs_addr)?];
    if open_abstract_socket {
        let abs_addr = SocketAddrUnix::new_abstract_name(path.as_bytes()).unwrap();
        sockets.push(open_socket(abs_addr)?);
    }
    Ok(sockets)
}

#[cfg(not(target_os = "linux"))]
fn open_x11_sockets_for_display(
    display: u32,
    _open_abstract_socket: bool,
) -> rustix::io::Result<Vec<UnixStream>> {
    let path = format!("/tmp/.X11-unix/X{}", display);
    let _ = ::std::fs::remove_file(&path);
    let fs_addr = SocketAddrUnix::new(path.as_bytes()).unwrap();
    Ok(vec![open_socket(fs_addr)?])
}

fn open_socket(addr: SocketAddrUnix) -> rustix::io::Result<UnixStream> {
    let fd = rustix::net::socket_with(
        rustix::net::AddressFamily::UNIX,
        rustix::net::SocketType::STREAM,
        rustix::net::SocketFlags::CLOEXEC,
        None,
    )?;
    rustix::net::bind(&fd, &addr)?;
    rustix::net::listen(&fd, 64)?;
    Ok(UnixStream::from(fd))
}

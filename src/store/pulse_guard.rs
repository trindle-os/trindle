//! Sound without the microphone. A box that may play sound but not record is
//! given this guard's socket in place of the sound server's: everything it
//! says is passed through, except a request to record, or to load a module
//! that could record for it, which is answered "not allowed". Shared memory
//! is turned off in the greeting, so all sound goes through the guard, where
//! it can be seen.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const COMMAND: u32 = 0xffff_ffff;
const ERROR: u32 = 0;
const CREATE_RECORD_STREAM: u32 = 5;
const EXIT: u32 = 7;
const AUTH: u32 = 8;
const LOAD_MODULE: u32 = 51;
const UNLOAD_MODULE: u32 = 52;

const REFUSED: &[u32] = &[CREATE_RECORD_STREAM, EXIT, LOAD_MODULE, UNLOAD_MODULE];

const NOT_ALLOWED: u32 = 1;

const SHARED_MEMORY: u32 = 0x8000_0000 | 0x4000_0000;

const LARGEST: usize = 16 << 20;
const TAG_U32: u8 = b'L';

pub fn real_socket() -> PathBuf {
    runtime_dir().join("pulse/native")
}

pub fn runtime_dir() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from)
        // SAFETY: getuid cannot fail and touches no memory.
        .unwrap_or_else(|| PathBuf::from(format!("/run/user/{}", unsafe { libc::getuid() })))
}

pub fn serve(listener: UnixListener, real: PathBuf) {
    for client in listener.incoming().flatten() {
        let real = real.clone();
        std::thread::spawn(move || relay(client, &real));
    }
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    b.get(at..at + 4).map(|s| u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

fn command_of(payload: &[u8]) -> Option<(u32, u32)> {
    if payload.first() != Some(&TAG_U32) || payload.get(5) != Some(&TAG_U32) {
        return None;
    }
    Some((u32_at(payload, 1)?, u32_at(payload, 6)?))
}

pub fn judge(payload: &mut [u8]) -> Result<(), u32> {
    let Some((cmd, tag)) = command_of(payload) else { return Ok(()) };
    if REFUSED.contains(&cmd) {
        return Err(tag);
    }
    if cmd == AUTH && payload.get(10) == Some(&TAG_U32) {
        if let Some(v) = u32_at(payload, 11) {
            payload[11..15].copy_from_slice(&(v & !SHARED_MEMORY).to_be_bytes());
        }
    }
    Ok(())
}

fn frame(channel: u32, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + payload.len());
    for v in [payload.len() as u32, channel, 0, 0, 0] {
        out.extend_from_slice(&v.to_be_bytes());
    }
    out.extend_from_slice(payload);
    out
}

pub fn refusal(tag: u32) -> Vec<u8> {
    let mut p = Vec::new();
    for v in [ERROR, tag, NOT_ALLOWED] {
        p.push(TAG_U32);
        p.extend_from_slice(&v.to_be_bytes());
    }
    frame(COMMAND, &p)
}

fn read_frame(from: &mut impl Read) -> Option<([u8; 20], Vec<u8>)> {
    let mut head = [0u8; 20];
    from.read_exact(&mut head).ok()?;
    let len = u32_at(&head, 0)? as usize;
    if len > LARGEST {
        return None;
    }
    let mut body = vec![0u8; len];
    from.read_exact(&mut body).ok()?;
    Some((head, body))
}

fn relay(mut client: UnixStream, real: &Path) {
    let Ok(mut server) = UnixStream::connect(real) else { return };
    let (Ok(mut server_in), Ok(client_out)) = (server.try_clone(), client.try_clone()) else { return };
    let back = Arc::new(Mutex::new(client_out));
    let back2 = back.clone();

    let down = std::thread::spawn(move || {
        while let Some((head, body)) = read_frame(&mut server_in) {
            let Ok(mut out) = back2.lock() else { break };
            if out.write_all(&head).and_then(|_| out.write_all(&body)).is_err() {
                break;
            }
        }
        if let Ok(out) = back2.lock() {
            let _ = out.shutdown(std::net::Shutdown::Both);
        }
    });
    while let Some((head, mut body)) = read_frame(&mut client) {
        if u32_at(&head, 4) == Some(COMMAND) {
            if let Err(tag) = judge(&mut body) {
                let Ok(mut out) = back.lock() else { break };
                if out.write_all(&refusal(tag)).is_err() {
                    break;
                }
                continue;
            }
        }
        if server.write_all(&head).and_then(|_| server.write_all(&body)).is_err() {
            break;
        }
    }
    let _ = server.shutdown(std::net::Shutdown::Both);
    let _ = down.join();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(cmd: u32, tag: u32, rest: &[u8]) -> Vec<u8> {
        let mut p = vec![TAG_U32];
        p.extend_from_slice(&cmd.to_be_bytes());
        p.push(TAG_U32);
        p.extend_from_slice(&tag.to_be_bytes());
        p.extend_from_slice(rest);
        frame(COMMAND, &p)
    }

    #[test]
    fn a_box_without_the_microphone_cannot_record() {
        let dir = std::env::temp_dir().join(format!("trindle-pulse-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let (real, guard) = (dir.join("real"), dir.join("guard"));
        let server = UnixListener::bind(&real).unwrap();
        let listener = UnixListener::bind(&guard).unwrap();
        let real2 = real.clone();
        std::thread::spawn(move || serve(listener, real2));
        let mut client = UnixStream::connect(&guard).unwrap();
        let (mut seen, _) = server.accept().unwrap();
        seen.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
        client.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();

        let mut auth = vec![TAG_U32];
        auth.extend_from_slice(&(35u32 | SHARED_MEMORY).to_be_bytes());
        client.write_all(&command(AUTH, 1, &auth)).unwrap();
        let (_, body) = read_frame(&mut seen).unwrap();
        assert_eq!(command_of(&body), Some((AUTH, 1)));
        assert_eq!(u32_at(&body, 11), Some(35));

        client.write_all(&command(CREATE_RECORD_STREAM, 7, &[])).unwrap();
        let (head, body) = read_frame(&mut client).unwrap();
        assert_eq!(u32_at(&head, 4), Some(COMMAND));
        assert_eq!(command_of(&body), Some((ERROR, 7)));
        assert_eq!(u32_at(&body, 11), Some(NOT_ALLOWED));
        client.write_all(&command(LOAD_MODULE, 8, &[])).unwrap();
        let (_, body) = read_frame(&mut client).unwrap();
        assert_eq!(command_of(&body), Some((ERROR, 8)));

        client.write_all(&command(3, 9, &[])).unwrap();
        let (_, body) = read_frame(&mut seen).unwrap();
        assert_eq!(command_of(&body), Some((3, 9)), "the record request was never passed on");
        seen.write_all(&command(2, 9, &[])).unwrap();
        let (_, body) = read_frame(&mut client).unwrap();
        assert_eq!(command_of(&body), Some((2, 9)));

        client.write_all(&frame(0, &[1, 2, 3])).unwrap();
        let (head, body) = read_frame(&mut seen).unwrap();
        assert_eq!((u32_at(&head, 4), body), (Some(0), vec![1, 2, 3]));
        let _ = std::fs::remove_dir_all(&dir);
    }
}

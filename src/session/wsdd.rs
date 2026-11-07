//! trindle-wsdd.

mod finder;
mod host;
mod http;
mod limits;
mod message;
mod net;
#[cfg(test)]
mod tests;
mod xml;

use std::net::Ipv4Addr;
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use finder::Finder;
use limits::started;

const GROUP: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const PORT: u16 = 3702;

const SOAP: &str = "http://www.w3.org/2003/05/soap-envelope";
const WSA: &str = "http://schemas.xmlsoap.org/ws/2004/08/addressing";
const WSD: &str = "http://schemas.xmlsoap.org/ws/2005/04/discovery";
const WSDP: &str = "http://schemas.xmlsoap.org/ws/2006/02/devprof";
const DISCOVERY_TO: &str = "urn:schemas-xmlsoap-org:ws:2005:04:discovery";
const GET: &str = "http://schemas.xmlsoap.org/ws/2004/09/transfer/Get";
const ANONYMOUS: &str = "http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous";
const PUB: &str = "http://schemas.microsoft.com/windows/pub/2005/07";
const MEX: &str = "http://schemas.xmlsoap.org/ws/2004/09/mex";
const PNPX: &str = "http://schemas.microsoft.com/windows/pnpx/2005/10";
const OUR_TYPES: &str = "wsdp:Device pub:Computer";
const HTTP_PORT: u16 = 5357;
const HOST_CHECK: Duration = Duration::from_secs(5);
const REQUEST_MOST: usize = 64 * 1024;

const PROBE_KEEP: Duration = Duration::from_secs(20);
const REPEATS: u32 = 4;
const HTTP_WAIT: Duration = Duration::from_secs(2);
const HTTP_MOST: u64 = 1 << 20;

fn clear_socket(path: &std::path::Path) {
    if std::fs::symlink_metadata(path).is_ok_and(|m| std::os::unix::fs::FileTypeExt::is_socket(&m.file_type())) {
        let _ = std::fs::remove_file(path);
    }
}

pub fn main(args: Vec<String>) -> i32 {
    let mut path = None;
    let mut rest = args.iter().skip(1);
    while let Some(a) = rest.next() {
        if a == "--listen" || a == "-l" {
            path = rest.next().map(PathBuf::from);
        }
    }
    let path = match path.or_else(default_socket) {
        Some(p) => p,
        None => {
            eprintln!("trindle-wsdd: no XDG_RUNTIME_DIR");
            return 1;
        }
    };
    if UnixStream::connect(&path).is_ok() {
        return 0;
    }
    clear_socket(&path);
    let listener = match UnixListener::bind(&path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("trindle-wsdd: {}: {e}", path.display());
            return 1;
        }
    };
    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));

    let finder = Arc::new(Finder::new());
    let host_side = finder.clone();
    if !started(move || host_side.keep_hosting()) {
        return 1;
    }
    let answering = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let resting = answering.clone();
    crate::common::renewed::watch_apart("trindle-wsdd", move || resting.load(std::sync::atomic::Ordering::SeqCst) == 0);
    for conn in listener.incoming().flatten() {
        let (finder, answering) = (finder.clone(), answering.clone());
        answering.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let done = answering.clone();
        if !started(move || {
            finder.serve(conn);
            done.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }) {
            answering.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    0
}

fn default_socket() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?).join("gvfsd");
    let _ = std::fs::DirBuilder::new().mode(0o700).create(&dir);
    Some(dir.join("wsdd"))
}

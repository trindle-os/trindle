//! The sockets, one pair for each network this computer is on.

use std::{io, thread};
use std::ffi::CStr;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::time::Duration;

use super::{GROUP, PORT, REPEATS};
use super::message::random_byte;

pub(super) fn shout(send: &UdpSocket, msg: &str) {
    let to = SocketAddrV4::new(GROUP, PORT);
    let mut gap = 50 + u64::from(random_byte()) % 200;
    for n in 0..REPEATS {
        let _ = send.send_to(msg.as_bytes(), to);
        if n + 1 < REPEATS {
            thread::sleep(Duration::from_millis(gap));
            gap = (gap * 2).min(500);
        }
    }
}

pub(super) struct Iface {
    pub(super) name: String,
    pub(super) addr: Ipv4Addr,
    index: i32,
}

pub(super) fn interfaces() -> Vec<Iface> {
    let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
    // SAFETY: getifaddrs fills head with a list that is freed below.
    if unsafe { libc::getifaddrs(&mut head) } != 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut p = head;
    while !p.is_null() {
        // SAFETY: p is a node of the list getifaddrs gave, not yet freed.
        let ifa = unsafe { &*p };
        p = ifa.ifa_next;
        let flags = ifa.ifa_flags as i32;
        if ifa.ifa_addr.is_null()
            || flags & libc::IFF_UP == 0
            || flags & libc::IFF_MULTICAST == 0
            || flags & libc::IFF_LOOPBACK != 0
        {
            continue;
        }
        // SAFETY: ifa_addr is not null and points into the same list.
        if i32::from(unsafe { (*ifa.ifa_addr).sa_family }) != libc::AF_INET {
            continue;
        }
        // SAFETY: the family says this address is a sockaddr_in.
        let sin = unsafe { &*(ifa.ifa_addr as *const libc::sockaddr_in) };
        // SAFETY: ifa_name is a nul-terminated string in the same list.
        let name = unsafe { CStr::from_ptr(ifa.ifa_name) };
        // SAFETY: name is nul-terminated and lives for the call.
        let index = unsafe { libc::if_nametoindex(name.as_ptr()) } as i32;
        if index == 0 {
            continue;
        }
        out.push(Iface {
            name: name.to_string_lossy().into_owned(),
            addr: Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr)),
            index,
        });
    }
    // SAFETY: head came from getifaddrs and is freed once, after its last use.
    unsafe { libc::freeifaddrs(head) };
    out
}

fn setopt<T>(fd: RawFd, level: i32, name: i32, value: &T) -> io::Result<()> {
    // SAFETY: value is a live T and the length given is its size.
    let r = unsafe {
        libc::setsockopt(
            fd,
            level,
            name,
            (value as *const T).cast(),
            std::mem::size_of::<T>() as libc::socklen_t,
        )
    };
    if r == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
}

fn mreqn(iface: &Iface, group: Ipv4Addr) -> libc::ip_mreqn {
    libc::ip_mreqn {
        imr_multiaddr: libc::in_addr { s_addr: u32::from(group).to_be() },
        imr_address: libc::in_addr { s_addr: u32::from(iface.addr).to_be() },
        imr_ifindex: iface.index,
    }
}

pub(super) fn listen_socket(iface: &Iface) -> io::Result<UdpSocket> {
    // SAFETY: a plain socket call; its result is checked.
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: fd was just made and nothing else owns it.
    let sock = unsafe { UdpSocket::from_raw_fd(fd) };
    setopt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, &1i32)?;
    setopt(fd, libc::IPPROTO_IP, libc::IP_MULTICAST_ALL, &0i32)?;
    setopt(fd, libc::IPPROTO_IP, libc::IP_ADD_MEMBERSHIP, &mreqn(iface, GROUP))?;
    let at = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: PORT.to_be(),
        sin_addr: libc::in_addr { s_addr: u32::from(GROUP).to_be() },
        sin_zero: [0; 8],
    };
    // SAFETY: at is a live sockaddr_in and the length given is its size.
    let r = unsafe {
        libc::bind(
            fd,
            (&at as *const libc::sockaddr_in).cast(),
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(sock)
}

pub(super) fn send_socket(iface: &Iface) -> io::Result<UdpSocket> {
    let sock = UdpSocket::bind(SocketAddrV4::new(iface.addr, 0))?;
    setopt(sock.as_raw_fd(), libc::IPPROTO_IP, libc::IP_MULTICAST_IF, &mreqn(iface, Ipv4Addr::UNSPECIFIED))?;
    sock.set_multicast_ttl_v4(1)?;
    sock.set_multicast_loop_v4(true)?;
    Ok(sock)
}

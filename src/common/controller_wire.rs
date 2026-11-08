//! The words between the compositor and trindle-controller, the root service that makes game controllers.


use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

pub const SOCKET: &str = "/run/trindle/controller.sock";

pub const NAME: &str = "Trindle virtual controller";

pub const EV_SYN: u16 = 0;
pub const EV_KEY: u16 = 1;
pub const EV_ABS: u16 = 3;

pub const SLOTS: u8 = 8;

pub const MOST_KEYS: usize = 16 + 15 + 4 + 40;
pub const MOST_AXES: usize = 11 + 8;
pub const MOST_EVENTS: usize = 64;
const LONGEST_MAKE: usize = 12 + MOST_KEYS * 2 + MOST_AXES * 18;
const LONGEST_SEND: usize = 4 + MOST_EVENTS * 8;
pub const LONGEST: usize = if LONGEST_MAKE > LONGEST_SEND { LONGEST_MAKE } else { LONGEST_SEND };

const MAKE: u8 = b'M';
const SEND: u8 = b'S';
const REMOVE: u8 = b'R';
const YES: u8 = b'K';
const NO: u8 = b'N';
pub const WHOLE: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refusal {
    Malformed = 1,
    NotAPadButton = 2,
    NotAPadAxis = 3,
    NoPadButton = 4,
    Twice = 5,
    BadRange = 6,
    BadBus = 7,
    BadSlot = 8,
    SlotInUse = 9,
    TooMany = 10,
    TooFast = 11,
    NotAllowed = 12,
    NotMade = 13,
    Failed = 14,
}

impl Refusal {
    fn from_u8(b: u8) -> Refusal {
        use Refusal::*;
        [Malformed, NotAPadButton, NotAPadAxis, NoPadButton, Twice, BadRange, BadBus, BadSlot, SlotInUse, TooMany, TooFast, NotAllowed, NotMade, Failed]
            .into_iter()
            .find(|r| *r as u8 == b)
            .unwrap_or(Malformed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axis {
    pub code: u16,
    pub min: i32,
    pub max: i32,
    pub fuzz: i32,
    pub flat: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pad {
    pub bus: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
    pub keys: Vec<u16>,
    pub axes: Vec<Axis>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub kind: u16,
    pub code: u16,
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Make { slot: u8, pad: Pad },
    Send { slot: u8, events: Vec<Event> },
    Remove { slot: u8 },
}

pub fn is_pad_button(code: u16) -> bool {
    matches!(code, 0x120..=0x12f | 0x130..=0x13e | 0x220..=0x223 | 0x2c0..=0x2e7)
}

pub fn is_pad_axis(code: u16) -> bool {
    matches!(code, 0x00..=0x0a | 0x10..=0x17)
}

fn is_pad_bus(bus: u16) -> bool {
    matches!(bus, 0x03 | 0x05 | 0x06)
}

impl Pad {

    pub fn check(&self) -> Result<(), Refusal> {
        if !is_pad_bus(self.bus) {
            return Err(Refusal::BadBus);
        }
        if self.keys.iter().any(|k| !is_pad_button(*k)) {
            return Err(Refusal::NotAPadButton);
        }
        if !self.keys.iter().any(|k| matches!(k, 0x120..=0x13e)) {
            return Err(Refusal::NoPadButton);
        }
        if self.axes.iter().any(|a| !is_pad_axis(a.code)) {
            return Err(Refusal::NotAPadAxis);
        }
        let mut keys = self.keys.clone();
        keys.sort_unstable();
        keys.dedup();
        let mut axes: Vec<u16> = self.axes.iter().map(|a| a.code).collect();
        axes.sort_unstable();
        axes.dedup();
        if keys.len() != self.keys.len() || axes.len() != self.axes.len() {
            return Err(Refusal::Twice);
        }
        for a in &self.axes {
            let span = a.max as i64 - a.min as i64;
            if span <= 0 || !(0..=span).contains(&(a.fuzz as i64)) || !(0..=span).contains(&(a.flat as i64)) {
                return Err(Refusal::BadRange);
            }
        }
        Ok(())
    }
}

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N], Refusal> {
        let (head, rest) = self.0.split_first_chunk::<N>().ok_or(Refusal::Malformed)?;
        self.0 = rest;
        Ok(*head)
    }
    fn u8(&mut self) -> Result<u8, Refusal> {
        Ok(self.take::<1>()?[0])
    }
    fn u16(&mut self) -> Result<u16, Refusal> {
        Ok(u16::from_le_bytes(self.take()?))
    }
    fn i32(&mut self) -> Result<i32, Refusal> {
        Ok(i32::from_le_bytes(self.take()?))
    }
}

pub fn parse(frame: &[u8]) -> Result<Request, Refusal> {
    if frame.len() > LONGEST {
        return Err(Refusal::Malformed);
    }
    let mut r = Reader(frame);
    let kind = r.u8()?;
    let slot = r.u8()?;
    let req = match kind {
        MAKE => {
            let (nkeys, naxes) = (r.u8()? as usize, r.u8()? as usize);
            if nkeys > MOST_KEYS || naxes > MOST_AXES {
                return Err(Refusal::Malformed);
            }
            let (bus, vendor, product, version) = (r.u16()?, r.u16()?, r.u16()?, r.u16()?);
            let keys = (0..nkeys).map(|_| r.u16()).collect::<Result<_, _>>()?;
            let axes = (0..naxes)
                .map(|_| Ok(Axis { code: r.u16()?, min: r.i32()?, max: r.i32()?, fuzz: r.i32()?, flat: r.i32()? }))
                .collect::<Result<_, _>>()?;
            Request::Make { slot, pad: Pad { bus, vendor, product, version, keys, axes } }
        }
        SEND => {
            let count = r.u16()? as usize;
            if count == 0 || count > MOST_EVENTS {
                return Err(Refusal::Malformed);
            }
            let events = (0..count).map(|_| Ok(Event { kind: r.u16()?, code: r.u16()?, value: r.i32()? })).collect::<Result<_, _>>()?;
            Request::Send { slot, events }
        }
        REMOVE => Request::Remove { slot },
        _ => return Err(Refusal::Malformed),
    };
    if !r.0.is_empty() {
        return Err(Refusal::Malformed);
    }
    if slot >= SLOTS {
        return Err(Refusal::BadSlot);
    }
    if let Request::Make { pad, .. } = &req {
        pad.check()?;
    }
    Ok(req)
}

pub fn encode(req: &Request) -> Vec<u8> {
    let mut out = Vec::new();
    match req {
        Request::Make { slot, pad } => {
            out.extend([MAKE, *slot, pad.keys.len() as u8, pad.axes.len() as u8]);
            for n in [pad.bus, pad.vendor, pad.product, pad.version] {
                out.extend(n.to_le_bytes());
            }
            pad.keys.iter().for_each(|k| out.extend(k.to_le_bytes()));
            for a in &pad.axes {
                out.extend(a.code.to_le_bytes());
                [a.min, a.max, a.fuzz, a.flat].iter().for_each(|n| out.extend(n.to_le_bytes()));
            }
        }
        Request::Send { slot, events } => {
            out.extend([SEND, *slot]);
            out.extend((events.len() as u16).to_le_bytes());
            for e in events {
                out.extend(e.kind.to_le_bytes());
                out.extend(e.code.to_le_bytes());
                out.extend(e.value.to_le_bytes());
            }
        }
        Request::Remove { slot } => out.extend([REMOVE, *slot]),
    }
    out
}

pub fn answer(slot: u8, result: Result<(), Refusal>) -> Vec<u8> {
    match result {
        Ok(()) => vec![YES, slot],
        Err(why) => vec![NO, slot, why as u8],
    }
}

fn read_answer(frame: &[u8]) -> Result<(), Refusal> {
    match frame {
        [YES, _] => Ok(()),
        [NO, _, why] => Err(Refusal::from_u8(*why)),
        _ => Err(Refusal::Malformed),
    }
}

fn check(r: libc::c_int) -> io::Result<libc::c_int> {
    if r < 0 { Err(io::Error::last_os_error()) } else { Ok(r) }
}

pub fn pair() -> io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [0; 2];
    // SAFETY: fds is two ints for the kernel to fill.
    check(unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0, fds.as_mut_ptr()) })?;
    // SAFETY: both are fresh descriptors nothing else owns.
    let ends = unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) };
    for end in [&ends.0, &ends.1] {
        pass_credentials(end.as_raw_fd());
    }
    Ok(ends)
}

pub fn pass_credentials(fd: RawFd) {
    let on: libc::c_int = 1;
    // SAFETY: on is a valid int for the length given.
    unsafe { libc::setsockopt(fd, libc::SOL_SOCKET, libc::SO_PASSCRED, (&on as *const libc::c_int).cast(), std::mem::size_of::<libc::c_int>() as libc::socklen_t) };
}

pub fn send_frame(fd: RawFd, frame: &[u8]) -> io::Result<()> {
    // SAFETY: frame is valid for its length; MSG_NOSIGNAL so a closed peer is an error, not a signal.
    let n = unsafe { libc::send(fd, frame.as_ptr().cast(), frame.len(), libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT) };
    if n < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}

pub fn recv_frame(fd: RawFd, buf: &mut [u8]) -> io::Result<Option<usize>> {
    // SAFETY: buf is valid for writing its length; MSG_TRUNC returns the whole frame's length.
    let n = unsafe { libc::recv(fd, buf.as_mut_ptr().cast(), buf.len(), libc::MSG_TRUNC) };
    match n {
        n if n < 0 => Err(io::Error::last_os_error()),
        0 => Ok(None),
        n if n as usize > buf.len() => Err(io::Error::new(io::ErrorKind::InvalidData, "frame too long")),
        n => Ok(Some(n as usize)),
    }
}

pub fn recv_frame_from(fd: RawFd, buf: &mut [u8]) -> io::Result<Option<(usize, Option<i32>)>> {
    let mut iov = libc::iovec { iov_base: buf.as_mut_ptr().cast(), iov_len: buf.len() };
    let mut control = [0u64; 8];
    // SAFETY: CMSG_SPACE is plain arithmetic.
    let room = unsafe { libc::CMSG_SPACE(std::mem::size_of::<libc::ucred>() as u32) } as usize;
    debug_assert!(room <= std::mem::size_of_val(&control));
    // SAFETY: msghdr is plain numbers; all zero is a valid start.
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control.as_mut_ptr().cast();
    msg.msg_controllen = room as _;
    // SAFETY: msg points at buffers that are valid for writing their lengths; MSG_TRUNC returns the whole frame's length.
    let n = unsafe { libc::recvmsg(fd, &mut msg, libc::MSG_TRUNC) };
    if n < 0 {
        return Err(io::Error::last_os_error());
    }
    if n == 0 {
        return Ok(None);
    }
    if n as usize > buf.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too long"));
    }
    let mut sender = None;
    // SAFETY: the kernel filled msg's control area; each header is read only inside it.
    unsafe {
        let mut c = libc::CMSG_FIRSTHDR(&msg);
        while !c.is_null() {
            if (*c).cmsg_level == libc::SOL_SOCKET && (*c).cmsg_type == libc::SCM_CREDENTIALS {
                sender = Some(std::ptr::read_unaligned(libc::CMSG_DATA(c).cast::<libc::ucred>()).pid);
            }
            c = libc::CMSG_NXTHDR(&msg, c);
        }
    }
    Ok(Some((n as usize, sender)))
}

pub const ANSWER_WAIT_MS: libc::suseconds_t = 250;

pub struct Client {
    fd: OwnedFd,
}

impl Client {
    pub fn connect() -> io::Result<Client> {
        // SAFETY: no pointers; the result is checked before it is owned.
        let fd = check(unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) })?;
        // SAFETY: fd is fresh and nothing else owns it.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        // SAFETY: sockaddr_un is plain numbers; all zero is a valid start.
        let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
        addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
        for (d, s) in addr.sun_path.iter_mut().zip(SOCKET.bytes()) {
            *d = s as libc::c_char;
        }
        let len = std::mem::size_of::<libc::sa_family_t>() + SOCKET.len() + 1;
        // SAFETY: addr is a whole sockaddr_un and len lies within it.
        check(unsafe { libc::connect(fd.as_raw_fd(), (&addr as *const libc::sockaddr_un).cast(), len as libc::socklen_t) })?;
        Client::waiting(fd, libc::timeval { tv_sec: 0, tv_usec: ANSWER_WAIT_MS * 1000 })
    }

    pub fn from_fd(fd: OwnedFd) -> io::Result<Client> {
        Client::waiting(fd, libc::timeval { tv_sec: 2, tv_usec: 0 })
    }

    fn waiting(fd: OwnedFd, wait: libc::timeval) -> io::Result<Client> {
        // SAFETY: wait is a whole timeval that lives through the call.
        check(unsafe {
            libc::setsockopt(fd.as_raw_fd(), libc::SOL_SOCKET, libc::SO_RCVTIMEO, (&wait as *const libc::timeval).cast(), std::mem::size_of::<libc::timeval>() as libc::socklen_t)
        })?;
        Ok(Client { fd })
    }

    fn ask(&self, req: &Request) -> io::Result<Result<(), Refusal>> {
        send_frame(self.fd.as_raw_fd(), &encode(req))?;
        let mut buf = [0u8; 8];
        match recv_frame(self.fd.as_raw_fd(), &mut buf)? {
            Some(n) => Ok(read_answer(&buf[..n])),
            None => Err(io::ErrorKind::ConnectionReset.into()),
        }
    }

    pub fn make(&self, slot: u8, pad: &Pad) -> io::Result<Result<(), Refusal>> {
        self.ask(&Request::Make { slot, pad: pad.clone() })
    }

    pub fn send(&self, slot: u8, events: &[Event]) -> io::Result<()> {
        for chunk in events.chunks(MOST_EVENTS) {
            send_frame(self.fd.as_raw_fd(), &encode(&Request::Send { slot, events: chunk.to_vec() }))?;
        }
        Ok(())
    }

    pub fn remove(&self, slot: u8) -> io::Result<Result<(), Refusal>> {
        self.ask(&Request::Remove { slot })
    }
}

pub fn double() -> io::Result<(Client, std::thread::JoinHandle<Vec<Request>>)> {
    let (ours, theirs) = pair()?;
    let seen = std::thread::spawn(move || {
        let mut got = Vec::new();
        let mut buf = [0u8; LONGEST];
        while let Ok(Some(n)) = recv_frame(theirs.as_raw_fd(), &mut buf) {
            let Ok(req) = parse(&buf[..n]) else { break };
            if let Request::Make { slot, .. } | Request::Remove { slot } = req {
                let _ = send_frame(theirs.as_raw_fd(), &answer(slot, Ok(())));
            }
            got.push(req);
        }
        got
    });
    Ok((Client::from_fd(ours)?, seen))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_compositor_waits_only_briefly_for_an_answer() {
        let (ours, _silent) = pair().unwrap();
        let client = Client::waiting(ours, libc::timeval { tv_sec: 0, tv_usec: ANSWER_WAIT_MS * 1000 }).unwrap();
        let pad = Pad { bus: 3, vendor: 1, product: 2, version: 1, keys: vec![0x130], axes: Vec::new() };
        let started = std::time::Instant::now();
        assert!(client.make(0, &pad).is_err(), "no answer is an error, never a yes");
        let took = started.elapsed();
        assert!(took >= std::time::Duration::from_millis(ANSWER_WAIT_MS as u64 / 2) && took < std::time::Duration::from_secs(1), "{took:?}");
    }
}

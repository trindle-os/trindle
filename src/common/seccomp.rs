//! The short list of kernel calls a box refuses, the same list Flatpak uses
//! (risky requests blocked). It is written
//! as a classic BPF program by hand, with libc only, and handed to bubblewrap
//! with `--seccomp FD`.

/// What a refused call gives back.
const EPERM: u32 = libc::EPERM as u32;
const ENOSYS: u32 = libc::ENOSYS as u32;
const EAFNOSUPPORT: u32 = libc::EAFNOSUPPORT as u32;

const RET_ALLOW: u32 = 0x7fff_0000;
const RET_ERRNO: u32 = 0x0005_0000;

const LD_ABS: u16 = 0x20;
const JEQ: u16 = 0x15;
const JGE: u16 = 0x35;
const JSET: u16 = 0x45;
const RET: u16 = 0x06;

/// Where the call's number, its architecture and its arguments sit in the
/// kernel's seccomp_data. Arguments are read by their low 32 bits, which on
/// both architectures (little-endian) come first; the kernel reads ioctl's
/// request and the flags below as 32-bit values too.
const NR: u32 = 0;
const ARCH: u32 = 4;
fn arg(i: u32) -> u32 {
    16 + 8 * i
}

const TIOCSTI: u32 = 0x5412;
const TIOCLINUX: u32 = 0x541c;
const CLONE_NEWUSER: u32 = 0x1000_0000;
/// The personalities Flatpak lets through: Linux, 32-bit Linux, and asking.
const PERSONALITIES: &[u32] = &[0, 8, 0xffff_ffff];
/// The socket families Flatpak lets through: AF_UNIX, AF_INET, AF_INET6 and
/// AF_NETLINK.
const FAMILIES: &[u32] = &[1, 2, 10, 16];

/// One architecture's numbers for the calls the filter looks at.
pub struct Calls {
    pub audit_arch: u32,
    /// Call numbers at or above this belong to another ABI (x86's x32).
    pub foreign_from: Option<u32>,
    pub ioctl: u32,
    pub clone: u32,
    pub personality: u32,
    pub socket: u32,
    /// Refused outright: (call, errno).
    pub refused: &'static [(u32, u32)],
}

pub const X86_64: Calls = Calls {
    audit_arch: 0xc000_003e,
    foreign_from: Some(0x4000_0000),
    ioctl: 16,
    clone: 56,
    personality: 135,
    socket: 41,
    refused: &[
        (103, EPERM), // syslog
        (134, EPERM), // uselib
        (163, EPERM), // acct
        (154, EPERM), // modify_ldt
        (179, EPERM), // quotactl
        (248, EPERM), // add_key
        (249, EPERM), // request_key
        (250, EPERM), // keyctl
        (279, EPERM), // move_pages
        (237, EPERM), // mbind
        (239, EPERM), // get_mempolicy
        (238, EPERM), // set_mempolicy
        (256, EPERM), // migrate_pages
        (272, EPERM), // unshare
        (308, EPERM), // setns
        (165, EPERM), // mount
        (166, EPERM), // umount2
        (155, EPERM), // pivot_root
        (161, EPERM), // chroot
        (298, EPERM), // perf_event_open
        (101, EPERM), // ptrace
        (175, EPERM), // init_module
        (313, EPERM), // finit_module
        (176, EPERM), // delete_module
        (246, EPERM), // kexec_load
        (320, EPERM), // kexec_file_load
        (321, EPERM), // bpf
        (227, EPERM), // clock_settime
        (164, EPERM), // settimeofday
        (167, EPERM), // swapon
        (168, EPERM), // swapoff
        (435, ENOSYS), // clone3: its flags cannot be read, so callers fall back to clone
        (428, ENOSYS), // open_tree
        (429, ENOSYS), // move_mount
        (430, ENOSYS), // fsopen
        (431, ENOSYS), // fsconfig
        (432, ENOSYS), // fsmount
        (433, ENOSYS), // fspick
        (442, ENOSYS), // mount_setattr
    ],
};

pub const AARCH64: Calls = Calls {
    audit_arch: 0xc000_00b7,
    foreign_from: None,
    ioctl: 29,
    clone: 220,
    personality: 92,
    socket: 198,
    refused: &[
        (116, EPERM), // syslog
        (89, EPERM),  // acct
        (60, EPERM),  // quotactl
        (217, EPERM), // add_key
        (218, EPERM), // request_key
        (219, EPERM), // keyctl
        (239, EPERM), // move_pages
        (235, EPERM), // mbind
        (236, EPERM), // get_mempolicy
        (237, EPERM), // set_mempolicy
        (238, EPERM), // migrate_pages
        (97, EPERM),  // unshare
        (268, EPERM), // setns
        (40, EPERM),  // mount
        (39, EPERM),  // umount2
        (41, EPERM),  // pivot_root
        (51, EPERM),  // chroot
        (241, EPERM), // perf_event_open
        (117, EPERM), // ptrace
        (105, EPERM), // init_module
        (273, EPERM), // finit_module
        (106, EPERM), // delete_module
        (104, EPERM), // kexec_load
        (294, EPERM), // kexec_file_load
        (280, EPERM), // bpf
        (112, EPERM), // clock_settime
        (170, EPERM), // settimeofday
        (224, EPERM), // swapon
        (225, EPERM), // swapoff
        (435, ENOSYS), // clone3
        (428, ENOSYS), // open_tree
        (429, ENOSYS), // move_mount
        (430, ENOSYS), // fsopen
        (431, ENOSYS), // fsconfig
        (432, ENOSYS), // fsmount
        (433, ENOSYS), // fspick
        (442, ENOSYS), // mount_setattr
    ],
};

/// This computer's numbers.
pub fn here() -> &'static Calls {
    if cfg!(target_arch = "aarch64") { &AARCH64 } else { &X86_64 }
}

fn op(code: u16, jt: u8, jf: u8, k: u32) -> [u8; 8] {
    let mut b = [0u8; 8];
    b[..2].copy_from_slice(&code.to_ne_bytes());
    b[2] = jt;
    b[3] = jf;
    b[4..].copy_from_slice(&k.to_ne_bytes());
    b
}

fn ret(k: u32) -> [u8; 8] {
    op(RET, 0, 0, k)
}

fn errno(e: u32) -> [u8; 8] {
    ret(RET_ERRNO | e)
}

/// A call whose argument decides: when the number matches, the block runs
/// and ends in a return; otherwise it is jumped over.
fn block(out: &mut Vec<[u8; 8]>, nr: u32, body: Vec<[u8; 8]>) {
    out.push(op(JEQ, 0, body.len() as u8, nr));
    out.extend(body);
}

/// The filter for one architecture, as bubblewrap reads it: each instruction
/// eight bytes, in the computer's own byte order.
pub fn filter(c: &Calls) -> Vec<u8> {
    let mut p: Vec<[u8; 8]> = vec![
        op(LD_ABS, 0, 0, ARCH),
        op(JEQ, 1, 0, c.audit_arch),
        // Another architecture's calls (32-bit x86 on a 64-bit kernel) have
        // other numbers, so none of them is let through.
        errno(ENOSYS),
        op(LD_ABS, 0, 0, NR),
    ];
    if let Some(from) = c.foreign_from {
        p.push(op(JGE, 0, 1, from));
        p.push(errno(ENOSYS));
    }
    for &(nr, e) in c.refused {
        p.push(op(JEQ, 0, 1, nr));
        p.push(errno(e));
    }
    block(&mut p, c.ioctl, vec![
        op(LD_ABS, 0, 0, arg(1)),
        op(JEQ, 2, 0, TIOCSTI),
        op(JEQ, 1, 0, TIOCLINUX),
        ret(RET_ALLOW),
        errno(EPERM),
    ]);
    block(&mut p, c.clone, vec![
        op(LD_ABS, 0, 0, arg(0)),
        op(JSET, 0, 1, CLONE_NEWUSER),
        errno(EPERM),
        ret(RET_ALLOW),
    ]);
    let mut person = vec![op(LD_ABS, 0, 0, arg(0))];
    let n = PERSONALITIES.len();
    for (i, k) in PERSONALITIES.iter().enumerate() {
        person.push(op(JEQ, (n - i) as u8, 0, *k));
    }
    person.push(errno(EPERM));
    person.push(ret(RET_ALLOW));
    block(&mut p, c.personality, person);
    let mut socket = vec![op(LD_ABS, 0, 0, arg(0))];
    let n = FAMILIES.len();
    for (i, k) in FAMILIES.iter().enumerate() {
        socket.push(op(JEQ, (n - i) as u8, 0, *k));
    }
    socket.push(errno(EAFNOSUPPORT));
    socket.push(ret(RET_ALLOW));
    block(&mut p, c.socket, socket);
    p.push(ret(RET_ALLOW));
    p.concat()
}

/// The filter in a memory file bubblewrap can read, left open across exec.
pub fn memfd() -> Option<std::os::fd::OwnedFd> {
    use std::io::{Seek, Write};
    use std::os::fd::FromRawFd;
    let name = c"trindle-seccomp";
    // SAFETY: memfd_create takes a NUL-terminated name and flags; it only
    // returns a new descriptor or -1.
    let fd = unsafe { libc::memfd_create(name.as_ptr(), 0) };
    if fd < 0 {
        return None;
    }
    // SAFETY: fd is a new descriptor owned by nothing else.
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    file.write_all(&filter(here())).ok()?;
    file.rewind().ok()?;
    Some(file.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Walks the filter as the kernel would, for one call.
    fn judge(prog: &[u8], arch: u32, nr: u32, args: [u32; 6]) -> u32 {
        let ins: Vec<(u16, u8, u8, u32)> = prog.chunks_exact(8)
            .map(|b| (u16::from_ne_bytes([b[0], b[1]]), b[2], b[3], u32::from_ne_bytes([b[4], b[5], b[6], b[7]])))
            .collect();
        let load = |k: u32| match k {
            0 => nr,
            4 => arch,
            k => args[((k - 16) / 8) as usize],
        };
        let (mut pc, mut a) = (0usize, 0u32);
        loop {
            let (code, jt, jf, k) = ins[pc];
            pc += 1;
            match code {
                LD_ABS => a = load(k),
                JEQ => pc += if a == k { jt } else { jf } as usize,
                JGE => pc += if a >= k { jt } else { jf } as usize,
                JSET => pc += if a & k != 0 { jt } else { jf } as usize,
                RET => return k,
                _ => panic!("unknown instruction {code:#x}"),
            }
        }
    }

    #[test]
    fn the_filter_refuses_what_flatpak_does() {
        for c in [&X86_64, &AARCH64] {
            let prog = filter(c);
            assert_eq!(prog.len() % 8, 0);
            assert!(prog.len() / 8 < 255, "short enough for one-byte jumps");
            let j = |nr: u32, args: [u32; 6]| judge(&prog, c.audit_arch, nr, args);
            let z = [0; 6];
            assert_eq!(j(c.ioctl, [0, TIOCSTI, 0, 0, 0, 0]), RET_ERRNO | EPERM);
            assert_eq!(j(c.ioctl, [0, TIOCLINUX, 0, 0, 0, 0]), RET_ERRNO | EPERM);
            assert_eq!(j(c.ioctl, [0, 0x5401, 0, 0, 0, 0]), RET_ALLOW, "TCGETS is ordinary");
            assert_eq!(j(c.clone, [CLONE_NEWUSER | 0x11, 0, 0, 0, 0, 0]), RET_ERRNO | EPERM);
            assert_eq!(j(c.clone, [0x0001_1200, 0, 0, 0, 0, 0]), RET_ALLOW, "a thread");
            assert_eq!(j(c.personality, [0x0040_0000, 0, 0, 0, 0, 0]), RET_ERRNO | EPERM);
            assert_eq!(j(c.personality, [0xffff_ffff, 0, 0, 0, 0, 0]), RET_ALLOW);
            assert_eq!(j(c.personality, z), RET_ALLOW);
            assert_eq!(j(c.socket, [1, 0, 0, 0, 0, 0]), RET_ALLOW);
            assert_eq!(j(c.socket, [10, 0, 0, 0, 0, 0]), RET_ALLOW);
            assert_eq!(j(c.socket, [17, 0, 0, 0, 0, 0]), RET_ERRNO | EAFNOSUPPORT, "raw packets");
            assert_eq!(j(c.socket, [40, 0, 0, 0, 0, 0]), RET_ERRNO | EAFNOSUPPORT, "vsock");
            assert_eq!(j(435, z), RET_ERRNO | ENOSYS, "clone3");
            for &(nr, e) in c.refused {
                assert_eq!(j(nr, z), RET_ERRNO | e);
            }
            assert_eq!(j(0, z), RET_ALLOW);
            assert_eq!(judge(&prog, 0x4000_0003, 0, z), RET_ERRNO | ENOSYS, "another architecture");
        }
        assert_eq!(judge(&filter(&X86_64), X86_64.audit_arch, 0x4000_0000 + 16, [0; 6]), RET_ERRNO | ENOSYS, "x32");
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn the_numbers_are_this_computers() {
        let c = here();
        let n = |s: libc::c_long| s as u32;
        assert_eq!((c.ioctl, c.clone, c.personality, c.socket),
                   (n(libc::SYS_ioctl), n(libc::SYS_clone), n(libc::SYS_personality), n(libc::SYS_socket)));
        for s in [libc::SYS_syslog, libc::SYS_unshare, libc::SYS_setns, libc::SYS_mount, libc::SYS_umount2,
                  libc::SYS_pivot_root, libc::SYS_chroot, libc::SYS_ptrace, libc::SYS_perf_event_open, libc::SYS_bpf,
                  libc::SYS_keyctl, libc::SYS_add_key, libc::SYS_request_key, libc::SYS_init_module,
                  libc::SYS_finit_module, libc::SYS_delete_module, libc::SYS_kexec_load, libc::SYS_kexec_file_load,
                  libc::SYS_clock_settime, libc::SYS_settimeofday, libc::SYS_swapon, libc::SYS_swapoff,
                  libc::SYS_quotactl, libc::SYS_acct, libc::SYS_clone3, libc::SYS_modify_ldt, libc::SYS_uselib,
                  libc::SYS_move_pages, libc::SYS_mbind, libc::SYS_get_mempolicy, libc::SYS_set_mempolicy,
                  libc::SYS_migrate_pages, libc::SYS_open_tree, libc::SYS_mount_setattr] {
            assert!(c.refused.iter().any(|r| r.0 == s as u32), "call {s} should be refused");
        }
        assert!(memfd().is_some());
    }
}

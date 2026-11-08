// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

#[cfg(test)]
use crate::rig::*;

fn ask(program: &mut UnixStream, request: u32, new: u32) {
    let mut words = Vec::new();
    for word in [1u32, (12 << 16) | request, new] {
        words.extend_from_slice(&word.to_ne_bytes());
    }
    program.write_all(&words).unwrap();
}

fn read(program: &mut UnixStream) -> (Vec<u8>, bool) {
    use std::io::Read;
    let (mut got, mut buf) = (Vec::new(), vec![0u8; 65536]);
    loop {
        match program.read(&mut buf) {
            Ok(0) => return (got, true),
            Ok(n) => got.extend_from_slice(&buf[..n]),
            Err(_) => return (got, false),
        }
    }
}

#[test]
fn a_program_too_busy_to_read_is_not_shut_out() {
    alone("rig::tests::connection::a_program_too_busy_to_read_is_not_shut_out", |dir| {
        let mut rig = Rig::new(dir);
        let (server_end, mut program) = UnixStream::pair().unwrap();
        rig.state.display_handle.insert_client(server_end, Arc::new(ClientState::default())).unwrap();
        program.set_nonblocking(true).unwrap();

        ask(&mut program, 1, 2);
        rig.pump();
        let (one, shut) = read(&mut program);
        assert!(!shut && one.len() > 200, "the list of what is offered is {} bytes", one.len());

        let lists = (2 << 20) / one.len() as u32 + 1;
        assert!((lists as usize) * one.len() < crate::limits::SLOW_CLIENT_ROOM / 2);
        for n in 0..lists {
            ask(&mut program, 1, 3 + n);
            if n % 64 == 63 {
                rig.pump();
            }
        }
        for _ in 0..20 {
            rig.pump();
        }

        let (mut all, mut quiet) = (0usize, 0);
        while quiet < 20 {
            let (got, shut) = read(&mut program);
            assert!(!shut, "the compositor shut out a program that was only slow, after {all} bytes");
            all += got.len();
            quiet = if got.is_empty() { quiet + 1 } else { 0 };
            rig.pump();
        }
        assert_eq!(all, lists as usize * one.len(), "not every answer arrived");

        let callback = 3 + lists;
        ask(&mut program, 0, callback);
        let mut answer = Vec::new();
        assert!(rig.until(Duration::from_secs(5), |_| {
            let (got, shut) = read(&mut program);
            assert!(!shut, "shut out after it had read everything");
            answer.extend(got);
            answer.len() >= 12
        }), "a program that had been slow was not answered again");
        assert_eq!(u32::from_ne_bytes([answer[0], answer[1], answer[2], answer[3]]), callback, "{answer:?}");
    });
}

//! The one way a Settings page runs a step of trindle-admin behind the password.

use zeroize::Zeroizing;

use super::{asked, Kit, Page};
use crate::common::run::{admin_helper, capture, captured_secret, on_thread};

fn argv(args: &[&str]) -> Vec<String> {
    ["pkexec".to_string(), admin_helper()].into_iter().chain(args.iter().map(|a| a.to_string())).collect()
}

pub fn admin_run(args: &[&str], done: impl FnOnce(Result<(), String>) + 'static) {
    let argv = argv(args);
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    capture(&argv, None, move |ok, _, err| done(asked(ok, &err)));
}

pub fn admin_run_secret(args: &[&str], secret: Zeroizing<String>, done: impl FnOnce(Result<(), String>) + 'static) {
    let argv = argv(args);
    on_thread(move || {
        let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
        let (ok, _, err) = captured_secret(&argv, secret);
        (ok, err)
    }, move |(ok, err)| done(asked(ok, &err)));
}

pub fn admin_step<P: Page + 'static>(kit: &mut Kit, key: &str, args: &[&str], waiting: &str, success: &'static str,
                                     then: impl FnOnce(&mut P, &mut Kit, bool) + 'static) {
    admin_run(args, step::<P>(kit, key, waiting, success, then));
}

pub fn admin_step_secret<P: Page + 'static>(kit: &mut Kit, key: &str, args: &[&str], secret: Zeroizing<String>, waiting: &str,
                                            success: &'static str, then: impl FnOnce(&mut P, &mut Kit, bool) + 'static) {
    admin_run_secret(args, secret, step::<P>(kit, key, waiting, success, then));
}

fn step<P: Page + 'static>(kit: &mut Kit, key: &str, waiting: &str, success: &'static str,
                           then: impl FnOnce(&mut P, &mut Kit, bool) + 'static) -> impl FnOnce(Result<(), String>) + 'static {
    kit.say(key, waiting, true, true);
    let key = key.to_string();
    kit.then::<P, Result<(), String>>(move |me, k, done| {
        k.done(&key, done.as_ref().err().map(String::as_str), success);
        then(me, k, done.is_ok());
    })
}

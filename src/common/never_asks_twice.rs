//! The rule.

use crate::keyring::service::{reseal, why};
use crate::keyring::store::{self, Store};

fn pam() -> &'static str {
    crate::common::source::code(include_str!("../keyring/pam.rs"))
}
fn migrate() -> &'static str {
    crate::common::source::code(include_str!("../keyring/migrate.rs"))
}
fn service() -> &'static str {
    crate::common::source::code(concat!(
        include_str!("../keyring/service.rs"),
        include_str!("../keyring/service/methods.rs"),
        include_str!("../keyring/service/password.rs"),
        include_str!("../keyring/service/prompts.rs"),
    ))
}
fn polkit() -> &'static str {
    crate::common::source::code(include_str!("../polkit/mod.rs"))
}

fn body<'a>(source: &'a str, signature: &str) -> &'a str {
    let start = source.find(signature).unwrap_or_else(|| panic!("no {signature}"));
    let rest = &source[start..];
    let end = rest[signature.len()..].find("\n    fn ").or_else(|| rest[signature.len()..].find("\nfn "))
        .map_or(rest.len(), |n| n + signature.len());
    &rest[..end]
}

#[test]
fn nothing_is_asked_twice() {
    let dir = std::env::temp_dir().join(format!("trindle-never-twice-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("keyring");
    let mut made = Store::create(&path, b"old password").unwrap();
    let salt = made.salt();
    let new_login = store::derive(b"new password", &salt);
    let mut again = Store::load(&path).unwrap();
    assert!(again.unlock_with_key(new_login.clone()).is_err(), "the login's key does not fit yet");
    again.unlock(b"old password").unwrap();
    assert!(reseal(&mut again, new_login.clone()), "typed once, sealed with the login's key");
    let mut next_login = Store::load(&path).unwrap();
    assert!(next_login.unlock_with_key(new_login.clone()).is_ok(), "the next log in opens it with no asking");
    made.lock();
    assert!(!reseal(&mut made, new_login));
    let _ = std::fs::remove_dir_all(&dir);

    let said = [why(true, false, true), why(false, true, true), why(false, false, false), why(false, false, true)];
    for (i, a) in said.iter().enumerate() {
        assert!(a.contains("password"), "{a}");
        for b in &said[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert!(said[2].contains("No password was typed"), "no password at log in is said plainly");

    let helper = body(pam(), "fn leave_key()");
    assert!(helper.contains("password.is_empty()") && helper.contains("remove_file"), "leave_key guards an empty password");
    assert!(body(migrate(), "pub fn at_login()").contains("password.is_empty()"), "at_login guards an empty password");
    assert!(helper.contains("!made"), "a moved person with no keyring has one made at log in");
    assert!(body(migrate(), "pub fn at_login()").contains("backup(&there)"));

    let start = body(service(), "pub fn start_with(");
    assert!(start.contains("pam::put_back(&key)") && start.contains("pam::clear_key()"));
    assert!(service().matches("k.use_login_key();").count() >= 2, "both ways of typing the password seal with the login's key");

    let call = body(polkit(), "fn call(");
    assert!(call.contains("self.enter(r)") && !call.contains(".start("), "every request waits its turn through enter");
    assert!(!polkit().replace("self.start(None)", "").contains(".start(None)"), "start is only through begin");
    assert!(body(polkit(), "fn left(").contains("reuse"));
    assert!(body(polkit(), "fn ask(").contains("self.reuse.borrow_mut().take()"), "the accepted password is tried before any window");
}

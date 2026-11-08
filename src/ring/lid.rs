//! Closing a laptop's lid does what Settings, Power says: sleep, hibernate,
//! lock the screen, nothing, or shut down, with one choice on the battery
//! and another plugged in. With a second screen plugged in it does nothing,
//! so the laptop can be closed and used from the bigger screen.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::common::files::setting;
use crate::toolkit::bus::{Bus, Props};

use super::ctx::Ctx;
use crate::toolkit::t;

const UPOWER: &str = "org.freedesktop.UPower";
const UPOWER_PATH: &str = "/org/freedesktop/UPower";
const PROPERTIES: &str = "org.freedesktop.DBus.Properties";
const LOGIN: &str = "org.freedesktop.login1";
const LOGIN_PATH: &str = "/org/freedesktop/login1";
const LOGIN_IFACE: &str = "org.freedesktop.login1.Manager";

const HAND_OVER: Duration = Duration::from_millis(2000);

const ANSWER: Duration = Duration::from_millis(10000);

pub fn choices() -> [(&'static str, &'static str); 5] {
    [
        ("suspend", t!("ring-lid-sleep")),
        ("hibernate", t!("ring-lid-hibernate")),
        ("lock", t!("ring-lid-lock")),
        ("nothing", t!("ring-lid-nothing")),
        ("poweroff", t!("ring-lid-shut-down")),
    ]
}

thread_local! {

    static HELD: RefCell<Option<zbus::zvariant::OwnedFd>> = const { RefCell::new(None) };

    static HEARING: RefCell<Option<crate::toolkit::bus::Listen>> = const { RefCell::new(None) };
}

pub fn choice() -> String {
    let key = if crate::common::charge::on_battery() { "lid_close_battery" } else { "lid_close" };
    let v = setting(key);
    if choices().iter().any(|(id, _)| *id == v) { v } else { "suspend".into() }
}

pub fn start(ctx: &Rc<Ctx>) {
    if !crate::common::charge::laptop() {
        return;
    }
    let Some(bus) = ctx.system.clone() else { return };

    let args = ("handle-lid-switch", "Trindle", t!("ring-lid-reason"), "block");
    let c = ctx.clone();
    let b = bus.clone();
    bus.call(LOGIN, LOGIN_PATH, LOGIN_IFACE).method("Inhibit", &args, false, Some(HAND_OVER), &ctx.lp, move |r| {
        match r.and_then(|m| m.body().deserialize::<zbus::zvariant::OwnedFd>().map_err(Into::into)) {
            Ok(fd) => {
                HELD.with(|h| *h.borrow_mut() = Some(fd));
                hear(&c, &b);
            }
            Err(e) => eprintln!("trindle: could not take over the lid: {e}"),
        }
    });
}

fn hear(ctx: &Rc<Ctx>, bus: &Bus) {
    let c = ctx.clone();
    let heard = bus.listen(Some(UPOWER), PROPERTIES, Some("PropertiesChanged"), Some(UPOWER_PATH), move |msg| {
        let Ok((_, changed, _)) = msg.body().deserialize::<(String, Props, Vec<String>)>() else { return };
        let closed = changed.get("LidIsClosed").and_then(|v| bool::try_from(v).ok());
        if closed == Some(true) {
            let now = c.clone();
            c.lp.after(Duration::ZERO, move || closed_now(&now));
        }
    });
    HEARING.with(|h| *h.borrow_mut() = heard.ok());
}

fn another_screen(ctx: &Ctx) -> bool {
    ctx.display.screens().len() > 1
}

fn closed_now(ctx: &Rc<Ctx>) {
    if another_screen(ctx) {
        return;
    }
    match choice().as_str() {
        "nothing" => {}
        "lock" => {
            crate::common::run::spawn(&["trindle-lock", "--now"]);
        }
        "poweroff" => super::view::panel::session_end(ctx, "PowerOff"),
        "hibernate" => logind(ctx, "Hibernate"),
        _ => logind(ctx, "Suspend"),
    }
}

fn logind(ctx: &Ctx, method: &str) {
    if let Some(bus) = &ctx.system {
        bus.call(LOGIN, LOGIN_PATH, LOGIN_IFACE).method(method, &(false,), false, Some(ANSWER), &ctx.lp, |_| {});
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_choice_has_words() {
        assert!(super::choices().iter().all(|(id, words)| !id.is_empty() && !words.is_empty()));
        assert_eq!(super::choices()[0].0, "suspend");
    }
}

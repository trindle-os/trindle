//! Users: accounts, administrators, passwords. Changes run trindle-admin via pkexec.

use std::collections::HashMap;
use std::ffi::CStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::Duration;

use image::imageops::FilterType;
use image::DynamicImage;
use tiny_skia::{FillRule, FilterQuality, Paint, Pattern, Pixmap, SpreadMode, Transform};
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

use crate::common::faces::path_for;
use crate::common::files::home;
use crate::common::run;
use crate::keyring::migrate as keyring;
use crate::settings::ui::{confirm, group, headline, icon, line, on, page, why, Kit, Msg, Page, no_chooser};
use crate::toolkit::bus::Listen;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::picture;
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, drawing, grid, row, switch, entry, word_button, Length, Node, Tone};

const ADMIN_GROUP: &CStr = c"sudo";

const FACE_SIZE: u32 = 256;

const FACE: f32 = 56.0;

pub fn banded_rows(rows: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    crate::toolkit::widget::banded_list(rows)
}

fn waiting() -> &'static str {
    t!("settings-users-waiting")
}
const ADD_KEY: &str = "users:add";
const KEYRING_KEY: &str = "users:keyring";

struct Person {
    name: String,
    full: String,
    uid: u32,
    admin: bool,
}

fn text(p: *const libc::c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        // SAFETY: a non-null field of a passwd or group entry is a NUL-terminated string.
        unsafe { CStr::from_ptr(p) }.to_string_lossy().to_string()
    }
}

fn login_defs() -> (u32, u32) {
    let (mut lo, mut hi) = (1000, 60000);
    for line in std::fs::read_to_string("/etc/login.defs").unwrap_or_default().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            match parts[0] {
                "UID_MIN" => lo = parts[1].parse().unwrap_or(lo),
                "UID_MAX" => hi = parts[1].parse().unwrap_or(hi),
                _ => {}
            }
        }
    }
    (lo, hi)
}

fn admins() -> Vec<String> {
    let mut out = Vec::new();
    // SAFETY: getgrnam returns null or an entry whose gr_mem is a null-terminated

    unsafe {
        let g = libc::getgrnam(ADMIN_GROUP.as_ptr());
        if g.is_null() {
            return out;
        }
        let mut m = (*g).gr_mem;
        while !m.is_null() && !(*m).is_null() {
            out.push(text(*m));
            m = m.add(1);
        }
    }
    out
}

fn people() -> Vec<Person> {
    let (lo, hi) = login_defs();
    let admins = admins();
    let mut out = Vec::new();
    // SAFETY: the passwd walk is only ever used here, on the main thread.
    unsafe {
        libc::setpwent();
        loop {
            let p = libc::getpwent();
            if p.is_null() {
                break;
            }
            let uid = (*p).pw_uid;
            let shell = text((*p).pw_shell);
            if lo <= uid && uid <= hi && !shell.ends_with("nologin") && !shell.ends_with("false") {
                let name = text((*p).pw_name);
                let full = text((*p).pw_gecos).split(',').next().unwrap_or("").to_string();
                let admin = admins.contains(&name);
                out.push(Person { name, full, uid, admin });
            }
        }
        libc::endpwent();
    }
    out.sort_by_key(|u| u.uid);
    out
}

fn my_name() -> String {
    // SAFETY: getpwuid returns null or an entry that is read at once.
    unsafe {
        let p = libc::getpwuid(libc::getuid());
        if p.is_null() { String::new() } else { text((*p).pw_name) }
    }
}

fn sum_up(people: &[Person], me: &str) -> (String, Tone) {
    let admins = people.iter().filter(|p| p.admin).count();
    let i_am = people.iter().any(|p| p.name == me && p.admin);
    match (people.len(), admins) {
        (_, 0) => (t!("settings-users-sum-nobody-admin").into(), Tone::Warn),
        (1, _) if i_am => (t!("settings-users-sum-only-you").into(), Tone::Plain),
        (1, _) => (t!("settings-users-sum-only-one").into(), Tone::Plain),
        (n, 1) if i_am => (t!("settings-users-sum-you-admin", n = n), Tone::Plain),
        (n, 1) => (t!("settings-users-sum-one-admin", n = n), Tone::Plain),
        (n, a) => (t!("settings-users-sum-admins", n = n, a = a), Tone::Plain),
    }
}

fn fresh_png(source: &Path) -> Option<Vec<u8>> {
    let img = picture::load(source)?;
    let (w, h) = (img.width(), img.height());
    let side = w.min(h);
    if side == 0 {
        return None;
    }
    let small = img.crop_imm((w - side) / 2, (h - side) / 2, side, side)
        .resize_exact(FACE_SIZE, FACE_SIZE, FilterType::Triangle);
    let mut out = std::io::Cursor::new(Vec::new());
    small.write_to(&mut out, image::ImageFormat::Png).ok()?;
    Some(out.into_inner())
}

fn pictures() -> PathBuf {
    pictures_named(&run::text(&["xdg-user-dir", "PICTURES"]))
}

fn pictures_named(said: &str) -> PathBuf {
    let dir = PathBuf::from(said.trim());
    if dir.is_absolute() { dir } else { home() }
}

fn face(img: Option<Rc<DynamicImage>>) -> Node<Msg> {
    let Some(img) = img else { return icon("avatar-default-symbolic", FACE) };
    drawing((FACE, FACE), "", move |canvas, pen| {
        let size = canvas.width().min(canvas.height());
        let Some(mut pic) = Pixmap::new(size, size) else { return };
        if picture::draw(&mut pic.as_mut(), &img, 0, 0, size, size).is_none() {
            return;
        }
        let whole = crate::toolkit::widget::Rect { x: 0.0, y: 0.0, w: size as f32, h: size as f32 };
        let Some(circle) = crate::toolkit::ui::rounded(whole, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, size as f32, pen.scale)) else { return };
        let paint = Paint {
            shader: Pattern::new(pic.as_ref(), SpreadMode::Pad, FilterQuality::Bilinear, 1.0, Transform::identity()),
            anti_alias: true,
            ..Paint::default()
        };
        canvas.fill_path(&circle, &paint, FillRule::Winding, Transform::identity(), None);
    })
}

fn pipe_face(name: &str, data: &[u8]) -> Result<(bool, String), String> {
    let helper = run::admin_helper();
    let mut child = Command::new("pkexec").args([helper.as_str(), "face", name])
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped())
        .spawn().map_err(|e| e.to_string())?;
    if let Some(mut input) = child.stdin.take() {
        let _ = input.write_all(data);
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok((out.status.success(), String::from_utf8_lossy(&out.stderr).trim().to_string()))
}

fn admin(kit: &mut Kit, key: &str, args: &[&str], success: &'static str, stdin: Option<String>,
         then: impl FnOnce(&mut Users, &mut Kit) + 'static) {
    kit.say(key, waiting(), true, true);
    let helper = run::admin_helper();
    let mut argv = vec!["pkexec", helper.as_str()];
    argv.extend_from_slice(args);
    let k = key.to_string();
    let done = kit.then::<Users, (bool, String)>(move |me, kit, (ok, err)| {
        let err = run::admin_error(&err);
        kit.done(&k, if ok { None } else { Some(&err) }, success);
        if ok {
            then(me, kit);
        }
    });
    run::capture(&argv, stdin.as_deref(), move |ok, _out, err| done((ok, err)));
}

const SECRETS: &str = "org.freedesktop.secrets";
const SECRETS_PATH: &str = "/org/freedesktop/secrets";
const LOGIN_KEYRING: &str = "/org/freedesktop/secrets/collection/login";

fn rekey(kit: &mut Kit, key: String, old: String, new: String) {
    let Some(bus) = kit.session.clone() else {

        kit.say(&key, not_rekeyed(), false, false);
        return;
    };
    let events = kit.events.clone();
    let failed = kit.then::<Users, ()>(move |_, k, ()| k.say(&key, not_rekeyed(), false, false));
    let b = bus.clone();
    bus.call(SECRETS, SECRETS_PATH, "org.freedesktop.Secret.Service").method(
        "OpenSession", &("plain", Value::from("")), false, Some(Duration::from_secs(5)), &kit.events, move |res| {
            let opened = match res {
                Ok(m) => m.body().deserialize::<(OwnedValue, OwnedObjectPath)>().ok().map(|(_, s)| s),

                Err(e) if no_keyring(&e) => return,
                Err(_) => None,
            };
            let (Some(session), Ok(login)) = (opened, ObjectPath::try_from(LOGIN_KEYRING)) else {
                failed(());
                return;
            };
            let secret = |p: &str| (session.clone(), Vec::<u8>::new(), p.as_bytes().to_vec(), "text/plain");
            let body = (login, secret(&old), secret(&new));
            b.call(SECRETS, SECRETS_PATH, "org.gnome.keyring.InternalUnsupportedGuiltRiddenInterface").method(
                "ChangeWithMasterPassword", &body, false, Some(Duration::from_secs(10)), &events, move |res| {
                    if let Err(e) = res {

                        let missing = |s: &str| s.contains("NoSuchObject") || s.contains("No such");
                        if !missing(&e.name) && !missing(&e.text) {
                            failed(());
                        }
                    }
                });
        });
}

fn not_rekeyed() -> &'static str {
    t!("settings-users-not-rekeyed")
}

fn no_keyring(e: &crate::toolkit::bus::Failed) -> bool {
    ["ServiceUnknown", "NameHasNoOwner", "Spawn.ServiceNotFound"].iter().any(|n| e.name.contains(n))
}

fn person_key(name: &str) -> String {
    format!("users:{name}")
}

#[derive(Default)]
pub struct Users {
    everyone: Vec<Person>,
    faces: HashMap<String, Rc<DynamicImage>>,
    me: String,

    full: String,
    name: String,
    pw: String,
    admin: bool,
    auto: bool,

    changing: Option<String>,
    now: String,
    one: String,
    two: String,

    switched: bool,
    asking: bool,
    login: String,

    shut: Vec<String>,
    extra: Vec<String>,
    moving: bool,
    chooser: Option<Listen>,

    turn: u32,
}

impl Users {
    fn read(&mut self) {
        self.me = my_name();
        self.everyone = people();
        self.faces = self.everyone.iter()
            .filter_map(|u| path_for(&u.name).and_then(|p| picture::load(&p)).map(|i| (u.name.clone(), Rc::new(i))))
            .collect();
        self.switched = keyring::switched();
    }

    fn person(&self, kit: &Kit, u: &Person) -> Node<Msg> {
        let mine = u.name == self.me;
        let key = person_key(&u.name);
        let title = if u.full.is_empty() { u.name.clone() } else { u.full.clone() };
        let sub = if mine { t!("settings-users-this-is-you").to_string() } else { t!("settings-users-login-name").to_string() };
        let n = u.name.clone();
        let mut controls = vec![
            button("image-x-generic-symbolic", t!("settings-users-choose-picture")).on_press(on::<Users>(move |me, k| me.picture(k, n.clone()))),
        ];
        let n = u.name.clone();
        controls.push(button("dialog-password-symbolic", t!("settings-users-change-password")).on_press(on::<Users>(move |me, _| {
            me.changing = Some(n.clone());
            me.now.clear();
            me.one.clear();
            me.two.clear();
            me.turn += 1;
        })));
        if !mine {
            let (n, k) = (u.name.clone(), key.clone());
            controls.push(confirm::<Users>(kit, &format!("users:delete:{}", u.name), "user-trash-symbolic",
                                           t!("settings-users-delete"), move |_, kit| {
                admin(kit, &k, &["delete", n.as_str()], t!("settings-users-deleted"), None, |me, _| me.read());
            }));
        }
        let mut who = line(u.name.clone()).sub(sub).controls(controls).key(key);
        if self.changing.as_deref() == Some(u.name.as_str()) {
            who = who.below(self.password_form(&u.name, mine));
        }
        let top = row(vec![face(self.faces.get(&u.name).cloned()), who.node(kit)]).gap(12.0);

        let akey = format!("users:admin:{}", u.name);
        let stuck = mine && u.admin;
        let tip = if stuck { t!("settings-users-cannot-take-away") } else { t!("settings-users-administrator") };
        let (n, k) = (u.name.clone(), akey.clone());
        let adm = switch(u.admin, tip, move |on_now| {
            let (n, k) = (n.clone(), k.clone());
            on::<Users>(move |_, kit| {
                if stuck {
                    kit.tell(&k, t!("settings-users-cannot-take-away"));
                    return;
                }
                let success = if on_now { t!("settings-users-admin-now") } else { t!("settings-users-admin-no-longer") };
                admin(kit, &k, &["admin", n.as_str(), if on_now { "yes" } else { "no" }], success, None, |me, _| me.read());
            })
        }).off(stuck);
        let adm_line = line(t!("settings-users-administrator")).sub(t!("settings-users-admin-can"))
            .control(adm).key(akey).node(kit);
        group(title).known_as(person_key(&u.name)).icon("avatar-default-symbolic")
            .about(t!("settings-users-person-about"))
            .rows(vec![top, adm_line])
            .node(kit)
    }

    fn password_form(&self, name: &str, mine: bool) -> Node<Msg> {

        let save = || on::<Users>(|me, k| me.save_password(k));
        let mut boxes = Vec::new();
        if mine {
            boxes.push(entry(self.now.clone(), t!("settings-users-current-password"), |t| on::<Users>(move |me, _| me.now = t.clone()))
                .secret().key(format!("users:now:{name}")).grab(self.turn));
        }
        let mut one = entry(self.one.clone(), t!("settings-users-new-password"), |t| on::<Users>(move |me, _| me.one = t.clone()))
            .secret().key(format!("users:one:{name}"));
        if !mine {
            one = one.grab(self.turn);
        }
        boxes.push(one);
        boxes.push(entry(self.two.clone(), t!("settings-users-again"), |t| on::<Users>(move |me, _| me.two = t.clone()))
            .secret().key(format!("users:two:{name}")).on_submit(save()));
        let buttons = row(vec![

            word_button(t!("settings-users-save")).spoken(t!("settings-users-save-new")).on_press(save()),
            button("window-close-symbolic", t!("settings-users-cancel")).on_press(on::<Users>(|me, _| me.close_password())),
        ]).gap(8.0);

        column(vec![grid(160.0, boxes).gap(8.0), buttons]).gap(8.0).width(Length::Fill)
    }

    fn close_password(&mut self) {
        self.changing = None;
        self.now.clear();
        self.one.clear();
        self.two.clear();
    }

    fn save_password(&mut self, kit: &mut Kit) {
        let Some(name) = self.changing.clone() else { return };
        let key = person_key(&name);
        let mine = name == self.me;
        let (old, a, b) = (self.now.clone(), self.one.clone(), self.two.clone());
        if mine && old.is_empty() {
            kit.say(&key, t!("settings-users-need-current"), false, false);
            return;
        }
        if a.chars().count() < 8 {
            kit.say(&key, t!("settings-users-at-least-8"), false, false);
            return;
        }
        if a != b {
            kit.say(&key, t!("settings-users-no-match"), false, false);
            return;
        }
        self.close_password();
        let (k, new) = (key.clone(), a.clone());
        admin(kit, &key, &["password", name.as_str()], t!("settings-users-password-changed"), Some(format!("{a}\n")), move |_, kit| {
            if mine {
                rekey(kit, k, old, new);
            }
        });
    }

    fn picture(&mut self, kit: &mut Kit, name: String) {
        let key = person_key(&name);
        let Some(bus) = kit.session_for(&key) else { return };
        let globs = ["*.png", "*.PNG", "*.jpg", "*.JPG", "*.jpeg", "*.JPEG", "*.webp", "*.WEBP"];
        let ask = Ask {
            title: t!("settings-users-choose-picture").into(),
            folder: Some(pictures()),
            name: None,
            filters: vec![(t!("settings-users-pictures-filter").into(), globs.iter().map(|g| g.to_string()).collect())],
        };
        let k = key.clone();
        let chosen = kit.then::<Users, Option<PathBuf>>(move |me, kit, picked| {
            me.chooser = None;
            let Some(path) = picked else { return kit.unanswered(&k) };
            let made = kit.then::<Users, Option<Vec<u8>>>(move |me, kit, png| match png {
                Some(data) => me.send_face(kit, name, data),
                None => kit.say(&k, t!("settings-users-png-please"), false, false),
            });
            run::on_thread(move || fresh_png(&path), made);
        });
        self.chooser = chooser::open(&bus, &kit.events, ask, chosen);
        kit.started(&key, self.chooser.is_some(), no_chooser());
    }

    fn send_face(&mut self, kit: &mut Kit, name: String, data: Vec<u8>) {
        let key = person_key(&name);
        let who = name.clone();
        kit.say(&key, waiting(), true, true);
        let sent = kit.then::<Users, (Vec<u8>, Result<(bool, String), String>)>(move |me, kit, (data, res)| match res {
            Err(e) => kit.say(&key, &why(&e, t!("settings-users-picture-not-changed")), false, false),
            Ok((true, _)) => {
                kit.say(&key, t!("settings-users-saved"), true, false);
                if name == me.me {

                    kit.settle_quietly(&key, crate::common::files::write_whole(&home().join(".face"), &data),
                                       t!("settings-users-picture-not-shared"));
                }
                me.read();
            }
            Ok((false, err)) => kit.say(&key, &run::admin_error(&err), false, false),
        });
        run::on_thread(move || {
            let res = pipe_face(&who, &data);
            (data, res)
        }, sent);
    }

    fn add_form(&self, kit: &Kit) -> Node<Msg> {
        let full = entry(self.full.clone(), t!("settings-users-full-name"), |t| on::<Users>(move |me, _| {
            me.full = t.clone();

            if me.name.is_empty() || me.auto {
                let first = t.split(' ').next().unwrap_or("").to_lowercase();
                me.name = first.chars().filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit()).take(32).collect();
                me.auto = true;
            }
        })).key("users:full");
        let name = entry(self.name.clone(), t!("settings-users-login-name-lower"), |t| on::<Users>(move |me, _| {
            me.auto = t.is_empty();
            me.name = t.clone();
        })).key("users:name");
        let pw = entry(self.pw.clone(), t!("settings-users-password"), |t| on::<Users>(move |me, _| me.pw = t.clone()))
            .secret().key("users:pw");
        let adm = switch(self.admin, t!("settings-users-administrator"), |v| on::<Users>(move |me, _| me.admin = v));
        let add = button("list-add-symbolic", t!("settings-users-add-this")).on_press(on::<Users>(|me, k| me.add(k)));
        let form = grid(220.0, vec![full, name, pw]).gap(8.0);
        group(t!("settings-users-add-someone")).icon("list-add-symbolic")
            .about(t!("settings-users-add-about"))
            .rows(vec![line(t!("settings-users-new-account")).sub(t!("settings-users-new-account-sub"))
                .control(add).below(form).key(ADD_KEY).node(kit),
                line(t!("settings-users-administrator")).sub(t!("settings-users-admin-can")).control(adm).node(kit)])
            .node(kit)
    }

    fn add(&mut self, kit: &mut Kit) {
        let n = self.name.trim().to_string();
        let p = self.pw.clone();
        if !crate::admin::valid_name_ok(&n) {
            kit.say(ADD_KEY, t!("settings-users-login-rules"), false, false);
            return;
        }
        if p.chars().count() < 8 {
            kit.say(ADD_KEY, t!("settings-users-password-8"), false, false);
            return;
        }
        if people().iter().any(|u| u.name == n) {
            kit.say(ADD_KEY, &t!("settings-users-exists", name = &n), false, false);
            return;
        }
        let f = self.full.trim().to_string();
        let f = if f.is_empty() { n.clone() } else { f };
        let yes = if self.admin { "yes" } else { "no" };
        admin(kit, ADD_KEY, &["add", n.as_str(), f.as_str(), yes], t!("settings-users-added"), Some(format!("{p}\n")), |me, _| {
            me.full.clear();
            me.name.clear();
            me.pw.clear();
            me.admin = false;
            me.auto = false;
            me.read();
        });
    }

    fn keyring_card(&self, kit: &Kit) -> Node<Msg> {
        let on_now = self.switched;
        let mut l = line(if on_now { t!("settings-users-kept-trindle") } else { t!("settings-users-kept-gnome") })
            .sub(if on_now { t!("settings-users-opens-login") } else { t!("settings-users-could-not-move") })
            .key(KEYRING_KEY);
        if !on_now && !self.moving {
            l = l.control(button("document-send-symbolic", t!("settings-users-move-keyring")).on_press(on::<Users>(|me, _| {
                me.open_keyring(String::new(), Vec::new());
            })));
        }
        if self.asking {
            l = l.below(self.keyring_form());
        }
        group(t!("settings-users-saved-logins")).icon("dialog-password-symbolic")
            .about(t!("settings-users-keyring-about"))
            .rows(vec![l.node(kit)])
            .node(kit)
    }

    fn open_keyring(&mut self, login: String, shut: Vec<String>) {
        self.asking = true;
        self.login = login;
        self.extra = vec![String::new(); shut.len()];
        self.shut = shut;
        self.turn += 1;
    }

    fn close_keyring(&mut self) {
        self.asking = false;
        self.login.clear();
        self.shut.clear();
        self.extra.clear();
    }

    fn keyring_form(&self) -> Node<Msg> {
        let go = || on::<Users>(|me, k| me.go_ahead(k));
        let mut pw = entry(self.login.clone(), t!("settings-users-your-password"), |t| on::<Users>(move |me, _| me.login = t.clone()))
            .secret().key("users:login").on_submit(go());
        if self.shut.is_empty() {
            pw = pw.grab(self.turn);
        }
        let top = row(vec![
            pw,
            word_button(t!("settings-users-go-ahead")).on_press(go()),
            button("window-close-symbolic", t!("settings-users-cancel")).on_press(on::<Users>(|me, _| me.close_keyring())),
        ]).gap(8.0);

        let mut stack = vec![top];
        for (i, label) in self.shut.iter().enumerate() {
            let mut e = entry(self.extra.get(i).cloned().unwrap_or_default(), t!("settings-users-password-for", name = label),
                              move |t| on::<Users>(move |me, _| {
                                  if let Some(x) = me.extra.get_mut(i) {
                                      *x = t.clone();
                                  }
                              }))
                .secret().key(format!("users:shut:{i}")).on_submit(go());
            if i == 0 {
                e = e.grab(self.turn);
            }
            stack.push(e);
        }
        column(stack).gap(8.0)
    }

    fn go_ahead(&mut self, kit: &mut Kit) {
        let password = self.login.clone();
        if password.is_empty() {
            kit.say(KEYRING_KEY, t!("settings-users-your-password-please"), false, false);
            return;
        }
        let extra: Vec<String> = self.extra.iter().filter(|t| !t.is_empty()).cloned().collect();
        self.close_keyring();
        self.moving = true;
        kit.say(KEYRING_KEY, t!("settings-users-copying"), true, true);
        let me = self.me.clone();
        let login = password.clone();
        let done = kit.then::<Users, Result<usize, keyring::Stopped>>(move |page, kit, done| {
            page.moving = false;
            page.switched = keyring::switched();
            match done {
                Ok(copied) => kit.say(KEYRING_KEY, &t!("settings-users-copied", n = copied), true, false),
                Err(keyring::Stopped::Shut(left)) => {
                    kit.say(KEYRING_KEY, &t!("settings-users-nothing-moved", n = left.len(), names = left.join(", ")), false, false);
                    page.open_keyring(login, left);
                }
                Err(keyring::Stopped::Failed(e)) => kit.say(KEYRING_KEY, &e, false, false),
            }
        });
        run::on_thread(move || {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| keyring::switch(&me, &password, &extra)))
                .unwrap_or_else(|_| Err(keyring::Stopped::Failed(t!("settings-users-stopped").into())))
        }, done);
    }
}

impl Page for Users {
    fn id(&self) -> &'static str {
        "users"
    }
    fn title(&self) -> &'static str {
        t!("settings-users-title")
    }
    fn icon(&self) -> &'static str {
        "system-users-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-users-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-users-help")
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (words, _) = sum_up(&self.everyone, &self.me);
        let mut items = vec![headline(kit, words)];

        let mut people: Vec<&Person> = self.everyone.iter().collect();
        people.sort_by_key(|u| u.name != self.me);
        items.extend(people.into_iter().map(|u| self.person(kit, u)));

        if self.everyone.is_empty() {
            let top = row(vec![face(None), line(t!("settings-users-looking-accounts")).node(kit).width(Length::Fill)]).gap(12.0);
            items.push(group(t!("settings-users-people")).icon("avatar-default-symbolic")
                .rows(vec![top, line(t!("settings-users-administrator")).sub(t!("settings-users-looking")).node(kit)]).node(kit));
        }
        items.push(self.add_form(kit));
        items.push(self.keyring_card(kit));
        page(kit, self.title(), self.help(), items)
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.read();
    }
    fn hidden(&mut self, _kit: &mut Kit) {
        self.close_password();
        self.close_keyring();
        self.pw.clear();
    }
}

#[cfg(test)]
impl Users {

    pub(crate) fn made_up() -> Users {
        let person = |name: &str, full: &str, uid: u32, admin: bool| Person { name: name.into(), full: full.into(), uid, admin };
        Users { everyone: vec![person("john", "John Smith", 1000, true), person("jane", "Jane Smith", 1001, false)],
                me: "john".into(), switched: true, ..Users::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;

    #[test]
    fn the_pictures_folder_is_named_and_never_looked_for() {
        assert_eq!(pictures_named("/mnt/share/Pictures\n"), PathBuf::from("/mnt/share/Pictures"));
        assert_eq!(pictures_named("/trindle-test/not-there"), PathBuf::from("/trindle-test/not-there"));
        assert_eq!((pictures_named(""), pictures_named("Pictures")), (home(), home()));
    }
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::card;

    fn form() -> Ui<Msg> {
        let users = Users { now: "old-password".into(), one: "john-smith-17".into(), two: "john-smith-17".into(), ..Users::made_up() };
        let mut ui = Ui::new(column(vec![card(users.password_form("john", true))]).padding(24.0), Theme::new(false, "#fbc711"));
        ui.lay_at(760, 180);
        ui
    }

    #[test]
    fn each_password_box_has_its_own_eye() {
        let mut ui = form();
        let before = ui.boxes();
        let keys = ["users:now:john", "users:one:john", "users:two:john"];
        for key in keys {
            assert!(ui.eye_place(key).is_some(), "{key} has no eye");
        }
        ui.press_eye("users:one:john");
        assert!(ui.shows_secret("users:one:john") && !ui.shows_secret("users:now:john") && !ui.shows_secret("users:two:john"));
        assert_eq!(ui.boxes(), before);
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_a_password_box() {
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let mut ui = form();
        let shoot = |ui: &mut Ui<Msg>, name: &str| {
            let mut pix = tiny_skia::Pixmap::new(760, 180).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(std::path::Path::new(&dir).join(format!("eye-settings-password-{name}.png"))).unwrap();
        };
        shoot(&mut ui, "hidden");
        ui.press_eye("users:one:john");
        shoot(&mut ui, "shown");
    }
}

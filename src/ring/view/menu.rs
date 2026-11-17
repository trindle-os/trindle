//! An app's right click menu: its windows, the star, starting at login and Uninstall, with the Flatpak card.

use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crate::common::{autostart, favourites};
use crate::common::removal::{self, How};
use crate::common::run::{admin_error, admin_helper, capture, on_thread};
use crate::ring::{kept, menu_button, open_windows, store};
use crate::ring::ctx::{Ctx, msg, Msg, Program};
use crate::ring::windows::Window;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, label, Length, Node, note, ring, row, space, title, Tone};

use super::Face;

fn failure(detail: String) -> String {
    eprintln!("trindle-ring: a removal failed: {}", detail.trim());
    t!("ring-uninstall-failed").to_string()
}

fn begin_removal(removing: &mut Vec<String>, id: &str) -> bool {
    if removing.iter().any(|r| r == id) {
        return false;
    }
    removing.push(id.to_string());
    true
}

fn uninstall_words() -> [&'static str; 3] {
    [t!("ring-uninstall"), t!("ring-uninstalling"), t!("ring-uninstalled")]
}

fn hide_words() -> [&'static str; 3] {
    [t!("ring-take-off"), t!("ring-taking-off"), t!("ring-taken-off")]
}

const TIP_SECONDS: u64 = 2;

pub(super) const MENU_TITLE_CHARS: usize = 28;

pub(super) enum Gone {
    Done(bool, String),
}

pub(super) struct FlatAsk {
    id: String,
    bytes: Option<u64>,
    uncounted: bool,
    working: bool,
    said: Option<(String, Tone)>,
}

pub(super) fn startup_button(launcher: &Path) -> Node<Msg> {
    let on = autostart::of_launcher(launcher).is_some();
    let launcher = launcher.to_path_buf();

    let b = if on { button("system-run-symbolic", t!("ring-startup-remove")).on(true) } else { crate::toolkit::widget::remove("system-run-symbolic", t!("ring-startup-add")) };
    b.on_press(msg(move |f| {
            let _ = autostart::toggle_app(&launcher);
            f.close_menu();
            f.win_menu = None;
        }))
}

pub(super) fn still_at<'a>(all: &'a [Program], i: usize, id: &str) -> Option<&'a Program> {
    all.get(i).filter(|a| a.id == id)
}

impl Face {

    pub(super) fn menu_app(&self, i: usize) -> Option<Program> {
        let (_, id, _) = self.menu.as_ref().filter(|(j, ..)| *j == i)?;
        still_at(&self.all(), i, id).cloned()
    }

    pub(in crate::ring) fn close_menu(&mut self) {
        self.menu = None;
        self.gone = None;
        self.how = None;
    }

    pub(super) fn open_menu(&mut self, i: usize, at: [f32; 4]) {
        self.how = None;
        self.gone = None;
        self.wipe = false;
        self.flat_ask = None;
        let Some(app) = self.all().get(i).cloned() else {
            self.menu = None;
            return;
        };
        let id = app.id.clone();
        self.menu = Some((i, id.clone(), at));
        let (ctx, images) = (self.ctx.clone(), self.images_there.clone());
        on_thread(move || removal::how(&app, &removal::Real, &images), move |how| {
            ctx.face(move |f| if f.menu_app(i).is_some_and(|a| a.id == id) { f.how = Some((id, how)) })
        });
    }

    fn how_of(&self, i: usize) -> Option<(Program, How)> {
        let app = self.menu_app(i)?;
        match &self.how {
            Some((id, Some(how))) if *id == app.id => Some((app, how.clone())),
            _ => None,
        }
    }

    fn ask_flatpak(&mut self, i: usize) {
        let Some((app, How::Flatpak { id, .. })) = self.how_of(i) else { return };
        self.flat_ask = Some(FlatAsk { id: app.id.clone(), bytes: None, uncounted: false, working: false, said: None });
        let ctx = self.ctx.clone();
        on_thread(move || removal::data_dir(&crate::common::files::home(), &id).map_or(Ok(0), |d| removal::data_size(&d)), move |n: std::io::Result<u64>| {
            ctx.face(move |f| {
                if let Some(a) = f.flat_ask.as_mut().filter(|a| a.id == app.id) {
                    match n {
                        Ok(n) => a.bytes = Some(n),
                        Err(_) => a.uncounted = true,
                    }
                }
            })
        });
    }

    fn flatpak_answer(&mut self, i: usize, wipe: bool) {
        if !kept::own_press() {
            return;
        }
        self.flat_ask = None;
        self.wipe = wipe;
        self.uninstall(i);
    }

    fn flatpak_data_only(&mut self, i: usize) {
        if !kept::own_press() {
            return;
        }
        let Some((app, How::Flatpak { id, .. })) = self.how_of(i) else { return };
        let Some(a) = self.flat_ask.as_mut().filter(|a| a.id == app.id && !a.working) else { return };
        a.working = true;
        a.said = Some((t!("store-flatpak-data-going").into(), Tone::Plain));
        let ctx = self.ctx.clone();
        on_thread(move || removal::delete_data_only(&crate::common::files::home(), &id), move |res: Result<(), &'static str>| {
            ctx.face(move |f| {
                let Some(a) = f.flat_ask.as_mut().filter(|a| a.id == app.id) else { return };
                a.working = false;
                match res {
                    Ok(()) => {
                        a.bytes = Some(0);
                        a.said = Some((t!("store-flatpak-data-gone").into(), Tone::Plain));
                    }
                    Err(why) => a.said = Some((why.into(), Tone::Problem)),
                }
            })
        });
    }

    fn flatpak_card(&self, i: usize, a: &FlatAsk, name: &str) -> Node<Msg> {
        use crate::toolkit::widget::{form_label, main_word};
        let size = match a.bytes {
            None if a.uncounted => t!("store-flatpak-ask-uncounted").to_string(),
            None => t!("store-flatpak-ask-measuring").to_string(),
            Some(0) => t!("store-flatpak-ask-none").to_string(),
            Some(n) => t!("store-flatpak-ask-size", size = crate::common::volumes::exact_size_words(n)),
        };
        let why = if a.bytes == Some(0) { Some(t!("store-flatpak-data-only-none")) } else { crate::common::ui::wait_while((a.bytes.is_none() && !a.uncounted) || a.working) };
        let mut rows = vec![form_label(t!("store-flatpak-ask-head", name = name)), note(size)];
        if let Some((w, tone)) = &a.said {
            rows.push(note(w.clone()).tone(*tone));
        }
        rows.push(column(vec![
            main_word(t!("store-flatpak-keep")).on_press(msg(move |f| f.flatpak_answer(i, false))).off_because(crate::common::ui::wait_while(a.working)),
            main_word(t!("store-flatpak-wipe")).cannot_be_undone().on_press(msg(move |f| f.flatpak_answer(i, true))).off_because(why),
            main_word(t!("store-flatpak-data-only")).cannot_be_undone().on_press(msg(move |f| f.flatpak_data_only(i))).off_because(why),
            button("window-close-symbolic", t!("store-flatpak-cancel")).size(menu_button()).on_press(msg(|f| f.flat_ask = None)),
        ]).gap(8.0));
        rows.push(note(t!("store-flatpak-ask-undone")));
        card(column(rows).gap(8.0).width(Length::Shrink)).width(Length::Shrink).on_escape(msg(|f| f.flat_ask = None))
    }

    fn uninstall(&mut self, i: usize) {

        if !kept::own_press() {
            return;
        }
        let Some((app, how)) = self.how_of(i) else { return };
        if matches!(how, How::Refused(_)) {
            return;
        }
        if !begin_removal(&mut self.removing, &app.id) {
            return;
        }
        let done = Self::removal_done(self.ctx.clone(), app.id.clone());
        Self::start_removal(how, &app.id, &mut self.wipe, done);
    }

    fn removal_done(ctx: Rc<Ctx>, id: String) -> impl FnOnce(bool, String) + 'static {
        move |ok: bool, err: String| {
            ctx.face(move |f| {
                f.removing.retain(|r| *r != id);
                f.gone = (ok || !crate::common::refused::closed_box(&err)).then(|| (id.clone(), Gone::Done(ok, err.trim().to_string())));
                if ok {
                    let ctx = f.ctx.clone();
                    f.ctx.lp.after(Duration::from_secs(TIP_SECONDS), move || {
                        ctx.reread_apps();
                        ctx.face(move |f| {

                            if f.menu.as_ref().is_some_and(|(_, j, _)| *j == id) {
                                f.close_menu();
                            }
                            f.set_favourite(&id, false);
                            f.reload();
                        })
                    });
                }
            })
        }
    }

    fn start_removal(how: How, key: &str, wipe: &mut bool, done: impl FnOnce(bool, String) + 'static) {
        let on_disk = |r: std::io::Result<()>| match r {
            Ok(()) => (true, String::new()),
            Err(e) => (false, failure(e.to_string())),
        };
        match how {
            How::Flatpak { place, id, launcher } => {
                let argv = removal::flatpak_uninstall(place, &id, std::mem::take(wipe));
                let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
                let (home, key) = (crate::common::files::home(), key.to_string());
                capture(&argv, None, move |ok, _, err| {
                    if !ok {
                        return done(false, failure(err));
                    }

                    on_thread(move || launcher.map(|l| removal::hide(&l, &key, &home)), move |hidden| match hidden {
                        Some(Err(e)) => done(false, failure(e.to_string())),
                        _ => done(true, String::new()),
                    });
                });
            }
            How::Image(file) => on_thread(move || on_disk(crate::icons::appimages::remove(&file)), move |(ok, e)| done(ok, e)),
            How::Boxed(name) => on_thread(move || on_disk(crate::store::boxed::install::remove(&name)), move |(ok, e)| done(ok, e)),
            How::Hide { launcher, id } => {
                let home = crate::common::files::home();
                on_thread(move || on_disk(removal::hide(&launcher, &id, &home)), move |(ok, e)| done(ok, e));
            }
            How::Package(package) => {
                let helper = admin_helper();
                capture(&["pkexec", helper.as_str(), "app-remove", package.as_str()], None,
                        move |ok, _, err| done(ok, if ok { err } else { admin_error(&err) }));
            }
            How::Refused(_) => {}
        }
    }

    pub(super) fn menu_card(&self, i: usize) -> Node<Msg> {
        let Some(app) = self.menu_app(i) else { return space() };
        if let Some(ask) = self.flat_ask.as_ref().filter(|a| a.id == app.id) {
            return self.flatpak_card(i, ask, &app.name);
        }
        let open = self.windows_of(&app);
        let mut items = vec![title(app.name.chars().take(MENU_TITLE_CHARS).collect::<String>())];
        items.extend(Self::window_rows(&app, &open));
        let mut tools = Self::open_tools(i, open.len());
        let fav = self.favourites.contains(&app.id);
        let full = !fav && self.favourites.len() >= store::MAX_FAVOURITES;
        tools.push(Self::star_button(&app, fav, full));
        if !favourites::is_path(&app.id) {
            tools.push(startup_button(&app.path));
        }

        let how = match &self.how {
            Some((id, Some(how))) if *id == app.id => Some(how),
            _ => None,
        };
        let refused = match how {
            Some(How::Refused(why)) => Some(*why),
            _ => None,
        };
        if let Some(how) = how.filter(|h| !matches!(h, How::Refused(_))) {
            tools.push(self.uninstall_button(i, &app.id, how));
        }

        items.push(row(tools.into_iter().map(|t| t.size(menu_button())).collect()).gap(8.0));
        if let Some(why) = refused {
            items.push(note(why));
        }
        if full {
            items.push(note(t!("ring-middle-full", n = store::MAX_FAVOURITES)));
        }

        card(column(items).gap(8.0).width(Length::Shrink)).width(Length::Shrink).on_escape(msg(|f| f.close_menu()))
    }

    fn window_rows(app: &Program, open: &[Window]) -> Vec<Node<Msg>> {
        let mut rows = Vec::new();
        for (n, w) in open.iter().cloned().enumerate() {
            let shown = if w.title.is_empty() { app.name.clone() } else { open_windows::shown_title(&w.title) };

            let line = card(label(shown)).flat().on_press(msg(move |f| {
                if let Some(ws) = f.windows.clone() {
                    ws.activate(&w);
                    f.close();
                }
            }));
            rows.push(if n % 2 == 1 { line.banded() } else { card(line).snug().flat() });
        }
        rows
    }

    fn open_tools(i: usize, windows: usize) -> Vec<Node<Msg>> {
        let mut tools = vec![button(if windows == 0 { "media-playback-start-symbolic" } else { "window-new-symbolic" },
                                    if windows == 0 { t!("ring-open") } else { t!("ring-new-window") })
            .on_press(msg(move |f| {
                if let Some(a) = f.menu_app(i) {
                    f.launch_new(&a)
                }
            }))];
        if windows > 0 {
            let words = t!("ring-close-window", n = windows);
            tools.push(button("window-close-symbolic", words).on_press(msg(move |f| {
                let wins = f.menu_app(i).map(|a| f.windows_of(&a)).unwrap_or_default();
                if let Some(ws) = &f.windows {
                    wins.iter().for_each(|w| ws.close(w));
                }
                f.close_menu();
            })));
        }
        tools
    }

    fn star_button(app: &Program, fav: bool, full: bool) -> Node<Msg> {
        let id = app.id.clone();

        let star = if favourites::is_path(&app.id) { t!("ring-star-unpin") } else if fav { t!("ring-star-remove") } else { t!("ring-star-add") };
        button(if fav { "starred-symbolic" } else { "non-starred-symbolic" }, star)
            .on(fav)
            .off(full)
            .on_press(msg(move |f| {
                f.set_favourite(&id, !fav);
                f.close_menu();
            }))
    }

    fn uninstall_button(&self, i: usize, id: &str, how: &How) -> Node<Msg> {
        let [words, running, finished] = if how.only_hides() { hide_words() } else { uninstall_words() };
        match &self.gone {
            _ if self.removing.iter().any(|r| r == id) => row(vec![ring(0.0, 16.0), note(running)]).gap(8.0),
            Some((j, Gone::Done(true, _))) if j == id => note(finished),
            Some((j, Gone::Done(false, e))) if j == id => button("dialog-error-symbolic", e.clone()).tone(Tone::Problem).spoken(t!("ring-not-done")),

            _ if matches!(how, How::Flatpak { .. }) => button("user-trash-symbolic", words).key(format!("uninstall-{i}"))
                .on_press(msg(move |f| f.ask_flatpak(i))),

            _ => crate::toolkit::widget::remove("user-trash-symbolic", words).key(format!("uninstall-{i}"))
                .on_press(msg(move |f| f.uninstall(i))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{begin_removal, failure, still_at, Program};

    #[test]
    fn a_failed_removal_shows_a_plain_sentence_not_the_programs_words() {
        let shown = failure("error: Failed to uninstall org.example.One: not installed\n".into());
        assert_eq!(shown, "Couldn't remove it.");
    }

    #[test]
    fn a_removal_already_running_is_not_started_again() {
        let mut running = Vec::new();
        assert!(begin_removal(&mut running, "a.desktop"));
        assert!(!begin_removal(&mut running, "a.desktop"));
        assert!(begin_removal(&mut running, "b.desktop"));
    }

    fn app(id: &str) -> Program {
        Program { id: id.into(), name: id.into(), icon: None, exec: String::new(), try_exec: None, wm_class: String::new(), program: String::new(),
                  kinds: Vec::new(), path: Default::default(), category: "", keywords: Vec::new(), flatpak: None, appimage: None, terminal: false }
    }

    #[test]
    fn a_menu_opened_on_an_app_does_not_follow_another_into_its_place() {
        let before = [app("a.desktop"), app("b.desktop"), app("c.desktop")];
        assert_eq!(still_at(&before, 1, "b.desktop").map(|a| a.id.as_str()), Some("b.desktop"));
        let after = [app("a.desktop"), app("c.desktop")];
        assert!(still_at(&after, 1, "b.desktop").is_none(), "c now sits where b was");
        assert!(still_at(&after, 2, "b.desktop").is_none(), "the place is gone");
    }
}

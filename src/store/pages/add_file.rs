//! Adding what the person brings.

use crate::store::came_from::{self, CameFrom};
use crate::store::{github, go, Msg, Store, View};
use crate::common::copies::Put;
use crate::common::run;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, note, row, space, Length, Node, Tone};
use std::cell::RefCell;
use std::rc::Rc;
use super::installed::installed_opened;

pub(super) fn add_appimage(s: &mut Store) {
    let Some(bus) = s.bus.clone() else {
        s.owned.adding = Some((t!("store-add-no-bus").into(), Tone::Problem));
        return;
    };
    let ask = Ask {
        title: t!("store-add-choose-title").into(),
        folder: Some(crate::common::files::home().join("Downloads")),
        name: None,
        filters: vec![(t!("store-add-filter").into(), vec!["*.AppImage".into(), "*.appimage".into()])],
    };
    let chosen = s.then(|s: &mut Store, picked: Option<std::path::PathBuf>| {
        s.chooser = None;
        let Some(source) = picked else {
            if crate::toolkit::chooser::failed() {
                s.owned.adding = Some((crate::toolkit::chooser::did_not_answer().into(), Tone::Problem));
            }
            return;
        };
        add_appimage_file(s, source, false, CameFrom::NotKnown);
    });
    s.chooser = chooser::open(&bus, &s.events, ask, chosen);
    if s.chooser.is_none() {
        s.owned.adding = Some((t!("store-add-no-chooser").into(), Tone::Problem));
    }
}

pub(crate) fn add_appimage_file(s: &mut Store, source: std::path::PathBuf, on_top: bool, from: CameFrom) {
    fn say(s: &mut Store, on_top: bool, words: String, tone: Tone) {
        if on_top {
            s.top = Some((words.clone(), tone));
        }
        s.owned.adding = Some((words, tone));
    }
    say(s, on_top, t!("store-add-adding").into(), Tone::Plain);
    let kept = from.clone();
    let back = s.then(move |s: &mut Store, result: Result<Put, String>| match result {
        Ok(Put::Taken(t)) => {
            s.owned.adding = None;
            if on_top {
                s.top = Some((t!("store-add-already", name = &t.name), Tone::Plain));
            }
            s.owned.appimage_taken = Some((t, from));
            if !on_top {
                s.select_nav("installed");
            }
        }
        Ok(_) => {
            s.top = None;
            appimage_in(s, !on_top && stays_put(s));
        }
        Err(e) => say(s, on_top, e, Tone::Problem),
    });
    run::on_thread(move || {
        let put = crate::icons::appimages::add(&source).map_err(plain_error);
        if let Ok(Put::Made(at) | Put::Already(at)) = &put {
            settled(&source, at, &kept);
        }
        put
    }, back);
}

fn settled(source: &std::path::Path, at: &std::path::Path, from: &CameFrom) {
    let _ = came_from::keep(&at.file_name().unwrap_or_default().to_string_lossy(), from);
    if source.parent() == Some(github::downloads().as_path()) && source != at {
        let _ = std::fs::remove_file(source);
    }
}

pub(crate) fn install_flatpakref(s: &mut Store, file: std::path::PathBuf) {
    let Some(about) = crate::store::dropped::reference(&file) else {
        s.top = Some((t!("store-add-bad-ref").into(), Tone::Problem));
        return;
    };
    s.top = Some((t!("store-add-installing-ref", name = &about.name, host = &about.host), Tone::Plain));
    let done = s.then(move |s: &mut Store, (ok, err): (bool, String)| {
        if ok {
            s.top = Some((t!("store-add-installed-ref", name = &about.name), Tone::Plain));
            installed_opened(s);
        } else {
            let why = err.trim().lines().last().unwrap_or(t!("store-add-ref-refused")).to_string();
            s.top = Some((t!("store-add-ref-failed", name = &about.name, why = why), Tone::Problem));
        }
    });
    let argv = crate::store::dropped::install_argv(&file);
    run::on_thread(move || {
        let (ok, _, err) = run::captured(&argv, None);
        (ok, err)
    }, done);
}

fn plain_error(e: std::io::Error) -> String {
    let t = e.to_string();
    t.split(" (os error ").next().unwrap_or(&t).to_string()
}

fn stays_put(s: &Store) -> bool {
    s.image.is_some() || matches!(s.view, View::Home | View::Category(_) | View::Search(_))
}

const APPIMAGE_TAB: usize = 3;

fn appimage_in(s: &mut Store, stay: bool) {
    if !stay {
        s.select_nav("installed");
        s.owned.tab = APPIMAGE_TAB;
        s.owned.find.clear();
    }
    let said: String = t!("store-add-in-ring").into();
    s.owned.adding = Some((said.clone(), Tone::Plain));
    let clear = s.then(move |s: &mut Store, ()| {
        if s.owned.adding.as_ref().is_some_and(|(w, _)| *w == said) {
            s.owned.adding = None;
        }
    });
    s.events.after(std::time::Duration::from_secs(4), move || clear(()));
    let f = s.then(|s: &mut Store, ()| installed_opened(s));
    run::capture(&["trindle-icons", "--quiet"], None, move |_, _, _| f(()));
}

fn appimage_skip(s: &mut Store) {
    s.owned.appimage_taken = None;
}

fn appimage_replace(s: &mut Store) {
    let Some((taken, from)) = s.owned.appimage_taken.take() else { return };
    s.owned.adding = Some((t!("store-add-replacing").into(), Tone::Plain));
    let back = s.then(|s: &mut Store, result: Result<(), String>| match result {
        Ok(()) => appimage_in(s, false),
        Err(e) => s.owned.adding = Some((e, Tone::Problem)),
    });
    run::on_thread(move || {
        let at = crate::icons::appimages::replace(&taken).map_err(plain_error)?;
        settled(&taken.from, &at, &from);
        Ok(())
    }, back);
}

pub(super) fn replace_card(t: &crate::common::copies::Taken) -> Node<Msg> {
    use crate::common::volumes::exact_size_words as size_words;
    let side = |head: &str, size: u64| column(vec![crate::toolkit::widget::column_heading(head.to_string()), note(size_words(size))]).gap(4.0).width(Length::Fill);
    card(column(vec![
        crate::toolkit::widget::form_label(t!("store-replace-head", name = &t.name)),
        row(vec![side(t!("store-replace-new"), t.new_size), side(t!("store-replace-old"), t.old_size)]).gap(12.0).width(Length::Fill),
        row(vec![
            space().width(Length::Fill),
            button("media-skip-forward-symbolic", t!("store-replace-skip")).on_press(go(appimage_skip)),
            crate::toolkit::widget::main_word(t!("store-replace-do")).on_press(go(appimage_replace)),
        ]).gap(12.0).width(Length::Fill),
    ]).gap(12.0).width(Length::Fill))
}

pub fn open_deb(s: &mut Store, file: std::path::PathBuf) {
    let asks = s.asks.clone();
    let me: Rc<RefCell<std::rc::Weak<_>>> = Rc::new(RefCell::new(std::rc::Weak::new()));
    let gone = me.clone();
    let closed = move || {
        let gone = gone.borrow().clone();
        asks.send(crate::store::Do::Then(Box::new(move |s: &mut Store| {
            s.debs.retain(|d| !std::ptr::eq(Rc::as_ptr(d), gone.as_ptr()));
            if s.debs.is_empty() && s.hidden && s.working == 0 {
                s.events.quit();
            }
        })));
    };
    let Some(display) = s.display.clone() else { return };
    if let Some(shown) = crate::store::deb::open(&s.events, &display, file, closed) {
        *me.borrow_mut() = Rc::downgrade(&shown);
        s.debs.push(shown);
    }
}

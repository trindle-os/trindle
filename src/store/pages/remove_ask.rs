//! The card that asks what to do with a Flatpak's saved data before it is removed.

use crate::store::catalogue::App;
use crate::store::{go, updates, Msg, Store};
use crate::common::run;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, main_word, note, row, Length, Node, Tone};
use super::install::{uninstall};
use super::installed::{remove_installed};

#[derive(Clone)]
pub(crate) struct RemoveAsk {
    pub(crate) id: String,
    name: String,
    app: Option<App>,
    item: Option<updates::Installed>,
    pub(crate) bytes: Option<u64>,
    pub(crate) uncounted: bool,
    working: bool,
    said: Option<(String, Tone)>,
}

pub(crate) fn ask_removing(s: &mut Store, app: Option<App>, item: Option<updates::Installed>) {
    let (id, name) = match (&app, &item) {
        (Some(a), _) => (a.id.clone(), a.name.clone()),
        (_, Some(i)) => (i.id.clone(), i.name.clone()),
        _ => return,
    };
    s.owned.ask = Some(RemoveAsk { id: id.clone(), name, app, item, bytes: None, uncounted: false, working: false, said: None });
    let back = s.then(move |s: &mut Store, bytes: std::io::Result<u64>| {
        if let Some(a) = s.owned.ask.as_mut().filter(|a| a.id == id) {
            match bytes {
                Ok(n) => a.bytes = Some(n),
                Err(_) => a.uncounted = true,
            }
        }
    });
    let id = s.owned.ask.as_ref().map(|a| a.id.clone()).unwrap_or_default();
    run::on_thread(move || crate::common::removal::data_dir(&crate::common::files::home(), &id).map_or(Ok(0), |d| crate::common::removal::data_size(&d)), back);
}

pub(crate) fn answer(s: &mut Store, wipe: bool) {
    if wipe && !crate::toolkit::window::persons_own(crate::common::person_only::APPS_DATA_DELETE) {
        return;
    }
    let Some(ask) = s.owned.ask.take() else { return };
    match (ask.app, ask.item) {
        (Some(a), _) => uninstall(s, &a, wipe),
        (_, Some(i)) => remove_installed(s, &i, wipe),
        _ => {}
    }
}

fn data_only(s: &mut Store) {
    if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_DATA_DELETE) {
        return;
    }
    let Some(ask) = s.owned.ask.as_mut().filter(|a| !a.working) else { return };
    ask.working = true;
    ask.said = Some((t!("store-flatpak-data-going").into(), Tone::Plain));
    let id = ask.id.clone();
    let run_id = id.clone();
    let back = s.then(move |s: &mut Store, res: Result<(), &'static str>| {
        let Some(a) = s.owned.ask.as_mut().filter(|a| a.id == id) else { return };
        a.working = false;
        match res {
            Ok(()) => {
                a.bytes = Some(0);
                a.said = Some((t!("store-flatpak-data-gone").into(), Tone::Plain));
            }
            Err(why) => a.said = Some((why.into(), Tone::Problem)),
        }
    });
    run::on_thread(move || crate::common::removal::delete_data_only(&crate::common::files::home(), &run_id), back);
}

fn removal_cancelled(s: &mut Store) {
    s.owned.ask = None;
}

pub(crate) fn remove_card(a: &RemoveAsk) -> Node<Msg> {
    use crate::common::volumes::exact_size_words as size_words;
    let size = match a.bytes {
        None if a.uncounted => t!("store-flatpak-ask-uncounted").to_string(),
        None => t!("store-flatpak-ask-measuring").to_string(),
        Some(0) => t!("store-flatpak-ask-none").to_string(),
        Some(n) => t!("store-flatpak-ask-size", size = size_words(n)),
    };
    let why = if a.bytes == Some(0) { Some(t!("store-flatpak-data-only-none")) } else { crate::common::ui::wait_while((a.bytes.is_none() && !a.uncounted) || a.working) };
    let mut rows = vec![crate::toolkit::widget::form_label(t!("store-flatpak-ask-head", name = &a.name)), note(size)];
    if let Some((w, tone)) = &a.said {
        rows.push(note(w.clone()).tone(*tone));
    }
    rows.push(row(vec![
        main_word(t!("store-flatpak-keep")).on_press(go(|s| answer(s, false))).off_because(crate::common::ui::wait_while(a.working)).key("remove-keep"),
        main_word(t!("store-flatpak-wipe")).cannot_be_undone().on_press(go(|s| answer(s, true))).off_because(why).key("remove-wipe"),
        main_word(t!("store-flatpak-data-only")).cannot_be_undone().on_press(go(data_only)).off_because(why).key("remove-data-only"),
        button("window-close-symbolic", t!("store-flatpak-cancel")).on_press(go(removal_cancelled)).key("remove-cancel"),
    ]).gap(12.0).wraps());
    rows.push(note(t!("store-flatpak-ask-undone")));
    card(column(rows).gap(12.0).width(Length::Fill)).on_escape(go(removal_cancelled))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleting_saved_data_is_the_persons_own_press_and_cancel_removes_nothing() {
        use crate::toolkit::window::{refused_act, with_named, with_press};
        use updates::{Installed, Removal, Source};
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::store::Do| {});
        let item = Installed { id: "org.example.NoSuchDataTest".into(), name: "Thing".into(), version: String::new(), size: String::new(),
                               source: Source::Flatpak { origin: "flathub".into(), installation: "user".into() } };
        let mut s = Store::new(&events, asks, None);
        let noted = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        s.owned.removals = Some(noted.clone());
        let made_up = |s: &mut Store, act: &dyn Fn(&mut Store)| {
            let _ = refused_act();
            with_named(true, Some(7), || with_press(Some(8), || act(s)));
            assert!(refused_act(), "a refused press asked for no words");
        };
        ask_removing(&mut s, None, Some(item.clone()));
        made_up(&mut s, &|s| answer(s, true));
        assert!(noted.borrow().is_empty() && s.owned.ask.is_some());
        s.owned.ask.as_mut().unwrap().bytes = Some(10);
        made_up(&mut s, &|s| data_only(s));
        assert!(s.owned.ask.as_ref().is_some_and(|a| !a.working && a.said.is_none()), "a made-up press began deleting data");
        with_named(true, Some(7), || with_press(Some(8), || answer(&mut s, false)));
        assert_eq!(*noted.borrow(), [(Removal::Flatpak { system: false, delete_data: false }, item.id.clone())]);
        ask_removing(&mut s, None, Some(item.clone()));
        s.owned.ask.as_mut().unwrap().bytes = Some(10);
        with_named(true, Some(7), || with_press(Some(7), || data_only(&mut s)));
        assert!(s.owned.ask.as_ref().is_some_and(|a| a.working && a.said.is_some()), "the person's own press was refused");
        noted.borrow_mut().clear();
        removal_cancelled(&mut s);
        assert!(s.owned.ask.is_none() && noted.borrow().is_empty());
        ask_removing(&mut s, None, Some(item.clone()));
        s.owned.ask.as_mut().unwrap().uncounted = true;
        let card = remove_card(s.owned.ask.as_ref().unwrap());
        let shut = card.shut();
        for tip in ["Remove and delete its data", "Delete its data only"] {
            assert!(shut.iter().any(|(t, off, _)| t == tip && !*off), "{tip} is greyed when the data could not be counted");
        }
        let a = s.owned.ask.as_mut().unwrap();
        (a.uncounted, a.bytes) = (false, Some(0));
        let shut = remove_card(s.owned.ask.as_ref().unwrap()).shut();
        for tip in ["Remove and delete its data", "Delete its data only"] {
            assert!(shut.iter().any(|(t, off, why)| t == tip && *off && why.as_deref() == Some("It has no saved data to delete.")), "{tip}: {shut:?}");
        }
        assert!(shut.iter().any(|(t, off, _)| t.starts_with("Remove, keep") && !*off));
    }
}

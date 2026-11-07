//! Tests of the login screen.

use super::face::initials_colour;

use super::*;

const Q: &str = "Trindle Pocketkey: use your phone? (yes/no)";

struct Fake(Rc<RefCell<Vec<Value>>>);

impl Backend for Fake {
    fn send(&self, request: Value, _done: greetd::Done) {
        self.0.borrow_mut().push(request);
    }
    fn reset(&self) {}
}

fn greeter(user: &str) -> (Greeter, Rc<RefCell<Vec<Value>>>) {
    let events = Loop::new();
    let sent = Rc::new(RefCell::new(Vec::new()));
    let me = events.post(|_: Msg| {});
    (Greeter::new(Box::new(Fake(sent.clone())), events, me, user.into()), sent)
}

#[test]
fn the_login_screen_follows_the_language_and_falls_back_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let french = "greeter-welcome-back = Bon retour.\ngreeter-no-match = Ce nom et ce mot de passe ne vont pas ensemble.\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        let (g, _) = greeter("john");
        assert_eq!(g.welcome(), "Bon retour.");
        assert!(g.ready().starts_with("Type your password"), "{}", g.ready());
        let refused = json!({"type": "error", "error_type": "auth_error", "description": "AUTH_ERR"});
        assert_eq!(plain_error(&refused), "Ce nom et ce mot de passe ne vont pas ensemble.");
        let expired = json!({"type": "error", "error_type": "auth_error", "description": "ACCT_EXPIRED"});
        assert_eq!(plain_error(&expired), "This account has expired.");
    });
}

#[test]
fn the_login_screen_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    use crate::toolkit::ui::Ui;
    drawn::none(drawn::in_every_look("the login screen", 60, || {
        for (i, user) in ('a'..='z').enumerate() {
            let (mut g, _) = greeter(&user.to_string());
            g.phone = i % 2 == 0;
            g.light = Some(crate::common::style::light());
            if i % 3 == 0 {
                g.password = "wrong".into();
                g.problem = Some((no_match().into(), Under::Password));
            }
            drawn::page(&user.to_string());
            let mut ui = Ui::new(g.view(), Theme::chosen());
            let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }));
}

#[test]
#[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
fn shoot_the_login_screen() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
    for (name, phone, wrong) in [("greeter", false, false), ("greeter-phone", true, false), ("greeter-wrong", false, true)] {
        let (mut g, _) = greeter("john");
        g.phone = phone;
        g.light = Some(false);
        if wrong {
            g.password = "wrong".into();
            g.problem = Some((no_match().into(), Under::Password));
        }
        for (w, h) in [(1280u32, 800u32), (1920, 1080)] {
            let mut ui = Ui::new(g.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
            let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(std::path::Path::new(&dir).join(format!("{name}-{w}.png"))).unwrap();
        }
    }
    let (mut g, _) = greeter("john");
    g.light = Some(false);
    g.update(Msg::Pass("john-smith-17".into()));
    let mut ui = Ui::new(g.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
    ui.lay_at(1280, 800);
    let shoot = |ui: &mut Ui<Msg>, name: &str| {
        let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(std::path::Path::new(&dir).join(format!("eye-login-{name}.png"))).unwrap();
    };
    shoot(&mut ui, "hidden");
    ui.press_eye("pass");
    shoot(&mut ui, "shown");
}

#[test]
fn the_password_box_has_the_eye_which_hides_again_by_itself() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use std::time::{Duration, Instant};
    let (mut g, _) = greeter("john");
    g.light = Some(false);
    g.update(Msg::Pass("john-smith-17".into()));
    assert!(g.view().said().contains(&"secret: Password".to_string()));
    let mut ui = Ui::new(g.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
    ui.lay_at(1280, 800);
    let before = ui.boxes();
    let eye = ui.eye_place("pass").expect("the password box has no eye");
    let wide = |ui: &Ui<Msg>, id: &str| ui.boxes().into_iter().find(|(name, ..)| name == id).map(|(_, r, _)| r).unwrap();
    let (name, pass) = (wide(&ui, "name"), wide(&ui, "pass"));
    assert!(eye.x > pass.x && eye.x + eye.w < pass.x + pass.w && eye.y > pass.y && eye.y + eye.h < pass.y + pass.h);
    assert_eq!((name.x, name.h), (pass.x, pass.h));
    ui.press_eye("pass");
    assert!(ui.shows_secret("pass"));
    assert_eq!(ui.boxes(), before, "showing the password moved something");
    assert!(ui.tick(Instant::now() + crate::common::eye::REST + Duration::from_secs(1)));
    assert!(!ui.shows_secret("pass"));
    ui.press_eye("pass");
    g.update(Msg::Submit);
    g.update(Msg::Back(Step::Reply, Some(json!({"type": "error", "error_type": "auth_error", "description": "pam_authenticate: AUTH_ERR"})), None));
    ui.set_view(g.view());
    ui.lay_at(1280, 800);
    assert!(g.password.is_empty() && !ui.shows_secret("pass"));
    let places = |all: &[(String, crate::toolkit::widget::Rect, Option<crate::toolkit::widget::Rect>)]| -> Vec<(f32, f32, f32, f32)> {
        all.iter().map(|(_, r, _)| (r.x, r.y, r.w, if r.h >= 30.0 { r.h } else { 0.0 })).collect()
    };
    assert_eq!(places(&ui.boxes()), places(&before), "a wrong password moved something");
}

#[test]
fn the_phone_button_follows_the_name_typed() {
    let (mut g, _) = greeter("john");
    g.phone = true;
    g.update(Msg::NameDone);
    assert!(!g.phone, "a name Pocketkey does not vouch for has no phone button");
    g.phone = true;
    g.update(Msg::Pass("x".into()));
    g.update(Msg::Submit);
    assert!(!g.phone);
}

#[test]
fn when_the_phones_time_is_up_the_label_shows_and_the_phone_button_goes() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    for secs in crate::common::pocketkey::REAUTH.into_iter().filter(|s| *s != 0) {
        let (mut g, _) = greeter("john");
        g.phone = false;
        g.due = Some(secs);
        let view = g.view();
        let said = view.said().join("\n");
        assert!(said.contains(&crate::common::pocketkey::due_words(secs)), "{said}");
        assert!(!view.buttons().iter().any(|(tip, _)| tip == t!("greeter-use-phone")), "no phone button");
        let ui = Ui::new(view, Theme::chosen());
        let h = ui.natural_height(1024.0);
        assert!(h <= crate::common::screen_scale::LOGIN_ROOM as f32, "{h}");
    }
    let (g, _) = greeter("john");
    assert!(g.due.is_none(), "no daemon here, so nothing is due");
}

#[test]
fn everything_fits_a_small_screen() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let (mut g, _) = greeter("john");
    g.light = Some(true);
    g.phone = true;
    g.hint = true;
    let ui = Ui::new(g.view(), Theme::chosen());
    let h = ui.natural_height(1024.0);
    assert!(h <= crate::common::screen_scale::LOGIN_ROOM as f32, "{h}");
}

#[test]
fn a_failure_is_said_in_red_under_its_box_and_the_welcome_stays() {
    let (mut g, _) = greeter("john");
    assert_eq!(g.headline, welcome_back());
    g.update(Msg::Pass("wrong".into()));
    g.update(Msg::Submit);
    assert_eq!(g.headline, "Logging in\u{2026}");
    g.update(Msg::Back(Step::Reply, Some(json!({"type": "error", "error_type": "auth_error", "description": "pam_authenticate: AUTH_ERR"})), None));
    assert_eq!(g.headline, welcome_back());
    assert_eq!(g.problem, Some((no_match().into(), Under::Password)));
    let words = format!("{:?}", g.view().said());
    assert!(words.contains(no_match()), "{words}");
}

#[test]
fn a_question_from_the_login_goes_in_the_sentence_at_the_top() {
    let (mut g, _) = greeter("john");
    g.phone = false;
    g.update(Msg::Pass("hunter2".into()));
    g.update(Msg::Submit);
    g.update(Msg::Back(Step::Reply, Some(json!({"type": "auth_message", "auth_message_type": "visible", "auth_message": "Code from your app:"})), None));
    assert_eq!(g.headline, "Code from your app:");
    assert_eq!(g.placeholder, "Code from your app:");
    assert!(g.pending_question && !g.busy);
}

#[test]
fn the_same_name_always_gets_the_same_colour() {
    assert_eq!(initials_colour("john"), initials_colour("john"));
    let all: std::collections::HashSet<String> = ["john", "jane", "sam", "alex", "kim", "lee"].iter()
        .map(|n| format!("{:?}", initials_colour(n))).collect();
    assert!(all.len() > 1);
}

#[test]
fn logging_in_asks_greetd_for_the_name_then_sends_the_password() {
    let (mut g, sent) = greeter("");
    g.phone = false;
    g.update(Msg::Submit);
    assert_eq!(g.problem, Some((t!("greeter-name-first").into(), Under::Name)));
    assert!(sent.borrow().is_empty());
    assert!(g.grab_name > g.grab_pass);

    g.update(Msg::Name("john".into()));
    g.update(Msg::NameDone);
    assert!(g.grab_pass > g.grab_name);
    g.update(Msg::Pass("hunter2".into()));
    g.update(Msg::Submit);
    assert!(g.busy);
    assert_eq!(sent.borrow()[0], json!({"type": "create_session", "username": "john"}));
    g.update(Msg::Pass("other".into()));
    assert_eq!(g.password, "hunter2");

    g.update(Msg::Back(Step::Reply, Some(json!({"type": "auth_message", "auth_message_type": "secret", "auth_message": "Password:"})), None));
    assert_eq!(sent.borrow()[1], json!({"type": "post_auth_message_response", "response": "hunter2"}));
    g.update(Msg::Back(Step::Reply, Some(json!({"type": "error", "error_type": "auth_error", "description": "pam_authenticate: AUTH_ERR"})), None));
    assert_eq!(g.problem, Some((no_match().into(), Under::Password)));
    assert_eq!(g.headline, welcome_new());
    assert!(!g.busy);
    assert!(g.password.is_empty());
    assert_eq!(sent.borrow()[2], json!({"type": "cancel_session"}));
}

#[test]
fn power_needs_two_presses() {
    let (mut g, _) = greeter("john");
    g.update(Msg::Power("Reboot"));
    assert_eq!(g.armed, Some("Reboot"));
    g.update(Msg::Disarm);
    assert_eq!(g.armed, None);
}

#[test]
fn the_phone_question_is_the_pocketkey_one() {
    assert_eq!(pocketkey::QUESTION.to_str(), Ok(Q));
}

#[test]
fn only_the_chosen_phone_says_yes() {
    let mut secret = Some("hunter2".to_string());
    assert_eq!(answer("visible", Q, true, &mut secret), Answer::Send(Some("yes".into())));
    assert_eq!(answer("visible", Q, false, &mut secret), Answer::Send(Some("no".into())));
    assert_eq!(answer("secret", "Password:", false, &mut secret), Answer::Send(Some("hunter2".into())));
    assert_eq!(secret, None);
    for near in ["Trindle Pocketkey: use your phone? (yes/no) ", "trindle pocketkey: use your phone? (yes/no)", "Use your phone?", ""] {
        assert_eq!(answer("visible", near, true, &mut None), Answer::Ask, "{near:?}");
    }
    assert_eq!(answer("secret", Q, false, &mut Some("hunter2".into())), Answer::Send(Some("no".into())), "the question is not a password prompt");
    assert_eq!(answer("secret", Q, true, &mut None), Answer::Send(Some("yes".into())));
}

#[test]
fn the_password_never_answers_the_phone_question() {
    let mut secret = Some("hunter2".to_string());
    assert_eq!(answer("visible", Q, false, &mut secret), Answer::Send(Some("no".into())));
    assert_eq!(answer("info", "Hello", false, &mut secret), Answer::Send(None));
    assert_eq!(secret.as_deref(), Some("hunter2"));
}

#[test]
fn a_password_prompt_after_the_phone_means_it_did_not_work() {
    assert_eq!(answer("secret", "Password:", true, &mut None), Answer::PhoneFailed);
    assert_eq!(answer("info", "Touch your phone", true, &mut None), Answer::Send(None));
}

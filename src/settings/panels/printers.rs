//! Printers via the CUPS command-line tools.

use std::time::Duration;

use crate::common::cups::{explain, is_job_id, parse_default, parse_jobs, parse_printers, Printer, Queued};
use crate::common::run::{capture, have};
use crate::settings::panels::network::banded;
use crate::settings::ui::{confirm, group, headline, line, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, note, row, Length, Node, Tone};

const ANNOUNCED_SECS: f64 = 8.0;
const ASKED_SECS: f64 = 25.0;

const TEST_PAGE: &str = "/usr/share/cups/data/testprint";

const SEARCH: &str = "printers:search";
const ADDRESS: &str = "printers:address";
const CUPS: &str = "printers:cups";
const JOINED: Duration = Duration::from_millis(600);

fn best_driver(list: &str, model: &str) -> Option<String> {
    let wanted = model.to_lowercase();
    let words: Vec<&str> = wanted.split_whitespace().filter(|w| w.len() > 2).collect();
    let mut best: Option<(usize, String)> = None;
    for line in list.lines() {
        let Some((driver, description)) = line.trim().split_once(' ') else { continue };
        let description = description.to_lowercase();
        let score = words.iter().filter(|w| description.contains(**w)).count();
        if score == 0 {
            continue;
        }
        if best.as_ref().is_none_or(|(b, _)| score > *b) {
            best = Some((score, driver.to_string()));
        }
    }
    best.filter(|(score, _)| *score * 2 >= words.len()).map(|(_, d)| d)
}

#[derive(Clone)]
struct Found {
    uri: String,
    model: Option<String>,
}

fn parse_found(out: &str) -> Vec<Found> {
    out.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("ipp://") || l.starts_with("ipps://"))
        .map(|l| Found { uri: l.to_string(), model: None })
        .collect()
}

fn parse_probe(out: &str) -> Vec<Found> {
    out.lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("network ")?;
            let (uri, rest) = rest.split_once(' ')?;
            let mut quoted = rest.split('"').skip(1).step_by(2);
            let model = quoted.nth(1).map(str::to_string).filter(|m| !m.is_empty());
            Some(Found { uri: uri.to_string(), model })
        })
        .collect()
}

fn unescape(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = s.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn queue_name(uri: &str) -> String {
    let rest = uri.strip_prefix("ipps://").or_else(|| uri.strip_prefix("ipp://")).unwrap_or(uri);
    let host = rest.split('/').next().unwrap_or("").split(':').next().unwrap_or("");
    let host = unescape(host).unwrap_or(host.to_string());
    let host = host.replace(".local", "");
    let mut name = String::new();
    let mut bad = false;
    for c in host.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            if bad {
                name.push('_');
                bad = false;
            }
            name.push(c);
        } else {
            bad = true;
        }
    }
    if bad {
        name.push('_');
    }
    let name: String = name.trim_matches('_').chars().take(60).collect();
    if name.is_empty() { "Printer".to_string() } else { name }
}

fn fail(ok: bool, err: &str) -> Option<String> {
    (!ok).then(|| explain(err))
}

fn sum_up(printers: &[Printer], default: Option<&str>) -> (String, Tone) {
    let first = default.and_then(|d| printers.iter().find(|p| p.name == d));
    match first {
        _ if printers.is_empty() => (t!("settings-printers-sum-none").into(), Tone::Plain),
        Some(p) if p.disabled => (t!("settings-printers-sum-paused", name = &p.name), Tone::Warn),
        Some(p) if p.state == "printing" => (t!("settings-printers-sum-printing", name = &p.name), Tone::Calm),
        Some(p) if p.state == "idle" => (t!("settings-printers-sum-ready", name = &p.name), Tone::Plain),
        Some(p) => (t!("settings-printers-sum-state", name = &p.name, state = &p.state), Tone::Calm),
        None if printers.len() == 1 => (t!("settings-printers-sum-one-unchosen", name = &printers[0].name), Tone::Calm),
        None => (t!("settings-printers-sum-many-unchosen", n = printers.len()), Tone::Calm),
    }
}

fn run(kit: &Kit, argv: &[&str], f: impl FnOnce(&mut Printers, &mut Kit, bool, String, String) + 'static) {
    let back = kit.then::<Printers, (bool, String, String)>(move |me, k, (ok, out, err)| f(me, k, ok, out, err));
    capture(argv, None, move |ok, out, err| back((ok, out, err)));
}

fn found_key(uri: &str) -> String {
    format!("found:{uri}")
}

fn printer_key(name: &str) -> String {
    format!("printer:{name}")
}

#[derive(Default)]
pub struct Printers {
    installed: bool,
    read: bool,
    broken: Option<String>,
    printers: Vec<Printer>,
    default: Option<String>,
    searched: bool,
    can_search: bool,
    found: Option<Vec<Found>>,
    round: u32,
    address: String,
    jobs: Vec<Queued>,
    jobs_read: bool,
    waited: u32,
}

const WATCH_SECS: u32 = 3;
const JOBS: &str = "printers:jobs";

fn job_key(id: &str) -> String {
    format!("job:{id}")
}

impl Printers {
    fn refresh(&mut self, kit: &mut Kit) {
        run(kit, &["lpstat", "-p", "-d"], |me, k, ok, out, err| me.got(k, ok, &out, &err));
    }

    fn refresh_jobs(&mut self, kit: &mut Kit) {
        self.waited = 0;
        run(kit, &["lpstat", "-o"], |me, _k, ok, out, _err| {
            if ok {
                me.jobs = parse_jobs(&out);
                me.jobs_read = true;
            }
        });
    }

    fn cancel(&mut self, kit: &mut Kit, id: &str) {
        let key = job_key(id);
        if !is_job_id(id) {
            kit.done(&key, Some(t!("settings-printers-bad-job")), t!("settings-printers-cancelled"));
            return;
        }
        run(kit, &["cancel", id], move |me, k, ok, _o, e| {
            k.done(&key, fail(ok, &e).as_deref(), t!("settings-printers-cancelled"));
            me.refresh_jobs(k);
        });
    }

    fn got(&mut self, kit: &mut Kit, ok: bool, out: &str, err: &str) {
        self.read = true;
        if !ok && !err.contains("No destinations") && !err.to_lowercase().contains("no system default") {
            let e = err.trim();
            self.broken = Some(e.to_string());
            kit.say(CUPS, if e.is_empty() { t!("settings-printers-lpstat-failed") } else { e }, false, false);
            return;
        }
        self.broken = None;
        self.printers = parse_printers(out);
        self.default = parse_default(out);
        self.refresh_jobs(kit);
        self.search(kit);
    }

    fn known(&self, name: &str) -> bool {
        self.printers.iter().any(|p| p.name == name)
    }

    fn search(&mut self, kit: &mut Kit) {
        self.round += 1;
        let round = self.round;
        self.searched = true;
        self.found = None;
        self.can_search = have("driverless");
        if !self.can_search {
            return;
        }
        let d = kit.start(SEARCH, t!("settings-printers-searching"));
        d.step(1, 2, "");
        d.expect(ANNOUNCED_SECS / 2.0);
        let limit = format!("{ANNOUNCED_SECS}");
        run(kit, &["timeout", &limit, "driverless"], move |me, k, _ok, out, _err| {
            if me.round != round {
                return;
            }
            let announced = parse_found(&out);
            if let Some(d) = k.dial(SEARCH) {
                d.step(2, 2, "");
                d.expect(ASKED_SECS / 2.0);
            }
            let limit = format!("{ASKED_SECS}");
            run(k, &["timeout", &limit, "/usr/lib/cups/backend/snmp"], move |me, k, _ok, out, _err| {
                if me.round != round {
                    return;
                }
                let mut all = announced;
                for found in parse_probe(&out) {
                    if !all.iter().any(|a| a.uri == found.uri) {
                        all.push(found);
                    }
                }
                k.finish(SEARCH, true, t!("settings-printers-search-done"));
                k.after::<Printers>(JOINED, move |me, _| {
                    if me.round == round {
                        me.found = Some(all);
                    }
                });
            });
        });
    }

    fn make_default(&mut self, kit: &mut Kit, name: &str) {
        let key = printer_key(name);
        run(kit, &["lpoptions", "-d", name], move |me, k, ok, _o, e| {
            k.done(&key, fail(ok, &e).as_deref(), t!("settings-printers-default-now"));
            me.refresh(k);
        });
    }

    fn test_page(&mut self, kit: &mut Kit, name: &str) {
        let key = printer_key(name);
        run(kit, &["lp", "-d", name, TEST_PAGE], move |_, k, ok, _o, e| k.done(&key, fail(ok, &e).as_deref(), t!("settings-printers-sent")));
    }

    fn remove(&mut self, kit: &mut Kit, name: &str) {
        let key = printer_key(name);
        run(kit, &["lpadmin", "-x", name], move |me, k, ok, _o, e| {
            k.done(&key, fail(ok, &e).as_deref(), t!("settings-printers-removed"));
            if ok {
                me.refresh(k);
            }
        });
    }

    fn add_typed(&mut self, kit: &mut Kit) {
        let typed = self.address.trim().to_string();
        if typed.is_empty() {
            kit.say(ADDRESS, t!("settings-printers-type-address"), false, false);
            return;
        }
        let name = queue_name(&typed);
        let uri = if typed.contains("://") { typed.clone() } else { format!("ipp://{typed}/ipp/print") };
        let fallback = (!typed.contains("://")).then(|| format!("socket://{typed}:9100"));
        kit.say(ADDRESS, t!("settings-printers-adding"), true, true);
        let name2 = name.clone();
        run(kit, &["lpadmin", "-p", &name, "-E", "-v", &uri, "-m", "everywhere"], move |me, k, ok, _o, e| {
            if ok {
                k.done(ADDRESS, None, t!("settings-printers-added"));
                me.refresh(k);
                return;
            }
            let Some(fallback) = fallback else {
                k.done(ADDRESS, Some(&explain(&e)), t!("settings-printers-added"));
                return;
            };
            run(k, &["lpadmin", "-p", &name2, "-E", "-v", &fallback, "-m", "everywhere"], move |me, k, ok, _o, e| {
                k.done(ADDRESS, if ok { None } else { Some(explain(&e)) }.as_deref(), t!("settings-printers-added"));
                if ok {
                    me.refresh(k);
                }
            });
        });
    }

    fn add_found(&mut self, kit: &mut Kit, printer: &Found, name: &str) {
        let key = found_key(&printer.uri);
        if printer.uri.starts_with("ipp") {
            self.add(kit, &key, &printer.uri, name);
            return;
        }
        let Some(model) = printer.model.clone() else {
            self.add(kit, &key, &printer.uri, name);
            return;
        };
        kit.say(&key, t!("settings-printers-finding-driver"), true, true);
        let (uri, name) = (printer.uri.clone(), name.to_string());
        run(kit, &["timeout", "30", "lpinfo", "-m"], move |_, k, _ok, out, _e| {
            let driver = best_driver(&out, &model).unwrap_or_else(|| "everywhere".into());
            run(k, &["lpadmin", "-p", &name, "-E", "-v", &uri, "-m", &driver], move |me, k, ok, _o, e| {
                k.done(&key, if ok { None } else { Some(explain(&e)) }.as_deref(), t!("settings-printers-added"));
                if ok {
                    me.refresh(k);
                }
            });
        });
    }

    fn add(&mut self, kit: &mut Kit, key: &str, uri: &str, name: &str) {
        kit.say(key, t!("settings-printers-adding"), true, true);
        let key = key.to_string();
        run(kit, &["lpadmin", "-p", name, "-E", "-v", uri, "-m", "everywhere"], move |me, k, ok, _o, e| {
            k.done(&key, if ok { None } else { Some(explain(&e)) }.as_deref(), t!("settings-printers-added"));
            if ok {
                me.refresh(k);
            }
        });
    }

    fn jobs_card(&self, kit: &Kit) -> Node<Msg> {
        let rows = if self.broken.is_some() {
            vec![line(t!("settings-printers-cups-silent")).sub(t!("settings-printers-queue-unreadable")).key(JOBS).node(kit)]
        } else if !self.jobs_read {
            vec![line(t!("settings-printers-looking")).sub(t!("settings-printers-queue")).key(JOBS).node(kit)]
        } else if self.jobs.is_empty() {
            vec![line(t!("settings-printers-nothing-waiting")).sub(t!("settings-printers-nothing-waiting-sub")).key(JOBS).node(kit)]
        } else {
            banded(self.jobs.iter().map(|j| {
                let id = j.id.clone();
                line(t!("settings-printers-job", number = j.id.rsplit_once('-').map_or("", |(_, n)| n), printer = &j.printer))
                    .sub(t!("settings-printers-job-sub", user = &j.user, size = crate::common::volumes::size_words(j.bytes), when = &j.when))
                    .icon("document-print-symbolic")
                    .key(job_key(&j.id))
                    .control(button("window-close-symbolic", t!("settings-printers-cancel-job"))
                        .on_press(on::<Printers>(move |me, k| me.cancel(k, &id))))
                    .node(kit)
            }).collect())
        };
        group(t!("settings-printers-jobs")).icon("document-print-symbolic")
            .about(t!("settings-printers-jobs-about"))
            .span(5, 8)
            .rows(rows)
            .node(kit)
    }

    fn printer_line(&self, kit: &Kit, p: &Printer) -> Node<Msg> {
        let is_default = self.default.as_deref() == Some(p.name.as_str());
        let state = if p.disabled {
            t!("settings-printers-state-paused").to_string()
        } else if p.state == "idle" {
            t!("settings-printers-state-idle").to_string()
        } else if p.state == "printing" {
            t!("settings-printers-state-printing").to_string()
        } else {
            let mut c = p.state.chars();
            c.next().map(|f| f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase()).unwrap_or_default()
        };
        let sub = if is_default { t!("settings-printers-default-state", state = &state) } else { state };
        let key = printer_key(&p.name);
        let mut l = line(p.name.clone()).sub(sub).icon("printer-symbolic").key(key.clone());
        if !is_default {
            let n = p.name.clone();
            l = l.control(button("starred-symbolic", t!("settings-printers-make-default"))
                .on_press(on::<Printers>(move |me, k| me.make_default(k, &n))));
        }
        let n = p.name.clone();
        l = l.control(button("document-print-symbolic", t!("settings-printers-test-page"))
            .on_press(on::<Printers>(move |me, k| me.test_page(k, &n))));
        let n = p.name.clone();
        l = l.control(confirm::<Printers>(kit, &key, "user-trash-symbolic", t!("settings-printers-remove"),
                                          move |me, k| me.remove(k, &n)));
        l.node(kit)
    }

    fn add_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = Vec::new();
        if !self.searched {
        } else if !self.can_search {
            rows.push(line(t!("settings-printers-needs-filters")).sub(t!("settings-printers-install-filters")).node(kit));
        } else if let Some(found) = &self.found {
            let mut lines = Vec::new();
            for printer in found {
                let name = printer.model.clone().map(|m| queue_name(&m)).unwrap_or_else(|| queue_name(&printer.uri));
                if self.known(&name) {
                    continue;
                }
                let title = printer.model.clone().unwrap_or_else(|| name.clone());
                let p = printer.clone();
                lines.push(line(title).sub(printer.uri.clone()).icon("printer-network-symbolic").key(found_key(&printer.uri))
                    .control(button("list-add-symbolic", t!("settings-printers-add-found"))
                        .on_press(on::<Printers>(move |me, k| me.add_found(k, &p, &name))))
                    .node(kit));
            }
            rows.extend(banded(lines));
            if rows.is_empty() {
                rows.push(line(t!("settings-printers-nothing-new")).sub(t!("settings-printers-nothing-new-sub")).node(kit));
            }
        } else {
            let mut wait = line(t!("settings-printers-looking")).sub(t!("settings-printers-looking-sub"));
            if let Some(d) = kit.dials.get(SEARCH) {
                wait = wait.control(row(vec![d.node(), note(d.words()).tone(d.tone())]).gap(8.0).width(Length::Shrink));
            }
            rows.push(wait.node(kit));
        }
        let typing = entry(self.address.clone(), t!("settings-printers-address-placeholder"), |t| on::<Printers>(move |me, _| me.address = t.clone()))
            .width(Length::Fill)
            .on_submit(on::<Printers>(|me, k| me.add_typed(k)));
        let add = button("list-add-symbolic", t!("settings-printers-add-it")).on_press(on::<Printers>(|me, k| me.add_typed(k)));
        rows.push(line(t!("settings-printers-by-address"))
            .sub(t!("settings-printers-by-address-sub"))
            .key(ADDRESS)
            .below(row(vec![typing, add]).gap(8.0).width(Length::Fill))
            .node(kit));
        group(t!("settings-printers-add-card")).icon("list-add-symbolic")
            .about(t!("settings-printers-add-about"))
            .span(4, 8)
            .rows(rows)
            .node(kit)
    }
}

impl Page for Printers {
    fn id(&self) -> &'static str {
        "printers"
    }
    fn title(&self) -> &'static str {
        t!("settings-printers-title")
    }
    fn icon(&self) -> &'static str {
        "printer-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-printers-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-printers-help")
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (words, _) = if !self.installed {
            (t!("settings-printers-top-not-installed").to_string(), Tone::Plain)
        } else if !self.read {
            (t!("settings-printers-top-looking").to_string(), Tone::Plain)
        } else if self.broken.is_some() {
            (t!("settings-printers-top-silent").to_string(), Tone::Problem)
        } else {
            sum_up(&self.printers, self.default.as_deref())
        };
        let mut items = vec![headline(kit, words)];
        if !self.installed {
            items.push(group(t!("settings-printers-printing")).icon("printer-symbolic")
                .about(t!("settings-printers-printing-about"))
                .rows(vec![
                    line(t!("settings-printers-unavailable")).sub(t!("settings-printers-install-cups")).node(kit),
                    line(t!("settings-printers-then-found")).sub(t!("settings-printers-then-found-sub")).node(kit),
                ])
                .node(kit));
            return page(kit, self.title(), self.help(), items);
        }
        let rows = if self.broken.is_some() {
            vec![line(t!("settings-printers-cups-silent")).sub(t!("settings-printers-service")).key(CUPS).node(kit)]
        } else if self.printers.is_empty() {
            vec![line(t!("settings-printers-none")).sub(t!("settings-printers-none-sub")).node(kit)]
        } else {
            banded(self.printers.iter().map(|p| self.printer_line(kit, p)).collect())
        };
        items.push(group(t!("settings-printers-card")).icon("printer-symbolic")
            .about(t!("settings-printers-card-about"))
            .span(5, 8)
            .rows(rows)
            .node(kit));
        items.push(self.jobs_card(kit));
        items.push(self.add_card(kit));

        page(kit, self.title(), self.help(), items)
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.installed = have("lpstat");
        if self.installed {
            self.refresh(kit);
        }
    }
    fn second(&mut self, kit: &mut Kit) -> bool {
        if !self.installed || !self.read || self.broken.is_some() {
            return false;
        }
        self.waited += 1;
        if self.waited >= WATCH_SECS {
            self.refresh_jobs(kit);
        }
        false
    }
}

#[cfg(test)]
impl Printers {
    pub(crate) fn flipped() -> Vec<Printers> {
        let none = Printers { printers: Vec::new(), default: None, ..Printers::made_up() };
        let broken = Printers { broken: Some("It did not answer.".into()), ..Printers::made_up() };
        vec![Printers::made_up(), none, broken]
    }

    pub(crate) fn made_up() -> Printers {
        let printer = |name: &str, state: &str| Printer { name: name.into(), state: state.into(), disabled: false };
        Printers { installed: true, read: true, broken: None,
                   printers: vec![printer("Office", "idle"), printer("Upstairs", "idle")], default: Some("Office".into()),
                   searched: true, can_search: true,
                   found: Some(vec![Found { uri: "ipp://printer.example.com/ipp/print".into(),
                                            model: Some("Example Laser 2000".into()) }]),
                   round: 0, address: String::new(),
                   jobs: vec![Queued { id: "Office-12".into(), printer: "Office".into(), user: "john".into(), bytes: 20480,
                                       when: "Sat 18 Oct 2026 10:00:00 AM BST".into() }],
                   jobs_read: true, waited: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn printer(name: &str, state: &str, disabled: bool) -> Printer {
        Printer { name: name.into(), state: state.into(), disabled }
    }

    #[test]
    fn the_sentence_follows_the_printer_used_first() {
        let (words, tone) = sum_up(&[], None);
        assert!(words.starts_with("No printers yet"));
        assert_eq!(tone, Tone::Plain);
        let ready = [printer("Office", "idle", false)];
        assert_eq!(sum_up(&ready, Some("Office")), ("Ready to print on Office.".to_string(), Tone::Plain));
        let paused = [printer("Office", "idle", true)];
        assert_eq!(sum_up(&paused, Some("Office")).1, Tone::Warn);
        let two = [printer("A", "idle", false), printer("B", "idle", false)];
        assert!(sum_up(&two, None).0.starts_with("2 printers"));
    }

    #[test]
    fn a_queue_name_unescapes_the_host() {
        assert_eq!(queue_name("ipp://My%20Printer.local:631/ipp/print"), "My_Printer");
        assert_eq!(queue_name("ipp://%zz/"), "zz");
    }

    #[test]
    fn the_sentence_and_the_paper_names_follow_the_language() {
        use crate::common::cups::Paper;
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-printers-sum-ready = Prêt à imprimer sur { $name }.\nprinters-paper-letter = Lettre\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            let ready = [printer("Office", "idle", false)];
            assert_eq!(sum_up(&ready, Some("Office")).0, "Prêt à imprimer sur Office.");
            assert_eq!((Paper::Letter.label(), Paper::A4.label()), ("Lettre", "A4"));
            assert_eq!(sum_up(&[], None).0, "No printers yet. Any found on your network are listed below.");
        });
    }
}

//! Blocking: adverts and bad sites kept out for the whole computer, and name
//! lookups made private, on one switch. The work is done by trindle-dns; see
//! common/dns.rs. The page reads what it last wrote, once a second while on
//! screen.

use crate::common::dns::{self, Private, Provider, Status};
use crate::common::run::{admin_error, admin_helper, capture};
use crate::settings::panels::network::banded;
use crate::settings::ui::{dropdown, group, headline, line, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::tiles::stat;
use crate::toolkit::widget::{button, column, entry, scroll, switch, Length, Node};

const SWITCH: &str = "blocking:switch";
const PROVIDER: &str = "blocking:provider";
const ADD: &str = "blocking:add";

const LIST_H: f32 = 320.0;

fn fitted(key: &str, rows: Vec<Node<Msg>>, lines: usize) -> Node<Msg> {
    if lines as f32 * crate::toolkit::theme::sheet().rich_row <= LIST_H {
        column(rows)
    } else {
        scroll(key, column(rows)).height(Length::Fixed(LIST_H))
    }
}

#[derive(Default)]
pub struct Blocking {
    on: bool,
    busy: bool,
    status: Option<Status>,
    provider: Option<Provider>,
    allowed: Vec<String>,
    typed: String,
}

fn ago(then: u64, now: u64) -> String {
    let s = now.saturating_sub(then);
    match s {
        0..=59 => t!("settings-blocking-ago-now").into(),
        60..=3599 => t!("settings-blocking-ago-minutes", n = s / 60),
        3600..=86_399 => t!("settings-blocking-ago-hours", n = s / 3600),
        _ => t!("settings-blocking-ago-days", n = s / 86_400),
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

impl Blocking {
    fn look(&mut self) {
        self.on = dns::on();
        self.status = if self.on { dns::status() } else { None };
        self.provider = Some(dns::provider());
        self.allowed = dns::allowed();
    }

    fn admin(&mut self, kit: &mut Kit, key: &'static str, args: Vec<String>, success: &'static str) {
        self.busy = true;
        let back = kit.then::<Blocking, (bool, String, String)>(move |me, k, (ok, _, err)| {
            me.busy = false;
            let err = admin_error(&err);
            k.done(key, if ok { None } else { Some(&err) }, success);
            me.look();
        });
        let helper = admin_helper();
        let mut argv = vec!["pkexec", helper.as_str()];
        argv.extend(args.iter().map(String::as_str));
        capture(&argv, None, move |ok, out, err| back((ok, out, err)));
    }

    fn set(&mut self, kit: &mut Kit, on: bool) {
        self.on = on;
        kit.say(SWITCH, t!("settings-blocking-waiting"), true, true);
        self.admin(kit, SWITCH, vec![if on { "dns-on" } else { "dns-off" }.into()],
                   if on { t!("settings-blocking-done-on") } else { t!("settings-blocking-done-off") });
    }

    fn choose(&mut self, kit: &mut Kit, p: Provider) {
        self.provider = Some(p);
        self.admin(kit, PROVIDER, vec!["dns-provider".into(), p.key().into()], t!("settings-blocking-saved"));
    }

    fn allow(&mut self, kit: &mut Kit, key: &'static str, site: String, add: bool) {
        self.admin(kit, key, vec!["dns-allow".into(), if add { "add" } else { "remove" }.into(), site], t!("settings-blocking-saved"));
    }

    fn add_typed(&mut self, kit: &mut Kit) {
        let Some(site) = dns::site(&self.typed) else {
            kit.say(ADD, t!("settings-blocking-bad-site"), false, false);
            return;
        };
        self.typed.clear();
        self.allow(kit, ADD, site, true);
    }

    fn top(&self, kit: &Kit) -> Node<Msg> {
        let words = match &self.status {
            _ if !self.on => t!("settings-blocking-top-off").to_string(),
            None => t!("settings-blocking-top-starting").to_string(),
            Some(s) => match s.private {
                Private::Yes => t!("settings-blocking-top-private", provider = self.provider.unwrap_or(Provider::Quad9).label()),
                Private::Fallback => t!("settings-blocking-top-fallback").to_string(),
                Private::Network => t!("settings-blocking-top-network").to_string(),
                Private::Vpn => t!("settings-blocking-top-vpn").to_string(),
                Private::Starting => t!("settings-blocking-top-on").to_string(),
            },
        };
        headline(kit, words)
    }

    fn figures(&self) -> Vec<Node<Msg>> {
        let (true, Some(s)) = (self.on, &self.status) else {
            let (now, words) = if self.on {
                (t!("settings-blocking-tile-starting"), t!("settings-blocking-tile-starting-note"))
            } else {
                (t!("settings-blocking-tile-off"), t!("settings-blocking-tile-off-note"))
            };
            return vec![
                stat("action-unavailable-symbolic", t!("settings-blocking-tile-today"), now, words, None),
                stat("view-list-symbolic", t!("settings-blocking-tile-lists"), now, words, None),
            ];
        };
        vec![
            stat("action-unavailable-symbolic", t!("settings-blocking-tile-today"), crate::common::thousands(s.today),
                 t!("settings-blocking-tile-today-note"), None),
            if s.listed == 0 {
                stat("view-list-symbolic", t!("settings-blocking-tile-lists"),
                     if s.fetching { t!("settings-blocking-tile-fetching") } else { t!("settings-blocking-tile-none") },
                     t!("settings-blocking-tile-lists-note"), None)
            } else {
                stat("view-list-symbolic", t!("settings-blocking-tile-lists"), crate::common::thousands(s.listed),
                     t!("settings-blocking-tile-listed-note", ago = ago(s.fetched, now())), None)
            },
        ]
    }

    fn recent(&self, kit: &Kit) -> Node<Msg> {
        let names = self.status.as_ref().map(|s| s.recent.as_slice()).unwrap_or(&[]);
        let lines = names.len().max(1);
        let rows: Vec<Node<Msg>> = if names.is_empty() {
            vec![line(t!("settings-blocking-nothing")).sub(t!("settings-blocking-nothing-sub")).node(kit)]
        } else {
            banded(names.iter().filter(|n| !self.allowed.contains(n)).map(|n| {
                let site = n.clone();
                let through = button("list-add-symbolic", t!("settings-blocking-let-through")).off_because(crate::common::ui::wait_while(self.busy))
                    .on_press(on::<Blocking>(move |me, kit| me.allow(kit, ADD, site.clone(), true)));
                line(n.clone()).control(through).node(kit)
            }).collect())
        };
        group(t!("settings-blocking-recent")).icon("action-unavailable-symbolic")
            .about(t!("settings-blocking-recent-about"))
            .span(4, 6)
            .waits_for((!self.on).then_some(t!("settings-blocking-recent-waits")))
            .rows(vec![fitted("blocking:blocked", rows, lines)])
            .node(kit)
    }

    fn let_through(&self, kit: &Kit) -> Node<Msg> {
        let typed = entry(self.typed.clone(), t!("settings-blocking-site-placeholder"), |t| on::<Blocking>(move |me, _| me.typed = t.clone()))
            .on_submit(on::<Blocking>(|me, kit| me.add_typed(kit)));
        let add = button("list-add-symbolic", t!("settings-blocking-let-through")).off_because(crate::common::ui::wait_while(self.busy))
            .on_press(on::<Blocking>(|me, kit| me.add_typed(kit)));
        let mut rows = vec![line(t!("settings-blocking-add")).sub(t!("settings-blocking-add-sub"))
            .control(add).below(typed).key(ADD).node(kit)];
        rows.extend(banded(self.allowed.iter().map(|s| {
            let site = s.clone();
            let take = crate::toolkit::widget::remove("list-remove-symbolic", t!("settings-blocking-block-again")).key(format!("unallow:{site}")).off_because(crate::common::ui::wait_while(self.busy))
                .on_press(on::<Blocking>(move |me, kit| me.allow(kit, ADD, site.clone(), false)));
            line(s.clone()).control(take).node(kit)
        }).collect()));
        group(t!("settings-blocking-allowed")).icon("list-add-symbolic")
            .about(t!("settings-blocking-allowed-about"))
            .span(4, 6)
            .rows(vec![fitted("blocking:let-through", rows, self.allowed.len() + 2)])
            .node(kit)
    }
}

impl Page for Blocking {
    fn id(&self) -> &'static str {
        "blocking"
    }
    fn title(&self) -> &'static str {
        t!("settings-blocking-title")
    }
    fn icon(&self) -> &'static str {
        "security-high-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-blocking-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-blocking-help")
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.look();
    }
    fn second(&mut self, _kit: &mut Kit) -> bool {
        if !self.busy {
            self.look();
        }
        !self.busy
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {

        let mut items = vec![self.top(kit)];
        let toggle = switch(self.on, t!("settings-blocking-switch"), |v| on::<Blocking>(move |me, k| me.set(k, v)))
            .off_because(crate::common::ui::wait_while(self.busy));
        let options: Vec<String> = Provider::ALL.iter().map(|p| p.label().to_string()).collect();
        let chosen = self.provider.and_then(|p| Provider::ALL.iter().position(|&q| q == p));
        let pick = dropdown(kit, PROVIDER, options, chosen, t!("settings-blocking-where"),
                            |i| on::<Blocking>(move |me, k| me.choose(k, Provider::ALL[i])));
        items.push(group(t!("settings-blocking-title")).icon("security-high-symbolic")
            .about(t!("settings-blocking-card-about"))
            .span(5, 7)
            .rows(vec![
                line(t!("settings-blocking-switch"))
                    .sub(t!("settings-blocking-switch-sub"))
                    .key(SWITCH).control(toggle).node(kit),
                line(t!("settings-blocking-private"))
                    .sub(t!("settings-blocking-private-sub"))
                    .key(PROVIDER).below(pick).node(kit),
            ])
            .node(kit));

        items.extend(self.figures());
        items.push(self.recent(kit));
        items.push(self.let_through(kit));
        page(kit, self.title(), self.help(), items)

    }
}

#[cfg(test)]
impl Blocking {

    pub(crate) fn flipped() -> Vec<Blocking> {
        let off = Blocking { on: false, status: None, ..Blocking::made_up() };
        let bare = Blocking { on: false, status: None, allowed: Vec::new(), ..Blocking::made_up() };
        let starting = Blocking { status: None, ..Blocking::made_up() };
        vec![Blocking::made_up(), off, bare, starting]
    }

    pub(crate) fn made_up() -> Blocking {
        let status = Status { listed: 912_345, fetched: now().saturating_sub(3 * 3600), fetching: false, today: 1_284,
                              private: Private::Yes,
                              recent: ["ads.example.com", "tracker.example.net", "pixel.example.org", "metrics.example.com"]
                                  .iter().map(|s| s.to_string()).collect() };
        Blocking { on: true, busy: false, status: Some(status), provider: Some(Provider::Quad9),
                   allowed: vec!["shop.example.com".into(), "news.example.org".into()], typed: String::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::ago;
    use crate::common::thousands;

    #[test]
    fn words() {
        assert_eq!(thousands(912_345), "912,345");
        assert_eq!(thousands(7), "7");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(ago(100, 130), "just now");
        assert_eq!(ago(0, 3 * 3600 + 5), "3 hours ago");
        assert_eq!(ago(0, 60), "1 minute ago");
        assert_eq!(ago(0, 2 * 86_400), "2 days ago");
    }

    #[test]
    fn the_page_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-blocking-ago-now = à l'instant\nsettings-blocking-ago-hours =\n    { $n ->\n        [one] il y a { $n } heure\n       *[other] il y a { $n } heures\n    }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(ago(100, 130), "à l'instant");
            assert_eq!(ago(0, 3 * 3600 + 5), "il y a 3 heures");
            assert_eq!(ago(0, 60), "1 minute ago", "a word French lacks is English");
        });
    }
}

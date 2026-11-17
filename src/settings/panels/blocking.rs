//! Firewall and blocking: everything that keeps things out. The firewall's
//! switch first, then the ports opened by hand and sharing with other
//! computers at home, then adverts and bad sites kept out for the whole
//! computer and name lookups made private, on one switch (trindle-dns; see
//! common/dns.rs). The page reads what it last wrote, once a second while
//! on screen.

mod ports;
mod sharing;
#[cfg(test)]
mod firewall_tests;

use crate::common::dns::{self, Private, Provider, Status};
use crate::toolkit::widget::banded_list;
use crate::settings::ui::{admin_step, dropdown, group, headline, line, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::tiles::stat;
use crate::toolkit::widget::{button, column, entry, scroll, switch, Length, Node};

const SWITCH: &str = "blocking:switch";
const FIREWALL: &str = "blocking:firewall";
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

    firewall: bool,

    not_in_force: bool,

    sharing: bool,

    shared: Option<Result<Vec<crate::common::sharing::Shared>, crate::common::sharing::Problem>>,

    own_ports: Vec<crate::common::firewall::Door>,

    port_typed: String,
    port_proto: usize,
    on: bool,
    busy: bool,
    status: Option<Status>,
    provider: Option<Provider>,
    allowed: Vec<String>,
    typed: String,

    minute: u64,
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

    fn look_at(&mut self, minute: u64) -> bool {
        if self.busy {
            return false;
        }
        let before = (self.firewall, self.sharing, self.own_ports.clone(), self.on, self.status.clone(), self.provider, self.allowed.clone());
        self.look();
        let moved = self.status.is_some() && std::mem::replace(&mut self.minute, minute) != minute;
        moved || before != (self.firewall, self.sharing, self.own_ports.clone(), self.on, self.status.clone(), self.provider, self.allowed.clone())
    }

    fn look(&mut self) {
        self.firewall = crate::common::firewall::on();
        self.sharing = crate::common::firewall::sharing();
        self.read_ports();
        self.on = dns::on();
        self.status = if self.on { dns::status() } else { None };
        self.provider = Some(dns::provider());
        self.allowed = dns::allowed();
    }

    fn admin(&mut self, kit: &mut Kit, key: &'static str, args: &[&str], success: &'static str) {
        self.busy = true;
        admin_step::<Blocking>(kit, key, args, t!("settings-blocking-waiting"), success, |me, _, _| {
            me.busy = false;
            me.look();
        });
    }

    fn set_firewall(&mut self, kit: &mut Kit, on: bool) {
        self.firewall = on;

        self.not_in_force = false;
        self.admin(kit, FIREWALL, &["firewall", if on { "on" } else { "off" }],
                   if on { t!("settings-firewall-done-on") } else { t!("settings-firewall-done-off") });
    }

    fn firewall_card(&self, kit: &Kit) -> Node<Msg> {
        let toggle = switch(self.firewall, t!("settings-firewall-switch"), |v| on::<Blocking>(move |me, k| me.set_firewall(k, v)))
            .off_because(crate::common::ui::wait_while(self.busy));
        group(t!("settings-firewall")).icon("security-high-symbolic")
            .about(t!("settings-firewall-about"))
            .rows(vec![line(t!("settings-firewall-switch"))
                .sub(match (self.firewall, self.not_in_force) {
                    (true, true) => t!("settings-firewall-not-in-force"),
                    (true, false) => "",
                    (false, _) => t!("settings-firewall-off-sub"),
                })
                .key(FIREWALL).control(toggle).node(kit)])
            .node(kit)
    }

    fn set(&mut self, kit: &mut Kit, on: bool) {
        self.on = on;
        self.admin(kit, SWITCH, &[if on { "dns-on" } else { "dns-off" }],
                   if on { t!("settings-blocking-done-on") } else { t!("settings-blocking-done-off") });
    }

    fn choose(&mut self, kit: &mut Kit, p: Provider) {
        self.provider = Some(p);
        self.admin(kit, PROVIDER, &["dns-provider", p.key()], t!("settings-blocking-saved"));
    }

    fn allow(&mut self, kit: &mut Kit, key: &'static str, site: String, add: bool) {
        self.admin(kit, key, &["dns-allow", if add { "add" } else { "remove" }, &site], t!("settings-blocking-saved"));
    }

    fn add_typed(&mut self, kit: &mut Kit) {
        let Some(site) = dns::site(&self.typed) else {
            kit.say(ADD, t!("settings-blocking-bad-site"), false, false);
            return;
        };
        self.typed.clear();
        self.allow(kit, ADD, site, true);
    }

    fn top(&self) -> Node<Msg> {
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
        headline(words)
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

    fn still_blocked(&self) -> Vec<&String> {
        self.status.as_ref().map(|s| s.recent.iter().filter(|n| !self.allowed.contains(n)).collect()).unwrap_or_default()
    }

    fn recent(&self, kit: &Kit) -> Node<Msg> {
        let names = self.still_blocked();
        let lines = names.len().max(1);
        let rows: Vec<Node<Msg>> = if names.is_empty() {
            vec![line(t!("settings-blocking-nothing")).node(kit)]
        } else {
            banded_list(names.iter().map(|n| {
                let site = n.to_string();
                let through = button("list-add-symbolic", t!("settings-blocking-let-through")).off_because(crate::common::ui::wait_while(self.busy))
                    .on_press(on::<Blocking>(move |me, kit| me.allow(kit, ADD, site.clone(), true)));
                line(n.to_string()).control(through).node(kit)
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
        let mut rows = vec![line(t!("settings-blocking-add"))
            .control(add).below(typed).key(ADD).node(kit)];
        rows.extend(banded_list(self.allowed.iter().map(|s| {
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
    fn shown(&mut self, kit: &mut Kit) {
        self.look();

        self.not_in_force = self.firewall && crate::common::run::text(&["systemctl", "show", "-p", "Result", "--value", "trindle-firewall.service"]).trim() != "success";
        self.read_shared(kit);
    }
    fn second(&mut self, _kit: &mut Kit) -> bool {
        self.look_at(now() / 60)
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {

        let mut items = vec![self.top(), self.firewall_card(kit), self.ports_card(kit), self.sharing_card(kit)];
        let toggle = switch(self.on, t!("settings-blocking-switch"), |v| on::<Blocking>(move |me, k| me.set(k, v)))
            .off_because(crate::common::ui::wait_while(self.busy));
        let options: Vec<String> = Provider::ALL.iter().map(|p| p.label().to_string()).collect();
        let chosen = self.provider.and_then(|p| Provider::ALL.iter().position(|&q| q == p));
        let pick = dropdown(PROVIDER, options, chosen, t!("settings-blocking-where"),
                            |i| on::<Blocking>(move |me, k| me.choose(k, Provider::ALL[i])));
        items.push(group(t!("settings-blocking-card")).icon("action-unavailable-symbolic")
            .about(t!("settings-blocking-card-about"))
            .span(5, 7)
            .rows(vec![
                line(t!("settings-blocking-switch"))
                    .key(SWITCH).control(toggle).node(kit),
                line(t!("settings-blocking-private"))
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
                   allowed: vec!["shop.example.com".into(), "news.example.org".into()], typed: String::new(),
                   firewall: true, not_in_force: false, sharing: false,
                   shared: Some(Ok(vec![crate::common::sharing::Shared { name: "photos".into(), path: "/home/john/Photos".into(),
                                                                          write: false, guests: false }])),
                   own_ports: vec![crate::common::firewall::parse_own("tcp:37666").unwrap()],
                   port_typed: String::new(), port_proto: 0, minute: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::{ago, Blocking};

    #[test]
    fn a_page_left_open_finds_nothing_new_each_second() {
        let mut page = Blocking::default();
        page.look_at(7);
        let news: Vec<bool> = (0..5).map(|_| page.look_at(7)).collect();
        assert!(news.iter().all(|n| !n), "an unchanged page was drawn again: {news:?}");
    }

    #[test]
    fn sites_let_through_are_not_in_the_recently_blocked_list() {
        let mut page = Blocking::made_up();
        assert_eq!(page.still_blocked().len(), 4);
        page.allowed = page.status.as_ref().unwrap().recent.clone();
        assert!(page.still_blocked().is_empty());
        page.status = None;
        assert!(page.still_blocked().is_empty());
    }

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

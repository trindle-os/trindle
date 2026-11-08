//! StatusNotifierItem watcher and host for other programs' tray icons, which
//! sit in a row at the bottom of the ring's centre. Item menus are read over
//! com.canonical.dbusmenu and shown as a card beside the icon.

pub(super) mod look;
mod outside;
pub(super) mod menu;
#[cfg(test)]
pub mod tests;

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::time::Duration;

use look::{Look, SIZE};
use menu::{kids, menu_card, part, text, truth};
use outside::{largest, Seen, Step, wanted_outside};
use zbus::zvariant::{OwnedValue, Structure, Value};

use crate::ring::{chosen, Job, plugins};
use crate::ring::ctx::{Ctx, Do, Msg, msg};
use crate::toolkit::t;
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props, Served};
use crate::toolkit::icon::Icons;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::widget::{self, Length, Node, Pointer};

pub const WATCHER: &str = "org.kde.StatusNotifierWatcher";

const WATCHER_PATH: &str = "/StatusNotifierWatcher";

const ITEM: &str = "org.kde.StatusNotifierItem";

const MENU: &str = "com.canonical.dbusmenu";

const WAIT: Duration = Duration::from_secs(5);

const MENU_WAIT: Duration = Duration::from_secs(2);

const MAX_PIXMAP: i64 = 1024;

const SYSTEM_IDS: &[&str] = &["nm-applet", "blueman", "pasystray", "volumeicon", "cbatticon", "pnmixer"];

const WATCHER_XML: &str = r#"
  <interface name="org.kde.StatusNotifierWatcher">
    <method name="RegisterStatusNotifierItem"><arg type="s" direction="in"/></method>
    <method name="RegisterStatusNotifierHost"><arg type="s" direction="in"/></method>
    <property name="RegisteredStatusNotifierItems" type="as" access="read"/>
    <property name="IsStatusNotifierHostRegistered" type="b" access="read"/>
    <property name="ProtocolVersion" type="i" access="read"/>
    <signal name="StatusNotifierItemRegistered"><arg type="s"/></signal>
    <signal name="StatusNotifierItemUnregistered"><arg type="s"/></signal>
    <signal name="StatusNotifierHostRegistered"/>
  </interface>
"#;

pub(super) fn row_gap() -> f32 {
    crate::toolkit::theme::sheet().ring_button_gap
}

#[derive(Default)]
struct Shown {

    id: String,
    menu_path: String,
    is_menu: bool,
    tip: String,
    visible: bool,
    look: Option<Rc<Look>>,
}

struct Item {
    service: String,
    path: String,

    anchor: String,
    st: RefCell<Shown>,
    hearing: RefCell<Vec<Listen>>,

    outside: RefCell<Outside>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Want {
    name: String,
    folder: String,
}

#[derive(Default)]
struct Outside {

    wanted: Option<Want>,

    out: Option<Want>,

    shown: Option<Want>,

    stand_in: Option<Rc<Look>>,
}

pub struct Tray {
    ctx: Rc<Ctx>,
    bus: Bus,
    me: Weak<Tray>,
    served: RefCell<Option<Served>>,
    owner: RefCell<Option<Listen>>,
    held: Cell<bool>,
    items: RefCell<Vec<Rc<Item>>>,
}

struct Entry {
    id: i32,
    label: String,
    separator: bool,
    visible: bool,
    enabled: bool,
    ticked: bool,
    submenu: bool,
    kids: Vec<Entry>,
}

struct Target {
    bus: Bus,
    ctx: Rc<Ctx>,
    service: String,
    path: String,

    pid: Rc<Cell<u32>>,
}

fn key(service: &str, path: &str) -> String {
    format!("{service}{path}")
}

impl Tray {

    pub fn ids(&self) -> Vec<String> {
        let mut all: Vec<String> = plugins::ids();
        all.extend(self.items.borrow().iter().filter(|i| i.st.borrow().visible).map(|i| i.st.borrow().id.clone()));
        all
    }

    pub fn icons(&self, in_panel: bool) -> Vec<(String, Node<Msg>)> {
        let button = crate::toolkit::theme::sheet().ring_button;
        let mut all: Vec<(String, Node<Msg>)> = plugins::icons(in_panel).into_iter().map(|(id, b)| (id, b.size(button))).collect();
        for i in self.items.borrow().iter() {
            if let Some(n) = self.node(i, in_panel) {
                all.push((i.st.borrow().id.clone(), n));
            }
        }
        all
    }

    pub fn strip(&self) -> (Node<Msg>, f32) {
        let button = crate::toolkit::theme::sheet().ring_button;
        let all = self.icons(false);
        if all.is_empty() {
            return (widget::space().width(Length::Fixed(0.0)).height(Length::Fixed(0.0)), 0.0);
        }
        let shown = chosen::shown();
        let mut all: Vec<Option<(String, Node<Msg>)>> = all.into_iter().map(Some).collect();
        let mut row: Vec<Node<Msg>> = Vec::new();
        for id in &shown {
            let at = all.iter().position(|a| a.as_ref().is_some_and(|(i, _)| i == id));
            row.extend(at.and_then(|at| all[at].take()).map(|(_, n)| n));
        }
        row.push(
            widget::button("view-more-symbolic", t!("ring-tray-all")).size(button).tinted(Job::Tray.share())
                .on_press(msg(|f| f.open_quick("tray"))),
        );
        let n = row.len() as f32;
        (widget::row(row).gap(row_gap()).width(Length::Shrink), n * button + (n - 1.0) * row_gap())
    }

    fn node(&self, item: &Rc<Item>, in_panel: bool) -> Option<Node<Msg>> {
        let st = item.st.borrow();
        if !st.visible {
            return None;
        }
        let look = st.look.clone();
        let (w, it) = (self.me.clone(), Rc::downgrade(item));
        let press = msg(move |_| {
            if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                t.click(&i);
            }
        });
        let (w, it) = (self.me.clone(), Rc::downgrade(item));
        let button = widget::button("", st.tip.clone()).size(crate::toolkit::theme::sheet().ring_button).tinted(Job::Tray.share()).spoken(if st.tip.is_empty() { t!("ring-tray-icon") } else { st.tip.as_str() })
            .face(move |c, r, ink, pen| {
                if let Some(l) = &look {
                    l.paint(c, r, ink, pen);
                }
            })
            .on_press(press);

        let size = crate::toolkit::theme::sheet().ring_button;
        let (kw, kit) = (self.me.clone(), Rc::downgrade(item));
        let menu = widget::drawing((size, size), "", |_, _| {}).on_pointer(move |p| {
            let (w, it) = (w.clone(), it.clone());
            match p {
                Pointer::Menu(..) => msg(move |_| {
                    if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                        t.menu(&i);
                    }
                }),
                Pointer::Middle(..) => msg(move |_| {
                    if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                        t.middle(&i);
                    }
                }),
                _ => Msg::idle(),
            }
        })
        .on_keys(move |input| {
            let opens = match input {
                Input::Key { sym: Keysym::Menu, .. } => true,
                Input::Key { sym: Keysym::F10, mods, .. } => mods.shift,
                _ => false,
            };
            let (w, it) = (kw.clone(), kit.clone());
            opens.then(|| msg(move |_| {
                if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                    t.menu(&i);
                }
            }))
        });
        let key = if in_panel { format!("panel:{}", item.anchor) } else { item.anchor.clone() };
        Some(widget::layers(vec![button, menu])
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .key(key))
    }

    fn watcher_offer(&self) -> Offer {
        let (w1, w2) = (self.me.clone(), self.me.clone());
        Offer {
            name: WATCHER.into(),
            xml: WATCHER_XML.into(),
            call: Box::new(move |asked: Asked| {
                let Some(t) = w1.upgrade() else { return asked.reply(&()) };
                if asked.member() != "RegisterStatusNotifierItem" {
                    return asked.reply(&());
                }
                let arg = asked.message().body().deserialize::<String>().unwrap_or_default();

                let (service, path) = if arg.starts_with('/') {
                    (asked.sender(), arg)
                } else {
                    (arg, "/StatusNotifierItem".to_string())
                };
                asked.reply(&());
                if !service.is_empty() {
                    t.add(&service, &path);
                }
            }),
            props: Box::new(move || {
                let names: Vec<String> = w2.upgrade().map(|t| t.items.borrow().iter()
                    .map(|i| key(&i.service, &i.path)).collect()).unwrap_or_default();
                let mut p = Props::new();
                let mut put = |k: &str, v: Value| {
                    if let Ok(v) = OwnedValue::try_from(v) {
                        p.insert(k.into(), v);
                    }
                };
                put("RegisteredStatusNotifierItems", Value::from(names));
                put("IsStatusNotifierHostRegistered", Value::from(true));
                put("ProtocolVersion", Value::from(0i32));
                p
            }),
            set: None,
        }
    }

    fn offer_watcher(&self) {
        let Ok(served) = self.bus.serve(WATCHER_PATH, self.watcher_offer()) else { return };
        *self.served.borrow_mut() = Some(served);

        let w = self.me.clone();
        self.bus.own(WATCHER, true, &self.ctx.lp, move |got| {
            let Some(t) = w.upgrade() else { return };
            if got {
                let _ = t.bus.emit(WATCHER_PATH, WATCHER, "StatusNotifierHostRegistered", &());
                t.held.set(true);
            } else {
                eprintln!("trindle-ring: another tray has taken the tray's name");
            }
        });

        let w = self.me.clone();
        let watching = self.bus.watch(WATCHER, &self.ctx.lp, move |owner| {
            let Some(t) = w.upgrade() else { return };
            if t.held.get() && owner != t.bus.unique_name() {
                eprintln!("trindle-ring: another tray has taken the tray's name");
                t.held.set(false);
            }
        }, || {});
        *self.owner.borrow_mut() = watching.ok();
    }

    fn add(&self, service: &str, path: &str) {
        if self.items.borrow().iter().any(|i| i.service == service && i.path == path) {
            return;
        }
        let item = Rc::new(Item {
            service: service.into(),
            path: path.into(),
            anchor: format!("sni:{}", key(service, path)),
            st: RefCell::new(Shown::default()),
            hearing: RefCell::new(Vec::new()),
            outside: RefCell::default(),
        });
        self.items.borrow_mut().push(item.clone());
        let _ = self.bus.emit(WATCHER_PATH, WATCHER, "StatusNotifierItemRegistered", &(key(service, path),));
        for sig in ["NewIcon", "NewAttentionIcon", "NewStatus", "NewTitle", "NewToolTip"] {
            let (w, it) = (self.me.clone(), Rc::downgrade(&item));
            let heard = self.bus.listen(Some(service), ITEM, Some(sig), Some(path), move |_| {
                if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                    t.refresh(&i);
                }
            });
            if let Ok(l) = heard {
                item.hearing.borrow_mut().push(l);
            }
        }
        let (w, it) = (self.me.clone(), Rc::downgrade(&item));
        let gone = self.bus.watch(service, &self.ctx.lp, |_| {}, move || {
            if let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) {
                t.remove(&i);
            }
        });
        if let Ok(l) = gone {
            item.hearing.borrow_mut().push(l);
        }
        self.refresh(&item);
    }

    fn remove(&self, item: &Rc<Item>) {
        self.items.borrow_mut().retain(|i| !Rc::ptr_eq(i, item));
        item.hearing.borrow_mut().clear();
        let _ = self.bus.emit(WATCHER_PATH, WATCHER, "StatusNotifierItemUnregistered", &(key(&item.service, &item.path),));
        self.ctx.tell(Do::Redraw);
    }

    fn refresh(&self, item: &Rc<Item>) {
        let (w, it) = (self.me.clone(), Rc::downgrade(item));
        self.bus.call(&item.service, &item.path, "org.freedesktop.DBus.Properties")
            .method("GetAll", &(ITEM,), false, Some(WAIT), &self.ctx.lp, move |r| {
                let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) else { return };
                let Ok(props) = r.and_then(|m| m.body().deserialize::<Props>().map_err(Into::into)) else { return };
                t.took(&i, &props);
            });
    }

    fn took(&self, item: &Rc<Item>, props: &Props) {
        let field = |k: &str| props.get(k).and_then(|v| text(v)).unwrap_or_default();
        let status = field("Status");
        let wants = if status == "NeedsAttention" { "Attention" } else { "" };
        let name = field(&format!("{wants}IconName"));
        let name = if name.is_empty() { field("IconName") } else { name };
        let pixmap = props.get(&format!("{wants}IconPixmap")).or_else(|| props.get("IconPixmap")).and_then(|v| largest(v));
        let in_set = self.named(&name);
        let want = wanted_outside(&name, &field("IconThemePath"), in_set.is_some(), pixmap.is_some());
        let look = in_set
            .or_else(|| pixmap.map(|px| Look::Pixels { px, cache: RefCell::new(None) }))
            .unwrap_or_else(|| Look::shape(Rc::new(Icons::new()), crate::common::apps::generic_icon_for(&item.service, &field("Title"))));

        let tip = props.get("ToolTip").and_then(|v| part(v, 2)).and_then(text).filter(|s| !s.is_empty())
            .or_else(|| Some(field("Title")).filter(|s| !s.is_empty()))
            .unwrap_or_else(|| field("Id"));

        let system = matches!(field("Category").as_str(), "Hardware" | "SystemServices")
            || SYSTEM_IDS.contains(&field("Id").as_str());
        {
            let mut st = item.st.borrow_mut();
            st.id = chosen::program_id(&field("Id"), &item.service, &item.path);
            st.menu_path = field("Menu");
            st.is_menu = props.get("ItemIsMenu").and_then(|v| truth(v)).unwrap_or(false);
            st.tip = tip;
            st.visible = status != "Passive" && !system;
            let look = Rc::new(look);
            if !item.outside.borrow_mut().keeps(&want, look.clone()) {
                st.look = Some(look);
            }
        }
        let step = item.outside.borrow_mut().asks(want);
        self.step(item, step);
        chosen::settle();
        self.ctx.tell(Do::Redraw);
    }

    fn step(&self, item: &Rc<Item>, step: Step) {
        let Step::Look(want) = step else { return };
        let (w, it) = (self.me.clone(), Rc::downgrade(item));
        outside::look_aside(want, move |seen| {
            let (Some(t), Some(item)) = (w.upgrade(), it.upgrade()) else { return };
            let (ours, next) = item.outside.borrow_mut().back();
            let shows = if ours { item.outside.borrow_mut().shows(seen.map(Seen::look)) } else { None };
            if let Some(look) = shows {
                item.st.borrow_mut().look = Some(look);
                t.ctx.tell(Do::Redraw);
            }
            t.step(&item, next);
        });
    }

    fn named(&self, name: &str) -> Option<Look> {
        if name.is_empty() || name.starts_with('/') {
            return None;
        }
        let symbolic = format!("{name}-symbolic");
        let exact = self.ctx.icons.find(&symbolic, SIZE, 1)
            .is_some_and(|f| f.file_stem().is_some_and(|s| s.to_string_lossy() == symbolic));
        if exact {
            return Some(Look::Named(symbolic));
        }
        self.ctx.icons.find(name, SIZE, 1).is_some().then(|| Look::shape(self.ctx.icons.clone(), name.into()))
    }

    fn click(&self, item: &Rc<Item>) {
        if item.st.borrow().is_menu {
            return self.menu(item);
        }
        let (w, it) = (self.me.clone(), Rc::downgrade(item));
        let (bus, lp, service, path) = (self.bus.clone(), self.ctx.lp.clone(), item.service.clone(), item.path.clone());

        crate::common::run::vouched(move |token| {
            if let Some(token) = token {
                bus.call(&service, &path, ITEM)
                    .method("ProvideXdgActivationToken", &(token,), false, Some(WAIT), &lp, |_| {});
            }
            bus.call(&service, &path, ITEM)
                .method("Activate", &(0i32, 0i32), false, Some(WAIT), &lp.clone(), move |r| {
                    let (Some(t), Some(i)) = (w.upgrade(), it.upgrade()) else { return };

                    if r.is_err() {
                        t.menu(&i);
                    } else {

                        t.ctx.face(|f| f.close());
                    }
                });
        });
    }

    fn middle(&self, item: &Rc<Item>) {
        self.bus.call(&item.service, &item.path, ITEM)
            .method("SecondaryActivate", &(0i32, 0i32), false, Some(WAIT), &self.ctx.lp, |_| {});
    }

    fn menu(&self, item: &Rc<Item>) {
        let menu_path = item.st.borrow().menu_path.clone();
        let path = menu_path.clone();

        let pid = Rc::new(Cell::new(0u32));
        let got = pid.clone();
        self.bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus")
            .method("GetConnectionUnixProcessID", &(item.service.as_str(),), false, Some(WAIT), &self.ctx.lp, move |r| {
                got.set(r.ok().and_then(|m| m.body().deserialize::<u32>().ok()).unwrap_or(0));
            });
        let (w, service, anchor, id) = (self.me.clone(), item.service.clone(), item.anchor.clone(), item.st.borrow().id.clone());
        let show = move |t: &Rc<Tray>, entries: Vec<Entry>| {
            let entries = Rc::new(entries);
            let open = Rc::new(RefCell::new(HashSet::new()));
            let to = Rc::new(Target { bus: t.bus.clone(), ctx: t.ctx.clone(), service, path, pid });
            let card: Rc<dyn Fn() -> Node<Msg>> = Rc::new(move || menu_card(&entries, &open, &to, &id));
            t.ctx.face(move |f| f.show_beside(&anchor, card));
        };

        if menu_path.is_empty() || menu_path == "/" {
            if let Some(t) = w.upgrade() {
                show(&t, Vec::new());
            }
            return;
        }
        let at = self.bus.call(&item.service, &menu_path, MENU);

        at.method("AboutToShow", &(0i32,), false, Some(MENU_WAIT), &self.ctx.lp, |_| {});
        at.method("GetLayout", &(0i32, -1i32, Vec::<String>::new()), false, Some(WAIT), &self.ctx.lp, move |r| {
            let Some(t) = w.upgrade() else { return };
            let layout = r.ok().and_then(|m| {
                let body = m.body();
                let body = body.deserialize::<Structure>().ok()?;
                body.fields().get(1).map(kids)
            });
            show(&t, layout.unwrap_or_default());
        });
    }
}

thread_local! {
    static TRAY: RefCell<Option<Rc<Tray>>> = const { RefCell::new(None) };
}

pub fn start(ctx: &Rc<Ctx>) {
    let Some(bus) = ctx.session.clone() else { return };
    let tray = Rc::new_cyclic(|me| Tray {
        ctx: ctx.clone(),
        bus,
        me: me.clone(),
        served: RefCell::new(None),
        owner: RefCell::new(None),
        held: Cell::new(false),
        items: RefCell::new(Vec::new()),
    });
    tray.offer_watcher();
    TRAY.with(|t| *t.borrow_mut() = Some(tray));
}

pub fn tray() -> Option<Rc<Tray>> {
    TRAY.with(|t| t.borrow().clone())
}

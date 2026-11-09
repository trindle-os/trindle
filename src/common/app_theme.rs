//! Trindle's look for other people's programs: GNOME's shapes and spacing,
//! in Trindle's colours and accent. GNOME-style programs (GTK 4 and
//! libadwaita) take the colours as named values they already draw with;
//! older GTK 3 programs, whose theme has its colours baked in, are painted
//! over widget by widget. Written for the look chosen now, at each login and
//! whenever the look or accent changes, into trindle.css beside the user's
//! own gtk.css, which only gains a line bringing it in.

use super::files::{config_home, read_text, write_text};
use super::style::{accent, light};
use crate::toolkit::theme;

const FILE: &str = "trindle.css";
const IMPORT: &str = "@import url(\"trindle.css\");";

struct Look {
    window: String,
    view: String,
    bar: String,
    side: String,
    card: String,
    pop: String,
    fg: String,
    line: String,
    shade: &'static str,
    accent: String,
    on_accent: String,

    accent_text: String,

    select: String,
    on_select: String,

    select_side: String,

    danger: String,
    on_danger: String,
    success: String,
    on_success: String,
    warning: String,
    on_warning: String,

    danger_text: String,
    success_text: String,
    warning_text: String,
}

fn look() -> Look {
    let light = light();
    let t = theme::Theme::new(light, &accent());
    let light = t.light;
    let c = |name: &str| theme::named(name, light).css();
    let solid = |c: crate::toolkit::colour::Colour| crate::toolkit::colour::Colour { a: 1.0, ..c };

    let written = |c: crate::toolkit::colour::Colour| crate::toolkit::colour::Colour::parse(&solid(c).css()).unwrap_or(c);
    let read_on = |c: crate::toolkit::colour::Colour, under: &[crate::toolkit::colour::Colour]| theme::clear(solid(c), under, theme::AIM_WORDS).css();
    let select = written(theme::over(theme::alpha(t.accent, if light { 0.30 } else { 0.42 }), t.panel));
    let (success, warning) = (solid(theme::good()), solid(crate::toolkit::colour::Colour::parse(if light { "#8f6c03" } else { "#ffc006" }).unwrap_or(t.accent)));

    let grounds: Vec<_> = ["trindle-page", "trindle-card", "trindle-well", "trindle-box"].iter().map(|name| written(theme::named(name, light))).collect();
    let words = |c: crate::toolkit::colour::Colour| read_on(c, &grounds);

    Look {
        danger_text: words(t.danger_text),
        success_text: words(t.fg),
        warning_text: words(warning),
        window: c("trindle-page"),
        view: c("trindle-card"),
        bar: if light { c("trindle-well") } else { c("trindle-card") },

        side: if light { c("trindle-well") } else { c("trindle-page") },
        card: if light { c("trindle-card") } else { c("trindle-box") },
        pop: if light { c("trindle-card") } else { c("trindle-box") },
        fg: t.fg.css(),
        line: c("trindle-edge"),
        shade: if light { "rgba(24, 24, 40, 0.10)" } else { "rgba(0, 0, 0, 0.36)" },
        on_accent: t.on_accent.css(),
        accent_text: t.accent_text.css(),
        on_select: read_on(t.fg, &[select]),
        select: select.css(),
        select_side: c("trindle-control"),
        accent: t.accent.css(),
        on_danger: theme::on(t.danger).css(),
        danger: solid(t.danger).css(),
        on_success: theme::on(success).css(),
        success: success.css(),
        on_warning: theme::on(warning).css(),
        warning: warning.css(),
    }
}

fn gtk4(l: &Look) -> String {
    let pairs: [(&str, &str); 37] = [
        ("accent_bg_color", &l.accent), ("accent_fg_color", &l.on_accent), ("accent_color", &l.accent_text),
        ("destructive_bg_color", &l.danger), ("destructive_fg_color", &l.on_danger), ("destructive_color", &l.danger_text),
        ("success_bg_color", &l.success), ("success_fg_color", &l.on_success), ("success_color", &l.success_text),
        ("warning_bg_color", &l.warning), ("warning_fg_color", &l.on_warning), ("warning_color", &l.warning_text),
        ("error_bg_color", &l.danger), ("error_fg_color", &l.on_danger), ("error_color", &l.danger_text),
        ("window_bg_color", &l.window), ("window_fg_color", &l.fg),
        ("view_bg_color", &l.view), ("view_fg_color", &l.fg),
        ("headerbar_bg_color", &l.bar), ("headerbar_fg_color", &l.fg), ("headerbar_border_color", &l.line),
        ("headerbar_backdrop_color", &l.window), ("headerbar_shade_color", l.shade),
        ("sidebar_bg_color", &l.side), ("sidebar_fg_color", &l.fg), ("sidebar_backdrop_color", &l.window),
        ("sidebar_border_color", &l.line),
        ("secondary_sidebar_bg_color", &l.window), ("secondary_sidebar_fg_color", &l.fg),
        ("card_bg_color", &l.card), ("card_fg_color", &l.fg),
        ("dialog_bg_color", &l.card), ("dialog_fg_color", &l.fg),
        ("popover_bg_color", &l.pop), ("popover_fg_color", &l.fg),
        ("thumbnail_bg_color", &l.card),
    ];
    let mut named = String::new();
    let mut vars = String::from(":root {\n");
    for (name, value) in pairs.iter().chain([("shade_color", l.shade), ("card_shade_color", l.shade),
                                               ("popover_shade_color", l.shade)].iter()) {
        named.push_str(&format!("@define-color {name} {value};\n"));
        vars.push_str(&format!("  --{}: {value};\n", name.replace('_', "-")));
    }
    vars.push_str("}\n");
    format!("{}\n{named}\n{vars}\n{}", header(), tips(l))
}

fn gtk3(l: &Look) -> String {
    let Look { window, view, bar, side, card, pop, fg, line, accent, on_accent, accent_text, danger, on_danger, select, on_select, select_side, .. } = l;
    format!(r#"{header}
@define-color theme_bg_color {window};
@define-color theme_fg_color {fg};
@define-color theme_base_color {view};
@define-color theme_text_color {fg};
@define-color theme_selected_bg_color {select};
@define-color theme_selected_fg_color {on_select};
@define-color borders {line};

window.background, .background, dialog .dialog-vbox {{ background-color: {window}; color: {fg}; }}
.view, iconview, treeview.view, textview text, .content-view {{ background-color: {view}; color: {fg}; }}
headerbar, headerbar:backdrop, .titlebar:not(headerbar) {{
  background: {bar}; border-color: {line}; box-shadow: none; color: {fg}; }}
.sidebar, placessidebar list, stacksidebar list,
.sidebar .view, .sidebar treeview.view {{ background-color: {side}; }}
.sidebar, placessidebar {{ border-color: {line}; }}
popover, popover.background, menu, .menu, .context-menu {{ background-color: {pop}; color: {fg}; }}
{tips}
notebook > header {{ background-color: {bar}; border-color: {line}; }}
notebook > header > tabs > tab, notebook > header > tabs > tab:hover, notebook > header > tabs > tab:checked {{ box-shadow: none; }}
notebook > header > tabs > tab:checked {{ background-color: {card}; border-radius: 8px; }}
stackswitcher > button:checked, .stack-switcher > button:checked {{ box-shadow: none; }}
notebook > stack:not(:only-child) {{ background-color: {view}; }}
link, *:link {{ color: {accent_text}; }}

button:not(.flat):not(.titlebutton):not(.suggested-action):not(.destructive-action) {{
  background-image: none; background-color: {card}; border-color: {line}; box-shadow: none; text-shadow: none; }}
button:not(.flat):not(.titlebutton):hover {{ background-image: none; background-color: mix({card}, {fg}, 0.06); }}
button:not(.flat):not(.titlebutton):active, button:not(.flat):not(.titlebutton):checked {{
  background-image: none; background-color: mix({card}, {fg}, 0.12); }}
button.suggested-action {{ background-image: none; background-color: {accent}; border-color: {accent}; color: {on_accent}; }}
button.destructive-action {{ background-image: none; background-color: {danger}; border-color: {danger}; color: {on_danger}; }}

headerbar button.titlebutton, .titlebar button.titlebutton {{
  background-image: none; background-color: mix({card}, {fg}, 0.10); border: none; box-shadow: none;
  border-radius: 9999px; min-width: 24px; min-height: 24px; padding: 3px; margin: 0 2px; }}
headerbar button.titlebutton:hover, .titlebar button.titlebutton:hover {{ background-color: mix({card}, {fg}, 0.18); }}
headerbar button.titlebutton:active, .titlebar button.titlebutton:active {{ background-color: mix({card}, {fg}, 0.26); }}

entry, spinbutton:not(.vertical) {{ background-image: none; background-color: {view}; border-color: {line}; color: {fg}; box-shadow: none; }}
entry:focus, spinbutton:focus {{ border-color: {accent}; box-shadow: inset 0 0 0 1px {accent}; }}

*:selected, *:selected:focus, .view:selected, treeview.view:selected, row:selected, flowbox flowboxchild:selected,
selection {{ background-color: {select}; color: {on_select}; }}
.sidebar .view:selected, .sidebar treeview.view:selected, .sidebar row:selected,
placessidebar row:selected {{ background-color: {select_side}; color: {fg}; }}
switch:checked, progressbar progress, scale highlight {{
  background-image: none; background-color: {accent}; border-color: {accent}; }}
levelbar block.filled {{ background-image: none; background-color: mix({fg}, {view}, 0.45); border-color: transparent; }}
check:checked, radio:checked {{ background-image: none; background-color: {accent}; border-color: {accent}; color: {on_accent}; }}
"#, header = header(), tips = tips(l))
}

fn tips(_l: &Look) -> String {
    let k = theme::sheet();
    format!("tooltip, tooltip.background {{ background-color: {}; color: {}; border: 1px solid {}; border-radius: 9999px; \
             padding: {}px {}px; font-size: {}px; font-weight: {}; box-shadow: 0 0 0 1px {}, 0 {}px {}px rgba(0, 0, 0, {}); }}\n",
            k.label_back.css(), k.label_ink.css(), k.label_edge.css(), k.label_padding.1, k.label_padding.0, k.label_text,
            k.label_weight, k.label_outline.css(), k.label_shadow.y, k.label_shadow.blur, k.label_shadow.alpha)
}

fn header() -> &'static str {
    "/* Trindle's colours, written by Trindle whenever the look changes in\n   \
     Settings. Anything of your own belongs in gtk.css, which is left alone. */"
}

pub(crate) fn import_into(own: &std::path::Path) -> std::io::Result<()> {
    let theirs = crate::common::files::read_small(own)?;
    if theirs.lines().any(|line| line.trim() == IMPORT) {
        return Ok(());
    }
    write_text(own, &format!("{IMPORT}\n{theirs}"))
}

pub fn write() {
    let l = look();
    for (dir, text) in [("gtk-4.0", gtk4(&l)), ("gtk-3.0", gtk3(&l))] {
        let dir = config_home().join(dir);
        let file = dir.join(FILE);
        if read_text(&file) != text {
            let _ = write_text(&file, &text);
        }

        let _ = import_into(&dir.join("gtk.css"));
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::colour::Colour;
    use crate::toolkit::theme::{contrast, Theme, EDGES, WORDS};

    #[test]
    fn what_is_written_for_other_programs_is_only_pairs_the_table_holds() {
        let c = |s: &str| Colour::parse(s).unwrap_or_else(|| panic!("{s}"));
        let mut short = Vec::new();
        for light in [false, true] {
            for accent in crate::toolkit::theme::drawn::highlights() {
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((light, accent.clone())));
                let (l, t) = (super::look(), Theme::chosen());
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
                assert_eq!((l.accent.clone(), l.on_accent.clone(), l.accent_text.clone(), l.fg.clone()),
                           (t.accent.css(), t.on_accent.css(), t.accent_text.css(), t.fg.css()), "light {light}, {accent}");
                assert_eq!((l.window.clone(), l.view.clone()), (t.bg.css(), t.panel.css()), "light {light}, {accent}");
                let words = [("words on the window", &l.fg, &l.window), ("words on the files", &l.fg, &l.view), ("words on the bar", &l.fg, &l.bar),
                             ("words on the side", &l.fg, &l.side), ("words on a card", &l.fg, &l.card), ("words on a menu", &l.fg, &l.pop),
                             ("words on the highlight", &l.on_accent, &l.accent), ("words on what is selected", &l.on_select, &l.select),
                             ("words on what is selected at the side", &l.fg, &l.select_side), ("words on a button that removes", &l.on_danger, &l.danger),
                             ("words on success", &l.on_success, &l.success), ("words on a warning", &l.on_warning, &l.warning),
                             ("a link on the window", &l.accent_text, &l.window), ("a link on the files", &l.accent_text, &l.view),
                             ("a link on a card", &l.accent_text, &l.card)];
                for (what, ink, ground) in words {
                    let stands = contrast(c(ink), c(ground));
                    if stands < WORDS {
                        short.push(format!("light {light}, {accent}: {what}, {ink} on {ground}: {stands:.2}"));
                    }
                }
                for (what, ground) in [("the window", &l.window), ("the files", &l.view), ("a card", &l.card)] {
                    let stands = contrast(c(&l.accent), c(ground));
                    if stands < EDGES {
                        short.push(format!("light {light}, {accent}: the highlight on {what}, {} on {ground}: {stands:.2}", l.accent));
                    }
                }
            }
        }
        assert!(short.is_empty(), "{} pairs written for other programs fall short:\n{}", short.len(), short.join("\n"));
    }

    #[test]
    fn the_colours_other_programs_write_words_in_are_only_pairs_the_table_holds() {
        let c = |s: &str| Colour::parse(s).unwrap_or_else(|| panic!("{s}"));
        let mut short = Vec::new();
        for light in [false, true] {
            for accent in crate::toolkit::theme::drawn::highlights() {
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((light, accent.clone())));
                let css = super::gtk4(&super::look());
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
                let named: Vec<(&str, &str)> = css.lines().filter_map(|l| l.trim().strip_prefix("--")?.strip_suffix(';')?.split_once(": ")).collect();
                let grounds: Vec<&(&str, &str)> = named.iter().filter(|(n, _)| n.ends_with("-bg-color") && !n.starts_with("accent")
                    && !["destructive", "success", "warning", "error"].iter().any(|s| n.starts_with(s))).collect();
                let words: Vec<&(&str, &str)> = named.iter().filter(|(n, _)| n.ends_with("-color")
                    && !["-bg-color", "-fg-color", "-border-color", "-shade-color", "-backdrop-color"].iter().any(|e| n.ends_with(e)) && *n != "shade-color").collect();
                assert!(grounds.len() >= 7 && words.len() >= 5, "{grounds:?} {words:?}");
                for (name, ink) in &words {
                    for (on, ground) in &grounds {
                        let stands = contrast(c(ink), c(ground));
                        if stands < WORDS {
                            short.push(format!("{} with {accent}: --{name} {ink} on --{on} {ground}: {stands:.2}", if light { "light" } else { "dark" }));
                        }
                    }
                }
            }
        }
        short.sort();
        short.dedup();
        assert!(short.is_empty(), "{} colours other programs write words in fall short:\n{}", short.len(), short.join("\n"));
    }

    #[test]
    fn a_success_other_programs_write_in_words_is_plain_not_green() {
        for light in [false, true] {
            for accent in crate::toolkit::theme::drawn::highlights() {
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((light, accent.clone())));
                let l = super::look();
                crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
                let w = Colour::parse(&l.success_text).unwrap_or_else(|| panic!("{}", l.success_text));
                assert!(!(w.g > w.r + 0.08 && w.g > w.b + 0.08), "light {light}, {accent}: a success is written in green: {}", l.success_text);
                assert_eq!(l.success_text, l.fg, "light {light}, {accent}");
            }
        }
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_words_other_programs_write() {
        use crate::toolkit::text::{Fonts, Style};
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from) else { return };
        std::fs::create_dir_all(&dir).unwrap();
        let c = |s: &str| Colour::parse(s).unwrap_or_else(|| panic!("{s}"));
        let fonts = Fonts::new();
        for light in [false, true] {
            crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = Some((light, crate::common::style::DEFAULT_ACCENT.to_string())));
            let l = super::look();
            crate::common::style::ASKED_FOR_TEST.with(|a| *a.borrow_mut() = None);
            let grounds = [("the window", &l.window), ("the files", &l.view), ("a bar", &l.bar), ("the side", &l.side), ("a card", &l.card)];

            let green = crate::toolkit::theme::clear(c(&l.success), &grounds.map(|(_, g)| c(g)), crate::toolkit::theme::AIM_WORDS).css();
            let before = [("An error", &l.danger_text), ("A success", &green), ("A warning", &l.warning_text)];
            let after = [("An error", &l.danger_text), ("A success", &l.success_text), ("A warning", &l.warning_text)];
            for (when, inks) in [("before", before), ("after", after)] {
                let (across, down) = (250u32, 44u32);
                let mut pix = tiny_skia::Pixmap::new(across * grounds.len() as u32, down * (inks.len() as u32 + 1)).unwrap();
                for (col, (on, ground)) in grounds.iter().enumerate() {
                    let x = (col as u32 * across) as f32;
                    let mut paint = tiny_skia::Paint::default();
                    paint.set_color(c(ground).to_skia());
                    let all = tiny_skia::Rect::from_xywh(x, 0.0, across as f32, pix.height() as f32).unwrap();
                    pix.as_mut().fill_rect(all, &paint, tiny_skia::Transform::identity(), None);
                    let head = fonts.layout(&format!("On {on}"), &Style::new(14.0), None);
                    fonts.draw(&mut pix.as_mut(), &head, x + 12.0, 12.0, c(&l.fg));
                    for (row, (what, ink)) in inks.iter().enumerate() {
                        let words = format!("{what}, {:.2} to one", contrast(c(ink), c(ground)));
                        let laid = fonts.layout(&words, &Style::new(15.0), None);
                        fonts.draw(&mut pix.as_mut(), &laid, x + 12.0, ((row as u32 + 1) * down) as f32 + 10.0, c(ink));
                    }
                }
                pix.save_png(dir.join(format!("other-programs-words-{}-{when}.png", if light { "light" } else { "dark" }))).unwrap();
            }
        }
    }

    #[test]
    fn every_colour_has_both_spellings() {
        let l = super::Look {
            window: "#000001".into(), view: "#000002".into(), bar: "#000003".into(), side: "#000004".into(),
            card: "#000005".into(), pop: "#000006".into(), fg: "#000007".into(), line: "#000008".into(),
            shade: "#000009", accent: "#00000a".into(), on_accent: "#00000b".into(), accent_text: "#00000c".into(),
            danger: "#00000d".into(), success: "#00000e".into(), warning: "#00000f".into(),
            select: "#000010".into(), select_side: "#000011".into(), on_select: "#000012".into(), on_danger: "#000013".into(),
            on_success: "#000014".into(), on_warning: "#000015".into(),
            danger_text: "#000016".into(), success_text: "#000017".into(), warning_text: "#000018".into(),
        };
        let css = super::gtk4(&l);
        assert!(css.contains("@define-color window_bg_color #000001;"));
        assert!(css.contains("--window-bg-color: #000001;"));
        assert!(css.contains("--accent-bg-color: #00000a;"));
        assert_eq!(css.matches("@define-color").count(), css.matches("  --").count());

        for pair in ["--accent-fg-color: #00000b;", "--destructive-fg-color: #000013;", "--error-fg-color: #000013;", "--success-fg-color: #000014;",
                     "--warning-fg-color: #000015;"] {
            assert!(css.contains(pair), "{pair}");
        }

        for words in ["--destructive-color: #000016;", "--error-color: #000016;", "--success-color: #000017;", "--warning-color: #000018;"] {
            assert!(css.contains(words), "{words}");
        }

        assert!(super::gtk3(&l).contains("selection { background-color: #000010; color: #000012; }"));
        assert!(super::gtk3(&l).contains("@define-color theme_selected_bg_color #000010;"));
        assert!(super::gtk3(&l).contains("@define-color theme_selected_fg_color #000012;"));
        assert!(super::gtk3(&l).contains("button.destructive-action { background-image: none; background-color: #00000d; border-color: #00000d; color: #000013; }"));

        let tip = "tooltip, tooltip.background { background-color: #2e4468; color: #f3f7fd; border: 1px solid rgba(200, 222, 255, 0.2); \
                   border-radius: 9999px;";
        assert!(css.contains(tip) && super::gtk3(&l).contains(tip), "{css}");
    }
}

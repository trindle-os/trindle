

use tiny_skia::Pixmap;
use crate::ring::Job;

const SITE_APPS: [(&str, &str, &str); 24] = [
    ("trindle-settings", "Settings", ""), ("firefox", "Firefox", ""), ("thunderbird", "Thunderbird", ""), ("libreoffice-writer", "Writer", ""),
    ("gimp", "GIMP", ""), ("vlc", "VLC", ""), ("trindle-store", "Apps", ""), ("accessories-calculator", "Calculator", ""), ("system-file-manager", "Folders", ""),
    ("chromium", "Chromium", "internet"), ("brave-browser", "Brave", "internet"), ("telegram", "Telegram", "internet"),
    ("libreoffice-calc", "Calc", "office"), ("libreoffice-impress", "Impress", "office"), ("libreoffice-draw", "Draw", "office"),
    ("inkscape", "Inkscape", "graphics"), ("krita", "Krita", "graphics"), ("blender", "Blender", "graphics"),
    ("audacity", "Audacity", "media"), ("rhythmbox", "Rhythmbox", "media"),
    ("accessories-text-editor", "Text", "utilities"), ("transmission", "Transmission", "utilities"),
    ("utilities-terminal", "Terminal", "system"), ("trindle-activity", "Activity", "system"),
];

fn site_wallpaper(file: &str, pw: u32, ph: u32) -> Pixmap {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/wallpapers").join(file);
    let img = image::open(path).unwrap().to_rgba8();
    let (iw, ih) = (img.width() as f32, img.height() as f32);
    let k = (pw as f32 / iw).max(ph as f32 / ih);
    let (sw, sh) = ((iw * k).ceil() as u32, (ih * k).ceil() as u32);
    let big = image::imageops::resize(&img, sw, sh, image::imageops::FilterType::Lanczos3);
    let (ox, oy) = ((sw - pw) / 2, (sh - ph) / 2);
    let mut pix = Pixmap::new(pw, ph).unwrap();
    for (i, px) in pix.pixels_mut().iter_mut().enumerate() {
        let p = big.get_pixel(ox + i as u32 % pw, oy + i as u32 / pw);
        *px = tiny_skia::PremultipliedColorU8::from_rgba(p[0], p[1], p[2], 255).unwrap();
    }
    pix
}

fn push_site_windows(layer: &mut Vec<crate::toolkit::widget::Node<Msg>>, wins: &[(&'static str, &'static str, bool, bool)], lay: &Layout<usize>, (w, h): (f32, f32)) {
    use super::*;
    use crate::toolkit::widget::{drawing, placed, Node};
    let (cx, cy) = (w / 2.0, h / 2.0);
    if let Some(rg) = open_windows::ring(wins.len(), lay.disc_radius, (w.min(h) / 2.0 - MARGIN as f32) as f64, &open_windows::Sizes::of(crate::toolkit::theme::sheet())) {
        let looks: Vec<Look> = wins.iter().map(|(icon, title, front, hot)| Look {
            program: SITE_APPS.iter().find(|a| a.0 == *icon).map(|a| a.1.to_string()).unwrap_or_default(),
            icons: vec![format!("/usr/share/icons/Trindle-Circle/48x48/apps/{icon}.svg")], name: title.to_string(), front: *front, hot: *hot, uses: Vec::new() }).collect();
        let (r, l) = (rg.clone(), looks.clone());
        let dot = crate::toolkit::theme::sheet().ring_windows_dot;
        layer.push(drawing((w, h), "Open windows", move |pm, pen| open_windows::paint(pm, pen, &r, &l, (w / 2.0, h / 2.0), dot)));
        for (i, look) in looks.iter().enumerate() {
            let b = rg.button_of(i);
            let node: Node<Msg> = open_windows::tab_node(&rg, i, look).key(format!("win-{i}"));
            let at = (cx + b[0] as f32, cy + b[1] as f32);
            layer.push(placed(node, move |_, _| at));
        }
    }
}

fn site_tool_rows() -> (Vec<crate::toolkit::widget::Node<Msg>>, Vec<crate::toolkit::widget::Node<Msg>>, Vec<crate::toolkit::widget::Node<Msg>>) {
    use super::*;
    use crate::toolkit::widget::Node;
    let tool = |band: &str, n: usize, icon: &str, tip: &str, job: Job| button(icon, tip).size(core()).tinted(job.share()).on_press(msg(|_| {})).key(format!("tool-{band}-{n}"));
    let upper: Vec<Node<Msg>> = tool_rows()[0].iter().map(|(_, i, t)| (*i, *t)).chain([("software-update-available-symbolic", "Updates")])
        .enumerate().map(|(n, (i, t))| tool("upper", n, i, t, Job::Programs)).collect();
    let shown = [("audio-volume-high-symbolic", "Sound"), ("audio-input-microphone-symbolic", "Microphone"), ("bluetooth-active-symbolic", "Bluetooth"),
                 ("network-wireless-signal-good-symbolic", "Wi-Fi"), ("display-brightness-symbolic", "Screen brightness")];
    let status: Vec<Node<Msg>> = std::iter::once(&("system-shutdown-symbolic", "Power")).chain(shown.iter())
        .enumerate().map(|(n, (i, t))| tool("status", n, i, t, Job::Computer)).collect();
    let lower: Vec<Node<Msg>> = tool_rows()[1].iter().map(|(_, i, t)| (*i, *t)).chain([("input-keyboard-symbolic", "Keyboard")])
        .enumerate().map(|(n, (i, t))| tool("lower", n, i, t, Job::Tools)).collect();
    (upper, status, lower)
}

fn push_site_favourites(layer: &mut Vec<crate::toolkit::widget::Node<Msg>>, lay: &Layout<usize>) {
    use super::*;
    use crate::toolkit::widget::{drawing, placed, Node};
    for p in lay.placed.iter() {
        let (bw, bh) = button_size(lay.icon, lay.names);
        let at = [p.x as f32 - bw as f32 / 2.0, p.y as f32 - bh as f32 / 2.0, bw as f32, bh as f32];
        let icons = vec![format!("/usr/share/icons/Trindle-Circle/48x48/apps/{}.svg", SITE_APPS[p.item].0)];
        let (icon, shows, name) = (lay.icon, lay.names, SITE_APPS[p.item].1);
        let node: Node<Msg> = drawing((at[2], at[3]), name, move |pm, pen| paint_app(pm, pen, &icons, name, icon, shows, false, false, Ink::Usual)).key(format!("fav-{}", p.item));
        layer.push(placed(node, move |_, _| (at[0], at[1])));
    }
}

fn site_band_icon(half: f32, icon: &'static str, words: Option<&'static str>) -> crate::toolkit::widget::Node<Msg> {

    use crate::toolkit::widget::drawing;
    drawing((2.0 * half, SEGMENT as f32), "", move |canvas, pen| {
        let s = pen.scale;
        let size = (ALL_ICON * s).round();
        let said = words.map(|t| pen.fonts.layout(t, &Style::new(BATTERY_TEXT * s), None));
        let gap = (4.0 * s).round();
        let wide = size + said.as_ref().map_or(0.0, |l| gap + l.width);
        let x = ((canvas.width() as f32 - wide) / 2.0).round();
        let y = ((canvas.height() as f32 - size) / 2.0).round();
        pen.icons.draw(canvas, icon, x as i32, y as i32, size as u32, 1, pen.theme.fg);
        if let Some(l) = said {
            pen.fonts.draw(canvas, &l, x + size + gap, (canvas.height() as f32 - l.height) / 2.0, pen.theme.fg);
        }
    })
}

fn site_ring((w, h): (f32, f32), scale: f32, all: bool, wins: &[(&'static str, &'static str, bool, bool)], wallpaper: &str) -> Pixmap {
    use super::*;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::{drawing, layers, placed, Node};
    let favs: Vec<usize> = (0..9).collect();
    let groups: Vec<(String, Vec<usize>)> = if all {
        crate::common::apps::CATEGORIES.iter().filter_map(|c| {
            let these: Vec<usize> = (9..SITE_APPS.len()).filter(|&i| SITE_APPS[i].2 == *c).collect();
            (!these.is_empty()).then(|| (c.to_string(), these))
        }).collect()
    } else { Vec::new() };
    let lay = Rc::new(layout::<usize>(&favs, &groups, w as f64, h as f64));
    let (cx, cy) = (w / 2.0, h / 2.0);
    let back = lay.clone();
    let dim = crate::common::style::palette().dim as f32;
    let mut layer: Vec<Node<Msg>> = vec![drawing((w, h), "", move |pm, pen| paint_ring(pm, pen, &back, &Scene { seg: false, top: Some(false), bare: false, listening: false, pulse: 0.0, empty: false, all, dim }))];
    let wins = open_windows::grouped(wins.to_vec(), |w| w.0.to_string());
    if !all {
        push_site_windows(&mut layer, &wins, &lay, (w, h));
    }
    let (upper, status, lower) = site_tool_rows();
    let clock = time_and_date_drawn("14:32".into(), "Wed 14 Oct".into()).key("clock");
    layer.push(placed(centre_of(upper, status, clock, lower, None), |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0)));
    push_site_favourites(&mut layer, &lay);
    let m = CENTRE / 2.0;
    let half = (m * m - (m - SEGMENT).powi(2)).sqrt() as f32;
    let (top, foot) = (cy - m as f32, cy + m as f32 - SEGMENT as f32);
    layer.push(placed(site_band_icon(half, "battery-level-80-symbolic", Some("82%")).key("top-band"), move |_, _| (cx - half, top)));
    layer.push(placed(site_band_icon(half, "view-app-grid-symbolic", None).key("foot-band"), move |_, _| (cx - half, foot)));
    let tree = layers(layer).width(Length::Fixed(w)).height(Length::Fixed(h));
    let mut ui = Ui::new(tree, look());
    ui.see_through = true;
    let (pw, ph) = ((w * scale).ceil() as u32, (h * scale).ceil() as u32);
    let mut pix = site_wallpaper(wallpaper, pw, ph);
    let mut over = Pixmap::new(pw, ph).unwrap();
    let laid = crate::toolkit::theme::laid_over(&pix.as_mut(), (0, 0, pw, ph), 1.0);
    ui.paint(&mut over.as_mut(), scale);
    drop(laid);
    pix.draw_pixmap(0, 0, over.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    pix
}

#[test]
#[ignore = "writes pictures when SITE_SHOTS names a folder"]
fn site_ring_pictures() {
    let Some(dir) = std::env::var_os("SITE_SHOTS").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&dir).unwrap();
    let pattern = "2026-09-the-harbour-at-sunrise.jpg";
    site_ring((1280.0, 800.0), 1.0, false, &[], pattern).save_png(dir.join("ring-favourites-1280.png")).unwrap();
    site_ring((1280.0, 800.0), 1.0, true, &[], pattern).save_png(dir.join("ring-all-1280.png")).unwrap();
    let wins = [("firefox", "Firefox", false, false), ("thunderbird", "Thunderbird", true, false), ("system-file-manager", "Folders", false, false), ("libreoffice-writer", "Writer", false, false)];
    site_ring((1280.0, 800.0), 1.0, false, &wins, pattern).save_png(dir.join("ring-windows-1280.png")).unwrap();
    site_ring((1280.0, 800.0), 2.0, false, &[], pattern).save_png(dir.join("ring-favourites-2560.png")).unwrap();
}

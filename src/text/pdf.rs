//! A Markdown file's finished look as a PDF.

use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use tiny_skia::Pixmap;

use super::markdown::layout::{Lay, Palette, Row, mono};
use super::markdown::paint::paint_row;
use super::markdown::Block;
use crate::toolkit::text::Fonts;

const PAGE: (u32, u32) = (1240, 1754);
const MARGIN: f32 = 118.0;
const BODY: f32 = 23.0;
const POINTS: (f32, f32) = (595.28, 841.89);

fn pages(rows: &[Row], room: f32) -> Vec<Vec<&Row>> {
    let mut pages = vec![Vec::new()];
    let mut used = 0.0;
    for row in rows {
        let page_empty = pages.last().is_some_and(|p: &Vec<&Row>| p.is_empty());
        if row.pieces.is_empty() && page_empty {
            continue;
        }
        if used + row.height > room && !page_empty {
            pages.push(Vec::new());
            used = 0.0;
            if row.pieces.is_empty() {
                continue;
            }
        }
        used += row.height;
        if let Some(p) = pages.last_mut() {
            p.push(row);
        }
    }
    pages
}

pub fn render(fonts: &Fonts, blocks: &[Block], title: &str) -> Vec<u8> {
    let width = PAGE.0 as f32 - 2.0 * MARGIN;
    let laid = Lay::new(fonts, BODY, mono(&fonts.families())).lay(blocks, width);
    let ink = Palette::print();
    let mut pictures = Vec::new();
    for page in pages(&laid.rows, PAGE.1 as f32 - 2.0 * MARGIN) {
        let Some(mut pixmap) = Pixmap::new(PAGE.0, PAGE.1) else { continue };
        pixmap.fill(ink.page.to_skia());
        let mut y = MARGIN;
        for row in page {
            paint_row(&mut pixmap.as_mut(), fonts, row, MARGIN, y, &ink, None);
            y += row.height;
        }
        let rgb: Vec<u8> = pixmap.data().chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
        pictures.push(rgb);
    }
    write(&pictures, title)
}

fn squeeze(bytes: &[u8]) -> Vec<u8> {
    let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = z.write_all(bytes);
    z.finish().unwrap_or_default()
}

fn pdf_string(s: &str) -> Vec<u8> {
    if s.chars().all(|c| (' '..='~').contains(&c)) {
        let mut out = b"(".to_vec();
        for b in s.bytes() {
            if matches!(b, b'(' | b')' | b'\\') {
                out.push(b'\\');
            }
            out.push(b);
        }
        out.push(b')');
        return out;
    }
    let mut out = b"<FEFF".to_vec();
    for unit in s.encode_utf16() {
        out.extend_from_slice(format!("{unit:04X}").as_bytes());
    }
    out.push(b'>');
    out
}

fn write(pictures: &[Vec<u8>], title: &str) -> Vec<u8> {
    let mut out: Vec<u8> = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets: Vec<usize> = Vec::new();
    let count = pictures.len().max(1);
    let page_obj = |i: usize| 4 + i * 3;
    let object = |out: &mut Vec<u8>, offsets: &mut Vec<usize>, body: &[u8]| {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", offsets.len()).as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    };
    object(&mut out, &mut offsets, b"<< /Type /Catalog /Pages 2 0 R >>");
    let kids: Vec<String> = (0..count).map(|i| format!("{} 0 R", page_obj(i))).collect();
    object(&mut out, &mut offsets, format!("<< /Type /Pages /Kids [{}] /Count {count} >>", kids.join(" ")).as_bytes());
    let mut info = b"<< /Title ".to_vec();
    info.extend_from_slice(&pdf_string(title));
    info.extend_from_slice(b" /Producer (Trindle Text) >>");
    object(&mut out, &mut offsets, &info);
    let blank = vec![255u8; (PAGE.0 * PAGE.1 * 3) as usize];
    for i in 0..count {
        let picture = pictures.get(i).unwrap_or(&blank);
        let n = page_obj(i);
        let page = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /XObject << /Im0 {} 0 R >> >> /Contents {} 0 R >>",
            POINTS.0,
            POINTS.1,
            n + 1,
            n + 2
        );
        object(&mut out, &mut offsets, page.as_bytes());
        let data = squeeze(picture);
        let mut image = format!(
            "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode /Length {} >>\nstream\n",
            PAGE.0,
            PAGE.1,
            data.len()
        )
        .into_bytes();
        image.extend_from_slice(&data);
        image.extend_from_slice(b"\nendstream");
        object(&mut out, &mut offsets, &image);
        let draw = format!("q {} 0 0 {} 0 0 cm /Im0 Do Q", POINTS.0, POINTS.1);
        let mut contents = format!("<< /Length {} >>\nstream\n", draw.len()).into_bytes();
        contents.extend_from_slice(draw.as_bytes());
        contents.extend_from_slice(b"\nendstream");
        object(&mut out, &mut offsets, &contents);
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes());
    for o in &offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R /Info 3 0 R >>\nstartxref\n{xref}\n%%EOF\n", offsets.len() + 1).as_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::markdown;

    #[test]
    fn a_pdf_with_its_pages() {
        let fonts = Fonts::new();
        let mut text = String::from("# Long\n\n");
        for i in 0..120 {
            text.push_str(&format!("Paragraph {i} with a few words to fill the line.\n\n"));
        }
        let pdf = render(&fonts, &markdown::parse(&text), "Long (notes)");
        assert!(pdf.starts_with(b"%PDF-1.4\n"));
        assert!(pdf.ends_with(b"%%EOF\n"));
        let s = String::from_utf8_lossy(&pdf);
        let count: usize = s.split("/Count ").nth(1).and_then(|r| r.split(' ').next()).and_then(|n| n.parse().ok()).unwrap();
        assert!(count >= 2, "a long text runs onto more pages: {count}");
        assert_eq!(s.matches("/Type /Page ").count(), count);
        assert!(s.contains("/Title (Long \\(notes\\))"));
        let xref: usize = s.rsplit("startxref\n").next().and_then(|r| r.lines().next()).and_then(|n| n.parse().ok()).unwrap();
        assert!(pdf[xref..].starts_with(b"xref"));
        let table = String::from_utf8_lossy(&pdf[xref..]);
        for (k, line) in table.lines().skip(3).take_while(|l| l.ends_with(" n ")).enumerate() {
            let at: usize = line[..10].parse().unwrap();
            assert!(pdf[at..].starts_with(format!("{} 0 obj", k + 1).as_bytes()));
        }
        assert_eq!(pdf_string("Café"), b"<FEFF00430061006600E9>".to_vec());
    }
}

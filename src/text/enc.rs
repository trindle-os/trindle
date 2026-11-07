//! Reading a file's bytes as text and writing text back as bytes, in the encoding the file came in.

use encoding_rs::{Encoding, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Form {
    pub enc: &'static Encoding,
    pub bom: bool,
    pub crlf: bool,
}

impl Default for Form {
    fn default() -> Self {
        Form { enc: UTF_8, bom: false, crlf: false }
    }
}

pub const OFFERED: &[&str] = &[
    "UTF-8", "UTF-16LE", "UTF-16BE", "windows-1252", "ISO-8859-2", "ISO-8859-15", "windows-1250",
    "windows-1251", "KOI8-R", "ISO-8859-7", "windows-1256", "Shift_JIS", "EUC-JP", "GBK", "gb18030",
    "Big5", "EUC-KR",
];

pub fn by_name(name: &str) -> Option<&'static Encoding> {
    Encoding::for_label(name.as_bytes())
}

pub fn read(bytes: &[u8]) -> (String, Form) {
    if let Some((enc, skip)) = Encoding::for_bom(bytes) {
        return finish(decode(&bytes[skip..], enc), enc, true);
    }
    if let Some(enc) = utf16_guess(bytes) {
        return finish(decode(bytes, enc), enc, false);
    }
    if let Ok(s) = std::str::from_utf8(bytes) {
        return finish(s.to_string(), UTF_8, false);
    }
    finish(decode(bytes, WINDOWS_1252), WINDOWS_1252, false)
}

pub fn read_as(bytes: &[u8], enc: &'static Encoding) -> (String, Form) {
    let (bom, skip) = match Encoding::for_bom(bytes) {
        Some((found, skip)) if found == enc => (true, skip),
        _ => (false, 0),
    };
    finish(decode(&bytes[skip..], enc), enc, bom)
}

fn decode(bytes: &[u8], enc: &'static Encoding) -> String {
    enc.decode_without_bom_handling(bytes).0.into_owned()
}

fn finish(text: String, enc: &'static Encoding, bom: bool) -> (String, Form) {
    let crlf = text.contains("\r\n");
    let text = if crlf { text.replace("\r\n", "\n") } else { text };
    (text, Form { enc, bom, crlf })
}

fn utf16_guess(bytes: &[u8]) -> Option<&'static Encoding> {
    if bytes.len() < 4 || bytes.len() % 2 != 0 {
        return None;
    }
    let pairs = bytes.len() / 2;
    let even = bytes.iter().step_by(2).filter(|&&b| b == 0).count();
    let odd = bytes.iter().skip(1).step_by(2).filter(|&&b| b == 0).count();
    if odd * 10 >= pairs * 9 && even == 0 {
        Some(UTF_16LE)
    } else if even * 10 >= pairs * 9 && odd == 0 {
        Some(UTF_16BE)
    } else {
        None
    }
}

pub fn write(text: &str, form: &Form) -> Result<Vec<u8>, char> {
    let text = if form.crlf { std::borrow::Cow::Owned(text.replace('\n', "\r\n")) } else { std::borrow::Cow::Borrowed(text) };
    let mut out = Vec::with_capacity(text.len() + 3);
    if form.enc == UTF_16LE || form.enc == UTF_16BE {
        let le = form.enc == UTF_16LE;
        if form.bom {
            out.extend_from_slice(if le { &[0xFF, 0xFE] } else { &[0xFE, 0xFF] });
        }
        for unit in text.encode_utf16() {
            out.extend_from_slice(&if le { unit.to_le_bytes() } else { unit.to_be_bytes() });
        }
        return Ok(out);
    }
    if form.enc == UTF_8 {
        if form.bom {
            out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        }
        out.extend_from_slice(text.as_bytes());
        return Ok(out);
    }
    let mut encoder = form.enc.new_encoder();
    let mut rest: &str = &text;
    let mut buf = vec![0u8; 4096];
    loop {
        let (result, read, written) = encoder.encode_from_utf8_without_replacement(rest, &mut buf, true);
        out.extend_from_slice(&buf[..written]);
        rest = &rest[read..];
        match result {
            encoding_rs::EncoderResult::InputEmpty => return Ok(out),
            encoding_rs::EncoderResult::OutputFull => {}
            encoding_rs::EncoderResult::Unmappable(c) => return Err(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_is_the_default_and_comes_back_byte_for_byte() {
        let bytes = "Café, naïve, 東京\n".as_bytes();
        let (text, form) = read(bytes);
        assert_eq!(text, "Café, naïve, 東京\n");
        assert_eq!(form, Form::default());
        assert_eq!(write(&text, &form).unwrap(), bytes);
    }

    #[test]
    fn a_western_file_stays_western() {
        let bytes = b"Caf\xe9 \xa3 5\r\nline two\r\n";
        let (text, form) = read(bytes);
        assert_eq!(text, "Café £ 5\nline two\n");
        assert_eq!(form.enc, WINDOWS_1252);
        assert!(form.crlf && !form.bom);
        assert_eq!(write(&text, &form).unwrap(), bytes);
    }

    #[test]
    fn utf16_with_and_without_a_mark() {
        let mut bytes = vec![0xFF, 0xFE];
        for u in "Hi £".encode_utf16() {
            bytes.extend_from_slice(&u.to_le_bytes());
        }
        let (text, form) = read(&bytes);
        assert_eq!((text.as_str(), form.enc, form.bom), ("Hi £", UTF_16LE, true));
        assert_eq!(write(&text, &form).unwrap(), bytes);
        let plain: Vec<u8> = "Hello there".encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
        let (text, form) = read(&plain);
        assert_eq!((text.as_str(), form.enc, form.bom), ("Hello there", UTF_16BE, false));
    }

    #[test]
    fn a_utf8_mark_is_kept() {
        let bytes = b"\xEF\xBB\xBFhello";
        let (text, form) = read(bytes);
        assert_eq!(text, "hello");
        assert!(form.bom);
        assert_eq!(write(&text, &form).unwrap(), bytes);
    }

    #[test]
    fn letters_an_encoding_lacks_are_named() {
        let form = Form { enc: by_name("windows-1252").unwrap(), ..Form::default() };
        assert_eq!(write("Price: 5 €, 東", &form), Err('東'));
        let cyrillic = Form { enc: by_name("KOI8-R").unwrap(), ..Form::default() };
        let bytes = write("Привет", &cyrillic).unwrap();
        assert_eq!(read_as(&bytes, cyrillic.enc).0, "Привет");
    }
}

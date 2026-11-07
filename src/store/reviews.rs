//! Read-only ratings and reviews from ODRS (odrs.gnome.org).

use crate::toolkit::t;
use serde_json::{json, Value};

use crate::common::run::{self, capture};

const SERVICE: &str = "https://odrs.gnome.org/1.0/reviews/api";

#[derive(Clone, Debug, Default)]
pub struct Ratings {
    pub stars: [u64; 5],
    pub total: u64,
}

impl Ratings {
    pub fn average(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.stars.iter().enumerate().map(|(i, n)| (i as f64 + 1.0) * *n as f64).sum::<f64>() / self.total as f64
    }
}

#[derive(Clone, Debug)]
pub struct Review {
    pub summary: String,
    pub text: String,
    pub stars: u8,
    pub who: String,
    pub when: String,
}

pub fn stars_text(n: f64) -> String {
    let full = n.round().clamp(0.0, 5.0) as usize;
    format!("{}{}", "\u{2605}".repeat(full), "\u{2606}".repeat(5 - full))
}

pub fn parse_ratings(v: &Value) -> Ratings {
    let mut r = Ratings::default();
    for i in 0..5 {
        r.stars[i] = v[format!("star{}", i + 1)].as_u64().unwrap_or(0);
    }
    r.total = r.stars.iter().sum();
    r
}

pub fn parse_reviews(v: &Value) -> Vec<Review> {
    v.as_array().map(|list| list.iter().filter(|r| r["reported"].as_u64().unwrap_or(0) == 0).map(|r| {
        let when = r["date_created"].as_f64()
            .map(|t| crate::common::clock::Local::at(t as i64).line(crate::toolkit::lang::format::Pattern::Full)).unwrap_or_default();
        Review {
            summary: r["summary"].as_str().unwrap_or("").trim().to_string(),
            text: r["description"].as_str().unwrap_or("").trim().to_string(),
            stars: ((r["rating"].as_u64().unwrap_or(0) + 10) / 20).min(5) as u8,
            who: r["user_display"].as_str().filter(|s| !s.is_empty()).unwrap_or(t!("store-review-someone")).to_string(),
            when,
        }
    }).collect()).unwrap_or_default()
}

pub fn ratings(app_id: &str, done: impl FnOnce(Option<Ratings>) + 'static) {
    let url = format!("{SERVICE}/ratings/{app_id}");
    capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "15",
              "--max-filesize", run::MAX_ANSWER, &url], None, move |ok, out, _| {
        let v: Value = if ok { serde_json::from_str(&out).unwrap_or(Value::Null) } else { Value::Null };
        done((!v.is_null()).then(|| parse_ratings(&v)).filter(|r| r.total > 0));
    });
}

pub fn reviews(app_id: &str, done: impl FnOnce(Vec<Review>) + 'static) {
    let locale = std::env::var("LANG").ok().and_then(|l| l.split('.').next().map(String::from))
        .filter(|l| !l.is_empty() && l != "C").unwrap_or_else(|| "en_GB".into());
    let body = json!({
        "app_id": app_id, "locale": locale, "distro": "Debian", "version": "unknown", "limit": 10,
        "user_hash": "0000000000000000000000000000000000000000",
    }).to_string();
    let url = format!("{SERVICE}/fetch");
    capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "15",
              "--max-filesize", run::MAX_ANSWER, "-H", "Content-Type: application/json", "--data-binary", "@-", &url],
            Some(&body), move |ok, out, _| {
        done(if ok { parse_reviews(&serde_json::from_str(&out).unwrap_or(Value::Null)) } else { Vec::new() });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_ratings_and_reviews() {
        let r = parse_ratings(&json!({"star0": 0, "star1": 1, "star2": 0, "star3": 0, "star4": 0, "star5": 3, "total": 4}));
        assert_eq!(r.total, 4);
        assert!((r.average() - 4.0).abs() < 1e-9);
        assert_eq!(stars_text(4.0), "\u{2605}\u{2605}\u{2605}\u{2605}\u{2606}");
        let list = parse_reviews(&json!([
            {"summary": "Good", "description": "Works", "rating": 80, "user_display": "A", "date_created": 0.0, "reported": 0},
            {"summary": "Spam", "description": "x", "rating": 20, "user_display": "B", "date_created": 0.0, "reported": 3},
        ]));
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].stars, 4);
    }
}

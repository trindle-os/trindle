//! How far through an install Flatpak says it is.

use crate::toolkit::t;
use std::time::{Duration, Instant};

fn percent(line: &str) -> Option<f64> {
    let b = line.as_bytes();
    let mut found = None;
    for (i, c) in b.iter().enumerate() {
        if *c != b'%' {
            continue;
        }
        let mut start = i;
        while start > 0 && b[start - 1].is_ascii_digit() {
            start -= 1;
        }
        if start < i {
            found = line[start..i].parse::<f64>().ok();
        }
    }
    found
}

pub fn how_far(line: &str) -> Option<f64> {
    let line = line.trim();
    let (verb, rest) = line.split_once(' ')?;

    if !verb.ends_with("ing") {
        return None;
    }
    let (step, rest) = rest.split_once('/')?;
    let step: f64 = step.trim().parse().ok()?;
    let total: f64 = rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok()?;
    if step < 1.0 || total < step {
        return None;
    }
    let within = percent(rest).unwrap_or(0.0) / 100.0;
    Some((((step - 1.0) + within) / total).clamp(0.0, 1.0))
}

fn row_size(line: &str) -> Option<f64> {
    let line = line.trim();
    let (n, _) = line.split_once('.')?;
    if n.is_empty() || !n.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let (_, size) = line.rsplit_once('<')?;
    let mut words = size.split_whitespace();
    let figure: f64 = words.next()?.parse().ok()?;
    let unit = match words.next()? {
        "bytes" | "byte" => 1.0,
        "kB" => 1e3,
        "MB" => 1e6,
        "GB" => 1e9,
        _ => return None,
    };
    Some(figure * unit)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Where {

    pub share: f64,

    pub bytes: Option<(f64, f64)>,
}

#[derive(Default)]
pub struct Job {
    sizes: Vec<f64>,
    began: Option<Instant>,
}

impl Job {

    pub fn heard(&mut self, line: &str) -> Option<Where> {
        if let Some(size) = row_size(line) {
            self.sizes.push(size);
            return None;
        }
        let share = how_far(line)?;
        self.began.get_or_insert_with(Instant::now);
        Some(self.weigh(line, share))
    }

    fn weigh(&self, line: &str, even: f64) -> Where {
        let whole: f64 = self.sizes.iter().sum();
        let steps = step_of(line);
        let Some((step, _)) = steps.filter(|(_, t)| *t == self.sizes.len() && whole > 0.0) else {
            return Where { share: even, bytes: None };
        };
        let within = percent(line.split_once('/').map_or("", |(_, r)| r)).unwrap_or(0.0) / 100.0;
        let done: f64 = self.sizes[..step - 1].iter().sum::<f64>() + within * self.sizes[step - 1];
        Where { share: (done / whole).clamp(0.0, 1.0), bytes: Some((done, whole)) }
    }

    pub fn left(&self, at: Where) -> Option<Duration> {
        let (done, whole) = at.bytes?;
        time_left(self.began?.elapsed(), done, whole)
    }
}

fn step_of(line: &str) -> Option<(usize, usize)> {
    let (_, rest) = line.trim().split_once(' ')?;
    let (step, rest) = rest.split_once('/')?;
    let step: usize = step.trim().parse().ok()?;
    let total: usize = rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok()?;
    (step >= 1 && step <= total).then_some((step, total))
}

fn time_left(spent: Duration, done: f64, whole: f64) -> Option<Duration> {
    if spent < Duration::from_secs(10) || done <= 0.0 {
        return None;
    }
    Some(Duration::from_secs_f64(spent.as_secs_f64() * (whole - done).max(0.0) / done))
}

pub fn left_in_words(left: Duration) -> String {
    let mins = (left.as_secs_f64() / 60.0).round() as u64;
    match mins {
        0 | 1 => t!("store-left-minute").to_string(),
        m if m < 90 => t!("store-left-minutes", n = m),
        m => t!("store-left-hours", n = (m as f64 / 60.0).round() as u64),
    }
}

#[cfg(test)]
mod tests {
    use super::{how_far, row_size, time_left, Job};
    use std::time::Duration;

    fn near(got: Option<f64>, want: f64) -> bool {
        got.is_some_and(|g| (g - want).abs() < 0.001)
    }

    #[test]
    fn a_step_counts_towards_the_whole_job() {
        assert!(near(how_far("Installing 1/2\u{2026}"), 0.0));
        assert!(near(how_far("Installing 1/2\u{2026} \u{2588}\u{2588}  50%"), 0.25));
        assert!(near(how_far("Installing 2/2\u{2026} \u{2588}\u{2588}  44%"), 0.72));
        assert!(near(how_far("Installing 2/2\u{2026} \u{2588}\u{2588} 100%  9.8 MB/s"), 1.0));
        assert!(near(how_far("Uninstalling 1/1\u{2026} 30%"), 0.3));
    }

    #[test]
    fn everything_else_is_not_progress() {
        assert_eq!(how_far("Installation complete."), None);
        assert_eq!(how_far("Looking for matches\u{2026}"), None);
        assert_eq!(how_far("Installing app/org.mozilla.firefox/x86_64/stable"), None);
        assert_eq!(how_far("error: Flatpak refused"), None);
        assert_eq!(how_far(""), None);

        assert_eq!(how_far("Downloading 9.8 MB/s"), None);
    }

    #[test]
    fn the_table_gives_each_step_its_size() {
        assert_eq!(row_size(" 1.\t   \torg.gnome.Platform\t51\ti\tflathub\t< 432.2 MB"), Some(432.2e6));
        assert_eq!(row_size(" 5.\t\torg.gnome.Platform.Locale\t51\ti\tflathub\t< 391.3 MB (partial)"), Some(391.3e6));
        assert_eq!(row_size(" 2.\t\torg.gnome.Calculator\tstable\ti\tflathub\t< 713.4 kB"), Some(713.4e3));
        assert_eq!(row_size("Installing 1/2\u{2026}"), None);
        assert_eq!(row_size(" \tID\tBranch\tOp\tRemote\tDownload"), None);
    }

    #[test]
    fn a_big_step_counts_for_more() {
        let mut j = Job::default();
        assert!(j.heard(" 1.\t\torg.gnome.Platform\t51\ti\tflathub\t< 900 MB").is_none());
        assert!(j.heard(" 2.\t\torg.gnome.Calculator\tstable\ti\tflathub\t< 100 MB").is_none());
        let w = j.heard("Installing 1/2\u{2026} \u{2588}  50%  700 kB/s  01:52").unwrap();
        assert!((w.share - 0.45).abs() < 0.001);
        assert_eq!(w.bytes, Some((450e6, 1000e6)));
        let w = j.heard("Installing 2/2\u{2026} \u{2588} 100%").unwrap();
        assert!((w.share - 1.0).abs() < 0.001);
    }

    #[test]
    fn without_a_table_the_steps_share_evenly() {
        let mut j = Job::default();
        let w = j.heard("Installing 2/2\u{2026} 44%").unwrap();
        assert!((w.share - 0.72).abs() < 0.001);
        assert_eq!(w.bytes, None);
    }

    #[test]
    fn time_left_goes_by_the_speed_so_far() {
        assert_eq!(time_left(Duration::from_secs(5), 10.0, 100.0), None);
        assert_eq!(time_left(Duration::from_secs(60), 25.0, 100.0), Some(Duration::from_secs(180)));
    }
}

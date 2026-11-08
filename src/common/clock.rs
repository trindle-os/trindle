//! The time of day in this computer's own time zone, read from the C
//! library, which follows /etc/localtime and TZ.

use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub struct Local {
    pub unix: i64,
    tm: libc::tm,
}

impl Local {
    pub fn now() -> Local {
        Local::at(SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0))
    }

    pub fn at(unix: i64) -> Local {
        let t = unix as libc::time_t;
        // SAFETY: tm is plain data, so all zeroes is a valid value to fill in.
        let mut tm: libc::tm = unsafe { std::mem::zeroed() };
        // SAFETY: both pointers are to live locals for the length of the call;

        unsafe { libc::localtime_r(&t, &mut tm) };
        Local { unix, tm }
    }

    pub fn utc(unix: i64) -> Local {
        let t = unix as libc::time_t;
        // SAFETY: as in `at`.
        let mut tm: libc::tm = unsafe { std::mem::zeroed() };
        // SAFETY: as in `at`.
        unsafe { libc::gmtime_r(&t, &mut tm) };
        Local { unix, tm }
    }

    pub fn day_number(&self) -> i64 {
        (self.unix + self.utc_offset()).div_euclid(86_400)
    }

    pub fn year(&self) -> i32 {
        self.tm.tm_year + 1900
    }

    pub fn month(&self) -> i32 {
        self.tm.tm_mon + 1
    }

    pub fn day(&self) -> i32 {
        self.tm.tm_mday
    }

    pub fn hour(&self) -> i32 {
        self.tm.tm_hour
    }

    pub fn minute(&self) -> i32 {
        self.tm.tm_min
    }

    pub fn second(&self) -> i32 {
        self.tm.tm_sec
    }

    pub fn utc_offset(&self) -> i64 {
        self.tm.tm_gmtoff
    }

    pub fn moment(&self) -> trindle_toolkit::lang::format::Moment {
        trindle_toolkit::lang::format::Moment {
            year: self.year(),
            month: self.month().clamp(1, 12) as u32,
            day: self.day().max(0) as u32,
            weekday: self.tm.tm_wday.rem_euclid(7) as u32,
            hour: self.hour().max(0) as u32,
            minute: self.minute().max(0) as u32,
            second: self.second().max(0) as u32,
        }
    }

    pub fn words(&self, pattern: &str) -> String {
        trindle_toolkit::lang::format::format(&self.moment(), pattern)
    }

    pub fn line(&self, kind: trindle_toolkit::lang::format::Pattern) -> String {
        trindle_toolkit::lang::format::date(&self.moment(), kind)
    }

    pub fn format(&self, pattern: &str) -> String {
        let Ok(pattern) = CString::new(pattern) else { return String::new() };
        let mut out = vec![0u8; 256];
        // SAFETY: out is writable for its length, pattern is a C string and

        let n = unsafe { libc::strftime(out.as_mut_ptr().cast(), out.len(), pattern.as_ptr(), &self.tm) };
        out.truncate(n);
        String::from_utf8_lossy(&out).into_owned()
    }
}

pub fn utc_iso(unix: i64) -> String {
    let days = unix.div_euclid(86_400);
    let secs = unix.rem_euclid(86_400);
    let (y, m, d) = civil(days);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", secs / 3600, secs / 60 % 60, secs % 60)
}

pub fn moment_at(unix: i64, offset: i64) -> trindle_toolkit::lang::format::Moment {
    let local = unix + offset;
    let days = local.div_euclid(86_400);
    let secs = local.rem_euclid(86_400);
    let (year, month, day) = civil(days);
    trindle_toolkit::lang::format::Moment {
        year: year as i32,
        month: month as u32,
        day: day as u32,

        weekday: (days + 4).rem_euclid(7) as u32,
        hour: (secs / 3600) as u32,
        minute: (secs / 60 % 60) as u32,
        second: (secs % 60) as u32,
    }
}

pub fn moment_of_date(year: i64, month: u32, day: u32) -> trindle_toolkit::lang::format::Moment {
    let (y, m) = (if month <= 2 { year - 1 } else { year }, i64::from(month));
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    moment_at((era * 146_097 + doe - 719_468) * 86_400, 0)
}

pub fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_writes_a_known_moment() {
        assert_eq!(utc_iso(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc_iso(1_790_924_589), "2026-10-02T07:03:09Z");
        assert_eq!(utc_iso(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(Local::utc(1_790_924_589).format("%Y-%m-%d %H.%M.%S %a"), "2026-10-02 07.03.09 Fri");
        let t = Local::at(1_790_924_589);
        assert_eq!(t.format("%Y"), "2026");
        assert_eq!(t.second(), 9);
        assert_eq!(Local::at(0).format(""), "");
        let m = moment_at(1_790_924_589, 3600);
        assert_eq!((m.year, m.month, m.day, m.weekday, m.hour, m.minute, m.second), (2026, 10, 2, 5, 8, 3, 9));
        let m = moment_at(1_790_924_589, 20_700 + 4 * 3600);
        assert_eq!((m.day, m.weekday, m.hour, m.minute), (2, 5, 16, 48), "a Nepal-like zone");
        let m = moment_of_date(2026, 10, 1);
        assert_eq!((m.year, m.month, m.day, m.weekday, m.hour), (2026, 10, 1, 4, 0));
        let m = moment_of_date(2000, 2, 29);
        assert_eq!((m.day, m.weekday), (29, 2));
    }
}

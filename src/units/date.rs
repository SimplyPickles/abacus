use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;

use crate::error::AbacusError;
use crate::units::dimensions::Dimensions;
use crate::units::unit::Unit;
use crate::units::value::Value;

/// Enum representing days of the week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayOfWeek {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl DayOfWeek {
    pub fn from_iso_number(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::Monday),
            2 => Some(Self::Tuesday),
            3 => Some(Self::Wednesday),
            4 => Some(Self::Thursday),
            5 => Some(Self::Friday),
            6 => Some(Self::Saturday),
            7 => Some(Self::Sunday),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }
}

impl fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Structure representing a TimeZone with offset in minutes relative to UTC.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeZone {
    pub name: String,
    pub offset_minutes: i32,
}

impl TimeZone {
    pub fn new(name: impl Into<String>, offset_minutes: i32) -> Self {
        Self {
            name: name.into(),
            offset_minutes,
        }
    }

    pub fn utc() -> Self {
        Self::new("UTC", 0)
    }

    pub fn parse(s: &str) -> Result<Self, AbacusError> {
        let s = s.trim();
        let upper = s.to_ascii_uppercase();

        let offset = match upper.as_str() {
            "UTC" | "GMT" | "Z" => Some(0),
            "EST" => Some(-300),
            "EDT" => Some(-240),
            "CST" => Some(-360),
            "CDT" => Some(-300),
            "MST" => Some(-420),
            "MDT" => Some(-360),
            "PST" => Some(-480),
            "PDT" => Some(-420),
            "AKST" => Some(-540),
            "AKDT" => Some(-480),
            "HST" => Some(-600),
            "CET" | "BST" => Some(60),
            "CEST" | "EET" => Some(120),
            "EEST" | "MSK" => Some(180),
            "IST" => Some(330),         // +05:30
            "JST" | "KST" => Some(540), // +09:00
            "AEST" => Some(600),        // +10:00
            "NZST" => Some(720),        // +12:00
            _ => None,
        };

        if let Some(off) = offset {
            return Ok(Self::new(upper, off));
        }

        let clean = upper.strip_prefix("UTC").unwrap_or(&upper);
        if clean.starts_with('+') || clean.starts_with('-') {
            let sign = if clean.starts_with('-') { -1 } else { 1 };
            let body = &clean[1..];
            let parts: Vec<&str> = body.split(':').collect();
            let (hours, mins) = if parts.len() == 1 {
                let h = parts[0].parse::<i32>().map_err(|_| {
                    AbacusError::InvalidDate(format!("invalid timezone offset: '{s}'"))
                })?;
                (h, 0)
            } else if parts.len() == 2 {
                let h = parts[0].parse::<i32>().map_err(|_| {
                    AbacusError::InvalidDate(format!("invalid timezone offset: '{s}'"))
                })?;
                let m = parts[1].parse::<i32>().map_err(|_| {
                    AbacusError::InvalidDate(format!("invalid timezone offset: '{s}'"))
                })?;
                (h, m)
            } else {
                return Err(AbacusError::InvalidDate(format!(
                    "invalid timezone offset format: '{s}'"
                )));
            };

            let total_mins = sign * (hours * 60 + mins);
            return Ok(Self::new(s, total_mins));
        }

        Err(AbacusError::InvalidDate(format!("unknown timezone: '{s}'")))
    }

    pub fn format_offset(&self) -> String {
        let sign = if self.offset_minutes >= 0 { '+' } else { '-' };
        let abs_mins = self.offset_minutes.abs();
        let hours = abs_mins / 60;
        let mins = abs_mins % 60;
        if mins == 0 {
            format!("{sign}{hours:02}:00")
        } else {
            format!("{sign}{hours:02}:{mins:02}")
        }
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Structure representing a time of day with millisecond resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}

impl Time {
    pub fn new(hour: u32, minute: u32, second: u32, millisecond: u32) -> Self {
        Self {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn new_12h(
        hour_12: u32,
        minute: u32,
        second: u32,
        is_pm: bool,
    ) -> Result<Self, AbacusError> {
        if hour_12 < 1 || hour_12 > 12 {
            return Err(AbacusError::InvalidDate(format!(
                "invalid 12-hour value: {hour_12}"
            )));
        }
        let hour_24 = match (hour_12, is_pm) {
            (12, false) => 0,
            (12, true) => 12,
            (h, false) => h,
            (h, true) => h + 12,
        };
        Ok(Self::new(hour_24, minute, second, 0))
    }

    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Self {
        Self::new(hour, minute, second, 0)
    }

    pub fn is_valid(&self) -> bool {
        self.hour < 24 && self.minute < 60 && self.second < 60 && self.millisecond < 1000
    }

    pub fn to_total_milliseconds(&self) -> u64 {
        ((self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64) * 1000)
            + self.millisecond as u64
    }

    pub fn from_total_milliseconds(ms: u64) -> (Self, u64) {
        let ms_per_day = 86_400_000u64;
        let days_overflow = ms / ms_per_day;
        let rem_ms = ms % ms_per_day;

        let hour = (rem_ms / 3_600_000) as u32;
        let rem_ms = rem_ms % 3_600_000;
        let minute = (rem_ms / 60_000) as u32;
        let rem_ms = rem_ms % 60_000;
        let second = (rem_ms / 1000) as u32;
        let millisecond = (rem_ms % 1000) as u32;

        (
            Self {
                hour,
                minute,
                second,
                millisecond,
            },
            days_overflow,
        )
    }

    pub fn format(&self) -> String {
        if self.millisecond == 0 {
            format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        } else {
            format!(
                "{:02}:{:02}:{:02}.{:03}",
                self.hour, self.minute, self.second, self.millisecond
            )
        }
    }

    pub fn format_12h(&self) -> String {
        let (h12, is_pm) = match self.hour {
            0 => (12, false),
            1..=11 => (self.hour, false),
            12 => (12, true),
            _ => (self.hour - 12, true),
        };
        let am_pm = if is_pm { "PM" } else { "AM" };
        if self.millisecond == 0 {
            format!("{:02}:{:02}:{:02} {}", h12, self.minute, self.second, am_pm)
        } else {
            format!(
                "{:02}:{:02}:{:02}.{:03} {}",
                h12, self.minute, self.second, self.millisecond, am_pm
            )
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Helper function to check if a year is a leap year in the Gregorian calendar.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Helper function to return the number of days in a given month of a year.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Check if year, month, day form a valid calendar date.
pub fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    if month < 1 || month > 12 || day < 1 {
        return false;
    }
    day <= days_in_month(year, month)
}

/// Convert (year, month, day) to days since Unix epoch 1970-01-01 (Proleptic Gregorian algorithm).
pub fn date_to_epoch_days(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 {
        year as i64 - 1
    } else {
        year as i64
    };
    let m = if month <= 2 {
        month as i64 + 12
    } else {
        month as i64
    };
    let era = if y >= 0 { y / 400 } else { (y - 399) / 400 };
    let yoe = y - era * 400;
    let doy = (153 * (m - 3) + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Convert days since Unix epoch 1970-01-01 to (year, month, day).
pub fn epoch_days_to_date(epoch_days: i64) -> (i32, u32, u32) {
    let z = epoch_days + 719468;
    let era = if z >= 0 {
        z / 146097
    } else {
        (z - 146096) / 146097
    };
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year as i32, m as u32, d as u32)
}

/// Structure representing a calendar Date with Time and optional TimeZone.
#[derive(Debug, Clone)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub time: Time,
    pub timezone: Option<TimeZone>,
    pub format: DateFormat,
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.time == other.time
            && self.timezone == other.timezone
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day, &self.time, &self.timezone).cmp(&(
            other.year,
            other.month,
            other.day,
            &other.time,
            &other.timezone,
        ))
    }
}

impl std::hash::Hash for Date {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.year.hash(state);
        self.month.hash(state);
        self.day.hash(state);
        self.time.hash(state);
        self.timezone.hash(state);
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

impl Date {
    pub fn now() -> Self {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap_or_default();
        let total_ms = since_epoch.as_millis() as i64;
        Self::from_epoch_milliseconds(total_ms)
    }

    pub fn today() -> Self {
        let now = Self::now();
        Self::new(now.year, now.month, now.day)
    }

    pub fn tomorrow() -> Self {
        Self::today().add_days(1)
    }

    pub fn yesterday() -> Self {
        Self::today().add_days(-1)
    }

    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            time: Time::default(),
            timezone: None,
            format: DateFormat::default(),
        }
    }

    pub fn with_time(year: i32, month: u32, day: u32, time: Time) -> Self {
        Self {
            year,
            month,
            day,
            time,
            timezone: None,
            format: DateFormat::default(),
        }
    }

    pub fn with_timezone(mut self, tz: TimeZone) -> Self {
        self.timezone = Some(tz);
        self
    }

    pub fn with_format(mut self, format: DateFormat) -> Self {
        self.format = format;
        self
    }

    pub fn new_with_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Self {
        Self {
            year,
            month,
            day,
            time: Time::from_hms(hour, minute, second),
            timezone: None,
            format: DateFormat::default(),
        }
    }

    pub fn new_full(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millisecond: u32,
    ) -> Self {
        Self {
            year,
            month,
            day,
            time: Time::new(hour, minute, second, millisecond),
            timezone: None,
            format: DateFormat::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        is_valid_date(self.year, self.month, self.day) && self.time.is_valid()
    }

    pub fn day_of_week(&self) -> DayOfWeek {
        let epoch_days = self.to_epoch_days();
        let dow = (epoch_days + 3).rem_euclid(7) + 1;
        DayOfWeek::from_iso_number(dow as u32).unwrap_or(DayOfWeek::Thursday)
    }

    pub fn day_of_year(&self) -> u32 {
        let start_of_year = date_to_epoch_days(self.year, 1, 1);
        (self.to_epoch_days() - start_of_year + 1) as u32
    }

    pub fn to_epoch_days(&self) -> i64 {
        date_to_epoch_days(self.year, self.month, self.day)
    }

    pub fn from_epoch_days(days: i64) -> Self {
        let (year, month, day) = epoch_days_to_date(days);
        Self::new(year, month, day)
    }

    pub fn to_epoch_milliseconds(&self) -> i64 {
        let local_days_ms = self.to_epoch_days() * 86_400_000;
        let local_time_ms = self.time.to_total_milliseconds() as i64;
        let local_ms = local_days_ms + local_time_ms;

        if let Some(ref tz) = self.timezone {
            local_ms - (tz.offset_minutes as i64 * 60_000)
        } else {
            local_ms
        }
    }

    pub fn from_epoch_milliseconds(total_ms: i64) -> Self {
        let ms_per_day = 86_400_000i64;
        let days = total_ms.div_euclid(ms_per_day);
        let rem_ms = total_ms.rem_euclid(ms_per_day) as u64;

        let (time, _) = Time::from_total_milliseconds(rem_ms);
        let (year, month, day) = epoch_days_to_date(days);

        Self {
            year,
            month,
            day,
            time,
            timezone: None,
            format: DateFormat::default(),
        }
    }

    // TimeZone conversions
    pub fn to_utc(&self) -> Date {
        if let Some(ref tz) = self.timezone {
            if tz.offset_minutes == 0 {
                return self.clone();
            }
            let utc_ms = self.to_epoch_milliseconds();
            let mut d = Self::from_epoch_milliseconds(utc_ms);
            d.timezone = Some(TimeZone::utc());
            d.format = self.format;
            d
        } else {
            self.clone()
        }
    }

    pub fn to_timezone(&self, target_tz: &TimeZone) -> Date {
        let utc_ms = self.to_epoch_milliseconds();
        let target_ms = utc_ms + (target_tz.offset_minutes as i64 * 60_000);
        let mut d = Self::from_epoch_milliseconds(target_ms);
        d.timezone = Some(target_tz.clone());
        d.format = self.format;
        d
    }

    // Arithmetic methods
    pub fn add_milliseconds(&self, ms: i64) -> Self {
        if let Some(ref tz) = self.timezone {
            let utc_ms = self.to_epoch_milliseconds() + ms;
            let target_ms = utc_ms + (tz.offset_minutes as i64 * 60_000);
            let mut d = Self::from_epoch_milliseconds(target_ms);
            d.timezone = Some(tz.clone());
            d.format = self.format;
            d
        } else {
            let mut d = Self::from_epoch_milliseconds(self.to_epoch_milliseconds() + ms);
            d.format = self.format;
            d
        }
    }

    pub fn add_seconds(&self, seconds: i64) -> Self {
        self.add_milliseconds(seconds * 1000)
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        self.add_milliseconds(minutes * 60_000)
    }

    pub fn add_hours(&self, hours: i64) -> Self {
        self.add_milliseconds(hours * 3_600_000)
    }

    pub fn add_days(&self, days: i64) -> Self {
        self.add_milliseconds(days * 86_400_000)
    }

    pub fn sub_days(&self, days: i64) -> Self {
        self.add_days(-days)
    }

    pub fn add_months(&self, months: i32) -> Self {
        let total_months = (self.year as i64) * 12 + (self.month as i64 - 1) + (months as i64);
        let new_year = total_months.div_euclid(12) as i32;
        let new_month = (total_months.rem_euclid(12) + 1) as u32;

        let max_days = days_in_month(new_year, new_month);
        let new_day = self.day.min(max_days);

        Self {
            year: new_year,
            month: new_month,
            day: new_day,
            time: self.time,
            timezone: self.timezone.clone(),
            format: self.format,
        }
    }

    pub fn add_years(&self, years: i32) -> Self {
        self.add_months(years * 12)
    }

    pub fn days_between(&self, other: &Self) -> i64 {
        self.seconds_between(other) / 86_400
    }

    pub fn seconds_between(&self, other: &Self) -> i64 {
        (other.to_epoch_milliseconds() - self.to_epoch_milliseconds()) / 1000
    }

    pub fn milliseconds_between(&self, other: &Self) -> i64 {
        other.to_epoch_milliseconds() - self.to_epoch_milliseconds()
    }

    // Formatting methods
    pub fn format_with_style(&self, style: DateFormat) -> String {
        let tz_suffix = if let Some(ref tz) = self.timezone {
            format!(" {}", tz.name)
        } else {
            String::new()
        };
        let date_part = match style {
            DateFormat::DDMMYYYY => format!("{:02}-{:02}-{:04}", self.day, self.month, self.year),
            DateFormat::YYYYMMDD => format!("{:04}-{:02}-{:02}", self.year, self.month, self.day),
            DateFormat::MMDDYYYY => format!("{:02}-{:02}-{:04}", self.month, self.day, self.year),
        };

        if self.time.hour == 0
            && self.time.minute == 0
            && self.time.second == 0
            && self.time.millisecond == 0
        {
            format!("{date_part}{tz_suffix}")
        } else {
            format!("{date_part} {}{tz_suffix}", self.time.format())
        }
    }

    pub fn format(&self) -> String {
        self.format_with_style(self.format)
    }

    pub fn format_iso(&self) -> String {
        self.format_with_style(DateFormat::YYYYMMDD)
    }
    pub fn is_weekend(&self) -> bool {
        matches!(self.day_of_week(), DayOfWeek::Saturday | DayOfWeek::Sunday)
    }

    pub fn is_business_day(&self) -> bool {
        !self.is_weekend()
    }

    pub fn add_business_days(&self, n: i64) -> Self {
        if n == 0 {
            return self.clone();
        }
        let mut cur = self.clone();
        let step = if n > 0 { 1 } else { -1 };
        let mut remaining = n.abs();

        while remaining > 0 {
            cur = cur.add_days(step);
            if cur.is_business_day() {
                remaining -= 1;
            }
        }
        cur
    }

    pub fn business_days_between(&self, other: &Self) -> i64 {
        let self_days = self.to_epoch_days();
        let other_days = other.to_epoch_days();

        if self_days == other_days {
            return 0;
        }

        let (start, end, sign) = if self_days < other_days {
            (self_days, other_days, 1i64)
        } else {
            (other_days, self_days, -1i64)
        };

        let mut count = 0i64;
        let mut cur = start + 1;
        while cur <= end {
            let d = Date::from_epoch_days(cur);
            if d.is_business_day() {
                count += 1;
            }
            cur += 1;
        }

        count * sign
    }
}

/// Date format style enum for displaying dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DateFormat {
    /// DD-MM-YYYY format (e.g. 07-08-2026) - Default!
    #[default]
    DDMMYYYY,
    /// YYYY-MM-DD format (e.g. 2026-08-07)
    YYYYMMDD,
    /// MM-DD-YYYY format (e.g. 08-07-2026)
    MMDDYYYY,
}

impl Default for Date {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

// String Parsing
impl FromStr for Date {
    type Err = AbacusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let tokens: Vec<&str> = s.split_whitespace().collect();

        if tokens.is_empty() {
            return Err(AbacusError::InvalidDate("empty string".to_string()));
        }

        // Handle ISO T separator: e.g. "2026-08-07T10:00:00Z"
        let tokens: Vec<String> = if tokens.len() == 1 && tokens[0].contains('T') {
            tokens[0]
                .replace('T', " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        } else {
            tokens.iter().map(|s| s.to_string()).collect()
        };

        if tokens.is_empty() {
            return Err(AbacusError::InvalidDate(format!(
                "invalid date string '{s}'"
            )));
        }

        let date_str = &tokens[0];
        let sep = if date_str.contains('-') {
            '-'
        } else if date_str.contains('/') {
            '/'
        } else {
            return Err(AbacusError::InvalidDate(format!(
                "unrecognized date format: '{s}'"
            )));
        };

        let date_parts: Vec<&str> = date_str.split(sep).collect();
        if date_parts.len() != 3 {
            return Err(AbacusError::InvalidDate(format!(
                "invalid date components: '{s}'"
            )));
        }

        let (p1, p2, p3) = (date_parts[0], date_parts[1], date_parts[2]);
        let (year, month, day) = if p1.len() == 4 {
            (
                p1.parse::<i32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
                p2.parse::<u32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
                p3.parse::<u32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
            )
        } else if p3.len() == 4 {
            (
                p3.parse::<i32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
                p2.parse::<u32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
                p1.parse::<u32>()
                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
            )
        } else {
            return Err(AbacusError::InvalidDate(format!(
                "invalid year in date: '{s}'"
            )));
        };

        let mut time = Time::new(0, 0, 0, 0);
        let mut timezone = None;

        if tokens.len() >= 2 {
            let time_token = &tokens[1];
            if time_token.contains(':') {
                let hms: Vec<&str> = time_token.split(':').collect();
                if hms.len() >= 2 {
                    let h = hms[0]
                        .parse::<u32>()
                        .map_err(|_| AbacusError::InvalidDate(s.to_string()))?;
                    let m = hms[1]
                        .parse::<u32>()
                        .map_err(|_| AbacusError::InvalidDate(s.to_string()))?;
                    let (sec, ms) = if hms.len() >= 3 {
                        if hms[2].contains('.') {
                            let sec_ms: Vec<&str> = hms[2].split('.').collect();
                            let s_val = sec_ms[0]
                                .parse::<u32>()
                                .map_err(|_| AbacusError::InvalidDate(s.to_string()))?;
                            let ms_str = sec_ms[1];
                            let ms_val = match ms_str.len() {
                                1 => ms_str.parse::<u32>().unwrap_or(0) * 100,
                                2 => ms_str.parse::<u32>().unwrap_or(0) * 10,
                                3 => ms_str.parse::<u32>().unwrap_or(0),
                                _ => ms_str[..3].parse::<u32>().unwrap_or(0),
                            };
                            (s_val, ms_val)
                        } else {
                            (
                                hms[2]
                                    .parse::<u32>()
                                    .map_err(|_| AbacusError::InvalidDate(s.to_string()))?,
                                0,
                            )
                        }
                    } else {
                        (0, 0)
                    };
                    time = Time::new(h, m, sec, ms);
                }
            }

            let start_idx = if tokens[1].contains(':') { 2 } else { 1 };
            let mut remaining_words = tokens[start_idx..].join(" ");
            let upper_rem = remaining_words.to_ascii_uppercase();

            if upper_rem.starts_with("AM") || upper_rem.starts_with("PM") {
                let is_pm = upper_rem.starts_with("PM");
                if is_pm && time.hour < 12 {
                    time.hour += 12;
                } else if !is_pm && time.hour == 12 {
                    time.hour = 0;
                }
                let rest_idx = if remaining_words.len() >= 2 {
                    let mut split = remaining_words.split_whitespace();
                    split.next();
                    split.collect::<Vec<&str>>().join(" ")
                } else {
                    String::new()
                };
                remaining_words = rest_idx;
            }

            let tz_word = remaining_words.trim();
            if !tz_word.is_empty() {
                if let Ok(tz) = TimeZone::parse(tz_word) {
                    timezone = Some(tz);
                }
            }
        }

        let mut date = Date::with_time(year, month, day, time);
        date.timezone = timezone;

        if !date.is_valid() {
            return Err(AbacusError::InvalidDate(format!(
                "date out of bounds: '{s}'"
            )));
        }

        Ok(date)
    }
}

// Operators for Date arithmetic
impl Sub<&Date> for &Date {
    type Output = Value;

    fn sub(self, rhs: &Date) -> Self::Output {
        let diff_ms = self.to_epoch_milliseconds() - rhs.to_epoch_milliseconds();
        let seconds = diff_ms as f64 / 1000.0;

        let unit = Unit {
            scalar: 1.0,
            offset: 0.0,
            dimensions: Dimensions::TIME,
            display: crate::units::unit::UnitExpr::single("s"),
        };
        Value::new(seconds, Arc::new(unit))
    }
}

impl Sub<Date> for Date {
    type Output = Value;
    fn sub(self, rhs: Date) -> Self::Output {
        &self - &rhs
    }
}

impl Add<&Value> for &Date {
    type Output = Result<Date, AbacusError>;

    fn add(self, rhs: &Value) -> Self::Output {
        if rhs.unit.dimensions != Dimensions::TIME {
            return Err(AbacusError::IncompatibleDimensions);
        }
        let sym = rhs.unit.display.render().to_ascii_lowercase();
        if sym.contains("business") || sym.contains("work") || sym == "bday" || sym == "bdays" {
            let count = (rhs.canonical / 86400.0).round() as i64;
            Ok(self.add_business_days(count))
        } else {
            let ms = (rhs.canonical * 1000.0).round() as i64;
            Ok(self.add_milliseconds(ms))
        }
    }
}

impl Add<Value> for Date {
    type Output = Result<Date, AbacusError>;
    fn add(self, rhs: Value) -> Self::Output {
        &self + &rhs
    }
}

impl Sub<&Value> for &Date {
    type Output = Result<Date, AbacusError>;

    fn sub(self, rhs: &Value) -> Self::Output {
        if rhs.unit.dimensions != Dimensions::TIME {
            return Err(AbacusError::IncompatibleDimensions);
        }
        let sym = rhs.unit.display.render().to_ascii_lowercase();
        if sym.contains("business") || sym.contains("work") || sym == "bday" || sym == "bdays" {
            let count = (rhs.canonical / 86400.0).round() as i64;
            Ok(self.add_business_days(-count))
        } else {
            let ms = (rhs.canonical * 1000.0).round() as i64;
            Ok(self.add_milliseconds(-ms))
        }
    }
}

impl Sub<Value> for Date {
    type Output = Result<Date, AbacusError>;
    fn sub(self, rhs: Value) -> Self::Output {
        &self - &rhs
    }
}

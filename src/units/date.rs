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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub const SUPPORTED_TIMEZONES: &'static [(&'static str, i32)] = &[
        ("UTC", 0),
        ("GMT", 0),
        ("Z", 0),
        ("EST", -300),
        ("EDT", -240),
        ("CST", -360),
        ("CDT", -300),
        ("MST", -420),
        ("MDT", -360),
        ("PST", -480),
        ("PDT", -420),
        ("AKST", -540),
        ("AKDT", -480),
        ("HST", -600),
        ("CET", 60),
        ("BST", 60),
        ("CEST", 120),
        ("EET", 120),
        ("EEST", 180),
        ("MSK", 180),
        ("IST", 330), // +05:30
        ("JST", 540), // +09:00
        ("KST", 540),
        ("AEST", 600), // +10:00
        ("NZST", 720), // +12:00
    ];

    pub fn parse(s: &str) -> Result<Self, AbacusError> {
        let s = s.trim();
        let upper = s.to_ascii_uppercase();

        let offset = Self::SUPPORTED_TIMEZONES
            .iter()
            .find(|(name, _)| *name == upper.as_str())
            .map(|(_, off)| *off);

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        if !(1..=12).contains(&hour_12) {
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

    pub fn parse_time_spec(s: &str, has_at: bool) -> Option<(Time, usize)> {
        if s.is_empty() {
            return None;
        }

        let mut char_indices = s.char_indices().peekable();
        let &(start_idx, first_c) = char_indices.peek()?;
        if !first_c.is_ascii_digit() {
            return None;
        }

        let mut end_num1 = start_idx;
        while let Some(&(idx, c)) = char_indices.peek() {
            if c.is_ascii_digit() {
                end_num1 = idx + 1;
                char_indices.next();
            } else {
                break;
            }
        }

        let hour_str = &s[start_idx..end_num1];
        let mut hour = hour_str.parse::<u32>().ok()?;

        let mut minute = 0u32;
        let mut second = 0u32;
        let mut millisecond = 0u32;
        let mut current_end = end_num1;

        let has_colon = char_indices.peek().is_some_and(|&(_, c)| c == ':');

        if has_colon {
            char_indices.next(); // consume ':'
            let start_num2 = current_end + 1;
            let mut end_num2 = start_num2;

            while let Some(&(idx, c)) = char_indices.peek() {
                if c.is_ascii_digit() {
                    end_num2 = idx + 1;
                    char_indices.next();
                } else {
                    break;
                }
            }

            if end_num2 == start_num2 {
                return None;
            }

            minute = s[start_num2..end_num2].parse::<u32>().ok()?;
            current_end = end_num2;

            if char_indices.peek().is_some_and(|&(_, c)| c == ':') {
                char_indices.next(); // consume ':'
                let start_num3 = current_end + 1;
                let mut end_num3 = start_num3;

                while let Some(&(idx, c)) = char_indices.peek() {
                    if c.is_ascii_digit() {
                        end_num3 = idx + 1;
                        char_indices.next();
                    } else {
                        break;
                    }
                }

                if end_num3 > start_num3 {
                    second = s[start_num3..end_num3].parse::<u32>().ok()?;
                    current_end = end_num3;

                    if char_indices.peek().is_some_and(|&(_, c)| c == '.') {
                        let mut dot_lookahead = char_indices.clone();
                        dot_lookahead.next();
                        if dot_lookahead
                            .peek()
                            .is_some_and(|&(_, c)| c.is_ascii_digit())
                        {
                            char_indices.next(); // consume '.'
                            let start_ms = current_end + 1;
                            let mut end_ms = start_ms;
                            while let Some(&(idx, c)) = char_indices.peek() {
                                if c.is_ascii_digit() {
                                    end_ms = idx + 1;
                                    char_indices.next();
                                } else {
                                    break;
                                }
                            }
                            let ms_str = &s[start_ms..end_ms];
                            millisecond = match ms_str.len() {
                                1 => ms_str.parse::<u32>().unwrap_or(0) * 100,
                                2 => ms_str.parse::<u32>().unwrap_or(0) * 10,
                                3 => ms_str.parse::<u32>().unwrap_or(0),
                                _ => ms_str[..3].parse::<u32>().unwrap_or(0),
                            };
                            current_end = end_ms;
                        }
                    }
                }
            }
        }

        let after_time = &s[current_end..];
        let trimmed = after_time.trim_start();
        let ws_len = after_time.len() - trimmed.len();
        let upper_trimmed = trimmed.to_ascii_uppercase();

        let mut is_am_pm = false;
        let mut is_pm = false;

        if upper_trimmed.starts_with("AM") {
            if trimmed.len() == 2 || !trimmed.as_bytes()[2].is_ascii_alphanumeric() {
                is_am_pm = true;
                is_pm = false;
                current_end += ws_len + 2;
            }
        } else if upper_trimmed.starts_with("PM")
            && (trimmed.len() == 2 || !trimmed.as_bytes()[2].is_ascii_alphanumeric())
        {
            is_am_pm = true;
            is_pm = true;
            current_end += ws_len + 2;
        }

        if is_am_pm {
            if hour == 0 || hour > 12 {
                return None;
            }
            if is_pm && hour < 12 {
                hour += 12;
            } else if !is_pm && hour == 12 {
                hour = 0;
            }
        } else if !has_colon && !has_at {
            return None;
        }

        let time = Self::new(hour, minute, second, millisecond);
        if time.is_valid() {
            Some((time, current_end))
        } else {
            None
        }
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

impl FromStr for Time {
    type Err = AbacusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Time::parse_time_spec(s.trim(), false)
            .map(|(t, _)| t)
            .ok_or_else(|| AbacusError::InvalidDate(format!("invalid time format: '{s}'")))
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
    if !(1..=12).contains(&month) || day < 1 {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn parse_ymd_components(s: &str) -> Option<(i32, u32, u32, usize)> {
        let trimmed = s.trim_start();
        if trimmed.is_empty() || !trimmed.chars().next()?.is_ascii_digit() {
            return None;
        }

        // Find the date portion consisting strictly of digits and separators
        let end_idx = trimmed
            .find(|c: char| !c.is_ascii_digit() && c != '-' && c != '/')
            .unwrap_or(trimmed.len());
        let date_part = &trimmed[..end_idx];

        let sep = if date_part.contains('-') {
            '-'
        } else if date_part.contains('/') {
            '/'
        } else {
            return None;
        };

        let parts: Vec<&str> = date_part.split(sep).collect();
        if parts.len() != 3 {
            return None;
        }

        let (p1, p2, p3) = (parts[0], parts[1], parts[2]);
        if !(p1.len() == 4 || p3.len() == 4) {
            return None;
        }
        if p1.is_empty() || p2.is_empty() || p3.is_empty() {
            return None;
        }
        if !p1.chars().all(|c| c.is_ascii_digit())
            || !p2.chars().all(|c| c.is_ascii_digit())
            || !p3.chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }

        let prefix_ws = s.len() - trimmed.len();
        let consumed = prefix_ws + date_part.len();

        let (year, month, day) = if p1.len() == 4 {
            (
                p1.parse::<i32>().ok()?,
                p2.parse::<u32>().ok()?,
                p3.parse::<u32>().ok()?,
            )
        } else {
            (
                p3.parse::<i32>().ok()?,
                p2.parse::<u32>().ok()?,
                p1.parse::<u32>().ok()?,
            )
        };

        Some((year, month, day, consumed))
    }

    pub fn apply_time_value(&self, rhs: &Value, sign: i64) -> Result<Date, AbacusError> {
        if rhs.unit.dimensions != Dimensions::TIME {
            return Err(AbacusError::IncompatibleDimensions);
        }
        if rhs.unit.is_business_day_unit() {
            let count = (rhs.canonical / 86400.0).round() as i64;
            Ok(self.add_business_days(sign * count))
        } else {
            let ms = (rhs.canonical * 1000.0).round() as i64;
            Ok(self.add_milliseconds(sign * ms))
        }
    }

    /// Retrieves a numerical property value from this Date by property name.
    pub fn get_property(&self, prop: &str) -> Option<f64> {
        match prop {
            "year" => Some(self.year as f64),
            "month" => Some(self.month as f64),
            "day" => Some(self.day as f64),
            "hour" => Some(self.time.hour as f64),
            "minute" => Some(self.time.minute as f64),
            "second" => Some(self.time.second as f64),
            "millisecond" | "ms" => Some(self.time.millisecond as f64),
            "day_of_week" | "weekday" => Some(self.day_of_week() as u32 as f64),
            "day_of_year" => Some(self.day_of_year() as f64),
            "is_weekend" => Some(if self.is_weekend() { 1.0 } else { 0.0 }),
            "is_workday" | "is_business_day" => {
                Some(if self.is_business_day() { 1.0 } else { 0.0 })
            }
            "offset" | "offset_minutes" => {
                Some(self.timezone.as_ref().map_or(0.0, |tz| tz.offset_minutes as f64))
            }
            _ => None,
        }
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        if s.is_empty() {
            return Err(AbacusError::InvalidDate("empty string".to_string()));
        }

        // Handle ISO T separator: e.g. "2026-08-07T10:00:00Z"
        let normalized = if !s.contains(' ') && s.contains('T') {
            s.replace('T', " ")
        } else {
            s.to_string()
        };

        let (year, month, day, consumed) = Date::parse_ymd_components(&normalized)
            .ok_or_else(|| AbacusError::InvalidDate(format!("invalid date format: '{s}'")))?;

        let mut time = Time::new(0, 0, 0, 0);
        let mut timezone = None;

        let rem = normalized[consumed..].trim_start();
        if !rem.is_empty() {
            if let Some((parsed_time, time_len)) = Time::parse_time_spec(rem, false) {
                time = parsed_time;
                let tz_part = rem[time_len..].trim();
                if !tz_part.is_empty()
                    && let Ok(tz) = TimeZone::parse(tz_part)
                {
                    timezone = Some(tz);
                }
            } else {
                let tz_part = rem.trim();
                if let Ok(tz) = TimeZone::parse(tz_part) {
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
        self.apply_time_value(rhs, 1)
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
        self.apply_time_value(rhs, -1)
    }
}

impl Sub<Value> for Date {
    type Output = Result<Date, AbacusError>;
    fn sub(self, rhs: Value) -> Self::Output {
        &self - &rhs
    }
}

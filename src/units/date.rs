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
    let y = if month <= 2 { year as i64 - 1 } else { year as i64 };
    let m = if month <= 2 { month as i64 + 12 } else { month as i64 };
    let era = if y >= 0 { y / 400 } else { (y - 399) / 400 };
    let yoe = y - era * 400; // [0, 399]
    let doy = (153 * (m - 3) + 2) / 5 + day as i64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468 // 719468 is day offset for 1970-01-01
}

/// Convert days since Unix epoch 1970-01-01 to (year, month, day).
pub fn epoch_days_to_date(epoch_days: i64) -> (i32, u32, u32) {
    let z = epoch_days + 719468;
    let era = if z >= 0 { z / 146097 } else { (z - 146096) / 146097 };
    let doe = z - era * 146097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year as i32, m as u32, d as u32)
}

/// Structure representing a calendar Date with Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub time: Time,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            time: Time::default(),
        }
    }

    pub fn with_time(year: i32, month: u32, day: u32, time: Time) -> Self {
        Self {
            year,
            month,
            day,
            time,
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
        }
    }

    pub fn is_valid(&self) -> bool {
        is_valid_date(self.year, self.month, self.day) && self.time.is_valid()
    }

    pub fn day_of_week(&self) -> DayOfWeek {
        let epoch_days = self.to_epoch_days();
        // 1970-01-01 was a Thursday (4th day of week, 1=Mon...7=Sun)
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
        let days_ms = self.to_epoch_days() * 86_400_000;
        let time_ms = self.time.to_total_milliseconds() as i64;
        days_ms + time_ms
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
        }
    }

    // Arithmetic methods
    pub fn add_milliseconds(&self, ms: i64) -> Self {
        Self::from_epoch_milliseconds(self.to_epoch_milliseconds() + ms)
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
        }
    }

    pub fn add_years(&self, years: i32) -> Self {
        self.add_months(years * 12)
    }

    pub fn days_between(&self, other: &Self) -> i64 {
        other.to_epoch_days() - self.to_epoch_days()
    }

    pub fn seconds_between(&self, other: &Self) -> i64 {
        (other.to_epoch_milliseconds() - self.to_epoch_milliseconds()) / 1000
    }

    pub fn milliseconds_between(&self, other: &Self) -> i64 {
        other.to_epoch_milliseconds() - self.to_epoch_milliseconds()
    }

    // Formatting methods
    pub fn format(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            self.year,
            self.month,
            self.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
            self.time.millisecond
        )
    }

    pub fn format_iso(&self) -> String {
        if self.time.hour == 0
            && self.time.minute == 0
            && self.time.second == 0
            && self.time.millisecond == 0
        {
            format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
        } else {
            self.format()
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_iso())
    }
}

// String Parsing
impl FromStr for Date {
    type Err = AbacusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.is_empty() {
            return Err(AbacusError::InvalidDate("empty string".to_string()));
        }

        let (date_str, time_str) = if parts.len() == 1 {
            if parts[0].contains('T') {
                let split: Vec<&str> = parts[0].split('T').collect();
                (split[0], Some(split[1]))
            } else {
                (parts[0], None)
            }
        } else if parts.len() == 2 {
            (parts[0], Some(parts[1]))
        } else {
            return Err(AbacusError::InvalidDate(format!(
                "cannot parse date: '{s}'"
            )));
        };

        let ymd: Vec<&str> = date_str.split('-').collect();
        if ymd.len() != 3 {
            return Err(AbacusError::InvalidDate(format!(
                "invalid date components in '{date_str}'"
            )));
        }

        let year = ymd[0]
            .parse::<i32>()
            .map_err(|_| AbacusError::InvalidDate(format!("invalid year in '{date_str}'")))?;
        let month = ymd[1]
            .parse::<u32>()
            .map_err(|_| AbacusError::InvalidDate(format!("invalid month in '{date_str}'")))?;
        let day = ymd[2]
            .parse::<u32>()
            .map_err(|_| AbacusError::InvalidDate(format!("invalid day in '{date_str}'")))?;

        let time = if let Some(t_str) = time_str {
            let hms_parts: Vec<&str> = t_str.split(':').collect();
            if hms_parts.len() < 2 || hms_parts.len() > 3 {
                return Err(AbacusError::InvalidDate(format!(
                    "invalid time format in '{t_str}'"
                )));
            }
            let hour = hms_parts[0]
                .parse::<u32>()
                .map_err(|_| AbacusError::InvalidDate(format!("invalid hour in '{t_str}'")))?;
            let minute = hms_parts[1]
                .parse::<u32>()
                .map_err(|_| AbacusError::InvalidDate(format!("invalid minute in '{t_str}'")))?;

            let (second, millisecond) = if hms_parts.len() == 3 {
                if hms_parts[2].contains('.') {
                    let sec_ms: Vec<&str> = hms_parts[2].split('.').collect();
                    let sec = sec_ms[0].parse::<u32>().map_err(|_| {
                        AbacusError::InvalidDate(format!("invalid second in '{t_str}'"))
                    })?;
                    let ms_str = sec_ms[1];
                    let ms = match ms_str.len() {
                        1 => ms_str.parse::<u32>().unwrap_or(0) * 100,
                        2 => ms_str.parse::<u32>().unwrap_or(0) * 10,
                        3 => ms_str.parse::<u32>().unwrap_or(0),
                        _ => ms_str[..3].parse::<u32>().unwrap_or(0),
                    };
                    (sec, ms)
                } else {
                    let sec = hms_parts[2].parse::<u32>().map_err(|_| {
                        AbacusError::InvalidDate(format!("invalid second in '{t_str}'"))
                    })?;
                    (sec, 0)
                }
            } else {
                (0, 0)
            };

            Time::new(hour, minute, second, millisecond)
        } else {
            Time::default()
        };

        let date = Date::with_time(year, month, day, time);
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
        let ms = (rhs.canonical * 1000.0).round() as i64;
        Ok(self.add_milliseconds(ms))
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
        let ms = (rhs.canonical * 1000.0).round() as i64;
        Ok(self.add_milliseconds(-ms))
    }
}

impl Sub<Value> for Date {
    type Output = Result<Date, AbacusError>;
    fn sub(self, rhs: Value) -> Self::Output {
        &self - &rhs
    }
}

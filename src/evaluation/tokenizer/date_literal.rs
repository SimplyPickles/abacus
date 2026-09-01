use crate::units::date::DayOfWeek;

/// If `haystack` starts with the literal word `word` (case-insensitively) and is followed
/// by a non-alphanumeric character (or end-of-string), returns the word length; otherwise
/// returns `None`.
#[inline]
pub(crate) fn strip_word_prefix(haystack: &str, word: &str) -> Option<usize> {
    let len = word.len();
    if haystack.len() >= len
        && haystack.as_bytes()[..len].eq_ignore_ascii_case(word.as_bytes())
        && (haystack.len() == len || !haystack.as_bytes()[len].is_ascii_alphanumeric())
    {
        Some(len)
    } else {
        None
    }
}

pub(crate) fn parse_weekday(s: &str) -> Option<(DayOfWeek, usize)> {
    let matches = [
        ("wednesday", DayOfWeek::Wednesday),
        ("thursday", DayOfWeek::Thursday),
        ("saturday", DayOfWeek::Saturday),
        ("tuesday", DayOfWeek::Tuesday),
        ("monday", DayOfWeek::Monday),
        ("friday", DayOfWeek::Friday),
        ("sunday", DayOfWeek::Sunday),
        ("thurs", DayOfWeek::Thursday),
        ("tues", DayOfWeek::Tuesday),
        ("tue", DayOfWeek::Tuesday),
        ("wed", DayOfWeek::Wednesday),
        ("thu", DayOfWeek::Thursday),
        ("fri", DayOfWeek::Friday),
        ("sat", DayOfWeek::Saturday),
        ("sun", DayOfWeek::Sunday),
        ("mon", DayOfWeek::Monday),
    ];

    for (name, dow) in matches {
        if let Some(len) = strip_word_prefix(s, name) {
            return Some((dow, len));
        }
    }
    None
}

pub(crate) enum RelUnit {
    Week,
    Month,
    Year,
}

pub(crate) fn parse_rel_unit(s: &str) -> Option<(RelUnit, usize)> {
    let matches = [
        ("months", RelUnit::Month),
        ("month", RelUnit::Month),
        ("weeks", RelUnit::Week),
        ("week", RelUnit::Week),
        ("years", RelUnit::Year),
        ("year", RelUnit::Year),
    ];

    for (name, u) in matches {
        if let Some(len) = strip_word_prefix(s, name) {
            return Some((u, len));
        }
    }
    None
}

#[derive(Clone, Copy)]
pub(crate) enum RelModifier {
    Last,
    Next,
    This,
}

pub(crate) fn parse_rel_modifier(s: &str) -> Option<(RelModifier, usize)> {
    let matches = [
        ("previous", RelModifier::Last),
        ("last", RelModifier::Last),
        ("past", RelModifier::Last),
        ("next", RelModifier::Next),
        ("this", RelModifier::This),
    ];

    for (name, m) in matches {
        if let Some(len) = strip_word_prefix(s, name) {
            return Some((m, len));
        }
    }
    None
}

pub(crate) fn try_parse_relative_date_keyword(s: &str) -> Option<(crate::Date, usize)> {
    let ref_date = crate::Date::today();
    let today_dow = ref_date.day_of_week() as u32;

    // 1. Check modifier + space + subject
    if let Some((modifier, mod_len)) = parse_rel_modifier(s) {
        let rest = s[mod_len..].trim_start();
        let ws_len = s[mod_len..].len() - rest.len();
        if ws_len > 0 {
            if let Some((target_dow, sub_len)) = parse_weekday(rest) {
                let target_dow_num = target_dow as u32;
                let days_offset: i64 = match modifier {
                    RelModifier::Last => {
                        -i64::from((today_dow as i32 - target_dow_num as i32 + 6).rem_euclid(7) + 1)
                    }
                    RelModifier::Next => {
                        i64::from((target_dow_num as i32 - today_dow as i32 + 6).rem_euclid(7) + 1)
                    }
                    RelModifier::This => i64::from(target_dow_num as i32 - today_dow as i32),
                };
                let date = ref_date.add_days(days_offset);
                return Some((date, mod_len + ws_len + sub_len));
            }

            if let Some((unit, sub_len)) = parse_rel_unit(rest) {
                let date = match (modifier, unit) {
                    (RelModifier::Last, RelUnit::Week) => ref_date.add_days(-7),
                    (RelModifier::Last, RelUnit::Month) => ref_date.add_months(-1),
                    (RelModifier::Last, RelUnit::Year) => ref_date.add_years(-1),
                    (RelModifier::Next, RelUnit::Week) => ref_date.add_days(7),
                    (RelModifier::Next, RelUnit::Month) => ref_date.add_months(1),
                    (RelModifier::Next, RelUnit::Year) => ref_date.add_years(1),
                    (RelModifier::This, _) => ref_date,
                };
                return Some((date, mod_len + ws_len + sub_len));
            }
        }
    }

    // 2. Standalone relative keywords
    if let Some(len) = strip_word_prefix(s, "today") {
        return Some((crate::Date::today(), len));
    }
    if let Some(len) = strip_word_prefix(s, "tdy") {
        return Some((crate::Date::today(), len));
    }
    if let Some(len) = strip_word_prefix(s, "tomorrow") {
        return Some((crate::Date::tomorrow(), len));
    }
    if let Some(len) = strip_word_prefix(s, "tmr") {
        return Some((crate::Date::tomorrow(), len));
    }
    if let Some(len) = strip_word_prefix(s, "yesterday") {
        return Some((crate::Date::yesterday(), len));
    }
    if let Some(len) = strip_word_prefix(s, "now") {
        return Some((crate::Date::now(), len));
    }

    // 3. Bare weekday
    if let Some((target_dow, sub_len)) = parse_weekday(s) {
        let target_dow_num = target_dow as u32;
        let days_offset = if target_dow_num == today_dow {
            0
        } else {
            let add = (target_dow_num as i32 - today_dow as i32).rem_euclid(7);
            i64::from(if add == 0 { 7 } else { add })
        };
        let date = ref_date.add_days(days_offset);
        return Some((date, sub_len));
    }

    None
}

pub(crate) fn try_parse_date_literal(remaining: &str) -> Option<(crate::Date, usize)> {
    if let Some(stripped) = remaining.strip_prefix('@') {
        if let Some(end_idx) = stripped.find('@') {
            let inner = &stripped[..end_idx];
            if let Ok(date) = inner.parse::<crate::Date>() {
                return Some((date, end_idx + 2));
            }
        }
        return None;
    }

    if let Some((mut date, kw_len)) = try_parse_relative_date_keyword(remaining) {
        let mut end_idx = kw_len;
        let rest = remaining[end_idx..].trim_start();
        let skipped_ws = remaining[end_idx..].len() - rest.len();

        let (has_at, time_rest) = if rest.len() >= 3
            && rest.as_bytes()[..2].eq_ignore_ascii_case(b"at")
            && (rest.as_bytes()[2] == b' '
                || rest.as_bytes()[2] == b'\t'
                || rest.as_bytes()[2] == b'\n')
        {
            (true, rest[2..].trim_start())
        } else {
            (false, rest)
        };

        let time_skipped = rest.len() - time_rest.len();

        if let Some((time, time_len)) = crate::Time::parse_time_spec(time_rest, has_at) {
            date.time = time;
            end_idx += skipped_ws + time_skipped + time_len;

            // Check optional timezone
            let after_tz = remaining[end_idx..].trim_start();
            let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
            if let Some(next_word) = after_tz.split_whitespace().next()
                && let Ok(tz) = crate::units::date::TimeZone::parse(next_word)
            {
                date.timezone = Some(tz);
                end_idx += after_tz_skipped + next_word.len();
            }
        }
        return Some((date, end_idx));
    }

    // Standalone time literal: e.g. "3pm", "3 PM", "15:00"
    if let Some((time, time_len)) = crate::Time::parse_time_spec(remaining, false) {
        let mut date = crate::Date::today();
        date.time = time;
        let mut end_idx = time_len;

        let after_tz = remaining[end_idx..].trim_start();
        let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
        if let Some(next_word) = after_tz.split_whitespace().next()
            && let Ok(tz) = crate::units::date::TimeZone::parse(next_word)
        {
            date.timezone = Some(tz);
            end_idx += after_tz_skipped + next_word.len();
        }
        return Some((date, end_idx));
    }

    // Check numeric date literals (e.g. YYYY-MM-DD, DD-MM-YYYY)
    if let Some((_y, _m, _d, date_len)) = crate::Date::parse_ymd_components(remaining) {
        let mut end_idx = date_len;
        if end_idx < remaining.len()
            && (remaining.as_bytes()[end_idx] == b' ' || remaining.as_bytes()[end_idx] == b'T')
        {
            let rest = remaining[end_idx + 1..].trim_start();
            let skipped = remaining[end_idx..].len() - rest.len();
            if let Some((_time, time_len)) = crate::Time::parse_time_spec(rest, false) {
                end_idx += skipped + time_len;
                let after_tz = remaining[end_idx..].trim_start();
                let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
                if let Some(next_word) = after_tz.split_whitespace().next()
                    && crate::units::date::TimeZone::parse(next_word).is_ok()
                {
                    end_idx += after_tz_skipped + next_word.len();
                }
            }
        }

        let candidate = &remaining[..end_idx];
        if let Ok(date) = candidate.parse::<crate::Date>() {
            return Some((date, end_idx));
        }
    }

    None
}

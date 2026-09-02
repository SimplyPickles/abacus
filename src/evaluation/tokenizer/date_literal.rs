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

pub(crate) fn parse_ordinal(s: &str) -> Option<(i8, usize)> {
    let matches: &[(&str, i8)] = &[
        ("first", 1),
        ("1st", 1),
        ("second", 2),
        ("2nd", 2),
        ("third", 3),
        ("3rd", 3),
        ("fourth", 4),
        ("4th", 4),
        ("fifth", 5),
        ("5th", 5),
        ("last", -1),
    ];
    for &(word, val) in matches {
        if let Some(len) = strip_word_prefix(s, word) {
            return Some((val, len));
        }
    }
    None
}

pub(crate) fn parse_month(s: &str) -> Option<(u32, usize)> {
    let matches: &[(&str, u32)] = &[
        ("january", 1),
        ("jan", 1),
        ("february", 2),
        ("feb", 2),
        ("march", 3),
        ("mar", 3),
        ("april", 4),
        ("apr", 4),
        ("may", 5),
        ("june", 6),
        ("jun", 6),
        ("july", 7),
        ("jul", 7),
        ("august", 8),
        ("aug", 8),
        ("september", 9),
        ("sept", 9),
        ("sep", 9),
        ("october", 10),
        ("oct", 10),
        ("november", 11),
        ("nov", 11),
        ("december", 12),
        ("dec", 12),
    ];
    for &(word, m) in matches {
        if let Some(len) = strip_word_prefix(s, word) {
            return Some((m, len));
        }
    }
    None
}

pub(crate) fn parse_optional_year(s: &str) -> Option<(i32, usize)> {
    let s_trim = s.trim_start();
    let ws_len = s.len() - s_trim.len();
    if ws_len == 0 {
        return None;
    }
    let mut end = 0;
    for c in s_trim.chars() {
        if c.is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }
    if (2..=4).contains(&end) {
        let year_str = &s_trim[..end];
        let next_char = s_trim[end..].chars().next();
        if next_char.is_none_or(|nc| !nc.is_alphanumeric() && nc != '_')
            && let Ok(y) = year_str.parse::<i32>()
        {
            return Some((y, ws_len + end));
        }
    }
    None
}

pub(crate) fn try_parse_nth_weekday_of_month(
    s: &str,
    ref_date: &crate::Date,
) -> Option<(crate::Date, usize)> {
    let (n, ord_len) = parse_ordinal(s)?;
    let rest1 = s[ord_len..].trim_start();
    let ws1 = s[ord_len..].len() - rest1.len();
    if ws1 == 0 {
        return None;
    }
    let (dow, dow_len) = parse_weekday(rest1)?;
    let rest2 = rest1[dow_len..].trim_start();
    let ws2 = rest1[dow_len..].len() - rest2.len();
    if ws2 == 0 {
        return None;
    }
    let (conn_len, rest3) = if let Some(len) = strip_word_prefix(rest2, "of") {
        let r = rest2[len..].trim_start();
        (len + (rest2[len..].len() - r.len()), r)
    } else {
        let len = strip_word_prefix(rest2, "in")?;
        let r = rest2[len..].trim_start();
        (len + (rest2[len..].len() - r.len()), r)
    };
    let (month, month_len) = parse_month(rest3)?;
    let rest4 = &rest3[month_len..];

    let (year, year_len) = if let Some((y, y_len)) = parse_optional_year(rest4) {
        (y, y_len)
    } else {
        let this_year_date =
            crate::units::events::nth_weekday_of_month(ref_date.year, month, dow, n).ok()?;
        let y = if this_year_date < *ref_date {
            ref_date.year + 1
        } else {
            ref_date.year
        };
        (y, 0)
    };

    let date = crate::units::events::nth_weekday_of_month(year, month, dow, n).ok()?;
    let total_len = ord_len + ws1 + dow_len + ws2 + conn_len + month_len + year_len;
    Some((date, total_len))
}

pub(crate) fn try_parse_period_boundary(
    s: &str,
    ref_date: &crate::Date,
) -> Option<(crate::Date, usize)> {
    if let Some(len) = strip_word_prefix(s, "quarter end") {
        return Some((crate::units::events::end_of_quarter(ref_date), len));
    }

    let is_end = if let Some(len) = strip_word_prefix(s, "end of") {
        Some((true, len))
    } else if let Some(len) = strip_word_prefix(s, "start of") {
        Some((false, len))
    } else {
        strip_word_prefix(s, "beginning of").map(|len| (false, len))
    };

    if let Some((is_end, prefix_len)) = is_end {
        let mut rest = s[prefix_len..].trim_start();
        let ws_after_prefix = s[prefix_len..].len() - rest.len();
        if ws_after_prefix == 0 {
            return None;
        }

        let mut article_len = 0;
        for art in ["the ", "this ", "current "] {
            if rest.to_ascii_lowercase().starts_with(art) {
                article_len = art.len();
                rest = rest[art.len()..].trim_start();
                break;
            }
        }

        if let Some(len) = strip_word_prefix(rest, "next quarter") {
            let total = prefix_len + ws_after_prefix + article_len + len;
            let date = if is_end {
                crate::units::events::end_of_next_quarter(ref_date)
            } else {
                crate::units::events::start_of_next_quarter(ref_date)
            };
            return Some((date, total));
        }

        if let Some(len) = strip_word_prefix(rest, "quarter") {
            let after_q = &rest[len..];
            let (year, year_len) = if let Some((y, y_len)) = parse_optional_year(after_q) {
                (y, y_len)
            } else {
                (ref_date.year, 0)
            };
            let q = crate::units::events::quarter_of(ref_date);
            let date = if is_end {
                crate::units::events::quarter_end_date(q, year)
            } else {
                crate::units::events::quarter_start_date(q, year)
            };
            let total = prefix_len + ws_after_prefix + article_len + len + year_len;
            return Some((date, total));
        }

        for (q_name, q_num) in [("q1", 1), ("q2", 2), ("q3", 3), ("q4", 4)] {
            if let Some(len) = strip_word_prefix(rest, q_name) {
                let after_q = &rest[len..];
                let (year, year_len) = if let Some((y, y_len)) = parse_optional_year(after_q) {
                    (y, y_len)
                } else {
                    (ref_date.year, 0)
                };
                let date = if is_end {
                    crate::units::events::quarter_end_date(q_num, year)
                } else {
                    crate::units::events::quarter_start_date(q_num, year)
                };
                let total = prefix_len + ws_after_prefix + article_len + len + year_len;
                return Some((date, total));
            }
        }

        if let Some(len) = strip_word_prefix(rest, "next month") {
            let total = prefix_len + ws_after_prefix + article_len + len;
            let date = if is_end {
                crate::units::events::end_of_next_month(ref_date)
            } else {
                crate::Date::new(ref_date.year, ref_date.month, 1).add_months(1)
            };
            return Some((date, total));
        }

        if let Some(len) = strip_word_prefix(rest, "month") {
            let total = prefix_len + ws_after_prefix + article_len + len;
            let date = if is_end {
                crate::units::events::end_of_month(ref_date)
            } else {
                crate::units::events::start_of_month(ref_date)
            };
            return Some((date, total));
        }

        if let Some(len) = strip_word_prefix(rest, "next year") {
            let total = prefix_len + ws_after_prefix + article_len + len;
            let date = if is_end {
                crate::Date::new(ref_date.year + 1, 12, 31)
            } else {
                crate::units::events::start_of_next_year(ref_date)
            };
            return Some((date, total));
        }

        if let Some(len) = strip_word_prefix(rest, "year") {
            let total = prefix_len + ws_after_prefix + article_len + len;
            let date = if is_end {
                crate::units::events::end_of_year(ref_date)
            } else {
                crate::units::events::start_of_year(ref_date)
            };
            return Some((date, total));
        }
    }

    None
}

pub(crate) fn try_parse_named_holiday(
    s: &str,
    ref_date: &crate::Date,
) -> Option<(crate::Date, usize)> {
    type HolidayFn = fn(i32) -> crate::Date;
    let holidays: &[(&str, HolidayFn)] = &[
        ("christmas eve", crate::units::events::christmas_eve),
        ("christmas day", crate::units::events::christmas),
        ("christmas", crate::units::events::christmas),
        ("xmas", crate::units::events::christmas),
        ("boxing day", crate::units::events::boxing_day),
        ("new year's eve", crate::units::events::new_years_eve),
        ("new years eve", crate::units::events::new_years_eve),
        ("new year eve", crate::units::events::new_years_eve),
        ("new year's day", crate::units::events::new_year),
        ("new years day", crate::units::events::new_year),
        ("new year's", crate::units::events::new_year),
        ("new years", crate::units::events::new_year),
        ("new year", crate::units::events::new_year),
        ("thanksgiving day", crate::units::events::thanksgiving),
        ("thanksgiving", crate::units::events::thanksgiving),
        ("black friday", crate::units::events::black_friday),
        ("cyber monday", crate::units::events::cyber_monday),
        ("halloween", crate::units::events::halloween),
        ("valentines day", crate::units::events::valentines_day),
        ("valentine's day", crate::units::events::valentines_day),
        ("valentines", crate::units::events::valentines_day),
        ("st patricks day", crate::units::events::st_patricks_day),
        ("st patrick's day", crate::units::events::st_patricks_day),
        ("st paddy's day", crate::units::events::st_patricks_day),
        ("fourth of july", crate::units::events::fourth_of_july),
        ("4th of july", crate::units::events::fourth_of_july),
        ("independence day", crate::units::events::fourth_of_july),
        ("labor day", crate::units::events::labor_day),
        ("labour day", crate::units::events::labor_day),
        ("memorial day", crate::units::events::memorial_day),
        ("martin luther king jr day", crate::units::events::mlk_day),
        ("martin luther king day", crate::units::events::mlk_day),
        ("mlk day", crate::units::events::mlk_day),
        ("presidents day", crate::units::events::presidents_day),
        ("president's day", crate::units::events::presidents_day),
        ("juneteenth", crate::units::events::juneteenth),
        ("veterans day", crate::units::events::veterans_day),
        ("remembrance day", crate::units::events::veterans_day),
        ("easter sunday", crate::units::events::easter),
        ("easter", crate::units::events::easter),
    ];

    for &(name, func) in holidays {
        if let Some(name_len) = strip_word_prefix(s, name) {
            let after_name = &s[name_len..];
            let (year, year_len) = if let Some((y, y_len)) = parse_optional_year(after_name) {
                (y, y_len)
            } else {
                let this_year_date = func(ref_date.year);
                let y = if this_year_date < *ref_date {
                    ref_date.year + 1
                } else {
                    ref_date.year
                };
                (y, 0)
            };
            let date = func(year);
            return Some((date, name_len + year_len));
        }
    }

    None
}

pub(crate) fn try_parse_textual_date(
    s: &str,
    ref_date: &crate::Date,
) -> Option<(crate::Date, usize)> {
    // 1. Month first: e.g. "May 16, 2010", "May 16th, 2010", "May 16 2010", "May 16"
    if let Some((month, m_len)) = parse_month(s) {
        let rest1 = s[m_len..].trim_start();
        let ws1 = s[m_len..].len() - rest1.len();
        if ws1 > 0 {
            let (day, d_len) = if let Some((ord, ord_len)) = parse_ordinal(rest1) {
                (ord as u32, ord_len)
            } else {
                let mut d_end = 0;
                for c in rest1.chars() {
                    if c.is_ascii_digit() {
                        d_end += 1;
                    } else {
                        break;
                    }
                }
                if (1..=2).contains(&d_end) {
                    let d = rest1[..d_end].parse::<u32>().ok()?;
                    (d, d_end)
                } else {
                    return None;
                }
            };

            if (1..=31).contains(&day) {
                let rest2 = &rest1[d_len..];
                let rest_after_comma = if let Some(stripped) = rest2.strip_prefix(',') {
                    stripped
                } else {
                    rest2
                };

                let (year, yr_len) = if let Some((y, y_len)) = parse_optional_year(rest_after_comma)
                {
                    (y, (rest2.len() - rest_after_comma.len()) + y_len)
                } else {
                    (ref_date.year, 0)
                };

                if crate::units::date::is_valid_date(year, month, day) {
                    let total_len = m_len + ws1 + d_len + yr_len;
                    return Some((crate::Date::new(year, month, day), total_len));
                }
            }
        }
    }

    // 2. Day first: e.g. "16 May 2010", "16th May 2010", "16th of May 2010"
    let (day, d_len) = if let Some((ord, ord_len)) = parse_ordinal(s) {
        (ord as u32, ord_len)
    } else {
        let mut d_end = 0;
        for c in s.chars() {
            if c.is_ascii_digit() {
                d_end += 1;
            } else {
                break;
            }
        }
        if (1..=2).contains(&d_end) {
            let d = s[..d_end].parse::<u32>().ok()?;
            (d, d_end)
        } else {
            return None;
        }
    };

    if (1..=31).contains(&day) {
        let rest1 = s[d_len..].trim_start();
        let ws1 = s[d_len..].len() - rest1.len();
        if ws1 > 0 {
            let (conn_len, rest2) = if let Some(len) = strip_word_prefix(rest1, "of") {
                let r = rest1[len..].trim_start();
                (len + (rest1[len..].len() - r.len()), r)
            } else {
                (0, rest1)
            };

            if let Some((month, m_len)) = parse_month(rest2) {
                let rest3 = &rest2[m_len..];
                let rest_after_comma = if let Some(stripped) = rest3.strip_prefix(',') {
                    stripped
                } else {
                    rest3
                };

                let (year, yr_len) = if let Some((y, y_len)) = parse_optional_year(rest_after_comma)
                {
                    (y, (rest3.len() - rest_after_comma.len()) + y_len)
                } else {
                    (ref_date.year, 0)
                };

                if crate::units::date::is_valid_date(year, month, day) {
                    let total_len = d_len + ws1 + conn_len + m_len + yr_len;
                    return Some((crate::Date::new(year, month, day), total_len));
                }
            }
        }
    }

    None
}

pub(crate) fn try_parse_event_date(
    s: &str,
    ref_date: &crate::Date,
) -> Option<(crate::Date, usize)> {
    if let Some(res) = try_parse_textual_date(s, ref_date) {
        return Some(res);
    }
    if let Some(res) = try_parse_nth_weekday_of_month(s, ref_date) {
        return Some(res);
    }
    if let Some(res) = try_parse_period_boundary(s, ref_date) {
        return Some(res);
    }
    if let Some(res) = try_parse_named_holiday(s, ref_date) {
        return Some(res);
    }
    None
}

#[allow(dead_code)]
pub(crate) fn try_parse_relative_date_keyword(s: &str) -> Option<(crate::Date, usize)> {
    try_parse_relative_date_keyword_with_anchor(s, None)
}

pub(crate) fn try_parse_relative_date_keyword_with_anchor(
    s: &str,
    anchor: Option<&crate::Date>,
) -> Option<(crate::Date, usize)> {
    let today = crate::Date::today();
    let ref_date = anchor.unwrap_or(&today);
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
                    (RelModifier::This, _) => ref_date.clone(),
                };
                return Some((date, mod_len + ws_len + sub_len));
            }
        }
    }

    // 2. Standalone relative keywords
    if let Some(len) = strip_word_prefix(s, "today") {
        return Some((ref_date.clone(), len));
    }
    if let Some(len) = strip_word_prefix(s, "tdy") {
        return Some((ref_date.clone(), len));
    }
    if let Some(len) = strip_word_prefix(s, "tomorrow") {
        return Some((ref_date.add_days(1), len));
    }
    if let Some(len) = strip_word_prefix(s, "tmr") {
        return Some((ref_date.add_days(1), len));
    }
    if let Some(len) = strip_word_prefix(s, "yesterday") {
        return Some((ref_date.add_days(-1), len));
    }
    if let Some(len) = strip_word_prefix(s, "now") {
        return Some((ref_date.clone(), len));
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

#[allow(dead_code)]
pub(crate) fn try_parse_date_literal(remaining: &str) -> Option<(crate::Date, usize)> {
    try_parse_date_literal_with_anchor(remaining, None)
}

pub(crate) fn try_parse_date_literal_with_anchor(
    remaining: &str,
    anchor: Option<&crate::Date>,
) -> Option<(crate::Date, usize)> {
    let today = crate::Date::today();
    let ref_date = anchor.unwrap_or(&today);

    if let Some(stripped) = remaining.strip_prefix('@') {
        if let Some(end_idx) = stripped.find('@') {
            let inner = &stripped[..end_idx];
            if let Ok(date) = inner.parse::<crate::Date>() {
                return Some((date, end_idx + 2));
            }
        }
        return None;
    }

    // Event dates (e.g. "third thursday of november 2026", "end of quarter", "christmas")
    if let Some((mut date, event_len)) = try_parse_event_date(remaining, ref_date) {
        let mut end_idx = event_len;
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

    if let Some((mut date, kw_len)) =
        try_parse_relative_date_keyword_with_anchor(remaining, Some(ref_date))
    {
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

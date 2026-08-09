use crate::{
    AbacusError, UnitRegistry, Value,
    evaluation::tokenizer::{registry::token_registry::TokenRegistry, tokens::Token},
};

const CONVERSION_KEYWORDS: [&str; 3] = ["as", "to", "in"];

fn parse_weekday(s: &str) -> Option<(crate::units::date::DayOfWeek, usize)> {
    let lower = s.to_ascii_lowercase();
    let matches = [
        ("wednesday", crate::units::date::DayOfWeek::Wednesday),
        ("thursday", crate::units::date::DayOfWeek::Thursday),
        ("saturday", crate::units::date::DayOfWeek::Saturday),
        ("tuesday", crate::units::date::DayOfWeek::Tuesday),
        ("monday", crate::units::date::DayOfWeek::Monday),
        ("friday", crate::units::date::DayOfWeek::Friday),
        ("sunday", crate::units::date::DayOfWeek::Sunday),
        ("thurs", crate::units::date::DayOfWeek::Thursday),
        ("tues", crate::units::date::DayOfWeek::Tuesday),
        ("tue", crate::units::date::DayOfWeek::Tuesday),
        ("wed", crate::units::date::DayOfWeek::Wednesday),
        ("thu", crate::units::date::DayOfWeek::Thursday),
        ("fri", crate::units::date::DayOfWeek::Friday),
        ("sat", crate::units::date::DayOfWeek::Saturday),
        ("sun", crate::units::date::DayOfWeek::Sunday),
        ("mon", crate::units::date::DayOfWeek::Monday),
    ];

    for (name, dow) in matches {
        if lower.starts_with(name) {
            let len = name.len();
            if s.len() == len || !s.as_bytes()[len].is_ascii_alphanumeric() {
                return Some((dow, len));
            }
        }
    }
    None
}

enum RelUnit {
    Week,
    Month,
    Year,
}

fn parse_rel_unit(s: &str) -> Option<(RelUnit, usize)> {
    let lower = s.to_ascii_lowercase();
    let matches = [
        ("months", RelUnit::Month),
        ("month", RelUnit::Month),
        ("weeks", RelUnit::Week),
        ("week", RelUnit::Week),
        ("years", RelUnit::Year),
        ("year", RelUnit::Year),
    ];

    for (name, u) in matches {
        if lower.starts_with(name) {
            let len = name.len();
            if s.len() == len || !s.as_bytes()[len].is_ascii_alphanumeric() {
                return Some((u, len));
            }
        }
    }
    None
}

#[derive(Clone, Copy)]
enum RelModifier {
    Last,
    Next,
    This,
}

fn parse_rel_modifier(s: &str) -> Option<(RelModifier, usize)> {
    let lower = s.to_ascii_lowercase();
    let matches = [
        ("previous", RelModifier::Last),
        ("last", RelModifier::Last),
        ("past", RelModifier::Last),
        ("next", RelModifier::Next),
        ("this", RelModifier::This),
    ];

    for (name, m) in matches {
        if lower.starts_with(name) {
            let len = name.len();
            if s.len() == len || !s.as_bytes()[len].is_ascii_alphanumeric() {
                return Some((m, len));
            }
        }
    }
    None
}

fn try_parse_relative_date_keyword(s: &str) -> Option<(crate::Date, usize)> {
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
                        -(((today_dow as i32 - target_dow_num as i32 + 6).rem_euclid(7) + 1) as i64)
                    }
                    RelModifier::Next => {
                        ((target_dow_num as i32 - today_dow as i32 + 6).rem_euclid(7) + 1) as i64
                    }
                    RelModifier::This => (target_dow_num as i32 - today_dow as i32) as i64,
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
    let lower = s.to_ascii_lowercase();
    if lower.starts_with("today") && (s.len() == 5 || !s.as_bytes()[5].is_ascii_alphanumeric()) {
        return Some((crate::Date::today(), 5));
    }
    if lower.starts_with("tdy") && (s.len() == 3 || !s.as_bytes()[3].is_ascii_alphanumeric()) {
        return Some((crate::Date::today(), 3));
    }
    if lower.starts_with("tomorrow") && (s.len() == 8 || !s.as_bytes()[8].is_ascii_alphanumeric()) {
        return Some((crate::Date::tomorrow(), 8));
    }
    if lower.starts_with("tmr") && (s.len() == 3 || !s.as_bytes()[3].is_ascii_alphanumeric()) {
        return Some((crate::Date::tomorrow(), 3));
    }
    if lower.starts_with("yesterday") && (s.len() == 9 || !s.as_bytes()[9].is_ascii_alphanumeric()) {
        return Some((crate::Date::yesterday(), 9));
    }
    if lower.starts_with("now") && (s.len() == 3 || !s.as_bytes()[3].is_ascii_alphanumeric()) {
        return Some((crate::Date::now(), 3));
    }

    // 3. Bare weekday
    if let Some((target_dow, sub_len)) = parse_weekday(s) {
        let target_dow_num = target_dow as u32;
        let days_offset = if target_dow_num == today_dow {
            0
        } else {
            let add = (target_dow_num as i32 - today_dow as i32).rem_euclid(7);
            (if add == 0 { 7 } else { add }) as i64
        };
        let date = ref_date.add_days(days_offset);
        return Some((date, sub_len));
    }

    None
}

fn try_parse_time_spec(s: &str, has_at: bool) -> Option<(crate::Time, usize)> {
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

    let has_colon = char_indices.peek().map_or(false, |&(_, c)| c == ':');

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

        if char_indices.peek().map_or(false, |&(_, c)| c == ':') {
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

                if char_indices.peek().map_or(false, |&(_, c)| c == '.') {
                    let mut dot_lookahead = char_indices.clone();
                    dot_lookahead.next();
                    if dot_lookahead.peek().map_or(false, |&(_, c)| c.is_ascii_digit()) {
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
    } else if upper_trimmed.starts_with("PM") {
        if trimmed.len() == 2 || !trimmed.as_bytes()[2].is_ascii_alphanumeric() {
            is_am_pm = true;
            is_pm = true;
            current_end += ws_len + 2;
        }
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
    } else {
        if !has_colon && !has_at {
            return None;
        }
    }

    let time = crate::Time::new(hour, minute, second, millisecond);
    if time.is_valid() {
        Some((time, current_end))
    } else {
        None
    }
}

fn try_parse_date_literal(remaining: &str) -> Option<(crate::Date, usize)> {
    if remaining.starts_with('@') {
        if let Some(end_idx) = remaining[1..].find('@') {
            let inner = &remaining[1..=end_idx];
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

        let rest_lower = rest.to_ascii_lowercase();
        let (has_at, time_rest) = if rest_lower.starts_with("at ")
            || rest_lower.starts_with("at\t")
            || rest_lower.starts_with("at\n")
        {
            (true, rest[2..].trim_start())
        } else {
            (false, rest)
        };

        let time_skipped = rest.len() - time_rest.len();

        if let Some((time, time_len)) = try_parse_time_spec(time_rest, has_at) {
            date.time = time;
            end_idx += skipped_ws + time_skipped + time_len;

            // Check optional timezone
            let after_tz = remaining[end_idx..].trim_start();
            let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
            if let Some(next_word) = after_tz.split_whitespace().next() {
                if crate::units::date::TimeZone::parse(next_word).is_ok() {
                    let tz = crate::units::date::TimeZone::parse(next_word).unwrap();
                    date.timezone = Some(tz);
                    end_idx += after_tz_skipped + next_word.len();
                }
            }
        }
        return Some((date, end_idx));
    }

    // Standalone time literal: e.g. "3pm", "3 PM", "15:00"
    if let Some((time, time_len)) = try_parse_time_spec(remaining, false) {
        let mut date = crate::Date::today();
        date.time = time;
        let mut end_idx = time_len;

        let after_tz = remaining[end_idx..].trim_start();
        let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
        if let Some(next_word) = after_tz.split_whitespace().next() {
            if crate::units::date::TimeZone::parse(next_word).is_ok() {
                let tz = crate::units::date::TimeZone::parse(next_word).unwrap();
                date.timezone = Some(tz);
                end_idx += after_tz_skipped + next_word.len();
            }
        }
        return Some((date, end_idx));
    }

    // Check numeric date literals (e.g. YYYY-MM-DD, DD-MM-YYYY)
    let chars: Vec<char> = remaining.chars().collect();
    if chars.is_empty() || !chars[0].is_ascii_digit() {
        return None;
    }

    let mut idx = 0;
    while idx < chars.len()
        && (chars[idx].is_ascii_digit() || chars[idx] == '-' || chars[idx] == '/')
    {
        idx += 1;
    }

    let date_part = &remaining[..idx];
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
    let p1_len = p1.len();
    let p3_len = p3.len();

    if !(p1_len == 4 || p3_len == 4) {
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

    let mut end_idx = idx;
    if end_idx < chars.len() && (chars[end_idx] == ' ' || chars[end_idx] == 'T') {
        let rest = remaining[end_idx + 1..].trim_start();
        let skipped = remaining[end_idx..].len() - rest.len();
        if let Some((_time, time_len)) = try_parse_time_spec(rest, false) {
            end_idx += skipped + time_len;
            let after_tz = remaining[end_idx..].trim_start();
            let after_tz_skipped = remaining[end_idx..].len() - after_tz.len();
            if let Some(next_word) = after_tz.split_whitespace().next() {
                if crate::units::date::TimeZone::parse(next_word).is_ok() {
                    end_idx += after_tz_skipped + next_word.len();
                }
            }
        }
    }

    let candidate = &remaining[..end_idx];
    if let Ok(date) = candidate.parse::<crate::Date>() {
        Some((date, end_idx))
    } else {
        None
    }
}

pub fn tokenize_string<'a>(
    token_registry: &TokenRegistry,
    unit_registry: &UnitRegistry,
    input_text: &'a str,
) -> Result<Vec<Token<'a>>, AbacusError> {
    let mut tokens = Vec::new();
    let mut chars = input_text.char_indices().peekable();

    while let Some(&(i, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // Date literals (e.g. today, tomorrow, yesterday, now, @2026-08-07@, 07-08-2026, 12:00, 1:00 PM)
        if c == '@' || c.is_ascii_digit() || c.is_ascii_alphabetic() {
            if let Some((date, consumed_bytes)) = try_parse_date_literal(&input_text[i..]) {
                tokens.push(Token::Date(date));
                let target_idx = i + consumed_bytes;
                while let Some(&(idx, _)) = chars.peek() {
                    if idx < target_idx {
                        chars.next();
                    } else {
                        break;
                    }
                }
                continue;
            }
        }

        // Timezone offset literals (e.g. +02:00, -04:00, +05:30)
        if c == '+' || c == '-' {
            let remaining = &input_text[i..];
            if let Some(word) = remaining.split_whitespace().next() {
                if word.contains(':') && crate::units::date::TimeZone::parse(word).is_ok() {
                    tokens.push(Token::Unit(&input_text[i..i + word.len()]));
                    let target_idx = i + word.len();
                    while let Some(&(idx, _)) = chars.peek() {
                        if idx < target_idx {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        // Relative time operators (e.g. "ago", "from now", "before", "after")
        if c.is_ascii_alphabetic() {
            let remaining = &input_text[i..];
            let lower_rem = remaining.to_ascii_lowercase();

            let (rel_op, len) = if lower_rem.starts_with("from now")
                && (remaining.len() == 8 || !remaining.as_bytes()[8].is_ascii_alphanumeric())
            {
                (Some("from_now"), 8)
            } else if lower_rem.starts_with("from") {
                let rest = remaining[4..].trim_start();
                let ws_len = remaining[4..].len() - rest.len();
                if ws_len > 0
                    && rest.to_ascii_lowercase().starts_with("now")
                    && (rest.len() == 3 || !rest.as_bytes()[3].is_ascii_alphanumeric())
                {
                    (Some("from_now"), 4 + ws_len + 3)
                } else {
                    (None, 0)
                }
            } else if lower_rem.starts_with("ago")
                && (remaining.len() == 3 || !remaining.as_bytes()[3].is_ascii_alphanumeric())
            {
                (Some("ago"), 3)
            } else if lower_rem.starts_with("before")
                && (remaining.len() == 6 || !remaining.as_bytes()[6].is_ascii_alphanumeric())
            {
                (Some("before"), 6)
            } else if lower_rem.starts_with("after")
                && (remaining.len() == 5 || !remaining.as_bytes()[5].is_ascii_alphanumeric())
            {
                (Some("after"), 5)
            } else {
                (None, 0)
            };

            if let Some(op_name) = rel_op {
                tokens.push(Token::RelTimeOp(op_name));
                let target_idx = i + len;
                while let Some(&(idx, _)) = chars.peek() {
                    if idx < target_idx {
                        chars.next();
                    } else {
                        break;
                    }
                }
                continue;
            }
        }

        // Multi-word unit symbols (e.g. "business days", "business day", "work days", "work day", "working days", "working day")
        if c.is_ascii_alphabetic() {
            let remaining = &input_text[i..];
            let words: Vec<&str> = remaining
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .filter(|w| !w.is_empty())
                .collect();
            if words.len() >= 2 {
                let two_words = format!("{} {}", words[0], words[1]);
                if unit_registry.unit(&two_words).is_ok() {
                    let end_pos = remaining.find(words[1]).unwrap_or(0) + words[1].len();
                    let unit_str = &input_text[i..i + end_pos];
                    tokens.push(Token::Unit(unit_str));
                    let target_idx = i + end_pos;
                    while let Some(&(idx, _)) = chars.peek() {
                        if idx < target_idx {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        // Grouping Parentheses and Delimiters
        if c == '(' {
            tokens.push(Token::OpenParen);
            chars.next();
            continue;
        }

        if c == ')' {
            tokens.push(Token::CloseParen);
            chars.next();
            continue;
        }

        if c == ',' {
            tokens.push(Token::Comma);
            chars.next();
            continue;
        }

        if c == '[' {
            tokens.push(Token::OpenBracket);
            chars.next();
            continue;
        }

        if c == ']' {
            tokens.push(Token::CloseBracket);
            chars.next();
            continue;
        }

        if c == '.' {
            let mut dot_lookahead = chars.clone();
            dot_lookahead.next(); // skip '.'
            if let Some(&(_, '.')) = dot_lookahead.peek() {
                tokens.push(Token::Range);
                chars.next();
                chars.next();
                continue;
            } else if let Some(&(_, next_c)) = dot_lookahead.peek() {
                if next_c.is_alphabetic() || next_c == '_' {
                    chars.next(); // consume '.'
                    let start = chars.peek().unwrap().0;
                    let mut end = start;
                    while let Some(&(idx, sym_c)) = chars.peek() {
                        if sym_c.is_alphanumeric() || sym_c == '_' {
                            end = idx + sym_c.len_utf8();
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let prop = input_text[start..end].to_string();
                    tokens.push(Token::DotProperty(prop));
                    continue;
                }
            }
        }

        // Registered operator checks (prioritizing longer length operators like `++` over `+`)
        let remaining = &input_text[i..];
        enum MatchedOp {
            Binary(&'static str),
            Unary(&'static str),
            Func(&'static str),
        }

        let mut best_match: Option<(&str, MatchedOp)> = None;

        for (alias, op) in &token_registry.binary_operators {
            if remaining.starts_with(alias.as_str()) {
                if let Some(last_char) = alias.chars().last() {
                    if last_char.is_alphanumeric() || last_char == '_' {
                        let next_slice = &remaining[alias.len()..];
                        if let Some(next_char) = next_slice.chars().next() {
                            if next_char.is_alphanumeric()
                                || next_char == '_'
                                || next_char == '°'
                                || next_char == 'Å'
                                || next_char == 'Ω'
                            {
                                continue;
                            }
                        }
                    }
                }

                let match_len = alias.len();
                if let Some((best_alias, _)) = best_match {
                    if match_len > best_alias.len() {
                        best_match = Some((alias.as_str(), MatchedOp::Binary(op.alias)));
                    }
                } else {
                    best_match = Some((alias.as_str(), MatchedOp::Binary(op.alias)));
                }
            }
        }

        for (alias, op) in &token_registry.unary_operators {
            if remaining.starts_with(alias.as_str()) {
                if let Some(last_char) = alias.chars().last() {
                    if last_char.is_alphanumeric() || last_char == '_' {
                        let next_slice = &remaining[alias.len()..];
                        if let Some(next_char) = next_slice.chars().next() {
                            if next_char.is_alphanumeric()
                                || next_char == '_'
                                || next_char == '°'
                                || next_char == 'Å'
                                || next_char == 'Ω'
                            {
                                continue;
                            }
                        }
                    }
                }

                let match_len = alias.len();
                if let Some((best_alias, _)) = best_match {
                    if match_len > best_alias.len() {
                        best_match = Some((alias.as_str(), MatchedOp::Unary(op.alias)));
                    }
                } else {
                    best_match = Some((alias.as_str(), MatchedOp::Unary(op.alias)));
                }
            }
        }

        for (name, op) in &token_registry.function_operators {
            if remaining.starts_with(name.as_str()) {
                if unit_registry.contains(name.as_str()) {
                    let next_slice = remaining[name.len()..].trim_start();
                    if !next_slice.starts_with('(') {
                        continue;
                    }
                }
                if let Some(last_char) = name.chars().last() {
                    if last_char.is_alphanumeric() || last_char == '_' || last_char == '-' {
                        let next_slice = &remaining[name.len()..];
                        if let Some(next_char) = next_slice.chars().next() {
                            if next_char.is_alphanumeric()
                                || next_char == '_'
                                || next_char == '-'
                                || next_char == '°'
                                || next_char == 'Å'
                                || next_char == 'Ω'
                            {
                                continue;
                            }
                        }
                    }
                }

                let match_len = name.len();
                if let Some((best_alias, _)) = best_match {
                    if match_len > best_alias.len() {
                        best_match = Some((name.as_str(), MatchedOp::Func(op.name)));
                    }
                } else {
                    best_match = Some((name.as_str(), MatchedOp::Func(op.name)));
                }
            }
        }

        if let Some((alias, matched_op)) = best_match {
            let char_count = alias.chars().count();
            for _ in 0..char_count {
                chars.next();
            }
            match matched_op {
                MatchedOp::Binary(op_alias) => tokens.push(Token::BinaryOp(op_alias)),
                MatchedOp::Unary(op_alias) => tokens.push(Token::UnaryOp(op_alias)),
                MatchedOp::Func(fn_name) => tokens.push(Token::Function(fn_name)),
            }
            continue;
        }

        // Range operator `..` (must be checked before number parsing)
        if c == '.' {
            let mut lookahead = chars.clone();
            lookahead.next(); // consume first '.'
            if let Some(&(_, '.')) = lookahead.peek() {
                // It's `..`
                tokens.push(Token::Range);
                chars.next(); // consume first '.'
                chars.next(); // consume second '.'
                continue;
            }
        }

        // Numbers (digit or starting with '.' followed by a digit)
        if c.is_ascii_digit()
            || (c == '.'
                && chars
                    .clone()
                    .nth(1)
                    .map_or(false, |(_, next_c)| next_c.is_ascii_digit()))
        {
            let start = i;
            let mut has_dot = false;
            let mut has_exp = false;
            while let Some(&(_, num_c)) = chars.peek() {
                if num_c.is_ascii_digit() {
                    chars.next();
                } else if num_c == '.' && !has_dot && !has_exp {
                    // Peek ahead: only consume '.' as decimal point if followed by a digit.
                    // If followed by '.' (range `..`) or letter (property `.intercept`), stop number parsing.
                    let mut dot_lookahead = chars.clone();
                    dot_lookahead.next(); // skip past the '.'
                    match dot_lookahead.peek() {
                        Some(&(_, next_c)) if next_c.is_ascii_digit() => {
                            has_dot = true;
                            chars.next();
                        }
                        _ => break, // stop number here
                    }
                } else if (num_c == 'e' || num_c == 'E') && !has_exp {
                    let mut exp_lookahead = chars.clone();
                    exp_lookahead.next(); // skip 'e' or 'E'
                    if let Some(&(_, sign_c)) = exp_lookahead.peek() {
                        if sign_c == '+' || sign_c == '-' {
                            exp_lookahead.next();
                        }
                    }
                    if let Some(&(_, digit_c)) = exp_lookahead.peek() {
                        if digit_c.is_ascii_digit() {
                            has_exp = true;
                            chars.next(); // consume 'e'/'E'
                            if let Some(&(_, sign_c)) = chars.peek() {
                                if sign_c == '+' || sign_c == '-' {
                                    chars.next(); // consume '+' or '-'
                                }
                            }
                            continue;
                        }
                    }
                    break;
                } else {
                    break;
                }
            }
            let num_str =
                &input_text[start..chars.peek().map_or(input_text.len(), |&(idx, _)| idx)];
            let val = num_str
                .parse::<f64>()
                .map_err(|_| AbacusError::UnknownUnit(num_str.to_string()))?;

            // Check if immediately followed by an unspaced unit identifier (e.g. 5km, 10m, 1s^-1)
            if let Some(&(unit_start, unit_c)) = chars.peek() {
                if unit_c.is_alphabetic() || unit_c == '°' || unit_c == 'Å' || unit_c == 'Ω' {
                    let mut unit_end = unit_start;
                    let mut unit_chars = chars.clone();
                    while let Some((idx, sym_c)) = unit_chars.peek().cloned() {
                        if sym_c.is_alphanumeric()
                            || sym_c == '_'
                            || sym_c == '°'
                            || sym_c == 'Å'
                            || sym_c == 'Ω'
                        {
                            unit_end = idx + sym_c.len_utf8();
                            unit_chars.next();
                        } else if sym_c == '^' {
                            unit_end = idx + sym_c.len_utf8();
                            unit_chars.next();
                            if let Some((sign_idx, sign_c)) = unit_chars.peek().cloned() {
                                if sign_c == '+' || sign_c == '-' {
                                    unit_end = sign_idx + sign_c.len_utf8();
                                    unit_chars.next();
                                }
                            }
                            while let Some((digit_idx, digit_c)) = unit_chars.peek().cloned() {
                                if digit_c.is_ascii_digit() || digit_c == '.' {
                                    unit_end = digit_idx + digit_c.len_utf8();
                                    unit_chars.next();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let unit_candidate = &input_text[unit_start..unit_end];
                    if let Ok(unit) = unit_registry.unit(unit_candidate) {
                        tokens.push(Token::Val(Value::new(val, unit)));
                        chars = unit_chars;
                        continue;
                    }
                }
            }

            tokens.push(Token::Float(val));
            continue;
        }

        // Identifiers (units, conversion operators, named unary ops like sqrt)
        if c.is_alphabetic() || c == '_' || c == '°' || c == 'Å' || c == 'Ω' {
            let start = i;
            let mut end = i;
            while let Some(&(idx, sym_c)) = chars.peek() {
                if sym_c.is_alphanumeric()
                    || sym_c == '_'
                    || sym_c == '°'
                    || sym_c == 'Å'
                    || sym_c == 'Ω'
                {
                    end = idx + sym_c.len_utf8();
                    chars.next();
                } else if sym_c == '^' {
                    end = idx + sym_c.len_utf8();
                    chars.next();
                    if let Some(&(sign_idx, sign_c)) = chars.peek() {
                        if sign_c == '+' || sign_c == '-' {
                            end = sign_idx + sign_c.len_utf8();
                            chars.next();
                        }
                    }
                    while let Some(&(digit_idx, digit_c)) = chars.peek() {
                        if digit_c.is_ascii_digit() || digit_c == '.' {
                            end = digit_idx + digit_c.len_utf8();
                            chars.next();
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            let sym = &input_text[start..end];

            if CONVERSION_KEYWORDS.contains(&sym) {
                tokens.push(Token::ConversionOp);
            } else if let Some(op) = token_registry.function_operators.get(sym) {
                let rest = input_text[end..].trim_start();
                if rest.starts_with('(') || !unit_registry.contains(sym) {
                    tokens.push(Token::Function(op.name));
                } else {
                    tokens.push(Token::Unit(sym));
                }
            } else if let Some(op) = token_registry.binary_operators.get(sym) {
                tokens.push(Token::BinaryOp(op.alias));
            } else if let Some(op) = token_registry.unary_operators.get(sym) {
                tokens.push(Token::UnaryOp(op.alias));
            } else if unit_registry.contains(sym) {
                tokens.push(Token::Unit(sym));
            } else {
                return Err(AbacusError::UnknownUnit(sym.to_string()));
            }

            continue;
        }

        return Err(AbacusError::UnknownUnit(c.to_string()));
    }

    // Combine adjacent Float + Unit into Val if separated by space (e.g. `5.0` + `km`)
    let mut resolved: Vec<Token> = Vec::new();
    let mut idx = 0;
    while idx < tokens.len() {
        if idx + 1 < tokens.len() {
            if let (Token::Float(num), Token::Unit(unit_sym)) = (&tokens[idx], &tokens[idx + 1]) {
                if let Ok(unit) = unit_registry.unit(unit_sym) {
                    resolved.push(Token::Val(Value::new(*num, unit)));
                    idx += 2;
                    continue;
                }
            }
        }
        resolved.push(tokens[idx].clone());
        idx += 1;
    }

    // Insert implicit multiplication `BinaryOp("*")` between adjacent terms (e.g. `5(2+3)`, `2 sqrt(9)`)
    let is_left = |tok: &Token| {
        matches!(
            tok,
            Token::Val(_)
                | Token::Float(_)
                | Token::Unit(_)
                | Token::CloseParen
                | Token::CloseBracket
        )
    };

    let is_right = |tok: &Token| match tok {
        Token::Val(_)
        | Token::Float(_)
        | Token::Unit(_)
        | Token::OpenParen
        | Token::OpenBracket
        | Token::Function(_) => true,
        Token::UnaryOp(name) => {
            if let Some(op) = token_registry.unary_operators.get(*name) {
                op.prefix && op.alias != "++" && op.alias != "--"
            } else {
                false
            }
        }
        _ => false,
    };

    let mut final_tokens: Vec<Token> = Vec::new();
    for i in 0..resolved.len() {
        final_tokens.push(resolved[i].clone());
        if i + 1 < resolved.len() {
            if is_left(&resolved[i]) && is_right(&resolved[i + 1]) {
                final_tokens.push(Token::BinaryOp("*"));
            }
        }
    }

    Ok(final_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_spaced_and_unspaced_expressions() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        let tokens = tokenize_string(&token_reg, &unit_reg, "10m+5m").unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::Val(_)));
        assert_eq!(tokens[1], Token::BinaryOp("+"));
        assert!(matches!(tokens[2], Token::Val(_)));

        let tokens2 = tokenize_string(&token_reg, &unit_reg, "1 bbl in m^3").unwrap();
        assert_eq!(tokens2.len(), 3);
        assert!(matches!(tokens2[0], Token::Val(_)));
        assert_eq!(tokens2[1], Token::ConversionOp);
        assert_eq!(tokens2[2], Token::Unit("m^3"));
    }

    #[test]
    fn tokenizes_parens_and_unary_operators() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        let tokens = tokenize_string(&token_reg, &unit_reg, "sqrt(9 m^2)").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::UnaryOp("sqrt"));
        assert_eq!(tokens[1], Token::OpenParen);
        assert!(matches!(tokens[2], Token::Val(_)));
        assert_eq!(tokens[3], Token::CloseParen);
    }

    #[test]
    fn tokenizes_functions_and_commas() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        let tokens = tokenize_string(&token_reg, &unit_reg, "mean(10, 0.5, 5)").unwrap();
        assert_eq!(tokens[1], Token::OpenParen);
        assert_eq!(tokens[2], Token::Float(10.0));
        assert_eq!(tokens[3], Token::Comma);
        assert_eq!(tokens[4], Token::Float(0.5));
        assert_eq!(tokens[5], Token::Comma);
        assert_eq!(tokens[6], Token::Float(5.0));
        assert_eq!(tokens[7], Token::CloseParen);
    }

    #[test]
    fn executes_registered_functions() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        // sin(45 deg)
        let sin_op = &token_reg.function_operators["sin"];
        let angle = unit_reg.value(45.0, "deg").unwrap();
        let sin_res = sin_op.apply(&[angle]).unwrap();
        assert!(
            (sin_res.into_scalar().unwrap().canonical - (std::f64::consts::FRAC_1_SQRT_2)).abs()
                < 1e-10
        );

        // mean(10 m, 20 m, 30 m)
        let mean_op = &token_reg.function_operators["mean"];
        let v1 = unit_reg.value(10.0, "m").unwrap();
        let v2 = unit_reg.value(20.0, "m").unwrap();
        let v3 = unit_reg.value(30.0, "m").unwrap();
        let mean_res = mean_op.apply(&[v1, v2, v3]).unwrap();
        assert_eq!(mean_res.to_display(), "20 m");
    }

    #[test]
    fn fails_on_unknown_tokens() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        assert!(tokenize_string(&token_reg, &unit_reg, "xyz").is_err());
    }
}

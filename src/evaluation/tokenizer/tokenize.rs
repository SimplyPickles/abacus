use crate::{
    AbacusError, UnitRegistry, Value,
    evaluation::tokenizer::{
        registry::token_registry::{MatchedOpKind, TokenRegistry},
        tokens::Token,
    },
};

const CONVERSION_KEYWORDS: [&str; 3] = ["as", "to", "in"];

/// If `haystack` starts with the literal word `word` (case-insensitively) and is followed
/// by a non-alphanumeric character (or end-of-string), returns the word length; otherwise
/// returns `None`.
#[inline]
fn strip_word_prefix(haystack: &str, word: &str) -> Option<usize> {
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

fn parse_weekday(s: &str) -> Option<(crate::units::date::DayOfWeek, usize)> {
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
        if let Some(len) = strip_word_prefix(s, name) {
            return Some((dow, len));
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
enum RelModifier {
    Last,
    Next,
    This,
}

fn parse_rel_modifier(s: &str) -> Option<(RelModifier, usize)> {
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
            (if add == 0 { 7 } else { add }) as i64
        };
        let date = ref_date.add_days(days_offset);
        return Some((date, sub_len));
    }

    None
}

fn try_parse_date_literal(remaining: &str) -> Option<(crate::Date, usize)> {
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

pub fn tokenize_string<'a>(
    token_registry: &TokenRegistry,
    unit_registry: &UnitRegistry,
    input_text: &'a str,
) -> Result<Vec<Token<'a>>, AbacusError> {
    let mut tokens = Vec::new();
    let mut chars = input_text.char_indices().peekable();
    let ops_by_first_char = token_registry.operators_by_first_char();

    while let Some(&(i, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // Date literals (e.g. today, tomorrow, yesterday, now, @2026-08-07@, 07-08-2026, 12:00, 1:00 PM)
        let is_date_candidate = c == '@'
            || (c.is_ascii_digit() && {
                let rem_bytes = &input_text.as_bytes()[i..];
                let mut has_sep = false;
                let mut idx = 0;
                while idx < rem_bytes.len() && idx < 20 {
                    let b = rem_bytes[idx];
                    if b == b'-'
                        || b == b'/'
                        || b == b':'
                        || b == b'p'
                        || b == b'P'
                        || b == b'a'
                        || b == b'A'
                    {
                        has_sep = true;
                        break;
                    }
                    if b == b' '
                        || b == b'\t'
                        || b == b','
                        || b == b')'
                        || b == b']'
                        || b == b'+'
                        || b == b'*'
                    {
                        break;
                    }
                    idx += 1;
                }
                has_sep
            })
            || matches!(
                c,
                't' | 'T'
                    | 'y'
                    | 'Y'
                    | 'n'
                    | 'N'
                    | 'l'
                    | 'L'
                    | 'p'
                    | 'P'
                    | 'm'
                    | 'M'
                    | 'w'
                    | 'W'
                    | 'f'
                    | 'F'
                    | 's'
                    | 'S'
            );

        if is_date_candidate
            && let Some((date, consumed_bytes)) = try_parse_date_literal(&input_text[i..])
        {
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

        // Timezone offset literals (e.g. +02:00, -04:00, +05:30)
        if c == '+' || c == '-' {
            let remaining = &input_text[i..];
            if let Some(word) = remaining.split_whitespace().next()
                && word.contains(':')
                && crate::units::date::TimeZone::parse(word).is_ok()
            {
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
        if c == 'b' || c == 'B' || c == 'w' || c == 'W' {
            const MULTI_WORD_UNITS: &[&str] = &[
                "business days",
                "business day",
                "working days",
                "working day",
                "work days",
                "work day",
            ];
            let remaining = &input_text[i..];
            let mut matched_len = 0;
            for &unit_name in MULTI_WORD_UNITS {
                if remaining.len() >= unit_name.len()
                    && remaining.as_bytes()[..unit_name.len()]
                        .eq_ignore_ascii_case(unit_name.as_bytes())
                {
                    // Check word boundary
                    let next_char = remaining[unit_name.len()..].chars().next();
                    if next_char.is_none_or(|nc| !nc.is_alphanumeric() && nc != '_') {
                        matched_len = unit_name.len();
                        break;
                    }
                }
            }
            if matched_len > 0 {
                let unit_str = &input_text[i..i + matched_len];
                tokens.push(Token::Unit(unit_str));
                let target_idx = i + matched_len;
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
            } else if let Some(&(next_idx, next_c)) = dot_lookahead.peek()
                && (next_c.is_alphabetic() || next_c == '_')
            {
                chars.next(); // consume '.'
                let start = next_idx;
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

        // Registered operator checks (prioritizing longer length operators like `++` over `+`)
        if let Some(candidates) = ops_by_first_char.get(&c) {
            let remaining = &input_text[i..];
            let mut best_match: Option<(&str, MatchedOpKind)> = None;

            for &(pattern, kind) in candidates {
                if let Some(rest) = remaining.strip_prefix(pattern) {
                    match kind {
                        MatchedOpKind::Binary(_) => {
                            if pattern == "%" {
                                let after = rest.trim_start();
                                let is_of = after.to_ascii_lowercase().starts_with("of")
                                    && (after.len() == 2 || !after.as_bytes()[2].is_ascii_alphanumeric());
                                if is_of {
                                    continue;
                                }
                                let starts_expr = after.starts_with(|c: char| c.is_ascii_digit() || c == '(');
                                if !starts_expr {
                                    continue;
                                }
                            }
                            if let Some(last_char) = pattern.chars().last()
                                && (last_char.is_alphanumeric() || last_char == '_')
                                && let Some(next_char) = rest.chars().next()
                                && (next_char.is_alphanumeric()
                                    || next_char == '_'
                                    || next_char == '°'
                                    || next_char == 'Å'
                                    || next_char == 'Ω')
                            {
                                continue;
                            }
                        }
                        MatchedOpKind::Unary(_) => {
                            if let Some(last_char) = pattern.chars().last()
                                && (last_char.is_alphanumeric() || last_char == '_')
                                && let Some(next_char) = rest.chars().next()
                                && (next_char.is_alphanumeric()
                                    || next_char == '_'
                                    || next_char == '°'
                                    || next_char == 'Å'
                                    || next_char == 'Ω')
                            {
                                continue;
                            }
                        }
                        MatchedOpKind::Func(_) => {
                            if unit_registry.contains(pattern) {
                                let next_slice = rest.trim_start();
                                if !next_slice.starts_with('(') {
                                    continue;
                                }
                            }
                            if let Some(last_char) = pattern.chars().last()
                                && (last_char.is_alphanumeric() || last_char == '_' || last_char == '-')
                                && let Some(next_char) = rest.chars().next()
                                && (next_char.is_alphanumeric()
                                    || next_char == '_'
                                    || next_char == '-'
                                    || next_char == '°'
                                    || next_char == 'Å'
                                    || next_char == 'Ω')
                            {
                                continue;
                            }
                        }
                    }

                    best_match = Some((pattern, kind));
                    break;
                }
            }

            if let Some((alias, matched_op)) = best_match {
                let char_count = alias.chars().count();
                for _ in 0..char_count {
                    chars.next();
                }
                match matched_op {
                    MatchedOpKind::Binary(op_alias) => tokens.push(Token::BinaryOp(op_alias)),
                    MatchedOpKind::Unary(op_alias) => tokens.push(Token::UnaryOp(op_alias)),
                    MatchedOpKind::Func(fn_name) => tokens.push(Token::Function(fn_name)),
                }
                continue;
            }
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
                    .is_some_and(|(_, next_c)| next_c.is_ascii_digit()))
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
                    if let Some(&(_, sign_c)) = exp_lookahead.peek()
                        && (sign_c == '+' || sign_c == '-')
                    {
                        exp_lookahead.next();
                    }
                    if let Some(&(_, digit_c)) = exp_lookahead.peek()
                        && digit_c.is_ascii_digit()
                    {
                        has_exp = true;
                        chars.next(); // consume 'e'/'E'
                        if let Some(&(_, sign_c)) = chars.peek()
                            && (sign_c == '+' || sign_c == '-')
                        {
                            chars.next(); // consume '+' or '-'
                        }
                        continue;
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
                .map_err(|_| AbacusError::InvalidNumber(num_str.to_string()))?;

            // Check if immediately followed by an unspaced unit identifier (e.g. 5km, 10m, 1s^-1)
            if let Some(&(unit_start, unit_c)) = chars.peek()
                && (unit_c.is_alphabetic()
                    || unit_c == '°'
                    || unit_c == 'Å'
                    || unit_c == 'Ω'
                    || unit_c == '%')
            {
                let mut unit_end = unit_start;
                let mut unit_chars = chars.clone();
                while let Some((idx, sym_c)) = unit_chars.peek().cloned() {
                    if sym_c.is_alphanumeric()
                        || sym_c == '_'
                        || sym_c == '°'
                        || sym_c == 'Å'
                        || sym_c == 'Ω'
                        || sym_c == '%'
                    {
                        unit_end = idx + sym_c.len_utf8();
                        unit_chars.next();
                    } else if sym_c == '^' {
                        unit_end = idx + sym_c.len_utf8();
                        unit_chars.next();
                        if let Some((sign_idx, sign_c)) = unit_chars.peek().cloned()
                            && (sign_c == '+' || sign_c == '-')
                        {
                            unit_end = sign_idx + sign_c.len_utf8();
                            unit_chars.next();
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

            tokens.push(Token::Float(val));
            continue;
        }

        // Identifiers (units, conversion operators, named unary ops like sqrt)
        if c.is_alphabetic() || c == '_' || c == '°' || c == 'Å' || c == 'Ω' || c == '%' {
            let start = i;
            let mut end = i;
            while let Some(&(idx, sym_c)) = chars.peek() {
                if sym_c.is_alphanumeric()
                    || sym_c == '_'
                    || sym_c == '°'
                    || sym_c == 'Å'
                    || sym_c == 'Ω'
                    || sym_c == '%'
                {
                    end = idx + sym_c.len_utf8();
                    chars.next();
                } else if sym_c == '^' {
                    end = idx + sym_c.len_utf8();
                    chars.next();
                    if let Some(&(sign_idx, sign_c)) = chars.peek()
                        && (sign_c == '+' || sign_c == '-')
                    {
                        end = sign_idx + sign_c.len_utf8();
                        chars.next();
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
                if sym == "%" {
                    let after = input_text[end..].trim_start();
                    let is_of = after.to_ascii_lowercase().starts_with("of")
                        && (after.len() == 2 || !after.as_bytes()[2].is_ascii_alphanumeric());
                    let starts_expr = after.starts_with(|c: char| c.is_ascii_digit() || c == '(');
                    if !is_of && starts_expr {
                        tokens.push(Token::BinaryOp(op.alias));
                    } else if unit_registry.contains(sym) {
                        tokens.push(Token::Unit(sym));
                    } else {
                        tokens.push(Token::BinaryOp(op.alias));
                    }
                } else {
                    tokens.push(Token::BinaryOp(op.alias));
                }
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
    let mut resolved: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut iter = tokens.into_iter().peekable();
    while let Some(tok) = iter.next() {
        if let Token::Float(num) = tok {
            if let Some(Token::Unit(unit_sym)) = iter.peek()
                && let Ok(unit) = unit_registry.unit(unit_sym)
            {
                iter.next();
                resolved.push(Token::Val(Value::new(num, unit)));
                continue;
            }
            resolved.push(Token::Float(num));
        } else {
            resolved.push(tok);
        }
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

    let mut final_tokens: Vec<Token> = Vec::with_capacity(resolved.len() * 2);
    let mut iter = resolved.into_iter().peekable();
    while let Some(tok) = iter.next() {
        let insert_mul = if let Some(next_tok) = iter.peek() {
            is_left(&tok) && is_right(next_tok)
        } else {
            false
        };
        final_tokens.push(tok);
        if insert_mul {
            final_tokens.push(Token::BinaryOp("*"));
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
        let sin_res = sin_op.apply_scalar(&[angle]).unwrap();
        assert!(
            (sin_res.into_scalar().unwrap().canonical - (std::f64::consts::FRAC_1_SQRT_2)).abs()
                < 1e-10
        );

        // mean(10 m, 20 m, 30 m)
        let mean_op = &token_reg.function_operators["mean"];
        let v1 = unit_reg.value(10.0, "m").unwrap();
        let v2 = unit_reg.value(20.0, "m").unwrap();
        let v3 = unit_reg.value(30.0, "m").unwrap();
        let mean_res = mean_op.apply_scalar(&[v1, v2, v3]).unwrap();
        assert_eq!(mean_res.to_display(), "20 m");
    }

    #[test]
    fn fails_on_unknown_tokens() {
        let token_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();

        assert!(tokenize_string(&token_reg, &unit_reg, "xyz").is_err());
    }
}

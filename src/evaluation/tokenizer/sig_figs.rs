/// Count the number of significant figures in a numeric string literal without heap allocations.
///
/// Rules applied:
/// - Any non-zero digit is significant.
/// - Any zero between non-zero digits is significant.
/// - For numbers with a decimal point, leading zeros before the first non-zero digit are not significant,
///   but all trailing zeros after non-zero digits are significant (e.g. "12.30" -> 4, "0.00450" -> 3).
/// - For numbers without a decimal point, trailing zeros are considered not significant (e.g. "1200" -> 2).
/// - Exponents in scientific notation (e.g. `e4`, `E-2`) do not affect significant figures.
#[must_use]
pub fn count_significant_figures(s: &str) -> Option<usize> {
    let s = s.trim().trim_start_matches('+').trim_start_matches('-');
    if s.is_empty() {
        return None;
    }

    // Split mantissa from exponent if present (e.g. 1.23e4 -> 1.23)
    let mantissa = s.split(['e', 'E']).next()?.trim();
    if mantissa.is_empty() {
        return None;
    }

    if let Some((int_part, frac_part)) = mantissa.split_once('.') {
        let int_bytes = int_part.as_bytes();
        let frac_bytes = frac_part.as_bytes();

        let int_digits_count = int_bytes.iter().filter(|&&b| b.is_ascii_digit()).count();
        let frac_digits_count = frac_bytes.iter().filter(|&&b| b.is_ascii_digit()).count();

        let first_int_non_zero = int_bytes
            .iter()
            .filter(|&&b| b.is_ascii_digit())
            .position(|&b| b != b'0');
        let first_frac_non_zero = frac_bytes
            .iter()
            .filter(|&&b| b.is_ascii_digit())
            .position(|&b| b != b'0');

        if let Some(first_non_zero) = first_int_non_zero {
            let count = (int_digits_count - first_non_zero) + frac_digits_count;
            Some(count.max(1))
        } else if let Some(first_non_zero) = first_frac_non_zero {
            let count = frac_digits_count - first_non_zero;
            Some(count.max(1))
        } else {
            // All zeros e.g. "0.0" -> 2 sig figs
            Some((int_digits_count + frac_digits_count).max(1))
        }
    } else {
        let bytes = mantissa.as_bytes();
        let digit_count = bytes.iter().filter(|&&b| b.is_ascii_digit()).count();
        if digit_count == 0 {
            return None;
        }
        let first_non_zero = bytes
            .iter()
            .filter(|&&b| b.is_ascii_digit())
            .position(|&b| b != b'0');
        let rev_non_zero = bytes
            .iter()
            .filter(|&&b| b.is_ascii_digit())
            .rev()
            .position(|&b| b != b'0');

        match (first_non_zero, rev_non_zero) {
            (Some(first), Some(from_end)) => {
                let count = digit_count.saturating_sub(first + from_end);
                Some(count.max(1))
            }
            _ => Some(1),
        }
    }
}

/// Scans an expression string for numeric literals and returns the minimum number of significant figures
/// across all numbers found.
#[must_use]
pub fn min_significant_figures_in_expr(expr: &str) -> Option<usize> {
    let mut min_sig: Option<usize> = None;
    let mut chars = expr.char_indices().peekable();

    while let Some(&(i, c)) = chars.peek() {
        // Skip date blocks between '@'
        if c == '@' {
            chars.next();
            while let Some(&(_, ch)) = chars.peek() {
                chars.next();
                if ch == '@' {
                    break;
                }
            }
            continue;
        }

        // Detect number start (digit or leading dot followed by digit)
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
                    let mut lookahead = chars.clone();
                    lookahead.next();
                    if let Some(&(_, next_c)) = lookahead.peek()
                        && next_c.is_ascii_digit()
                    {
                        has_dot = true;
                        chars.next();
                    } else {
                        break;
                    }
                } else if (num_c == 'e' || num_c == 'E') && !has_exp {
                    let mut lookahead = chars.clone();
                    lookahead.next();
                    if let Some(&(_, sign_c)) = lookahead.peek()
                        && (sign_c == '+' || sign_c == '-')
                    {
                        lookahead.next();
                    }
                    if let Some(&(_, digit_c)) = lookahead.peek()
                        && digit_c.is_ascii_digit()
                    {
                        has_exp = true;
                        chars.next(); // 'e' or 'E'
                        if let Some(&(_, sign_c)) = chars.peek()
                            && (sign_c == '+' || sign_c == '-')
                        {
                            chars.next();
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let num_str = &expr[start..chars.peek().map_or(expr.len(), |&(idx, _)| idx)];
            if let Some(sig) = count_significant_figures(num_str) {
                min_sig = Some(min_sig.map_or(sig, |m| m.min(sig)));
            }
            continue;
        }

        chars.next();
    }

    min_sig
}

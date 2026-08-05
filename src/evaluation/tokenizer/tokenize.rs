use crate::{
    AbacusError, UnitRegistry, Value,
    evaluation::tokenizer::{registry::token_registry::TokenRegistry, tokens::Token},
};

const CONVERSION_KEYWORDS: [&str; 3] = ["as", "to", "in"];

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
                tokens.push(Token::Function(op.name));
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

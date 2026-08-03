use crate::{
    AbacusError, UnitRegistry, Value,
    evaluation::tokenizer::{registry::token_registry::TokenRegistry, tokens::Token},
};

const CONVERSION_KEYWORDS: [&str; 3] = ["as", "to", "in"];

pub fn tokenize_string(
    token_registry: &TokenRegistry,
    unit_registry: &UnitRegistry,
    input_text: &str,
) -> Result<Vec<Token>, AbacusError> {
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

        // Single-character operator checks (+, -, *, /, ^)
        let single_op_str = &input_text[i..i + c.len_utf8()];
        if token_registry.binary_operators.contains_key(single_op_str) {
            let op_alias = token_registry.binary_operators[single_op_str].alias;
            tokens.push(Token::BinaryOp(op_alias));
            chars.next();
            continue;
        }
        if token_registry.unary_operators.contains_key(single_op_str) {
            let op_alias = token_registry.unary_operators[single_op_str].alias;
            tokens.push(Token::UnaryOp(op_alias));
            chars.next();
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
            while let Some(&(_, num_c)) = chars.peek() {
                if num_c.is_ascii_digit() {
                    chars.next();
                } else if num_c == '.' && !has_dot {
                    // Peek ahead: if the next char after this '.' is also '.',
                    // then this is the start of `..` (range), not a decimal point.
                    let mut dot_lookahead = chars.clone();
                    dot_lookahead.next(); // skip past the '.'
                    if let Some(&(_, '.')) = dot_lookahead.peek() {
                        break; // stop number here — `..` is a range
                    }
                    has_dot = true;
                    chars.next();
                } else {
                    break;
                }
            }
            let num_str =
                &input_text[start..chars.peek().map_or(input_text.len(), |&(idx, _)| idx)];
            let val = num_str
                .parse::<f64>()
                .map_err(|_| AbacusError::UnknownUnit(num_str.to_string()))?;

            // Check if immediately followed by an unspaced unit identifier (e.g. 5km, 10m)
            if let Some(&(unit_start, unit_c)) = chars.peek() {
                if unit_c.is_alphabetic() || unit_c == '°' || unit_c == 'Å' || unit_c == 'Ω' {
                    let mut unit_end = unit_start;
                    let mut unit_chars = chars.clone();
                    while let Some((idx, sym_c)) = unit_chars.peek().cloned() {
                        if sym_c.is_alphanumeric()
                            || sym_c == '_'
                            || sym_c == '^'
                            || sym_c == '°'
                            || sym_c == 'Å'
                            || sym_c == 'Ω'
                        {
                            unit_end = idx + sym_c.len_utf8();
                            unit_chars.next();
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
                    || sym_c == '^'
                    || sym_c == '°'
                    || sym_c == 'Å'
                    || sym_c == 'Ω'
                {
                    end = idx + sym_c.len_utf8();
                    chars.next();
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
                tokens.push(Token::Unit(sym.to_string()));
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

    Ok(resolved)
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
        assert_eq!(tokens2[2], Token::Unit("m^3".to_string()));
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
        assert!((sin_res.canonical - (std::f64::consts::FRAC_1_SQRT_2)).abs() < 1e-10);

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

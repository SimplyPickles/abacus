use crate::{
    evaluation::tokenizer::{
        registry::token_registry::TokenRegistry,
        tokens::Token,
    },
    UnitRegistry, Value,
};

/// Combines adjacent `Float + Unit` tokens and inserts implicit `BinaryOp("*")` between adjacent terms.
pub(crate) fn resolve_tokens<'a>(
    tokens: Vec<Token<'a>>,
    unit_registry: &UnitRegistry,
    token_registry: &TokenRegistry,
    implicit_multiplication: bool,
) -> Vec<Token<'a>> {
    // 1. Combine adjacent Float + Unit into Val if separated by space (e.g. `5.0` + `km`)
    let mut resolved: Vec<Token<'a>> = Vec::with_capacity(tokens.len());
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

    if !implicit_multiplication {
        return resolved;
    }

    // 2. Insert implicit multiplication `BinaryOp("*")` between adjacent terms (e.g. `5(2+3)`, `2 sqrt(9)`)
    let is_left = |tok: &Token<'a>| {
        matches!(
            tok,
            Token::Val(_)
                | Token::Float(_)
                | Token::Unit(_)
                | Token::CloseParen
                | Token::CloseBracket
        )
    };

    let is_right = |tok: &Token<'a>| match tok {
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

    let mut final_tokens: Vec<Token<'a>> = Vec::with_capacity(resolved.len() * 2);
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

    final_tokens
}

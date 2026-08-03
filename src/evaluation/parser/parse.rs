use crate::{
    AbacusError, UnitRegistry, Value,
    evaluation::tokenizer::{registry::token_registry::TokenRegistry, tokens::Token},
};

/// A Pratt parser that consumes a `Vec<Token>` produced by the tokenizer
/// and evaluates it down to a single `Value`.
///
/// Precedence levels (binding power):
///   0 — ConversionOp (`in`, `to`, `as`)
///   1 — Addition / Subtraction (`+`, `-`)
///   2 — Multiplication / Division (`*`, `/`)
///   3 — Implicit multiplication (e.g. `5 m` already resolved, but `Val * Val` adjacency)
///   4 — Unary prefix (`-`, `sqrt`)
///   5 — Exponentiation (`^`, right-associative)
///   6 — Postfix (`!`)
pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    token_registry: &'a TokenRegistry,
    unit_registry: &'a UnitRegistry,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: Vec<Token>,
        token_registry: &'a TokenRegistry,
        unit_registry: &'a UnitRegistry,
    ) -> Self {
        Self {
            tokens,
            pos: 0,
            token_registry,
            unit_registry,
        }
    }

    /// Peek at the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Consume the current token and advance.
    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    /// Expect and consume a specific token, or return an error.
    fn expect(&mut self, expected: &Token) -> Result<(), AbacusError> {
        match self.peek() {
            Some(tok) if tok == expected => {
                self.advance();
                Ok(())
            }
            Some(tok) => Err(AbacusError::UnexpectedToken(format!("{:?}", tok))),
            None => Err(AbacusError::UnexpectedEnd),
        }
    }

    /// Entry point: parse the full expression at minimum binding power 0.
    pub fn parse(&mut self) -> Result<Value, AbacusError> {
        let result = self.parse_expr(0)?;

        // Ensure all tokens were consumed
        if self.pos < self.tokens.len() {
            if let Some(tok) = self.peek() {
                return Err(AbacusError::UnexpectedToken(format!("{:?}", tok)));
            }
            return Err(AbacusError::UnexpectedEnd);
        }

        Ok(result)
    }

    /// Core Pratt expression parser.
    ///
    /// `min_bp` is the minimum binding power — the parser will keep consuming
    /// infix operators whose left binding power is ≥ `min_bp`.
    fn parse_expr(&mut self, min_bp: u8) -> Result<Value, AbacusError> {
        // ── NUD (prefix / atom) ──
        let mut lhs = self.parse_prefix()?;

        // ── LED (infix / postfix) ──
        loop {
            // Check for postfix operators (e.g. `!`)
            if let Some(Token::UnaryOp(name)) = self.peek() {
                let name = *name;
                if let Some(op) = self.token_registry.unary_operators.get(name) {
                    if !op.prefix {
                        let bp = self.postfix_bp(name);
                        if bp < min_bp {
                            break;
                        }
                        self.advance();
                        lhs = op.apply(lhs)?;
                        continue;
                    }
                }
            }

            // Check for infix binary operators
            if let Some(Token::BinaryOp(name)) = self.peek() {
                let name = *name;
                let (l_bp, r_bp) = self.infix_bp(name);
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                let rhs = self.parse_expr(r_bp)?;
                let op = self
                    .token_registry
                    .binary_operators
                    .get(name)
                    .ok_or_else(|| AbacusError::UnexpectedToken(name.to_string()))?;
                lhs = op.apply(lhs, rhs)?;
                continue;
            }

            // Check for conversion operator (`in`, `to`, `as`)
            if let Some(Token::ConversionOp) = self.peek() {
                let l_bp = 1; // lowest infix precedence
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                // The RHS of a conversion can be a unit symbol or compound unit expression
                let target_val = self.parse_expr(1)?;
                lhs = lhs.convert_to(target_val.unit)?;
                continue;
            }

            // No more infix/postfix operators at this precedence level
            break;
        }

        Ok(lhs)
    }

    /// Parse a prefix expression (NUD in Pratt terminology).
    fn parse_prefix(&mut self) -> Result<Value, AbacusError> {
        match self.advance() {
            Some(Token::Val(val)) => Ok(val),

            Some(Token::Unit(unit_sym)) => {
                let unit = self.unit_registry.unit(&unit_sym)?;
                Ok(Value::new(1.0, unit))
            }

            Some(Token::Float(num)) => {
                // A bare float without a unit — wrap in dimensionless Value
                Ok(Value::new(
                    num,
                    self.unit_registry.unit("1").unwrap_or_else(|_| {
                        use crate::units::{
                            dimensions::Dimensions,
                            unit::{Unit, UnitExpr},
                        };
                        std::sync::Arc::new(Unit {
                            scalar: 1.0,
                            offset: 0.0,
                            dimensions: Dimensions::DIMENSIONLESS,
                            display: UnitExpr::dimensionless(),
                        })
                    }),
                ))
            }

            // Grouped expression: ( expr )
            Some(Token::OpenParen) => {
                let val = self.parse_expr(0)?;
                self.expect(&Token::CloseParen)
                    .map_err(|_| AbacusError::UnclosedParen)?;
                Ok(val)
            }

            // Prefix unary operator: -expr, sqrt(expr)
            // The tokenizer emits BinaryOp("-") for `-`, but in prefix position
            // it acts as unary negation.
            Some(Token::BinaryOp("-")) => {
                let op = self
                    .token_registry
                    .unary_operators
                    .get("-")
                    .ok_or_else(|| AbacusError::UnexpectedToken("-".to_string()))?
                    .clone();
                let bp = self.prefix_bp("-");
                let operand = self.parse_expr(bp)?;
                op.apply(operand)
            }

            Some(Token::UnaryOp(name)) => {
                let op = self
                    .token_registry
                    .unary_operators
                    .get(name)
                    .ok_or_else(|| AbacusError::UnexpectedToken(name.to_string()))?
                    .clone();
                let bp = self.prefix_bp(name);
                let operand = self.parse_expr(bp)?;
                op.apply(operand)
            }

            // Function call: name(arg1, arg2, ...)
            Some(Token::Function(name)) => {
                let func = self
                    .token_registry
                    .function_operators
                    .get(name)
                    .ok_or_else(|| AbacusError::UnexpectedToken(name.to_string()))?
                    .clone();

                self.expect(&Token::OpenParen).map_err(|_| {
                    AbacusError::UnexpectedToken("expected '(' after function name".to_string())
                })?;

                let mut args = Vec::new();

                // Handle empty argument list: func()
                if self.peek() != Some(&Token::CloseParen) {
                    loop {
                        let arg = self.parse_expr(0)?;

                        // Check for range: arg..end
                        if self.peek() == Some(&Token::Range) {
                            self.advance(); // consume `..`
                            let end = self.parse_expr(0)?;
                            let expanded = Self::expand_range(arg, end)?;
                            args.extend(expanded);
                        } else {
                            args.push(arg);
                        }

                        if self.peek() == Some(&Token::Comma) {
                            self.advance(); // consume ','
                        } else {
                            break;
                        }
                    }
                }

                self.expect(&Token::CloseParen)
                    .map_err(|_| AbacusError::UnclosedParen)?;

                func.apply(&args)
            }

            Some(tok) => Err(AbacusError::UnexpectedToken(format!("{:?}", tok))),
            None => Err(AbacusError::UnexpectedEnd),
        }
    }

    /// Returns (left_bp, right_bp) for an infix binary operator.
    ///
    /// For left-associative operators, right_bp = left_bp + 1.
    /// For right-associative operators (e.g. `^`), right_bp = left_bp.
    fn infix_bp(&self, name: &str) -> (u8, u8) {
        if let Some(op) = self.token_registry.binary_operators.get(name) {
            // Map the operator's registered precedence to Pratt binding powers.
            // We multiply by 2 and add a base offset to leave room for
            // conversion (bp 1) and prefix operators.
            let base = (op.precedence * 2) + 2;
            if op.right_associative {
                (base, base)
            } else {
                (base, base + 1)
            }
        } else {
            (0, 1)
        }
    }

    /// Returns the binding power for a prefix unary operator.
    fn prefix_bp(&self, name: &str) -> u8 {
        if let Some(op) = self.token_registry.unary_operators.get(name) {
            (op.precedence * 2) + 2
        } else {
            10 // high default
        }
    }

    /// Returns the binding power for a postfix unary operator.
    fn postfix_bp(&self, name: &str) -> u8 {
        if let Some(op) = self.token_registry.unary_operators.get(name) {
            (op.precedence * 2) + 2
        } else {
            10
        }
    }

    /// Expand a range between two values.
    /// The step size is determined by the start value's unit.
    fn expand_range(start: Value, end: Value) -> Result<Vec<Value>, AbacusError> {
        if !start.unit.is_compatible_with(&end.unit) {
            return Err(AbacusError::IncompatibleDimensions);
        }

        let start_val = start.canonical;
        let end_val = end.canonical;
        // Step size of 1 in the display unit, converted to canonical units
        let step = start.unit.scalar;

        let mut expanded = Vec::new();
        let mut current = start_val;

        // Use a small epsilon for floating point comparison issues at boundaries
        let epsilon = 1e-12 * step;

        if start_val <= end_val {
            while current <= end_val + epsilon {
                expanded.push(Value {
                    canonical: current,
                    unit: std::sync::Arc::clone(&start.unit),
                });
                current += step;
            }
        } else {
            while current >= end_val - epsilon {
                expanded.push(Value {
                    canonical: current,
                    unit: std::sync::Arc::clone(&start.unit),
                });
                current -= step;
            }
        }

        Ok(expanded)
    }
}

/// Convenience function: tokenize and parse an expression string into a `Value`.
pub fn evaluate(
    token_registry: &TokenRegistry,
    unit_registry: &UnitRegistry,
    input: &str,
) -> Result<Value, AbacusError> {
    let tokens = crate::evaluation::tokenizer::tokenize::tokenize_string(
        token_registry,
        unit_registry,
        input,
    )?;
    let mut parser = Parser::new(tokens, token_registry, unit_registry);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str) -> Result<String, AbacusError> {
        let tok_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();
        evaluate(&tok_reg, &unit_reg, input).map(|v| v.to_display())
    }

    fn eval_val(input: &str) -> Result<Value, AbacusError> {
        let tok_reg = TokenRegistry::standard();
        let unit_reg = UnitRegistry::standard();
        evaluate(&tok_reg, &unit_reg, input)
    }

    // ── Basic arithmetic ──

    #[test]
    fn evaluates_simple_addition() {
        let result = eval("5 m + 3 m").unwrap();
        assert_eq!(result, "8 m");
    }

    #[test]
    fn evaluates_subtraction() {
        let result = eval("10 kg - 3 kg").unwrap();
        assert_eq!(result, "7 kg");
    }

    #[test]
    fn evaluates_multiplication() {
        let result = eval("3 m * 4 m").unwrap();
        assert_eq!(result, "12 m^2");
    }

    #[test]
    fn evaluates_division() {
        let result = eval("10 m / 2 s").unwrap();
        assert_eq!(result, "5 m/s");
    }

    #[test]
    fn evaluates_bare_unit_division() {
        assert_eq!(eval("1m/m").unwrap(), "1");
        assert_eq!(eval("1 m / m").unwrap(), "1");
        assert_eq!(eval("5 km / m").unwrap(), "5000");
        assert_eq!(eval("km / h").unwrap(), "1 km/h");
    }

    // ── Operator precedence ──

    #[test]
    fn respects_multiplication_over_addition() {
        // 2 + 3 * 4 = 2 + 12 = 14 (not 20)
        let result = eval("2 + 3 * 4").unwrap();
        assert_eq!(result, "14");
    }

    #[test]
    fn respects_exponentiation_precedence() {
        // 2 ^ 3 ^ 2 = 2 ^ 9 = 512 (right-associative)
        let result = eval("2 ^ 3 ^ 2").unwrap();
        assert_eq!(result, "512");
    }

    // ── Parentheses ──

    #[test]
    fn evaluates_parenthesized_expression() {
        // (2 + 3) * 4 = 20
        let result = eval("(2 + 3) * 4").unwrap();
        assert_eq!(result, "20");
    }

    #[test]
    fn evaluates_nested_parentheses() {
        // ((2 + 3) * (4 - 1)) = 5 * 3 = 15
        let result = eval("((2 + 3) * (4 - 1))").unwrap();
        assert_eq!(result, "15");
    }

    // ── Unary operators ──

    #[test]
    fn evaluates_unary_negation() {
        let result = eval("-5 m + 8 m").unwrap();
        assert_eq!(result, "3 m");
    }

    #[test]
    fn evaluates_sqrt() {
        let result = eval("sqrt(9 m^2)").unwrap();
        assert_eq!(result, "3 m");
    }

    // ── Functions ──

    #[test]
    fn evaluates_sin_function() {
        let result = eval("sin(0)").unwrap();
        assert!((result.parse::<f64>().unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn evaluates_mean_function() {
        let result = eval("mean(10 m, 20 m, 30 m)").unwrap();
        assert_eq!(result, "20 m");
    }

    #[test]
    fn evaluates_nested_function_in_expression() {
        // mean(5 m + 5 m, 20 m) = mean(10 m, 20 m) = 15 m
        let result = eval("mean(5 m + 5 m, 20 m)").unwrap();
        assert_eq!(result, "15 m");
    }

    #[test]
    fn evaluates_function_in_larger_expression() {
        // max(3 m, 7 m) + 3 m = 7 m + 3 m = 10 m
        let result = eval("max(3 m, 7 m) + 3 m").unwrap();
        assert_eq!(result, "10 m");
    }

    // ── Conversion ──

    #[test]
    fn evaluates_unit_conversion() {
        let result = eval("5 km to m").unwrap();
        assert_eq!(result, "5000 m");
    }

    #[test]
    fn evaluates_expression_then_converts() {
        // (5 m + 20 cm) as m = 5.2 m
        let result = eval("(5 m + 20 cm) as m").unwrap();
        assert_eq!(result, "5.2 m");
    }

    #[test]
    fn evaluates_dimensionless_conversions() {
        assert_eq!(eval("1 as inches").unwrap(), "1 in");
        assert_eq!(eval("10 in m").unwrap(), "10 m");
        assert_eq!(eval("1 m/m in inches").unwrap(), "1 in");
    }

    // ── Ranges ──

    #[test]
    fn evaluates_range_in_function() {
        // mean(1..5) = mean(1, 2, 3, 4, 5) = 3
        let result = eval("mean(1..5)").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn evaluates_range_with_units() {
        // sum is not defined, but mean is.
        // mean(1 m..5 m) = mean(1m, 2m, 3m, 4m, 5m) = 3 m
        let result = eval("mean(1 m .. 5 m)").unwrap();
        assert_eq!(result, "3 m");
    }

    #[test]
    fn evaluates_range_with_different_units() {
        // mean(1 m .. 500 cm) = mean(1m, 2m, 3m, 4m, 5m) = 3 m
        let result = eval("mean(1 m .. 500 cm)").unwrap();
        assert_eq!(result, "3 m");
    }

    // ── Statistical & Distribution functions ──

    #[test]
    fn evaluates_stats_functions() {
        assert_eq!(eval("sum(1..5)").unwrap(), "15");
        assert_eq!(eval("range(1 m .. 5 m)").unwrap(), "4 m");
        assert_eq!(eval("median(1 m, 10 m, 5 m)").unwrap(), "5 m");
        assert_eq!(eval("mode(2 m, 5 m, 2 m)").unwrap(), "2 m");
        assert_eq!(eval("var(1 m .. 5 m)").unwrap(), "2.5 m^2");
        assert_eq!(eval("quantile(1 m .. 5 m, 0.5)").unwrap(), "3 m");
        assert_eq!(eval("percentile(1 m .. 5 m, 50)").unwrap(), "3 m");
        assert_eq!(eval("iqr(1 m .. 5 m)").unwrap(), "2 m");

        let corr_val = eval_val("corr(1..5, 2..6)").unwrap().canonical;
        assert!((corr_val - 1.0).abs() < 1e-6);
    }

    #[test]
    fn evaluates_distribution_functions() {
        // binompdf(10, 0.5, 5) = 252 * (0.5)^10 = 0.24609375
        let bpdf = eval_val("binompdf(10, 0.5, 5)").unwrap().canonical;
        assert!((bpdf - 0.24609375).abs() < 1e-6);

        // binomcdf(10, 0.5, 5)
        let bcdf = eval_val("binomcdf(10, 0.5, 5)").unwrap().canonical;
        assert!((bcdf - 0.623046875).abs() < 1e-6);

        // geompdf(0.5, 3) = (0.5)^2 * 0.5 = 0.125
        assert!((eval_val("geompdf(0.5, 3)").unwrap().canonical - 0.125).abs() < 1e-6);

        // poissonpdf(3, 2)
        assert!((eval_val("poissonpdf(3, 2)").unwrap().canonical - 0.2240418).abs() < 1e-5);

        // normcdf(0) = 0.5
        assert!((eval_val("normcdf(0)").unwrap().canonical - 0.5).abs() < 1e-6);

        // normcdf with units: normcdf(70 kg, 65 kg, 5 kg)
        assert!(
            (eval_val("normcdf(70 kg, 65 kg, 5 kg)").unwrap().canonical - 0.8413447).abs() < 1e-5
        );

        // Student's t: tcdf(10, 0) = 0.5
        assert!((eval_val("tcdf(10, 0)").unwrap().canonical - 0.5).abs() < 1e-6);

        // Chi-Square: chisqcdf(10, 10) ≈ 0.5595
        assert!((eval_val("chisqcdf(10, 10)").unwrap().canonical - 0.5595).abs() < 1e-3);

        // Exponential: expcdf(0.5, 2) = 1 - e^-1 ≈ 0.63212
        assert!((eval_val("expcdf(0.5, 2)").unwrap().canonical - 0.63212).abs() < 1e-4);

        // Hypergeometric: hypgeompdf(20, 7, 12, 4) = 45045 / 125970 ≈ 0.357585
        assert!((eval_val("hypgeompdf(20, 7, 12, 4)").unwrap().canonical - 0.357585).abs() < 1e-4);

        // Uniform: unifcdf(0, 10, 5) = 0.5
        assert!((eval_val("unifcdf(0, 10, 5)").unwrap().canonical - 0.5).abs() < 1e-6);

        // Inverse CDFs
        let z_star = eval_val("invnorm(0.975)").unwrap().canonical;
        assert!((z_star - 1.95996).abs() < 1e-3);

        let inv_norm_units = eval("invnorm(0.975, 100 kg, 15 kg)").unwrap();
        assert_eq!(inv_norm_units, "129.38573407034812 kg");

        let t_star = eval_val("invt(0.975, 10)").unwrap().canonical;
        assert!((t_star - 2.22814).abs() < 1e-3);

        let chisq_star = eval_val("invchisq(0.95, 10)").unwrap().canonical;
        assert!((chisq_star - 18.307).abs() < 1e-2);

        let exp_star = eval_val("invexp(0.63212, 0.5)").unwrap().canonical;
        assert!((exp_star - 2.0).abs() < 1e-3);

        let unif_star = eval_val("invunif(0.5, 0, 10)").unwrap().canonical;
        assert!((unif_star - 5.0).abs() < 1e-6);
    }

    // ── Implicit Multiplication ──

    #[test]
    fn evaluates_implicit_multiplication() {
        assert_eq!(eval("5(2 + 3)").unwrap(), "25");
        assert_eq!(eval("(2 + 3)(4 + 5)").unwrap(), "45");
        assert_eq!(eval("2 sqrt(9 m^2)").unwrap(), "6 m");
        assert_eq!(eval("2(10 m)").unwrap(), "20 m");
    }

    // ── Complex combined expressions ──

    #[test]
    fn evaluates_complex_expression() {
        // (10 m + 5 m) * 2 = 30 m
        let result = eval("(10 m + 5 m) * 2").unwrap();
        assert_eq!(result, "30 m");
    }

    // ── Error cases ──

    #[test]
    fn errors_on_unclosed_paren() {
        assert!(eval("(5 + 3").is_err());
    }

    #[test]
    fn errors_on_empty_input() {
        assert!(eval("").is_err());
    }

    #[test]
    fn errors_on_incompatible_addition() {
        assert!(eval("5 m + 3 s").is_err());
    }
}

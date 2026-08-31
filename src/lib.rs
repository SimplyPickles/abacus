#![warn(clippy::unwrap_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod evaluation;

pub mod error;
pub mod registry;
pub mod units;

use std::sync::Arc;

pub use error::AbacusError;
pub use registry::unit_registry::UnitRegistry;
pub use units::{
    date::{Date, DateFormat, DayOfWeek, Time, TimeZone},
    dimensions::Dimensions,
    eval_result::EvalResult,
    hash::Hash,
    interval::{Interval, IntervalStyle},
    unit::Unit,
    value::Value,
};

pub use evaluation::tokenizer::registry::{
    binary::operators::BinaryOp, function::operators::FunctionOp,
    function::operators::FunctionTarget, token_registry::TokenRegistry, unary::operators::UnaryOp,
};

pub use evaluation::tokenizer::tokens::Token;

use crate::evaluation::{parser::parse::evaluate, tokenizer::tokenize::tokenize_string};

pub struct Abacus {
    pub units: UnitRegistry,
    pub tokens: TokenRegistry,
    pub date_format: DateFormat,
}

impl Abacus {
    // Initialize a new `Abacus` instance with default units and tokens
    #[must_use]
    pub fn new() -> Self {
        Self {
            units: UnitRegistry::new(),
            tokens: TokenRegistry::new(),
            date_format: DateFormat::default(),
        }
    }

    // Initialize a new `Abacus` instance with custom units and tokens
    pub fn from_registry(units: UnitRegistry, tokens: TokenRegistry) -> Self {
        Self {
            units,
            tokens,
            date_format: DateFormat::default(),
        }
    }

    // Initialize a new `Abacus` instance with standard units and tokens
    #[must_use]
    pub fn standard() -> Self {
        Self {
            units: UnitRegistry::standard(),
            tokens: TokenRegistry::standard(),
            date_format: DateFormat::default(),
        }
    }

    // Set the date format for this `Abacus` instance
    pub fn set_date_format(mut self, format: DateFormat) -> Self {
        self.date_format = format;
        self
    }

    // Tokenize an expression into a vector of `Token`s
    pub fn tokenize<'a>(&self, expr: &'a str) -> Result<Vec<Token<'a>>, AbacusError> {
        tokenize_string(&self.tokens, &self.units, expr)
    }

    // Evaluate expressions, returning the result as an `EvalResult`
    // Evaluated dates are automatically formatted
    // Returns an error if the expression is invalid or cannot be evaluated.
    pub fn eval(&self, expr: &str) -> Result<EvalResult, AbacusError> {
        let mut res = evaluate(&self.tokens, &self.units, expr)?;
        if let EvalResult::Date(ref mut d) = res {
            d.format = self.date_format;
        }
        Ok(res)
    }

    /// Evaluate an expression, returning only scalar results.
    /// Returns an error if the result is an interval or hash.
    pub fn eval_scalar(&self, expr: &str) -> Result<Value, AbacusError> {
        self.eval(expr)?.into_scalar()
    }

    /// Evaluate an expression, returning only Hash results.
    /// Returns an error if the result is not a Hash.
    pub fn eval_hash(&self, expr: &str) -> Result<Hash, AbacusError> {
        self.eval(expr)?.into_hash()
    }

    /// Evaluate an expression, returning only Date results.
    /// Returns an error if the result is not a Date.
    pub fn eval_date(&self, expr: &str) -> Result<Date, AbacusError> {
        self.eval(expr)?.into_date()
    }

    // Format dates using configured styles
    pub fn format_date(&self, date: &Date) -> String {
        date.format_with_style(self.date_format)
    }

    pub fn format_date_with_style(&self, date: &Date, style: DateFormat) -> String {
        date.format_with_style(style)
    }

    // Registry functions for units & tokens
    pub fn register_unit(&mut self, alias: &str, unit: Unit) {
        self.units.insert_unit(alias, Arc::from(unit));
    }

    pub fn register_binop_token(&mut self, op: BinaryOp) {
        self.tokens.register_binary_operator(op.alias, op);
    }

    pub fn register_unop_token(&mut self, op: UnaryOp) {
        self.tokens.register_unary_operator(op.alias, op);
    }

    pub fn register_function_token(&mut self, op: FunctionOp) {
        self.tokens.register_function_operator(op.name, op);
    }

    /// Returns a reference to the globally shared standard `Abacus` instance.
    #[must_use]
    pub fn shared() -> &'static Self {
        &STANDARD
    }
}

pub static STANDARD: std::sync::LazyLock<Abacus> = std::sync::LazyLock::new(Abacus::standard);

impl Default for Abacus {
    fn default() -> Self {
        Abacus::new()
    }
}

// Convenience function for standard expression evaluation
pub fn eval(expr: &str) -> Result<EvalResult, AbacusError> {
    STANDARD.eval(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn global_units() -> &'static UnitRegistry {
        &Abacus::shared().units
    }

    pub fn unit(symbol: &str) -> Result<std::sync::Arc<Unit>, AbacusError> {
        global_units().unit(symbol)
    }

    pub fn value(amount: f64, symbol: &str) -> Result<Value, AbacusError> {
        Ok(Value::new(amount, unit(symbol)?))
    }

    #[test]
    fn looks_up_units_by_symbol() {
        assert_eq!(unit("km").unwrap().display.render(), "km");
        assert_eq!(unit("in").unwrap().display.render(), "in");
        assert_eq!(unit("h").unwrap().display.render(), "h");
    }

    #[test]
    fn looks_up_units_by_name() {
        assert_eq!(unit("hour").unwrap().display.render(), "h");
    }

    #[test]
    fn returns_an_error_for_unknown_units() {
        assert_eq!(
            unit("wat").unwrap_err(),
            AbacusError::UnknownUnit("wat".to_string())
        );
        assert_eq!(
            unit("").unwrap_err(),
            AbacusError::UnknownUnit("".to_string())
        );
        assert_eq!(
            unit("KM").unwrap_err(),
            AbacusError::UnknownUnit("KM".to_string())
        );
    }

    #[test]
    fn global_units_is_initialized_once() {
        let first = global_units() as *const _;
        let second = global_units() as *const _;

        assert_eq!(first, second);
        assert!(!global_units().is_empty());
    }

    #[test]
    fn global_units_contains_expected_metric_units() {
        let units = global_units();

        assert!(units.contains("km"));
        assert!(units.contains("in"));
        assert!(units.contains("h"));
        assert!(units.contains("hour"));
    }

    #[test]
    fn repeated_unit_lookups_return_equivalent_units() {
        let first = unit("km").unwrap();
        let second = unit("km").unwrap();

        assert_eq!(first.display.render(), second.display.render());
        assert_eq!(first.dimensions, second.dimensions);
    }

    #[test]
    fn repeated_unit_lookups_share_the_same_unit() {
        let first = unit("km").unwrap();
        let second = unit("km").unwrap();

        assert!(std::sync::Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn constructs_values_from_amount_and_unit_symbol() {
        let distance = value(5.0, "km").unwrap();

        assert_eq!(distance.to_display(), "5 km");
        assert_eq!(distance.unit.display.render(), "km");
        assert_eq!(distance.unit.dimensions, Dimensions::LENGTH);
    }

    #[test]
    fn constructs_zero_values() {
        let distance = value(0.0, "km").unwrap();

        assert_eq!(distance.to_display(), "0 km");
    }

    #[test]
    fn constructs_negative_values() {
        let distance = value(-5.0, "km").unwrap();

        assert_eq!(distance.to_display(), "-5 km");
    }

    #[test]
    fn value_propagates_unknown_unit_errors() {
        assert_eq!(
            value(5.0, "wat").unwrap_err(),
            AbacusError::UnknownUnit("wat".to_string())
        );
    }

    #[test]
    fn divides_values_with_different_dimensions() {
        let distance = value(5.0, "km").unwrap();
        let duration = value(1.0, "h").unwrap();
        let speed = (distance / duration).unwrap();

        assert_eq!(speed.to_display(), "5 km/h");
        assert_eq!(speed.unit.dimensions, Dimensions::LENGTH - Dimensions::TIME);
    }

    #[test]
    fn division_preserves_fractional_amounts() {
        let distance = value(10.0, "km").unwrap();
        let duration = value(4.0, "h").unwrap();
        let speed = (distance / duration).unwrap();

        assert_eq!(speed.to_display(), "2.5 km/h");
    }

    #[test]
    fn division_by_zero_returns_infinity() {
        let distance = value(5.0, "km").unwrap();
        let zero = value(0.0, "h").unwrap();

        assert_eq!((distance / zero).unwrap().canonical, f64::INFINITY);
    }

    #[test]
    fn division_by_negative_value_is_supported() {
        let distance = value(5.0, "km").unwrap();
        let duration = value(-2.0, "h").unwrap();
        let speed = (distance / duration).unwrap();

        assert_eq!(speed.to_display(), "-2.5 km/h");
    }

    #[test]
    fn division_with_unknown_left_unit_fails() {
        let invalid = value(5.0, "wat");
        let duration = value(1.0, "h").unwrap();

        assert!(invalid.is_err());
        assert!(invalid.and_then(|distance| distance / duration).is_err());
    }

    #[test]
    fn division_with_unknown_right_unit_fails() {
        let distance = value(5.0, "km").unwrap();
        let invalid = value(1.0, "wat");

        assert!(invalid.is_err());
        assert!(invalid.and_then(|duration| distance / duration).is_err());
    }
}

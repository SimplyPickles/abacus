use std::sync::Arc;

use crate::units::date::Date;
use crate::units::hash::Hash;
use crate::{AbacusError, BinaryOp, Interval, UnaryOp, Unit, UnitRegistry, Value};

/// The result of evaluating an expression — a scalar `Value`, an `Interval`, a `Hash` map, or a `Date`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EvalResult {
    Scalar(Value),
    Interval(Interval),
    Hash(Hash),
    Date(Date),
}

impl EvalResult {
    /// Apply a binary operator across two `EvalResult`s, promoting scalars to
    /// degenerate intervals when mixing scalar and interval operands.
    pub fn apply_binary(self, op: &BinaryOp, other: EvalResult) -> Result<EvalResult, AbacusError> {
        match (self, other) {
            (EvalResult::Scalar(l), EvalResult::Scalar(r)) => {
                Ok(EvalResult::Scalar(op.apply(l, r)?))
            }
            (EvalResult::Interval(l), EvalResult::Interval(r)) => {
                Ok(EvalResult::Interval(l.apply_binary(op, &r)?))
            }
            (EvalResult::Scalar(l), EvalResult::Interval(r)) => {
                let l = Interval::from_value_with_style(l, r.style);
                Ok(EvalResult::Interval(l.apply_binary(op, &r)?))
            }
            (EvalResult::Interval(l), EvalResult::Scalar(r)) => {
                let r = Interval::from_value_with_style(r, l.style);
                Ok(EvalResult::Interval(l.apply_binary(op, &r)?))
            }
            (EvalResult::Date(l), EvalResult::Date(r)) => {
                if op.alias == "-" {
                    Ok(EvalResult::Scalar(&l - &r))
                } else {
                    Err(AbacusError::IncompatibleOperatorType(format!(
                        "cannot perform operator '{}' on dates",
                        op.alias
                    )))
                }
            }
            (EvalResult::Date(l), EvalResult::Scalar(r)) => {
                if op.alias == "+" {
                    Ok(EvalResult::Date((&l + &r)?))
                } else if op.alias == "-" {
                    Ok(EvalResult::Date((&l - &r)?))
                } else {
                    Err(AbacusError::IncompatibleOperatorType(format!(
                        "cannot perform operator '{}' on date and scalar",
                        op.alias
                    )))
                }
            }
            (EvalResult::Scalar(l), EvalResult::Date(r)) => {
                if op.alias == "+" {
                    Ok(EvalResult::Date((&r + &l)?))
                } else {
                    Err(AbacusError::IncompatibleOperatorType(format!(
                        "cannot perform operator '{}' on scalar and date",
                        op.alias
                    )))
                }
            }
            _ => Err(AbacusError::IncompatibleOperatorType(
                "cannot perform arithmetic on hash or mismatched result types".to_string(),
            )),
        }
    }

    /// Apply a unary operator to an `EvalResult`.
    pub fn apply_unary(self, op: &UnaryOp) -> Result<EvalResult, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(EvalResult::Scalar(op.apply(v)?)),
            EvalResult::Interval(i) => Ok(EvalResult::Interval(i.apply_unary(op)?)),
            EvalResult::Hash(_) => Err(AbacusError::IncompatibleOperatorType(
                "cannot perform unary operation on hash result".to_string(),
            )),
            EvalResult::Date(_) => Err(AbacusError::IncompatibleOperatorType(
                "cannot perform unary operation on date result".to_string(),
            )),
        }
    }

    /// Convert the result to a target unit.
    pub fn convert_to(self, unit: Arc<Unit>) -> Result<EvalResult, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(EvalResult::Scalar(v.convert_to(unit)?)),
            EvalResult::Interval(i) => Ok(EvalResult::Interval(i.convert_to(unit)?)),
            EvalResult::Hash(_) => Err(AbacusError::IncompatibleWithConversion("hash".to_string())),
            EvalResult::Date(_) => Err(AbacusError::IncompatibleWithConversion("date".to_string())),
        }
    }

    /// Attempt automatic derived-unit reduction.
    pub fn to_derived(self, registry: &UnitRegistry) -> Result<EvalResult, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(EvalResult::Scalar(v.to_derived(registry)?)),
            EvalResult::Interval(i) => Ok(EvalResult::Interval(i.to_derived(registry)?)),
            EvalResult::Date(d) => Ok(EvalResult::Date(d)),
            EvalResult::Hash(h) => {
                let mut new_hash = Hash::new();
                for (k, v) in h.values {
                    new_hash.insert(k, v.to_derived(registry)?);
                }
                Ok(EvalResult::Hash(new_hash))
            }
        }
    }

    /// Simplify unit display.
    pub fn simplify_unit_display(&mut self, registry: &UnitRegistry) {
        match self {
            EvalResult::Scalar(v) => v.simplify_unit_display(registry),
            EvalResult::Interval(i) => i.simplify_unit_display(registry),
            EvalResult::Date(_) => {}
            EvalResult::Hash(h) => {
                for v in h.values.values_mut() {
                    v.simplify_unit_display(registry);
                }
            }
        }
    }

    /// Render to display string.
    #[must_use]
    pub fn to_display(&self) -> String {
        match self {
            EvalResult::Scalar(v) => v.to_display(),
            EvalResult::Interval(i) => i.to_display(),
            EvalResult::Hash(h) => h.to_display(),
            EvalResult::Date(d) => d.to_string(),
        }
    }

    /// Returns a new `EvalResult` with scalar and interval components rounded to `sig_figs` significant figures.
    #[must_use]
    pub fn round_to_sig_figs(self, sig_figs: usize) -> Self {
        match self {
            EvalResult::Scalar(v) => EvalResult::Scalar(v.round_to_sig_figs(sig_figs)),
            EvalResult::Interval(i) => EvalResult::Interval(i.round_to_sig_figs(sig_figs)),
            EvalResult::Hash(h) => EvalResult::Hash(h.round_to_sig_figs(sig_figs)),
            EvalResult::Date(d) => EvalResult::Date(d),
        }
    }

    /// Render to display string formatted to `sig_figs` significant figures.
    #[must_use]
    pub fn to_display_with_sig_figs(&self, sig_figs: usize) -> String {
        match self {
            EvalResult::Scalar(v) => v.to_display_with_sig_figs(sig_figs),
            EvalResult::Interval(i) => i.to_display_with_sig_figs(sig_figs),
            EvalResult::Hash(h) => h.to_display_with_sig_figs(sig_figs),
            EvalResult::Date(d) => d.to_string(),
        }
    }

    /// Borrow the scalar Value, or error if this is not a scalar.
    pub fn as_scalar(&self) -> Result<&Value, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(v),
            EvalResult::Interval(_) => Err(AbacusError::IntervalInFunction),
            EvalResult::Hash(_) => Err(AbacusError::UnexpectedToken(
                "hash result where scalar expected".to_string(),
            )),
            EvalResult::Date(_) => Err(AbacusError::UnexpectedToken(
                "date result where scalar expected".to_string(),
            )),
        }
    }

    /// Extract the scalar Value, or error if this is an Interval, Hash, or Date.
    pub fn into_scalar(self) -> Result<Value, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(v),
            EvalResult::Interval(_) => Err(AbacusError::UnexpectedToken(
                "interval result where scalar expected".to_string(),
            )),
            EvalResult::Hash(_) => Err(AbacusError::UnexpectedToken(
                "hash result where scalar expected".to_string(),
            )),
            EvalResult::Date(_) => Err(AbacusError::UnexpectedToken(
                "date result where scalar expected".to_string(),
            )),
        }
    }

    /// Extract the `Hash`, or error if this is not a Hash.
    pub fn into_hash(self) -> Result<Hash, AbacusError> {
        match self {
            EvalResult::Hash(h) => Ok(h),
            _ => Err(AbacusError::UnexpectedToken("hash expected".to_string())),
        }
    }

    /// Extract the `Date`, or error if this is not a Date.
    pub fn into_date(self) -> Result<Date, AbacusError> {
        match self {
            EvalResult::Date(d) => Ok(d),
            _ => Err(AbacusError::UnexpectedToken("date expected".to_string())),
        }
    }

    /// Get the unit from the result (for intervals, uses the `lo` endpoint's unit).
    #[must_use]
    pub fn unit(&self) -> &Arc<Unit> {
        match self {
            EvalResult::Scalar(v) => &v.unit,
            EvalResult::Interval(i) => &i.lo.unit,
            EvalResult::Date(_) | EvalResult::Hash(_) => Unit::dimensionless_arc_ref(),
        }
    }
}

impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

use std::sync::Arc;

use crate::error::AbacusError;
use crate::registry::UnitRegistry;
use crate::units::unit::Unit;
use crate::units::value::Value;

use crate::evaluation::tokenizer::registry::binary::operators::BinaryOp;
use crate::evaluation::tokenizer::registry::unary::operators::UnaryOp;

/// A physical interval representing a guaranteed range `[lo, hi]` where both
/// endpoints carry physical units. Used for worst-case/best-case analysis in
/// engineering and safety-critical computations.
#[derive(Debug, Clone)]
pub struct Interval {
    pub lo: Value,
    pub hi: Value,
}

impl Interval {
    /// Create a new interval from two `Value` endpoints.
    /// Validates dimension compatibility and normalizes so `lo.canonical <= hi.canonical`.
    pub fn new(a: Value, b: Value) -> Result<Self, AbacusError> {
        if !a.unit.is_compatible_with(&b.unit) {
            return Err(AbacusError::IncompatibleDimensions);
        }
        if a.canonical <= b.canonical {
            Ok(Self { lo: a, hi: b })
        } else {
            Ok(Self { lo: b, hi: a })
        }
    }

    /// Promote a single scalar `Value` to a degenerate interval `[v, v]`.
    pub fn from_value(v: Value) -> Self {
        Self {
            lo: v.clone(),
            hi: v,
        }
    }

    /// Apply a binary operator across two intervals by evaluating all four corner
    /// combinations and taking the min/max of the results.
    ///
    /// For monotonic operations (e.g. addition) this is exact.
    /// For non-monotonic operations this is conservative (may overestimate the interval).
    pub fn apply_binary(&self, op: &BinaryOp, other: &Interval) -> Result<Interval, AbacusError> {
        let corners = [
            op.apply(self.lo.clone(), other.lo.clone())?,
            op.apply(self.lo.clone(), other.hi.clone())?,
            op.apply(self.hi.clone(), other.lo.clone())?,
            op.apply(self.hi.clone(), other.hi.clone())?,
        ];

        let lo = corners
            .iter()
            .min_by(|a, b| {
                a.canonical
                    .partial_cmp(&b.canonical)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone();

        let hi = corners
            .iter()
            .max_by(|a, b| {
                a.canonical
                    .partial_cmp(&b.canonical)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone();

        Ok(Interval { lo, hi })
    }

    /// Apply a unary operator to an interval by evaluating both endpoints
    /// and normalizing the result.
    pub fn apply_unary(&self, op: &UnaryOp) -> Result<Interval, AbacusError> {
        let a = op.apply(self.lo.clone())?;
        let b = op.apply(self.hi.clone())?;

        if a.canonical <= b.canonical {
            Ok(Interval { lo: a, hi: b })
        } else {
            Ok(Interval { lo: b, hi: a })
        }
    }

    /// Convert both endpoints to a target unit.
    pub fn convert_to(&self, unit: Arc<Unit>) -> Result<Interval, AbacusError> {
        let lo = self.lo.convert_to(Arc::clone(&unit))?;
        let hi = self.hi.convert_to(unit)?;
        if lo.canonical <= hi.canonical {
            Ok(Interval { lo, hi })
        } else {
            Ok(Interval { lo: hi, hi: lo })
        }
    }

    /// Attempt automatic derived-unit reduction on both endpoints.
    pub fn to_derived(&self, registry: &UnitRegistry) -> Result<Interval, AbacusError> {
        let lo = self.lo.to_derived(registry)?;
        let hi = self.hi.to_derived(registry)?;
        Ok(Interval { lo, hi })
    }

    /// Simplify the unit display on both endpoints.
    pub fn simplify_unit_display(&mut self, registry: &UnitRegistry) {
        self.lo.simplify_unit_display(registry);
        self.hi.simplify_unit_display(registry);
    }

    /// Render the interval as `[lo_display, hi_display]`.
    pub fn to_display(&self) -> String {
        format!("[{}, {}]", self.lo.to_display(), self.hi.to_display())
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.lo == other.lo && self.hi == other.hi
    }
}

use crate::units::date::Date;
use crate::units::hash::Hash;

/// The result of evaluating an expression — a scalar `Value`, an `Interval`, a `Hash` map, or a `Date`.
#[derive(Debug, Clone, PartialEq)]
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
                let l = Interval::from_value(l);
                Ok(EvalResult::Interval(l.apply_binary(op, &r)?))
            }
            (EvalResult::Interval(l), EvalResult::Scalar(r)) => {
                let r = Interval::from_value(r);
                Ok(EvalResult::Interval(l.apply_binary(op, &r)?))
            }
            (EvalResult::Date(l), EvalResult::Date(r)) => {
                if op.alias == "-" {
                    Ok(EvalResult::Scalar(&l - &r))
                } else {
                    Err(AbacusError::UnexpectedToken(format!(
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
                    Err(AbacusError::UnexpectedToken(format!(
                        "cannot perform operator '{}' on date and scalar",
                        op.alias
                    )))
                }
            }
            (EvalResult::Scalar(l), EvalResult::Date(r)) => {
                if op.alias == "+" {
                    Ok(EvalResult::Date((&r + &l)?))
                } else {
                    Err(AbacusError::UnexpectedToken(format!(
                        "cannot perform operator '{}' on scalar and date",
                        op.alias
                    )))
                }
            }
            _ => Err(AbacusError::UnexpectedToken(
                "cannot perform arithmetic on hash or mismatched result types".to_string(),
            )),
        }
    }

    /// Apply a unary operator to an `EvalResult`.
    pub fn apply_unary(self, op: &UnaryOp) -> Result<EvalResult, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(EvalResult::Scalar(op.apply(v)?)),
            EvalResult::Interval(i) => Ok(EvalResult::Interval(i.apply_unary(op)?)),
            EvalResult::Hash(_) => Err(AbacusError::UnexpectedToken(
                "cannot perform unary operation on hash result".to_string(),
            )),
            EvalResult::Date(_) => Err(AbacusError::UnexpectedToken(
                "cannot perform unary operation on date result".to_string(),
            )),
        }
    }

    /// Convert the result to a target unit.
    pub fn convert_to(self, unit: Arc<Unit>) -> Result<EvalResult, AbacusError> {
        match self {
            EvalResult::Scalar(v) => Ok(EvalResult::Scalar(v.convert_to(unit)?)),
            EvalResult::Interval(i) => Ok(EvalResult::Interval(i.convert_to(unit)?)),
            EvalResult::Hash(_) => Err(AbacusError::UnexpectedToken(
                "cannot convert unit on hash result".to_string(),
            )),
            EvalResult::Date(_) => Err(AbacusError::UnexpectedToken(
                "cannot convert unit on date result".to_string(),
            )),
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
    pub fn to_display(&self) -> String {
        match self {
            EvalResult::Scalar(v) => v.to_display(),
            EvalResult::Interval(i) => i.to_display(),
            EvalResult::Hash(h) => h.to_display(),
            EvalResult::Date(d) => d.to_string(),
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

    /// Extract the Hash, or error if this is not a Hash.
    pub fn into_hash(self) -> Result<Hash, AbacusError> {
        match self {
            EvalResult::Hash(h) => Ok(h),
            _ => Err(AbacusError::UnexpectedToken(
                "hash expected".to_string(),
            )),
        }
    }

    /// Extract the Date, or error if this is not a Date.
    pub fn into_date(self) -> Result<Date, AbacusError> {
        match self {
            EvalResult::Date(d) => Ok(d),
            _ => Err(AbacusError::UnexpectedToken(
                "date expected".to_string(),
            )),
        }
    }

    /// Get the unit from the result (for intervals, uses the lo endpoint's unit).
    pub fn unit(&self) -> &Arc<Unit> {
        match self {
            EvalResult::Scalar(v) => &v.unit,
            EvalResult::Interval(i) => &i.lo.unit,
            EvalResult::Date(_) | EvalResult::Hash(_) => {
                static DIMENSIONLESS: std::sync::OnceLock<Arc<Unit>> = std::sync::OnceLock::new();
                DIMENSIONLESS.get_or_init(|| {
                    use crate::units::{dimensions::Dimensions, unit::UnitExpr};
                    Arc::new(Unit {
                        scalar: 1.0,
                        offset: 0.0,
                        dimensions: Dimensions::DIMENSIONLESS,
                        display: UnitExpr::dimensionless(),
                    })
                })
            }
        }
    }
}

impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

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
///
/// Ex. `[-25, 25] + 10 = [-15, 35]`
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

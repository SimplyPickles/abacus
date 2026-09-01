use crate::registry::UnitRegistry;
use crate::{error::AbacusError, units::unit::Unit};

use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    sync::Arc,
};

/// A dimensional quantity consisting of a canonical SI scalar and an associated physical unit.
///
/// # Examples
///
/// ```rust
/// use abacus::Abacus;
///
/// let abacus = Abacus::standard();
/// let length = abacus.units.value(5.0, "km").unwrap();
/// assert_eq!(length.to_display(), "5 km");
/// assert_eq!(length.canonical, 5000.0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value {
    pub canonical: f64,
    pub unit: Arc<Unit>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.unit.dimensions == other.unit.dimensions
            && (self.canonical - other.canonical).abs() <= 1e-12 * self.canonical.abs().max(1.0)
    }
}

#[allow(dead_code)]
impl Value {
    #[must_use]
    pub fn new(value: f64, unit: Arc<Unit>) -> Self {
        Self {
            canonical: value * unit.scalar + unit.offset,
            unit,
        }
    }

    /// Creates a dimensionless `Value` with the given scalar.
    #[must_use]
    pub fn dimensionless(val: f64) -> Self {
        Self {
            canonical: val,
            unit: Unit::dimensionless_arc(),
        }
    }

    /// Returns the display-unit amount: `(canonical − offset) / scalar`.
    #[inline]
    #[must_use]
    pub fn amount(&self) -> f64 {
        (self.canonical - self.unit.offset) / self.unit.scalar
    }

    /// Constructs a `Value` directly from a pre-computed canonical value.
    #[inline]
    #[must_use]
    pub fn from_canonical(canonical: f64, unit: Arc<Unit>) -> Self {
        Self { canonical, unit }
    }

    pub fn convert_to(&self, unit: Arc<Unit>) -> Result<Self, AbacusError> {
        if !self.unit.is_compatible_with(&unit) {
            if self.unit.is_dimensionless() {
                let amount = self.amount();
                return Ok(Self::new(amount, unit));
            }
            return Err(AbacusError::IncompatibleDimensions);
        }

        Ok(Self {
            canonical: self.canonical,
            unit,
        })
    }

    pub fn to(&self, registry: &UnitRegistry, symbol: &str) -> Result<Self, AbacusError> {
        self.convert_to(registry.unit(symbol)?)
    }

    pub fn as_unit(&self, registry: &UnitRegistry, symbol: &str) -> Result<Self, AbacusError> {
        self.to(registry, symbol)
    }

    pub fn to_derived(&self, registry: &UnitRegistry) -> Result<Self, AbacusError> {
        if let Some(derived_unit) = registry.find_unit_by_dimensions(&self.unit.dimensions) {
            self.convert_to(derived_unit)
        } else {
            Ok(self.clone())
        }
    }

    #[must_use]
    pub fn to_display(&self) -> String {
        let value = self.amount();
        let nearest_integer = value.round();
        let display_value = if value.is_finite()
            && (value - nearest_integer).abs() <= 1e-12 * value.abs().max(1.0)
        {
            nearest_integer
        } else {
            value
        };

        let unit_str = self.unit.display.render();
        if unit_str.is_empty() {
            display_value.to_string()
        } else {
            format!("{display_value} {unit_str}")
        }
    }

    #[must_use]
    pub fn to_units_display(&self) -> String {
        self.unit.display.render()
    }

    pub fn simplify_unit_display(&mut self, unit_registry: &UnitRegistry) {
        let unit = self
            .unit
            .simplify_display_with(|sym| unit_registry.get(sym));
        self.unit = Arc::new(unit);
    }

    #[must_use]
    pub fn to_human_display(&self) -> String {
        if self.unit.dimensions == crate::units::dimensions::Dimensions::TIME {
            format_human_duration(self.canonical)
        } else {
            self.to_display()
        }
    }
}

#[must_use]
pub fn format_human_duration(seconds_f64: f64) -> String {
    let is_negative = seconds_f64 < 0.0;
    let mut total = seconds_f64.abs().round() as i64;
    if total == 0 {
        return "0 seconds".to_string();
    }

    const DURATION_UNITS: &[(&str, i64)] = &[
        ("year", 31_536_000),
        ("day", 86_400),
        ("hour", 3_600),
        ("minute", 60),
        ("second", 1),
    ];

    let mut parts = Vec::new();
    for &(name, divisor) in DURATION_UNITS {
        let count = total / divisor;
        total %= divisor;
        if count > 0 {
            parts.push(format!(
                "{count} {name}{suffix}",
                suffix = if count == 1 { "" } else { "s" }
            ));
        }
    }

    let result = parts.join(", ");
    if is_negative {
        format!("-{result}")
    } else {
        result
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

// Add implementations
impl Add<&Value> for &Value {
    type Output = Result<Value, AbacusError>;

    fn add(self, rhs: &Value) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(AbacusError::AffineUnitOperation("add"));
        }

        if rhs.unit.is_percent() && !self.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical * (1.0 + rhs.canonical),
                unit: Arc::clone(&self.unit),
            });
        }
        if self.unit.is_percent() && !rhs.unit.is_percent() {
            return Ok(Value {
                canonical: rhs.canonical * (1.0 + self.canonical),
                unit: Arc::clone(&rhs.unit),
            });
        }

        if !self.unit.is_compatible_with(&rhs.unit) {
            if rhs.unit.is_dimensionless() && !self.unit.is_dimensionless() {
                let rhs_amount = rhs.amount();
                let rhs_promoted = Value::new(rhs_amount, Arc::clone(&self.unit));
                return Ok(Value {
                    canonical: self.canonical + rhs_promoted.canonical,
                    unit: Arc::clone(&self.unit),
                });
            } else if self.unit.is_dimensionless() && !rhs.unit.is_dimensionless() {
                let self_amount = self.amount();
                let self_promoted = Value::new(self_amount, Arc::clone(&rhs.unit));
                return Ok(Value {
                    canonical: self_promoted.canonical + rhs.canonical,
                    unit: Arc::clone(&rhs.unit),
                });
            }
            return Err(AbacusError::IncompatibleDimensions);
        }

        Ok(Value {
            canonical: self.canonical + rhs.canonical,
            unit: Arc::clone(&self.unit),
        })
    }
}

// Sub implementations
impl Sub<&Value> for &Value {
    type Output = Result<Value, AbacusError>;

    fn sub(self, rhs: &Value) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(AbacusError::AffineUnitOperation("subtract"));
        }

        if rhs.unit.is_percent() && !self.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical * (1.0 - rhs.canonical),
                unit: Arc::clone(&self.unit),
            });
        }

        if !self.unit.is_compatible_with(&rhs.unit) {
            if rhs.unit.is_dimensionless() && !self.unit.is_dimensionless() {
                let rhs_amount = rhs.amount();
                let rhs_promoted = Value::new(rhs_amount, Arc::clone(&self.unit));
                return Ok(Value {
                    canonical: self.canonical - rhs_promoted.canonical,
                    unit: Arc::clone(&self.unit),
                });
            } else if self.unit.is_dimensionless() && !rhs.unit.is_dimensionless() {
                let self_amount = self.amount();
                let self_promoted = Value::new(self_amount, Arc::clone(&rhs.unit));
                return Ok(Value {
                    canonical: self_promoted.canonical - rhs.canonical,
                    unit: Arc::clone(&rhs.unit),
                });
            }
            return Err(AbacusError::IncompatibleDimensions);
        }

        Ok(Value {
            canonical: self.canonical - rhs.canonical,
            unit: Arc::clone(&self.unit),
        })
    }
}

// Mul implementations
impl Mul<&Value> for &Value {
    type Output = Result<Value, AbacusError>;

    fn mul(self, rhs: &Value) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(AbacusError::AffineUnitOperation("multiply"));
        }

        if self.unit.is_percent() && !rhs.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical * rhs.canonical,
                unit: Arc::clone(&rhs.unit),
            });
        }
        if rhs.unit.is_percent() && !self.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical * rhs.canonical,
                unit: Arc::clone(&self.unit),
            });
        }

        let unit = Unit {
            scalar: self.unit.scalar * rhs.unit.scalar,
            offset: 0.0,
            dimensions: self.unit.dimensions + rhs.unit.dimensions,
            display: self.unit.display.multiply(&rhs.unit.display),
        };

        Ok(Value {
            canonical: self.canonical * rhs.canonical,
            unit: Arc::new(unit),
        })
    }
}

// Div implementations
impl Div<&Value> for &Value {
    type Output = Result<Value, AbacusError>;

    fn div(self, rhs: &Value) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(AbacusError::AffineUnitOperation("divide"));
        }

        if rhs.unit.is_percent() && !self.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical / rhs.canonical,
                unit: Arc::clone(&self.unit),
            });
        }
        if self.unit.is_percent() && !rhs.unit.is_percent() {
            return Ok(Value {
                canonical: self.canonical / rhs.canonical,
                unit: Arc::clone(&self.unit),
            });
        }

        let unit = Unit {
            scalar: self.unit.scalar / rhs.unit.scalar,
            offset: 0.0,
            dimensions: self.unit.dimensions - rhs.unit.dimensions,
            display: self.unit.display.divide(&rhs.unit.display),
        };

        Ok(Value {
            canonical: self.canonical / rhs.canonical,
            unit: Arc::new(unit),
        })
    }
}

/// Generates the three ownership-coercing forwarding impls for a binary operator whose
/// canonical implementation is `&Value op &Value`.
macro_rules! impl_op_forwarding {
    ($trait:ident, $method:ident) => {
        impl $trait<Value> for Value {
            type Output = Result<Value, AbacusError>;
            fn $method(self, rhs: Self) -> Self::Output {
                <&Value as $trait<&Value>>::$method(&self, &rhs)
            }
        }
        impl $trait<&Value> for Value {
            type Output = Result<Value, AbacusError>;
            fn $method(self, rhs: &Value) -> Self::Output {
                <&Value as $trait<&Value>>::$method(&self, rhs)
            }
        }
        impl $trait<Value> for &Value {
            type Output = Result<Value, AbacusError>;
            fn $method(self, rhs: Value) -> Self::Output {
                <&Value as $trait<&Value>>::$method(self, &rhs)
            }
        }
    };
}

impl_op_forwarding!(Add, add);
impl_op_forwarding!(Sub, sub);
impl_op_forwarding!(Mul, mul);
impl_op_forwarding!(Div, div);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::{dimensions::Dimensions, unit::UnitExpr};

    fn unit(scalar: f64, dimensions: Dimensions, display: &str) -> Arc<Unit> {
        Arc::new(Unit {
            scalar,
            dimensions,
            offset: 0f64,
            display: UnitExpr::single(display),
        })
    }

    #[test]
    fn converts_display_values_to_canonical_values() {
        let distance = Value::new(5.0, unit(1_000.0, Dimensions::LENGTH, "km"));
        let duration = Value::new(1.0, unit(3_600.0, Dimensions::TIME, "h"));

        assert_eq!(distance.canonical, 5_000.0);
        assert_eq!(duration.canonical, 3_600.0);
    }

    #[test]
    fn divides_values_and_units() {
        let distance = Value::new(5.0, unit(1_000.0, Dimensions::LENGTH, "km"));
        let duration = Value::new(1.0, unit(3_600.0, Dimensions::TIME, "h"));
        let speed = (distance / duration).unwrap();

        assert!((speed.canonical - 5_000.0 / 3_600.0).abs() < f64::EPSILON);
        assert!((speed.unit.scalar - 1_000.0 / 3_600.0).abs() < f64::EPSILON);
        assert_eq!(speed.unit.dimensions, Dimensions::LENGTH - Dimensions::TIME);
        assert_eq!(speed.to_display(), "5 km/h");
    }

    #[test]
    fn adds_and_subtracts_only_compatible_values() {
        let sum = (Value::new(2.0, unit(1_000.0, Dimensions::LENGTH, "km"))
            + Value::new(500.0, unit(1.0, Dimensions::LENGTH, "m")))
        .unwrap();
        assert_eq!(sum.to_display(), "2.5 km");

        let difference = (sum - Value::new(1.0, unit(1_000.0, Dimensions::LENGTH, "km"))).unwrap();
        assert_eq!(difference.to_display(), "1.5 km");

        let incompatible = Value::new(1.0, unit(1.0, Dimensions::LENGTH, "m"))
            + Value::new(1.0, unit(1.0, Dimensions::TIME, "s"));
        assert!(incompatible.is_err());
    }

    #[test]
    fn multiplies_values_with_different_dimensions() {
        let area = (Value::new(2.0, unit(1.0, Dimensions::LENGTH, "m"))
            * Value::new(3.0, unit(1.0, Dimensions::LENGTH, "m")))
        .unwrap();

        assert_eq!(area.canonical, 6.0);
        assert_eq!(
            area.unit.dimensions,
            Dimensions::LENGTH + Dimensions::LENGTH
        );
        assert_eq!(area.to_display(), "6 m^2");
    }

    #[test]
    fn simplifies_compound_unit_displays() {
        let speed = (Value::new(5.0, unit(1.0, Dimensions::LENGTH, "m"))
            / Value::new(1.0, unit(1.0, Dimensions::TIME, "s")))
        .unwrap();
        assert_eq!(speed.to_display(), "5 m/s");

        let mut distance = (speed * Value::new(5.0, unit(1.0, Dimensions::TIME, "s"))).unwrap();
        let std_unit_reg = UnitRegistry::standard();
        distance.simplify_unit_display(&std_unit_reg);

        assert_eq!(distance.canonical, 25.0);
        assert_eq!(distance.unit.dimensions, Dimensions::LENGTH);
        assert_eq!(distance.to_display(), "25 m");
    }

    #[test]
    fn supports_reference_arithmetic() {
        let a = Value::new(5.0, unit(1_000.0, Dimensions::LENGTH, "km"));
        let b = Value::new(500.0, unit(1.0, Dimensions::LENGTH, "m"));

        let sum = (&a + &b).unwrap();
        assert_eq!(sum.to_display(), "5.5 km");
        // Verify a and b are not consumed
        assert_eq!(a.to_display(), "5 km");
        assert_eq!(b.to_display(), "500 m");
    }

    #[test]
    fn supports_partial_eq_and_display() {
        let a = Value::new(5.0, unit(1_000.0, Dimensions::LENGTH, "km"));
        let b = Value::new(5000.0, unit(1.0, Dimensions::LENGTH, "m"));

        assert_eq!(a, b);
        assert_eq!(format!("{a}"), "5 km");
    }

    #[test]
    fn automatically_converts_to_matching_derived_units() {
        let registry = UnitRegistry::standard();

        let mass = registry.value(2.0, "kg").unwrap();
        let accel = (registry.value(9.8, "m").unwrap()
            / (registry.value(1.0, "s").unwrap() * registry.value(1.0, "s").unwrap()).unwrap())
        .unwrap();

        let force = (&mass * &accel).unwrap();
        let force_derived = force.to_derived(&registry).unwrap();
        assert_eq!(force_derived.to_display(), "19.6 N");

        let distance = registry.value(5.0, "m").unwrap();
        let work = (&force_derived * &distance).unwrap();
        let work_derived = work.to_derived(&registry).unwrap();
        assert_eq!(work_derived.to_display(), "98 J");
    }

    #[test]
    fn converts_dimensionless_values_to_target_units() {
        let registry = UnitRegistry::standard();
        let dimless = Value::new(10.0, Arc::new(Unit::dimensionless()));
        let converted = dimless.to(&registry, "m").unwrap();
        assert_eq!(converted.to_display(), "10 m");
    }
}

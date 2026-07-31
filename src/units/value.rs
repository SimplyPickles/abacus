use crate::units::unit::Unit;

use std::{
    ops::{Add, Div, Mul, Sub},
    sync::Arc,
};

#[derive(Debug)]
pub struct Value {
    pub canonical: f64,
    pub unit: Arc<Unit>,
}

#[allow(dead_code)]
impl Value {
    pub fn new(value: f64, unit: Arc<Unit>) -> Self {
        Self {
            canonical: value * unit.scalar + unit.offset,
            unit,
        }
    }

    pub fn convert_to(&self, unit: Arc<Unit>) -> Result<Self, String> {
        if !self.unit.is_compatible_with(&unit) {
            return Err(String::from(
                "cannot convert between units with incompatible dimensions",
            ));
        }

        Ok(Self {
            canonical: self.canonical,
            unit,
        })
    }

    pub fn to_display(&self) -> String {
        let value = (self.canonical - self.unit.offset) / self.unit.scalar;
        let nearest_integer = value.round();
        let display_value = if (value - nearest_integer).abs() < 1e-12 {
            nearest_integer
        } else {
            value
        };

        display_value.to_string() + &self.unit.display.render()
    }
}

impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(String::from("cannot add affine units"));
        }

        if !self.unit.is_compatible_with(&rhs.unit) {
            return Err(String::from(
                "cannot add values with incompatible dimensions",
            ));
        }

        Ok(Value {
            canonical: self.canonical + rhs.canonical,
            unit: self.unit,
        })
    }
}

impl Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(String::from("cannot subtract affine units"));
        }

        if !self.unit.is_compatible_with(&rhs.unit) {
            return Err(String::from(
                "cannot subtract values with incompatible dimensions",
            ));
        }

        Ok(Value {
            canonical: self.canonical - rhs.canonical,
            unit: self.unit,
        })
    }
}

impl Mul for Value {
    type Output = Result<Value, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(String::from("cannot multiply affine units"));
        }

        let unit = Arc::new(Unit {
            scalar: self.unit.scalar * rhs.unit.scalar,
            offset: 0.0,
            dimensions: self.unit.dimensions + rhs.unit.dimensions,
            display: self.unit.display.multiply(&rhs.unit.display),
        });

        Ok(Value {
            canonical: self.canonical * rhs.canonical,
            unit,
        })
    }
}

impl Div for Value {
    type Output = Result<Value, String>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.unit.is_affine() || rhs.unit.is_affine() {
            return Err(String::from("cannot divide affine units"));
        }

        let unit = Arc::new(Unit {
            scalar: self.unit.scalar / rhs.unit.scalar,
            offset: 0.0,
            dimensions: self.unit.dimensions - rhs.unit.dimensions,
            display: self.unit.display.divide(&rhs.unit.display),
        });

        Ok(Value {
            canonical: self.canonical / rhs.canonical,
            unit,
        })
    }
}

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
        assert_eq!(speed.to_display(), "5km/h");
    }

    #[test]
    fn adds_and_subtracts_only_compatible_values() {
        let sum = (Value::new(2.0, unit(1_000.0, Dimensions::LENGTH, "km"))
            + Value::new(500.0, unit(1.0, Dimensions::LENGTH, "m")))
        .unwrap();
        assert_eq!(sum.to_display(), "2.5km");

        let difference = (sum - Value::new(1.0, unit(1_000.0, Dimensions::LENGTH, "km"))).unwrap();
        assert_eq!(difference.to_display(), "1.5km");

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
        assert_eq!(area.to_display(), "6m^2");
    }

    #[test]
    fn simplifies_compound_unit_displays() {
        let speed = (Value::new(5.0, unit(1.0, Dimensions::LENGTH, "m"))
            / Value::new(1.0, unit(1.0, Dimensions::TIME, "s")))
        .unwrap();
        assert_eq!(speed.to_display(), "5m/s");

        let distance = (speed * Value::new(5.0, unit(1.0, Dimensions::TIME, "s"))).unwrap();
        assert_eq!(distance.canonical, 25.0);
        assert_eq!(distance.unit.dimensions, Dimensions::LENGTH);
        assert_eq!(distance.to_display(), "25m");
    }
}

use crate::{
    registry::unit_registry::UnitRegistry,
    units::{unit::Unit, value::Value},
};

mod error;
mod registry;
mod units;

pub use error::AbacusError;
use std::sync::OnceLock;

static UNITS: OnceLock<UnitRegistry> = OnceLock::new();

pub fn global_units() -> &'static UnitRegistry {
    UNITS.get_or_init(|| UnitRegistry::standard())
}

pub fn unit(symbol: &str) -> Result<std::sync::Arc<Unit>, AbacusError> {
    global_units().unit(symbol)
}

pub fn value(amount: f64, symbol: &str) -> Result<Value, AbacusError> {
    Ok(Value::new(amount, unit(symbol)?))
}

fn main() -> Result<(), AbacusError> {
    let registry = UnitRegistry::standard();

    let speed = registry.value(5.0, "m")? / registry.value(1.0, "s")?;

    let distance = (speed? * registry.value(5.0, "s")?)?;

    let mass = registry.value(2.0, "kg")?;
    let accel =
        registry.value(9.8, "m")? / (registry.value(1.0, "s")? * registry.value(1.0, "s")?)?;
    let force = (mass * accel?)?;
    // Automatic formula-to-derived-unit conversion: kg*m/s^2 -> N
    let force_derived = force.to_derived(&registry)?;

    // Volume addition and conversion using &Value reference arithmetic
    let barrels = registry.value(1.0, "bbl")?;
    let liters = registry.value(100.0, "L")?;
    let total_volume = (&barrels + &liters)?;
    let volume_m3 = total_volume.to(&registry, "m^3")?;

    println!("Distance: {distance}");
    println!("Force (auto-derived): {force_derived}");
    println!("1 bbl ({barrels}) + 100 L ({liters}) in m^3: {volume_m3}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::dimensions::Dimensions;
    use std::sync::Arc;

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

        assert!(Arc::ptr_eq(&first, &second));
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

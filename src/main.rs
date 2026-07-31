use crate::{
    registry::metric_units::register_metric_units,
    units::{unit::Unit, value::Value},
};

mod registry;
mod units;

use std::{collections::HashMap, sync::OnceLock};

static METRIC_UNITS: OnceLock<HashMap<String, std::sync::Arc<Unit>>> = OnceLock::new();

pub fn metric_units() -> &'static HashMap<String, std::sync::Arc<Unit>> {
    METRIC_UNITS.get_or_init(register_metric_units)
}

pub fn unit(symbol: &str) -> Result<std::sync::Arc<Unit>, String> {
    metric_units()
        .get(symbol)
        .cloned()
        .ok_or_else(|| format!("unknown unit: {symbol}"))
}

pub fn value(amount: f64, symbol: &str) -> Result<Value, String> {
    Ok(Value::new(amount, unit(symbol)?))
}

fn main() -> Result<(), String> {
    let speed = (value(5.0, "km")? / value(1.0, "h")?)?;

    println!("5km / 1h");
    println!("{}", speed.to_display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::dimensions::Dimensions;

    #[test]
    fn looks_up_units_by_symbol_or_name() {
        assert_eq!(unit("km").unwrap().display.render(), "km");
        assert_eq!(unit("hour").unwrap().display.render(), "h");
    }

    #[test]
    fn returns_an_error_for_unknown_units() {
        assert_eq!(unit("wat").unwrap_err(), "unknown unit: wat");
    }

    #[test]
    fn constructs_values_from_amount_and_unit_symbol() {
        let distance = value(5.0, "km").unwrap();
        let duration = value(1.0, "h").unwrap();
        let speed = (distance / duration).unwrap();

        assert_eq!(speed.to_display(), "5km/h");
        assert_eq!(speed.unit.dimensions, Dimensions::LENGTH - Dimensions::TIME);
    }
}

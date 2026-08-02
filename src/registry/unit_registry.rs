use std::{collections::HashMap, sync::Arc};

use crate::{
    error::AbacusError,
    registry::units::metric_units::register_metric_units,
    units::{unit::Unit, value::Value},
};



#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    units: HashMap<String, Arc<Unit>>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
        }
    }

    pub fn standard() -> Self {
        Self {
            units: register_metric_units(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn get(&self, symbol: &str) -> Option<&Arc<Unit>> {
        self.units.get(symbol)
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.units.contains_key(symbol)
    }

    pub fn unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        self.units
            .get(symbol)
            .cloned()
            .ok_or_else(|| AbacusError::UnknownUnit(symbol.to_string()))
    }

    pub fn value(&self, amount: f64, symbol: &str) -> Result<Value, AbacusError> {
        Ok(Value::new(amount, self.unit(symbol)?))
    }

    pub fn insert_unit(&mut self, key: impl Into<String>, unit: Arc<Unit>) {
        self.units.insert(key.into(), unit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_registry_contains_base_metric_units() {
        let registry = UnitRegistry::standard();

        assert!(registry.contains("m"));
        assert!(registry.contains("s"));
        assert!(registry.contains("g"));
        assert!(registry.contains("km"));
    }

    #[test]
    fn standard_registry_contains_derived_si_units() {
        let registry = UnitRegistry::standard();

        assert!(registry.contains("N"));
        assert!(registry.contains("J"));
        assert!(registry.contains("W"));
        assert!(registry.contains("Pa"));
        assert!(registry.contains("Hz"));
        assert!(registry.contains("V"));
        assert!(registry.contains("kHz"));
        assert!(registry.contains("kW"));
    }

    #[test]
    fn standard_registry_contains_volume_and_angle_units() {
        let registry = UnitRegistry::standard();

        assert!(registry.contains("L"));
        assert!(registry.contains("mL"));
        assert!(registry.contains("us_gal"));
        assert!(registry.contains("rad"));
        assert!(registry.contains("deg"));
    }

    #[test]
    fn performs_value_conversions_via_registry() {
        let registry = UnitRegistry::standard();

        let distance = registry.value(5.0, "km").unwrap();
        let meters = distance.to(&registry, "m").unwrap();

        assert_eq!(meters.to_display(), "5000 m");
        assert_eq!(meters.canonical, 5000.0);

        let volume = registry.value(1.0, "L").unwrap();
        let ml = volume.to(&registry, "mL").unwrap();

        assert_eq!(ml.to_display(), "1000 mL");

        let deg = registry.value(180.0, "deg").unwrap();
        let rad = deg.to(&registry, "rad").unwrap();

        assert!((rad.canonical - std::f64::consts::PI).abs() < 1e-10);
    }
}


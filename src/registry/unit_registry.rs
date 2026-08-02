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

    pub fn get(&self, symbol: &str) -> Option<Arc<Unit>> {
        if let Some(unit) = self.units.get(symbol) {
            return Some(Arc::clone(unit));
        }
        self.parse_exponent_unit(symbol).ok()
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.units.contains_key(symbol) || self.parse_exponent_unit(symbol).is_ok()
    }

    pub fn unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        if let Some(unit) = self.units.get(symbol) {
            return Ok(Arc::clone(unit));
        }
        self.parse_exponent_unit(symbol)
    }

    fn parse_exponent_unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        if let Some((base_sym, exp_str)) = symbol.rsplit_once('^') {
            if let Ok(exp) = exp_str.parse::<i8>() {
                if exp > 0 {
                    if let Some(base_unit) = self.units.get(base_sym) {
                        let exp_usize = exp as usize;
                        let scalar = base_unit.scalar.powi(exp as i32);
                        let dimensions = base_unit.dimensions * exp;
                        let display = crate::units::unit::UnitExpr {
                            numerator: vec![base_sym.to_string(); exp_usize],
                            denominator: Vec::new(),
                        };
                        return Ok(Arc::new(Unit {
                            scalar,
                            offset: 0.0,
                            dimensions,
                            display,
                        }));
                    }
                }
            }
        }
        Err(AbacusError::UnknownUnit(symbol.to_string()))
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
    }

    #[test]
    fn converts_and_adds_volume_units() {

        let registry = UnitRegistry::standard();

        // 1 oil barrel (bbl) to m^3
        let bbl = registry.value(1.0, "bbl").unwrap();
        let m3 = bbl.to(&registry, "m^3").unwrap();
        assert_eq!(m3.to_display(), "0.158987294928 m^3");

        // 1000 L to m^3
        let liters = registry.value(1000.0, "L").unwrap();
        let liters_m3 = liters.to(&registry, "m^3").unwrap();
        assert_eq!(liters_m3.to_display(), "1 m^3");

        // Addition of bbl and L
        let sum = (bbl + registry.value(100.0, "L").unwrap()).unwrap();
        let sum_m3 = sum.to(&registry, "m^3").unwrap();
        assert_eq!(sum_m3.to_display(), "0.258987294928 m^3");
    }
}



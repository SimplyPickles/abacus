use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    error::AbacusError,
    registry::units::metric_units::register_metric_units,
    units::{unit::Unit, value::Value},
};

#[derive(Debug, Default)]
pub struct UnitRegistry {
    units: HashMap<String, Arc<Unit>>,
    cache: RwLock<HashMap<String, Arc<Unit>>>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn standard() -> Self {
        Self {
            units: register_metric_units(),
            cache: RwLock::new(HashMap::new()),
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

    pub fn find_unit_by_dimensions(
        &self,
        dimensions: &crate::units::dimensions::Dimensions,
    ) -> Option<Arc<Unit>> {
        let priority_symbols = [
            "N", "J", "W", "Pa", "Hz", "V", "C", "F", "Ω", "S", "Wb", "T", "H", "A", "lx", "kat",
            "Bq", "Gy", "Sv", "lm",
        ];

        for &sym in &priority_symbols {
            if let Some(unit) = self.units.get(sym)
                && &unit.dimensions == dimensions {
                    return Some(Arc::clone(unit));
                }
        }

        None
    }

    pub fn unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        if let Some(unit) = self.units.get(symbol) {
            return Ok(Arc::clone(unit));
        }
        self.parse_exponent_unit(symbol)
    }

    fn parse_exponent_unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        if let Ok(guard) = self.cache.read()
            && let Some(cached) = guard.get(symbol) {
                return Ok(Arc::clone(cached));
            }

        if let Some((base_sym, exp_str)) = symbol.rsplit_once('^')
            && let Ok(exp) = exp_str.parse::<f64>()
                && let Some(base_unit) = self.units.get(base_sym) {
                    let scalar = base_unit.scalar.powf(exp);
                    let dimensions = base_unit.dimensions * exp;

                    let is_integer = (exp - exp.round()).abs() < 1e-9;
                    let display = if is_integer {
                        let exp_int = exp.round() as i64;
                        if exp_int > 0 {
                            crate::units::unit::UnitExpr {
                                numerator: vec![base_sym.to_string(); exp_int as usize],
                                denominator: Vec::new(),
                            }
                        } else if exp_int < 0 {
                            crate::units::unit::UnitExpr {
                                numerator: Vec::new(),
                                denominator: vec![base_sym.to_string(); (-exp_int) as usize],
                            }
                        } else {
                            crate::units::unit::UnitExpr::dimensionless()
                        }
                    } else if exp > 0.0 {
                        crate::units::unit::UnitExpr {
                            numerator: vec![format!("{base_sym}^{exp_str}")],
                            denominator: Vec::new(),
                        }
                    } else {
                        let pos_exp_str = exp_str.trim_start_matches('-');
                        crate::units::unit::UnitExpr {
                            numerator: Vec::new(),
                            denominator: vec![format!("{base_sym}^{pos_exp_str}")],
                        }
                    };

                    let unit = Arc::new(Unit {
                        scalar,
                        offset: 0.0,
                        dimensions,
                        display,
                    });

                    if let Ok(mut guard) = self.cache.write() {
                        guard.insert(symbol.to_string(), Arc::clone(&unit));
                    }

                    return Ok(unit);
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

    #[test]
    fn caches_generated_exponent_units() {
        let registry = UnitRegistry::standard();

        let unit1 = registry.unit("m^3").unwrap();
        let unit2 = registry.unit("m^3").unwrap();

        assert!(Arc::ptr_eq(&unit1, &unit2));
    }
}

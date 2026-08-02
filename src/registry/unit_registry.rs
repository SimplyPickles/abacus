use std::{collections::HashMap, hash::Hash, sync::Arc};

use crate::{
    error::AbacusError, registry::units::{derived_units, metric_units::register_metric_units}, units::{unit::Unit, value::Value},
};

#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    units: HashMap<String, Arc<Unit>>,
    derived: HashMap<&'static str, Arc<Unit>>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
            derived: HashMap::new(),
        }
    }

    pub fn standard() -> Self {
        let units = register_metric_units();
        let mut derived: HashMap<&'static str, Arc<Unit>> = HashMap::new();

        for i in derived_units::DERIVED_UNITS {
            derived.insert(i.equation, Arc::from(units.get(i.alias).unwrap().clone()));
        }

        Self {
            units,
            derived,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn get(&self, symbol: &str) -> Option<&Arc<Unit>> {
        self.units.get(symbol).or_else(|| self.derived.get(symbol))
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.units.contains_key(symbol) || self.derived.contains_key(symbol)
    }

    pub fn unit(&self, symbol: &str) -> Result<Arc<Unit>, AbacusError> {
        self.units
            .get(symbol)
            .cloned()
            .or_else(|| self.derived.get(symbol).cloned())
            .ok_or_else(|| AbacusError::UnknownUnit(symbol.to_string()))
    }

    pub fn value(&self, amount: f64, symbol: &str) -> Result<Value, AbacusError> {
        Ok(Value::new(amount, self.unit(symbol)?))
    }

    pub fn insert_unit(&mut self, key: impl Into<String>, unit: Arc<Unit>) {
        self.units.insert(key.into(), unit);
    }

    pub fn insert_derived(&mut self, key: impl Into<&'static str>, unit: Arc<Unit>) {
        self.derived.insert(key.into(), unit);
    }
}

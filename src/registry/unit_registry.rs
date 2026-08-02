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

    pub fn insert(&mut self, key: impl Into<String>, unit: Arc<Unit>) {
        self.units.insert(key.into(), unit);
    }
}

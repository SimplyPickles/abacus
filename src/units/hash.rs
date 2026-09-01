use crate::units::value::Value;
use std::collections::HashMap;

/// A map of named key-value pairs representing structured calculation outputs (e.g. regression model results, inference test results).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hash {
    pub values: HashMap<String, Value>,
}

// Implementation functions for Hash
impl Hash {
    // Default constructor for the Hash
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    #[must_use]
    pub fn from(values: HashMap<String, Value>) -> Self {
        Self { values }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: Value) {
        self.values.insert(key.into(), value);
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn simplify_unit_display(
        &mut self,
        registry: &crate::registry::unit_registry::UnitRegistry,
    ) {
        for v in self.values.values_mut() {
            v.simplify_unit_display(registry);
        }
    }

    #[must_use]
    pub fn to_display(&self) -> String {
        self.to_string()
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::new()
    }
}

/// Display implementation for `Hash`
/// Formats the `Hash` as a human-readable string
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        let mut entries: Vec<_> = self.values.iter().collect();
        entries.sort_by_key(|&(k, _)| k);
        for (i, (k, v)) in entries.into_iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{k}: {v}")?;
        }
        write!(f, " }}")
    }
}

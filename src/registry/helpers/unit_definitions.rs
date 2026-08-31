use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

/// A static description of a single unit with one or more lookup keys.
pub struct UnitDefinition {
    pub keys: &'static [&'static str],
    pub display: &'static str,
    pub scalar: f64,
    pub offset: f64,
    pub dimensions: Dimensions,
}

/// Registers a slice of `UnitDefinition`s into `map`, creating one `Arc<Unit>` per
/// definition and inserting it under every key.
pub fn register_unit_definitions(map: &mut HashMap<String, Arc<Unit>>, defs: &[UnitDefinition]) {
    for def in defs {
        let unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: def.offset,
            dimensions: def.dimensions,
            display: UnitExpr::single(def.display),
        });
        for &key in def.keys {
            map.insert(key.to_string(), Arc::clone(&unit));
        }
    }
}

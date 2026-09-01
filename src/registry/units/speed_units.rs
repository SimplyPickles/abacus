use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

pub const SPEED_UNITS: &[UnitDefinition] = &[
    UnitDefinition {
        keys: &["mph", "miles_per_hour"],
        display: "mph",
        scalar: 1_609.344 / 3600.0,
        offset: 0.0,
        dimensions: Dimensions::from_f64([1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    UnitDefinition {
        keys: &["kmph", "kph", "kilometers_per_hour", "kilometres_per_hour"],
        display: "kmph",
        scalar: 1000.0 / 3600.0,
        offset: 0.0,
        dimensions: Dimensions::from_f64([1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
];

pub fn register_speed_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, SPEED_UNITS);
}

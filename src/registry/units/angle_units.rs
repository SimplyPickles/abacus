use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const ANGLE_UNITS: &[UnitDefinition] = &[
    UnitDefinition {
        keys: &["radian", "radians", "rad"],
        display: "rad",
        scalar: 1.0,
        offset: 0.0,
        dimensions: Dimensions::DIMENSIONLESS,
    },
    UnitDefinition {
        keys: &["degree", "degrees", "deg", "°"],
        display: "deg",
        scalar: std::f64::consts::PI / 180.0,
        offset: 0.0,
        dimensions: Dimensions::DIMENSIONLESS,
    },
    UnitDefinition {
        keys: &["turn", "turns"],
        display: "turn",
        scalar: 2.0 * std::f64::consts::PI,
        offset: 0.0,
        dimensions: Dimensions::DIMENSIONLESS,
    },
    UnitDefinition {
        keys: &["arcminute", "arcminutes", "arcmin"],
        display: "arcmin",
        scalar: std::f64::consts::PI / (180.0 * 60.0),
        offset: 0.0,
        dimensions: Dimensions::DIMENSIONLESS,
    },
    UnitDefinition {
        keys: &["arcsecond", "arcseconds", "arcsec"],
        display: "arcsec",
        scalar: std::f64::consts::PI / (180.0 * 3_600.0),
        offset: 0.0,
        dimensions: Dimensions::DIMENSIONLESS,
    },
];

pub fn register_angle_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, ANGLE_UNITS);
}

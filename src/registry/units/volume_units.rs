use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::{
        helpers::{UnitDefinition, register_unit_definitions},
        units::metric_units::METRIC_PREFIXES,
    },
    units::{
        dimensions::Dimensions,
        unit::{Unit, UnitExpr},
    },
};

const VOLUME_AND_AREA_UNITS: &[UnitDefinition] = &[
    // Metric Liter (1 L = 0.001 m^3)
    UnitDefinition {
        keys: &["liter", "liters", "litre", "litres", "L"],
        display: "L",
        scalar: 1e-3,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    // Land Area: Hectare (1 ha = 10,000 m^2)
    UnitDefinition {
        keys: &["hectare", "ha"],
        display: "ha",
        scalar: 10_000.0,
        offset: 0.0,
        dimensions: Dimensions::AREA,
    },
    // Are (1 a = 100 m^2)
    UnitDefinition {
        keys: &["are", "a"],
        display: "a",
        scalar: 100.0,
        offset: 0.0,
        dimensions: Dimensions::AREA,
    },
    // US Customary liquid measures
    UnitDefinition {
        keys: &["us_gallon", "us_gallons", "us_gal"],
        display: "us_gal",
        scalar: 0.003_785_411_784,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["us_quart", "us_quarts", "us_qt"],
        display: "us_qt",
        scalar: 0.000_946_352_946,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["us_pint", "us_pints", "us_pt"],
        display: "us_pt",
        scalar: 0.000_473_176_473,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["us_fluid_ounce", "us_fluid_ounces", "us_fl_oz"],
        display: "us_fl_oz",
        scalar: 0.000_029_573_529_562_5,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["cup", "cups"],
        display: "cup",
        scalar: 0.000_236_588_236_5,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["tablespoon", "tablespoons", "tbsp"],
        display: "tbsp",
        scalar: 0.000_014_786_764_781_25,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["teaspoon", "teaspoons", "tsp"],
        display: "tsp",
        scalar: 0.000_004_928_921_593_75,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
];

pub fn register_volume_and_area_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, VOLUME_AND_AREA_UNITS);

    // Prefixed liters (mL, cL, dL, kL, etc.)
    let liter_base_scalar = 1e-3;
    for pref in METRIC_PREFIXES {
        let pref_unit = Arc::new(Unit {
            scalar: pref.scalar * liter_base_scalar,
            offset: 0.0,
            dimensions: Dimensions::VOLUME,
            display: UnitExpr::single(format!("{}L", pref.alias)),
        });

        map.insert(format!("{}liter", pref.name), Arc::clone(&pref_unit));
        map.insert(format!("{}liters", pref.name), Arc::clone(&pref_unit));
        map.insert(format!("{}litre", pref.name), Arc::clone(&pref_unit));
        map.insert(format!("{}litres", pref.name), Arc::clone(&pref_unit));
        map.insert(format!("{}L", pref.alias), pref_unit);
    }
}

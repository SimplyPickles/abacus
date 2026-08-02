use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct AngleUnit {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
}

const ANGLE_UNITS: &[AngleUnit] = &[
    AngleUnit {
        keys: &["radian", "radians", "rad"],
        display: "rad",
        scalar: 1.0,
    },
    AngleUnit {
        keys: &["degree", "degrees", "deg", "°"],
        display: "deg",
        scalar: std::f64::consts::PI / 180.0,
    },
    AngleUnit {
        keys: &["turn", "turns"],
        display: "turn",
        scalar: 2.0 * std::f64::consts::PI,
    },
    AngleUnit {
        keys: &["arcminute", "arcminutes", "arcmin"],
        display: "arcmin",
        scalar: std::f64::consts::PI / (180.0 * 60.0),
    },
    AngleUnit {
        keys: &["arcsecond", "arcseconds", "arcsec"],
        display: "arcsec",
        scalar: std::f64::consts::PI / (180.0 * 3_600.0),
    },
];

pub fn register_angle_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in ANGLE_UNITS {
        let unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: Dimensions::DIMENSIONLESS,
            display: UnitExpr::single(def.display),
        });

        for &key in def.keys {
            map.insert(key.to_string(), Arc::clone(&unit));
        }
    }
}

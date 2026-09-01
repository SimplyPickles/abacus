use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const TRADE_HISTORICAL_UNITS: &[UnitDefinition] = &[
    // Oil Barrel (bbl): 42 US gallons = 0.158987294928 m^3
    UnitDefinition {
        keys: &["barrel", "barrels", "bbl"],
        display: "bbl",
        scalar: 0.158_987_294_928,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    // Hogshead: 63 US gallons = 0.238480942392 m^3
    UnitDefinition {
        keys: &["hogshead", "hogsheads"],
        display: "hogshead",
        scalar: 0.238_480_942_392,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    // Carat (ct): 0.2 g
    UnitDefinition {
        keys: &["carat", "carats", "ct"],
        display: "ct",
        scalar: 0.2,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    // Troy Ounce (ozt): 31.1034768 g
    UnitDefinition {
        keys: &["troy_ounce", "troy_ounces", "ozt"],
        display: "ozt",
        scalar: 31.103_476_8,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    // Slug: 14.5939029372 kg = 14593.9029372 g
    UnitDefinition {
        keys: &["slug", "slugs"],
        display: "slug",
        scalar: 14_593.902_937_2,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    // Poundal (pdl): 0.138254954376 N = 138.254954376 g*m/s^2
    UnitDefinition {
        keys: &["poundal", "poundals", "pdl"],
        display: "pdl",
        scalar: 138.254_954_376,
        offset: 0.0,
        dimensions: Dimensions::from_f64([1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
];

pub fn register_trade_historical_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, TRADE_HISTORICAL_UNITS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_trade_historical_units() {
        let mut units = HashMap::new();
        register_trade_historical_units(&mut units);

        assert_eq!(units.get("bbl").unwrap().scalar, 0.158_987_294_928);
        assert_eq!(units.get("ct").unwrap().scalar, 0.2);
    }
}

use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct TradeUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const TRADE_HISTORICAL_UNITS: &[TradeUnitDef] = &[
    // Oil Barrel (bbl): 42 US gallons = 0.158987294928 m^3
    TradeUnitDef {
        keys: &["barrel", "barrels", "bbl"],
        display: "bbl",
        scalar: 0.158_987_294_928,
        dimensions: Dimensions::VOLUME,
    },
    // Hogshead: 63 US gallons = 0.238480942392 m^3
    TradeUnitDef {
        keys: &["hogshead", "hogsheads"],
        display: "hogshead",
        scalar: 0.238_480_942_392,
        dimensions: Dimensions::VOLUME,
    },
    // Carat (ct): 0.2 g
    TradeUnitDef {
        keys: &["carat", "carats", "ct"],
        display: "ct",
        scalar: 0.2,
        dimensions: Dimensions::MASS,
    },
    // Troy Ounce (ozt): 31.1034768 g
    TradeUnitDef {
        keys: &["troy_ounce", "troy_ounces", "ozt"],
        display: "ozt",
        scalar: 31.103_476_8,
        dimensions: Dimensions::MASS,
    },
    // Slug: 14.5939029372 kg = 14593.9029372 g
    TradeUnitDef {
        keys: &["slug", "slugs"],
        display: "slug",
        scalar: 14_593.902_937_2,
        dimensions: Dimensions::MASS,
    },
    // Poundal (pdl): 0.138254954376 N = 138.254954376 g*m/s^2
    TradeUnitDef {
        keys: &["poundal", "poundals", "pdl"],
        display: "pdl",
        scalar: 138.254_954_376,
        dimensions: Dimensions([1, 1, -2, 0, 0, 0, 0, 0]),
    },
];

pub fn register_trade_historical_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in TRADE_HISTORICAL_UNITS {
        let unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: def.dimensions,
            display: UnitExpr::single(def.display),
        });

        for &key in def.keys {
            map.insert(key.to_string(), Arc::clone(&unit));
        }
    }
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

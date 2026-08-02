use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct NauticalUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const NAUTICAL_UNITS: &[NauticalUnitDef] = &[
    // Nautical Mile: 1852 m
    NauticalUnitDef {
        keys: &["nautical_mile", "nmi", "NM"],
        display: "nmi",
        scalar: 1852.0,
        dimensions: Dimensions::LENGTH,
    },
    // Knot: 1 nmi / 1 h = 1852 m / 3600 s = 0.5144444444444445 m/s
    NauticalUnitDef {
        keys: &["knot", "knots", "kt", "kts"],
        display: "kt",
        scalar: 1852.0 / 3600.0,
        dimensions: Dimensions([1, 0, -1, 0, 0, 0, 0, 0]),
    },
    // Fathom: 6 feet = 1.8288 m
    NauticalUnitDef {
        keys: &["fathom", "fathoms", "ftm"],
        display: "ftm",
        scalar: 1.8288,
        dimensions: Dimensions::LENGTH,
    },
    // Cable: 0.1 nmi = 185.2 m
    NauticalUnitDef {
        keys: &["cable", "cables"],
        display: "cable",
        scalar: 185.2,
        dimensions: Dimensions::LENGTH,
    },
    // Rod / Perch / Pole: 16.5 feet = 5.0292 m
    NauticalUnitDef {
        keys: &["rod", "rods", "perch", "pole"],
        display: "rod",
        scalar: 5.0292,
        dimensions: Dimensions::LENGTH,
    },
    // Link: 7.92 inches = 0.201168 m
    NauticalUnitDef {
        keys: &["link", "links"],
        display: "link",
        scalar: 0.201168,
        dimensions: Dimensions::LENGTH,
    },
    // League: 3 miles = 4828.032 m
    NauticalUnitDef {
        keys: &["league", "leagues"],
        display: "league",
        scalar: 4828.032,
        dimensions: Dimensions::LENGTH,
    },
];

pub fn register_nautical_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in NAUTICAL_UNITS {
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
    fn registers_nautical_units() {
        let mut units = HashMap::new();
        register_nautical_units(&mut units);

        assert_eq!(units.get("nmi").unwrap().scalar, 1852.0);
        assert_eq!(units.get("kt").unwrap().scalar, 1852.0 / 3600.0);
        assert_eq!(units.get("ftm").unwrap().scalar, 1.8288);
    }
}

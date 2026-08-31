use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const NAUTICAL_UNITS: &[UnitDefinition] = &[
    // Nautical Mile: 1852 m
    UnitDefinition {
        keys: &["nautical_mile", "nmi", "NM"],
        display: "nmi",
        scalar: 1852.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Knot: 1 nmi / 1 h = 1852 m / 3600 s
    UnitDefinition {
        keys: &["knot", "knots", "kt", "kts"],
        display: "kt",
        scalar: 1852.0 / 3600.0,
        offset: 0.0,
        dimensions: Dimensions([1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Fathom: 6 feet = 1.8288 m
    UnitDefinition {
        keys: &["fathom", "fathoms", "ftm"],
        display: "ftm",
        scalar: 1.8288,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Cable: 0.1 nmi = 185.2 m
    UnitDefinition {
        keys: &["cable", "cables"],
        display: "cable",
        scalar: 185.2,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Rod / Perch / Pole: 16.5 feet = 5.0292 m
    UnitDefinition {
        keys: &["rod", "rods", "perch", "pole"],
        display: "rod",
        scalar: 5.0292,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Link: 7.92 inches = 0.201168 m
    UnitDefinition {
        keys: &["link", "links"],
        display: "link",
        scalar: 0.201168,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // League: 3 miles = 4828.032 m
    UnitDefinition {
        keys: &["league", "leagues"],
        display: "league",
        scalar: 4828.032,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
];

pub fn register_nautical_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, NAUTICAL_UNITS);
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

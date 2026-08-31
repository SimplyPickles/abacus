use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const HUMOROUS_UNITS: &[UnitDefinition] = &[
    // Smoot: 67 inches = 1.7018 m
    UnitDefinition {
        keys: &["smoot", "smoots"],
        display: "smoot",
        scalar: 1.7018,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Shake: 10 ns = 1e-8 s
    UnitDefinition {
        keys: &["shake", "shakes"],
        display: "shake",
        scalar: 1e-8,
        offset: 0.0,
        dimensions: Dimensions::TIME,
    },
    // Jiffy: 1/60 s
    UnitDefinition {
        keys: &["jiffy", "jiffies"],
        display: "jiffy",
        scalar: 1.0 / 60.0,
        offset: 0.0,
        dimensions: Dimensions::TIME,
    },
    // Fortnight: 14 days = 1,209,600 s
    UnitDefinition {
        keys: &["fortnight", "fortnights"],
        display: "fortnight",
        scalar: 1_209_600.0,
        offset: 0.0,
        dimensions: Dimensions::TIME,
    },
    // Furlong per fortnight: 201.168 m / 1,209,600 s
    UnitDefinition {
        keys: &["furlong_per_fortnight", "furlongs_per_fortnight"],
        display: "fur/fortnight",
        scalar: 201.168 / 1_209_600.0,
        offset: 0.0,
        dimensions: Dimensions([1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Barn-megaparsec: 1e-28 m^2 * 3.0856775814913673e22 m
    UnitDefinition {
        keys: &["barn_megaparsec", "barn_Mpc"],
        display: "barn-Mpc",
        scalar: 3.085_677_581_491_367_3e-6,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    // Attoparsec: 1e-18 * 3.0856775814913673e16 m = 0.030856775814913673 m
    UnitDefinition {
        keys: &["attoparsec", "apc"],
        display: "apc",
        scalar: 0.030_856_775_814_913_673,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
];

pub fn register_humorous_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, HUMOROUS_UNITS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_humorous_units() {
        let mut units = HashMap::new();
        register_humorous_units(&mut units);

        assert_eq!(units.get("smoot").unwrap().scalar, 1.7018);
        assert_eq!(units.get("fortnight").unwrap().scalar, 1_209_600.0);
    }
}

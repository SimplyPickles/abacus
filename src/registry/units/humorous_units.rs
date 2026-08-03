use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct HumorousUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const HUMOROUS_UNITS: &[HumorousUnitDef] = &[
    // Smoot: 67 inches = 1.7018 m
    HumorousUnitDef {
        keys: &["smoot", "smoots"],
        display: "smoot",
        scalar: 1.7018,
        dimensions: Dimensions::LENGTH,
    },
    // Shake: 10 ns = 1e-8 s
    HumorousUnitDef {
        keys: &["shake", "shakes"],
        display: "shake",
        scalar: 1e-8,
        dimensions: Dimensions::TIME,
    },
    // Jiffy: 1/60 s
    HumorousUnitDef {
        keys: &["jiffy", "jiffies"],
        display: "jiffy",
        scalar: 1.0 / 60.0,
        dimensions: Dimensions::TIME,
    },
    // Fortnight: 14 days = 1,209,600 s
    HumorousUnitDef {
        keys: &["fortnight", "fortnights"],
        display: "fortnight",
        scalar: 1_209_600.0,
        dimensions: Dimensions::TIME,
    },
    // Furlong per fortnight: 201.168 m / 1,209,600 s = 0.0001663095238095238 m/s
    HumorousUnitDef {
        keys: &["furlong_per_fortnight", "furlongs_per_fortnight"],
        display: "fur/fortnight",
        scalar: 201.168 / 1_209_600.0,
        dimensions: Dimensions([1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Barn-megaparsec: 1e-28 m^2 * 3.0856775814913673e22 m = 3.0856775814913673e-6 m^3
    HumorousUnitDef {
        keys: &["barn_megaparsec", "barn_Mpc"],
        display: "barn-Mpc",
        scalar: 3.085_677_581_491_367_3e-6,
        dimensions: Dimensions::VOLUME,
    },
    // Attoparsec: 1e-18 * 3.0856775814913673e16 m = 0.030856775814913673 m
    HumorousUnitDef {
        keys: &["attoparsec", "apc"],
        display: "apc",
        scalar: 0.030_856_775_814_913_673,
        dimensions: Dimensions::LENGTH,
    },
];

pub fn register_humorous_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in HUMOROUS_UNITS {
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
    fn registers_humorous_units() {
        let mut units = HashMap::new();
        register_humorous_units(&mut units);

        assert_eq!(units.get("smoot").unwrap().scalar, 1.7018);
        assert_eq!(units.get("fortnight").unwrap().scalar, 1_209_600.0);
    }
}

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

/// Units that are NOT given metric prefixes.
const CGS_SIMPLE: &[UnitDefinition] = &[
    // Angstrom: 1e-10 m
    UnitDefinition {
        keys: &["angstrom", "Å"],
        display: "Å",
        scalar: 1e-10,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Atmosphere: 101,325 Pa = 1.01325e8 g/(m*s^2)
    UnitDefinition {
        keys: &["atmosphere", "atm"],
        display: "atm",
        scalar: 1.013_25e8,
        offset: 0.0,
        dimensions: Dimensions([-1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Torr / mmHg
    UnitDefinition {
        keys: &["torr", "Torr", "mmHg"],
        display: "torr",
        scalar: 101_325.0 * 1000.0 / 760.0,
        offset: 0.0,
        dimensions: Dimensions([-1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Barn: 1e-28 m^2
    UnitDefinition {
        keys: &["barn", "b_barn"],
        display: "barn",
        scalar: 1e-28,
        offset: 0.0,
        dimensions: Dimensions::AREA,
    },
];

/// Units that ARE given metric prefixes (base key → display symbol → scalar → dimensions).
struct PrefixableCgs {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const CGS_PREFIXABLE: &[PrefixableCgs] = &[
    // Electronvolt: 1.602176634e-19 J = 1.602176634e-16 g*m^2/s^2
    PrefixableCgs {
        keys: &["electronvolt", "eV"],
        display: "eV",
        scalar: 1.602_176_634e-16,
        dimensions: Dimensions([2.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Dalton / amu: 1.6605390666e-24 g
    PrefixableCgs {
        keys: &["dalton", "Da", "amu"],
        display: "Da",
        scalar: 1.660_539_066_6e-24,
        dimensions: Dimensions::MASS,
    },
    // Bar: 100,000 Pa = 1e8 g/(m*s^2)
    PrefixableCgs {
        keys: &["bar"],
        display: "bar",
        scalar: 1e8,
        dimensions: Dimensions([-1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Gauss: 1e-4 T = 0.1 g/(s^2*A)
    PrefixableCgs {
        keys: &["gauss", "G"],
        display: "G",
        scalar: 0.1,
        dimensions: Dimensions([0.0, 1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Maxwell: 1e-8 Wb = 1e-5 g*m^2/(s^2*A)
    PrefixableCgs {
        keys: &["maxwell", "Mx"],
        display: "Mx",
        scalar: 1e-5,
        dimensions: Dimensions([2.0, 1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Poise: 0.1 Pa*s = 100 g/(m*s)
    PrefixableCgs {
        keys: &["poise", "P"],
        display: "P",
        scalar: 100.0,
        dimensions: Dimensions([-1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Stokes: 1e-4 m^2/s
    PrefixableCgs {
        keys: &["stokes", "St"],
        display: "St",
        scalar: 1e-4,
        dimensions: Dimensions([2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
    // Galileo: 0.01 m/s^2
    PrefixableCgs {
        keys: &["galileo", "Gal"],
        display: "Gal",
        scalar: 0.01,
        dimensions: Dimensions([1.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
];

pub fn register_cgs_physics_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, CGS_SIMPLE);

    for def in CGS_PREFIXABLE {
        let base_unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: def.dimensions,
            display: UnitExpr::single(def.display),
        });

        for &key in def.keys {
            map.insert(key.to_string(), Arc::clone(&base_unit));
        }

        for pref in METRIC_PREFIXES {
            let pref_unit = Arc::new(Unit {
                scalar: def.scalar * pref.scalar,
                offset: 0.0,
                dimensions: def.dimensions,
                display: UnitExpr::single(format!("{}{}", pref.alias, def.display)),
            });

            map.insert(format!("{}{}", pref.alias, def.display), pref_unit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_cgs_physics_units() {
        let mut units = HashMap::new();
        register_cgs_physics_units(&mut units);

        assert_eq!(units.get("Å").unwrap().scalar, 1e-10);
        assert_eq!(units.get("eV").unwrap().scalar, 1.602_176_634e-16);
        assert!((units.get("keV").unwrap().scalar - 1.602_176_634e-13).abs() < 1e-25);
        assert_eq!(units.get("atm").unwrap().scalar, 1.013_25e8);
    }
}

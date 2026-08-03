use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::units::metric_units::METRIC_PREFIXES,
    units::{
        dimensions::Dimensions,
        unit::{Unit, UnitExpr},
    },
};

struct CgsUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
    prefixable: bool,
}

const CGS_PHYSICS_UNITS: &[CgsUnitDef] = &[
    // Angstrom: 1e-10 m
    CgsUnitDef {
        keys: &["angstrom", "Å"],
        display: "Å",
        scalar: 1e-10,
        dimensions: Dimensions::LENGTH,
        prefixable: false,
    },
    // Electronvolt: 1.602176634e-19 J = 1.602176634e-16 g*m^2/s^2
    CgsUnitDef {
        keys: &["electronvolt", "eV"],
        display: "eV",
        scalar: 1.602_176_634e-16,
        dimensions: Dimensions([2, 1, -2, 0, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Dalton / amu: 1.6605390666e-24 g
    CgsUnitDef {
        keys: &["dalton", "Da", "amu"],
        display: "Da",
        scalar: 1.660_539_066_6e-24,
        dimensions: Dimensions::MASS,
        prefixable: true,
    },
    // Bar: 100,000 Pa = 100,000 * 1000 g/(m*s^2) = 1e8 g/(m*s^2)
    CgsUnitDef {
        keys: &["bar"],
        display: "bar",
        scalar: 1e8,
        dimensions: Dimensions([-1, 1, -2, 0, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Atmosphere: 101,325 Pa = 1.01325e8 g/(m*s^2)
    CgsUnitDef {
        keys: &["atmosphere", "atm"],
        display: "atm",
        scalar: 1.013_25e8,
        dimensions: Dimensions([-1, 1, -2, 0, 0, 0, 0, 0]),
        prefixable: false,
    },
    // Torr / mmHg: 101325 / 760 Pa = 1.3332236842105263e5 g/(m*s^2)
    CgsUnitDef {
        keys: &["torr", "Torr", "mmHg"],
        display: "torr",
        scalar: 101_325.0 * 1000.0 / 760.0,
        dimensions: Dimensions([-1, 1, -2, 0, 0, 0, 0, 0]),
        prefixable: false,
    },
    // Barn: 1e-28 m^2
    CgsUnitDef {
        keys: &["barn", "b_barn"],
        display: "barn",
        scalar: 1e-28,
        dimensions: Dimensions::AREA,
        prefixable: false,
    },
    // Gauss: 1e-4 T = 1e-1 g/(s^2*A)
    CgsUnitDef {
        keys: &["gauss", "G"],
        display: "G",
        scalar: 0.1,
        dimensions: Dimensions([0, 1, -2, -1, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Maxwell: 1e-8 Wb = 1e-5 g*m^2/(s^2*A)
    CgsUnitDef {
        keys: &["maxwell", "Mx"],
        display: "Mx",
        scalar: 1e-5,
        dimensions: Dimensions([2, 1, -2, -1, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Poise: 0.1 Pa*s = 100 g/(m*s)
    CgsUnitDef {
        keys: &["poise", "P"],
        display: "P",
        scalar: 100.0,
        dimensions: Dimensions([-1, 1, -1, 0, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Stokes: 1e-4 m^2/s
    CgsUnitDef {
        keys: &["stokes", "St"],
        display: "St",
        scalar: 1e-4,
        dimensions: Dimensions([2, 0, -1, 0, 0, 0, 0, 0]),
        prefixable: true,
    },
    // Galileo: 0.01 m/s^2
    CgsUnitDef {
        keys: &["galileo", "Gal"],
        display: "Gal",
        scalar: 0.01,
        dimensions: Dimensions([1, 0, -2, 0, 0, 0, 0, 0]),
        prefixable: true,
    },
];

pub fn register_cgs_physics_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in CGS_PHYSICS_UNITS {
        let base_unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: def.dimensions,
            display: UnitExpr::single(def.display),
        });

        for &key in def.keys {
            map.insert(key.to_string(), Arc::clone(&base_unit));
        }

        if def.prefixable {
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

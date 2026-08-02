use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::units::metric_units::METRIC_PREFIXES,
    units::{
        dimensions::Dimensions,
        unit::{Unit, UnitExpr},
    },
};

struct AstroUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const ASTRO_UNITS: &[AstroUnitDef] = &[
    // AU: 149,597,870,700 m
    AstroUnitDef {
        keys: &["astronomical_unit", "au", "AU"],
        display: "au",
        scalar: 149_597_870_700.0,
        dimensions: Dimensions::LENGTH,
    },
    // Light year: 9,460,730,472,580,800 m
    AstroUnitDef {
        keys: &["light_year", "lightyear", "ly"],
        display: "ly",
        scalar: 9_460_730_472_580_800.0,
        dimensions: Dimensions::LENGTH,
    },
    // Parsec: 30,856,775,814,913,673 m
    AstroUnitDef {
        keys: &["parsec", "pc"],
        display: "pc",
        scalar: 30_856_775_814_913_673.0,
        dimensions: Dimensions::LENGTH,
    },
    // Solar Mass: 1.98847e30 kg = 1.98847e33 g
    AstroUnitDef {
        keys: &["solar_mass", "M_sun"],
        display: "M_sun",
        scalar: 1.988_47e33,
        dimensions: Dimensions::MASS,
    },
    // Jansky: 1e-26 W/(m^2 * Hz) = 1e-23 g/s^2
    AstroUnitDef {
        keys: &["jansky", "Jy"],
        display: "Jy",
        scalar: 1e-23,
        dimensions: Dimensions([0, 1, -2, 0, 0, 0, 0, 0]),
    },
];

pub fn register_astronomical_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in ASTRO_UNITS {
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

    // Prefixed parsec (kpc, Mpc, Gpc)
    let pc_scalar = 30_856_775_814_913_673.0;
    for pref in METRIC_PREFIXES {
        let pref_unit = Arc::new(Unit {
            scalar: pc_scalar * pref.scalar,
            offset: 0.0,
            dimensions: Dimensions::LENGTH,
            display: UnitExpr::single(format!("{}pc", pref.alias)),
        });

        map.insert(format!("{}parsec", pref.name), Arc::clone(&pref_unit));
        map.insert(format!("{}pc", pref.alias), pref_unit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_astronomical_units() {
        let mut units = HashMap::new();
        register_astronomical_units(&mut units);

        assert_eq!(units.get("au").unwrap().scalar, 149_597_870_700.0);
        assert_eq!(units.get("ly").unwrap().scalar, 9_460_730_472_580_800.0);
        assert_eq!(units.get("kpc").unwrap().scalar, 30_856_775_814_913_673.0 * 1e3);
    }
}

use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::{
        helpers::{UnitDefinition, register_unit_definitions},
        units::metric_units::register_metric_prefixed_units,
    },
    units::{dimensions::Dimensions, unit::Unit},
};

const ASTRO_UNITS: &[UnitDefinition] = &[
    // AU: 149,597,870,700 m
    UnitDefinition {
        keys: &["astronomical_unit", "au", "AU"],
        display: "au",
        scalar: 149_597_870_700.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Light year: 9,460,730,472,580,800 m
    UnitDefinition {
        keys: &["light_year", "lightyear", "ly"],
        display: "ly",
        scalar: 9_460_730_472_580_800.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Parsec: 30,856,775,814,913,673 m
    UnitDefinition {
        keys: &["parsec", "pc"],
        display: "pc",
        scalar: 30_856_775_814_913_673.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Solar Mass: 1.98847e30 kg = 1.98847e33 g
    UnitDefinition {
        keys: &["solar_mass", "M_sun"],
        display: "M_sun",
        scalar: 1.988_47e33,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    // Jansky: 1e-26 W/(m^2 * Hz) = 1e-23 g/s^2
    UnitDefinition {
        keys: &["jansky", "Jy"],
        display: "Jy",
        scalar: 1e-23,
        offset: 0.0,
        dimensions: Dimensions::from_f64([0.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    },
];

pub fn register_astronomical_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, ASTRO_UNITS);

    // Prefixed parsec (kpc, Mpc, Gpc)
    register_metric_prefixed_units(
        map,
        Some("parsec"),
        "pc",
        30_856_775_814_913_673.0,
        Dimensions::LENGTH,
        false,
        &[],
    );
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
        assert_eq!(
            units.get("kpc").unwrap().scalar,
            30_856_775_814_913_673.0 * 1e3
        );
    }
}

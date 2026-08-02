use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::units::metric_units::METRIC_PREFIXES,
    units::{
        dimensions::Dimensions,
        unit::{Unit, UnitExpr},
    },
};

struct DerivedUnitDef {
    name: &'static str,
    alias: &'static str,
    dimensions: Dimensions,
    scalar: f64,
    prefixable: bool,
}

const DERIVED_UNITS: &[DerivedUnitDef] = &[
    // Hz (Hertz): s^-1
    DerivedUnitDef {
        name: "hertz",
        alias: "Hz",
        dimensions: Dimensions([0, 0, -1, 0, 0, 0, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // N (Newton): kg*m/s^2 = 1000 g*m/s^2
    DerivedUnitDef {
        name: "newton",
        alias: "N",
        dimensions: Dimensions([1, 1, -2, 0, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // Pa (Pascal): N/m^2 = 1000 g/(m*s^2)
    DerivedUnitDef {
        name: "pascal",
        alias: "Pa",
        dimensions: Dimensions([-1, 1, -2, 0, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // J (Joule): N*m = 1000 g*m^2/s^2
    DerivedUnitDef {
        name: "joule",
        alias: "J",
        dimensions: Dimensions([2, 1, -2, 0, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // W (Watt): J/s = 1000 g*m^2/s^3
    DerivedUnitDef {
        name: "watt",
        alias: "W",
        dimensions: Dimensions([2, 1, -3, 0, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // C (Coulomb): A*s
    DerivedUnitDef {
        name: "coulomb",
        alias: "C",
        dimensions: Dimensions([0, 0, 1, 1, 0, 0, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // V (Volt): W/A = 1000 g*m^2/(s^3*A)
    DerivedUnitDef {
        name: "volt",
        alias: "V",
        dimensions: Dimensions([2, 1, -3, -1, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // F (Farad): C/V = 0.001 A^2*s^4/(g*m^2)
    DerivedUnitDef {
        name: "farad",
        alias: "F",
        dimensions: Dimensions([-2, -1, 4, 2, 0, 0, 0, 0]),
        scalar: 0.001,
        prefixable: true,
    },
    // Ω (Ohm): V/A
    DerivedUnitDef {
        name: "ohm",
        alias: "Ω",
        dimensions: Dimensions([2, 1, -3, -2, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // S (Siemens): 1/Ω
    DerivedUnitDef {
        name: "siemens",
        alias: "S",
        dimensions: Dimensions([-2, -1, 3, 2, 0, 0, 0, 0]),
        scalar: 0.001,
        prefixable: true,
    },
    // Wb (Weber): V*s
    DerivedUnitDef {
        name: "weber",
        alias: "Wb",
        dimensions: Dimensions([2, 1, -2, -1, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // T (Tesla): Wb/m^2
    DerivedUnitDef {
        name: "tesla",
        alias: "T",
        dimensions: Dimensions([0, 1, -2, -1, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // H (Henry): Wb/A
    DerivedUnitDef {
        name: "henry",
        alias: "H",
        dimensions: Dimensions([2, 1, -2, -2, 0, 0, 0, 0]),
        scalar: 1000.0,
        prefixable: true,
    },
    // lm (Lumen): cd
    DerivedUnitDef {
        name: "lumen",
        alias: "lm",
        dimensions: Dimensions::LUMINOUS_INTENSITY,
        scalar: 1.0,
        prefixable: true,
    },
    // lx (Lux): lm/m^2
    DerivedUnitDef {
        name: "lux",
        alias: "lx",
        dimensions: Dimensions([-2, 0, 0, 0, 0, 0, 1, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // Bq (Becquerel): s^-1
    DerivedUnitDef {
        name: "becquerel",
        alias: "Bq",
        dimensions: Dimensions([0, 0, -1, 0, 0, 0, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // Gy (Gray): J/kg = m^2/s^2
    DerivedUnitDef {
        name: "gray",
        alias: "Gy",
        dimensions: Dimensions([2, 0, -2, 0, 0, 0, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // Sv (Sievert): J/kg
    DerivedUnitDef {
        name: "sievert",
        alias: "Sv",
        dimensions: Dimensions([2, 0, -2, 0, 0, 0, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
    // kat (Katal): mol/s
    DerivedUnitDef {
        name: "katal",
        alias: "kat",
        dimensions: Dimensions([0, 0, -1, 0, 0, 1, 0, 0]),
        scalar: 1.0,
        prefixable: true,
    },
];

pub fn register_derived_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in DERIVED_UNITS {
        let base_unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: def.dimensions,
            display: UnitExpr::single(def.alias),
        });

        map.insert(def.name.to_string(), Arc::clone(&base_unit));
        map.insert(def.alias.to_string(), base_unit);

        if def.prefixable {
            for pref in METRIC_PREFIXES {
                let pref_unit = Arc::new(Unit {
                    scalar: def.scalar * pref.scalar,
                    offset: 0.0,
                    dimensions: def.dimensions,
                    display: UnitExpr::single(format!("{}{}", pref.alias, def.alias)),
                });

                map.insert(format!("{}{}", pref.name, def.name), Arc::clone(&pref_unit));
                map.insert(format!("{}{}", pref.alias, def.alias), pref_unit);
            }
        }
    }
}

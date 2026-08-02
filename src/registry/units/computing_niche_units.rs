use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct InfoNicheDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
    dimensions: Dimensions,
}

const COMPUTING_NICHE_UNITS: &[InfoNicheDef] = &[
    // Nibble: 4 bits
    InfoNicheDef {
        keys: &["nibble", "nibbles"],
        display: "nibble",
        scalar: 4.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Crumb: 2 bits
    InfoNicheDef {
        keys: &["crumb", "crumbs"],
        display: "crumb",
        scalar: 2.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Word: 16 bits
    InfoNicheDef {
        keys: &["word", "words"],
        display: "word",
        scalar: 16.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Dword: 32 bits
    InfoNicheDef {
        keys: &["dword", "dwords"],
        display: "dword",
        scalar: 32.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Qword: 64 bits
    InfoNicheDef {
        keys: &["qword", "qwords"],
        display: "qword",
        scalar: 64.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Shannon (bit of entropy): 1 bit
    InfoNicheDef {
        keys: &["shannon", "shannons"],
        display: "Sh",
        scalar: 1.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Hartley (ban): log2(10) bits = 3.321928094887362 bits
    InfoNicheDef {
        keys: &["hartley", "hartleys", "ban"],
        display: "Hart",
        scalar: 3.321_928_094_887_362,
        dimensions: Dimensions::INFORMATION,
    },
    // Nat (natural unit of information): log2(e) bits = 1.4426950408889634 bits
    InfoNicheDef {
        keys: &["nat", "nats"],
        display: "nat",
        scalar: 1.442_695_040_888_963_4,
        dimensions: Dimensions::INFORMATION,
    },
];

pub fn register_computing_niche_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in COMPUTING_NICHE_UNITS {
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
    fn registers_computing_niche_units() {
        let mut units = HashMap::new();
        register_computing_niche_units(&mut units);

        assert_eq!(units.get("nibble").unwrap().scalar, 4.0);
        assert_eq!(units.get("crumb").unwrap().scalar, 2.0);
        assert_eq!(units.get("dword").unwrap().scalar, 32.0);
    }
}

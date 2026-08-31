use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const COMPUTING_NICHE_UNITS: &[UnitDefinition] = &[
    // Nibble: 4 bits
    UnitDefinition {
        keys: &["nibble", "nibbles"],
        display: "nibble",
        scalar: 4.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Crumb: 2 bits
    UnitDefinition {
        keys: &["crumb", "crumbs"],
        display: "crumb",
        scalar: 2.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Word: 16 bits
    UnitDefinition {
        keys: &["word", "words"],
        display: "word",
        scalar: 16.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Dword: 32 bits
    UnitDefinition {
        keys: &["dword", "dwords"],
        display: "dword",
        scalar: 32.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Qword: 64 bits
    UnitDefinition {
        keys: &["qword", "qwords"],
        display: "qword",
        scalar: 64.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Shannon (bit of entropy): 1 bit
    UnitDefinition {
        keys: &["shannon", "shannons"],
        display: "Sh",
        scalar: 1.0,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Hartley (ban): log2(10) bits
    UnitDefinition {
        keys: &["hartley", "hartleys", "ban"],
        display: "Hart",
        scalar: std::f64::consts::LOG2_10,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
    // Nat (natural unit of information): log2(e) bits
    UnitDefinition {
        keys: &["nat", "nats"],
        display: "nat",
        scalar: std::f64::consts::LOG2_E,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
    },
];

pub fn register_computing_niche_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, COMPUTING_NICHE_UNITS);
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

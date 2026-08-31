use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const TYPO_UNITS: &[UnitDefinition] = &[
    // Point (pt): 1/72 inch = 0.0254 / 72 m
    UnitDefinition {
        keys: &["point", "points", "pt_type"],
        display: "pt_type",
        scalar: 0.0254 / 72.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Pica: 12 points = 1/6 inch
    UnitDefinition {
        keys: &["pica", "picas"],
        display: "pica",
        scalar: 0.0254 / 6.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    // Twip: 1/20 point = 1/1440 inch
    UnitDefinition {
        keys: &["twip", "twips"],
        display: "twip",
        scalar: 0.0254 / 1440.0,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
];

pub fn register_typography_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, TYPO_UNITS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_typography_units() {
        let mut units = HashMap::new();
        register_typography_units(&mut units);

        assert_eq!(units.get("pica").unwrap().scalar, 0.0254 / 6.0);
    }
}

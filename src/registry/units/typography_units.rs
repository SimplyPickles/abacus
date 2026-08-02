use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct TypoUnitDef {
    keys: &'static [&'static str],
    display: &'static str,
    scalar: f64,
}

const TYPO_UNITS: &[TypoUnitDef] = &[
    // Point (pt): 1/72 inch = 0.0254 / 72 m = 0.0003527777777777778 m
    TypoUnitDef {
        keys: &["point", "points", "pt_type"],
        display: "pt_type",
        scalar: 0.0254 / 72.0,
    },
    // Pica: 12 points = 1/6 inch = 0.004233333333333333 m
    TypoUnitDef {
        keys: &["pica", "picas"],
        display: "pica",
        scalar: 0.0254 / 6.0,
    },
    // Twip: 1/20 point = 1/1440 inch = 0.00001763888888888889 m
    TypoUnitDef {
        keys: &["twip", "twips"],
        display: "twip",
        scalar: 0.0254 / 1440.0,
    },
];

pub fn register_typography_units(map: &mut HashMap<String, Arc<Unit>>) {
    for def in TYPO_UNITS {
        let unit = Arc::new(Unit {
            scalar: def.scalar,
            offset: 0.0,
            dimensions: Dimensions::LENGTH,
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
    fn registers_typography_units() {
        let mut units = HashMap::new();
        register_typography_units(&mut units);

        assert_eq!(units.get("pica").unwrap().scalar, 0.0254 / 6.0);
    }
}

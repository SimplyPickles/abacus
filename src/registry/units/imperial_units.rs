use std::{collections::HashMap, sync::Arc};

use crate::{
    registry::helpers::{UnitDefinition, register_unit_definitions},
    units::{dimensions::Dimensions, unit::Unit},
};

const IMPERIAL_UNITS: &[UnitDefinition] = &[
    // International yard and pound definitions used by the British Imperial system.
    UnitDefinition {
        keys: &["inch", "inches", "in"],
        display: "in",
        scalar: 0.0254,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["foot", "feet", "ft"],
        display: "ft",
        scalar: 0.3048,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["yard", "yards", "yd"],
        display: "yd",
        scalar: 0.9144,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["chain", "chains", "ch"],
        display: "ch",
        scalar: 20.1168,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["furlong", "furlongs", "fur"],
        display: "fur",
        scalar: 201.168,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["mile", "miles", "mi"],
        display: "mi",
        scalar: 1_609.344,
        offset: 0.0,
        dimensions: Dimensions::LENGTH,
    },
    UnitDefinition {
        keys: &["acre", "acres", "ac"],
        display: "ac",
        scalar: 4_046.856_422_4,
        offset: 0.0,
        dimensions: Dimensions::AREA,
    },
    // Mass
    UnitDefinition {
        keys: &["grain", "grains", "gr"],
        display: "gr",
        scalar: 0.064_798_91,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["dram", "drams", "dr"],
        display: "dr",
        scalar: 1.771_845_195_312_5,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["ounce", "ounces", "oz"],
        display: "oz",
        scalar: 28.349_523_125,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["pound", "pounds", "lb"],
        display: "lb",
        scalar: 453.592_37,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["stone", "stones", "st"],
        display: "st",
        scalar: 6_350.293_18,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["hundredweight", "hundredweights", "cwt"],
        display: "cwt",
        scalar: 50_802.345_44,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    UnitDefinition {
        keys: &["long ton", "long tons", "imperial ton", "ton"],
        display: "ton",
        scalar: 1_016_046.908_8,
        offset: 0.0,
        dimensions: Dimensions::MASS,
    },
    // British Imperial liquid measures. US customary units have different scalars.
    UnitDefinition {
        keys: &["fluid ounce", "fluid ounces", "fl oz", "floz"],
        display: "fl oz",
        scalar: 0.000_028_413_062_5,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["gill", "gills", "gi"],
        display: "gi",
        scalar: 0.000_142_065_312_5,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["pint", "pints", "pt"],
        display: "pt",
        scalar: 0.000_568_261_25,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["quart", "quarts", "qt"],
        display: "qt",
        scalar: 0.001_136_522_5,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    UnitDefinition {
        keys: &["gallon", "gallons", "gal"],
        display: "gal",
        scalar: 0.004_546_09,
        offset: 0.0,
        dimensions: Dimensions::VOLUME,
    },
    // Temperature (affine)
    UnitDefinition {
        keys: &["fahrenheit", "degF", "°F"],
        display: "°F",
        scalar: 5.0 / 9.0,
        offset: 255.372_222_222_222_2,
        dimensions: Dimensions::TEMPERATURE,
    },
];

pub fn register_imperial_units(map: &mut HashMap<String, Arc<Unit>>) {
    register_unit_definitions(map, IMPERIAL_UNITS);
}

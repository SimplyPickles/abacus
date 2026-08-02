use std::{collections::HashMap, sync::Arc};

use crate::units::{
    dimensions::Dimensions,
    unit::{Unit, UnitExpr},
};

struct StoragePrefix {
    name: &'static str,
    alias: &'static str,
    scalar: f64,
}

const DECIMAL_PREFIXES: &[StoragePrefix] = &[
    StoragePrefix {
        name: "kilo",
        alias: "k",
        scalar: 1e3,
    },
    StoragePrefix {
        name: "mega",
        alias: "M",
        scalar: 1e6,
    },
    StoragePrefix {
        name: "giga",
        alias: "G",
        scalar: 1e9,
    },
    StoragePrefix {
        name: "tera",
        alias: "T",
        scalar: 1e12,
    },
    StoragePrefix {
        name: "peta",
        alias: "P",
        scalar: 1e15,
    },
    StoragePrefix {
        name: "exa",
        alias: "E",
        scalar: 1e18,
    },
    StoragePrefix {
        name: "zetta",
        alias: "Z",
        scalar: 1e21,
    },
    StoragePrefix {
        name: "yotta",
        alias: "Y",
        scalar: 1e24,
    },
    StoragePrefix {
        name: "ronna",
        alias: "R",
        scalar: 1e27,
    },
    StoragePrefix {
        name: "quetta",
        alias: "Q",
        scalar: 1e30,
    },
];

const BINARY_PREFIXES: &[StoragePrefix] = &[
    StoragePrefix {
        name: "kibi",
        alias: "Ki",
        scalar: 1_024.0,
    },
    StoragePrefix {
        name: "mebi",
        alias: "Mi",
        scalar: 1_048_576.0,
    },
    StoragePrefix {
        name: "gibi",
        alias: "Gi",
        scalar: 1_073_741_824.0,
    },
    StoragePrefix {
        name: "tebi",
        alias: "Ti",
        scalar: 1_099_511_627_776.0,
    },
    StoragePrefix {
        name: "pebi",
        alias: "Pi",
        scalar: 1_125_899_906_842_624.0,
    },
    StoragePrefix {
        name: "exbi",
        alias: "Ei",
        scalar: 1_152_921_504_606_846_976.0,
    },
    StoragePrefix {
        name: "zebi",
        alias: "Zi",
        scalar: 1_180_591_620_717_411_303_424.0,
    },
    StoragePrefix {
        name: "yobi",
        alias: "Yi",
        scalar: 1_208_925_819_614_629_174_706_176.0,
    },
];

const STORAGE_BASE_UNITS: &[(&str, &str, f64)] = &[("bit", "b", 1.0), ("byte", "B", 8.0)];

fn insert_unit(map: &mut HashMap<String, Arc<Unit>>, name: String, alias: String, scalar: f64) {
    let unit = Arc::new(Unit {
        scalar,
        offset: 0.0,
        dimensions: Dimensions::INFORMATION,
        display: UnitExpr::single(alias.clone()),
    });

    map.insert(name, Arc::clone(&unit));
    map.insert(alias, unit);
}

pub fn register_storage_units(map: &mut HashMap<String, Arc<Unit>>) {
    for &(name, alias, base_scalar) in STORAGE_BASE_UNITS {
        insert_unit(map, name.to_string(), alias.to_string(), base_scalar);

        for prefix in DECIMAL_PREFIXES.iter().chain(BINARY_PREFIXES) {
            insert_unit(
                map,
                format!("{}{}", prefix.name, name),
                format!("{}{}", prefix.alias, alias),
                prefix.scalar * base_scalar,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_bits_and_bytes() {
        let mut units = HashMap::new();
        register_storage_units(&mut units);

        assert_eq!(units.get("b").unwrap().scalar, 1.0);
        assert_eq!(units.get("B").unwrap().scalar, 8.0);
        assert_eq!(units.get("B").unwrap().dimensions, Dimensions::INFORMATION);
        assert!(Arc::ptr_eq(
            units.get("byte").unwrap(),
            units.get("B").unwrap()
        ));
    }

    #[test]
    fn registers_decimal_storage_units() {
        let mut units = HashMap::new();
        register_storage_units(&mut units);

        assert_eq!(units.get("Mb").unwrap().scalar, 1_000_000.0);
        assert_eq!(units.get("MB").unwrap().scalar, 8_000_000.0);
        assert!(Arc::ptr_eq(
            units.get("megabyte").unwrap(),
            units.get("MB").unwrap()
        ));
    }

    #[test]
    fn registers_binary_storage_units() {
        let mut units = HashMap::new();
        register_storage_units(&mut units);

        assert_eq!(units.get("Mib").unwrap().scalar, 1_048_576.0);
        assert_eq!(units.get("MiB").unwrap().scalar, 8_388_608.0);
        assert!(Arc::ptr_eq(
            units.get("mebibyte").unwrap(),
            units.get("MiB").unwrap()
        ));
    }
}

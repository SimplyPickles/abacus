pub struct StoragePrefix {
    pub name: &'static str,
    pub alias: &'static str,
    pub scalar: f64,
}

pub const DECIMAL_STORAGE_PREFIXES: &[StoragePrefix] = &[
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

pub const BINARY_STORAGE_PREFIXES: &[StoragePrefix] = &[
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

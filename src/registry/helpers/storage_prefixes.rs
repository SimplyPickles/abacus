use crate::registry::helpers::scalar_prefixes::ScalarPrefix;

pub const BINARY_STORAGE_PREFIXES: &[ScalarPrefix] = &[
    ScalarPrefix {
        name: "kibi",
        alias: "Ki",
        scalar: 1_024.0,
    },
    ScalarPrefix {
        name: "mebi",
        alias: "Mi",
        scalar: 1_048_576.0,
    },
    ScalarPrefix {
        name: "gibi",
        alias: "Gi",
        scalar: 1_073_741_824.0,
    },
    ScalarPrefix {
        name: "tebi",
        alias: "Ti",
        scalar: 1_099_511_627_776.0,
    },
    ScalarPrefix {
        name: "pebi",
        alias: "Pi",
        scalar: 1_125_899_906_842_624.0,
    },
    ScalarPrefix {
        name: "exbi",
        alias: "Ei",
        scalar: 1_152_921_504_606_846_976.0,
    },
    ScalarPrefix {
        name: "zebi",
        alias: "Zi",
        scalar: 1_180_591_620_717_411_303_424.0,
    },
    ScalarPrefix {
        name: "yobi",
        alias: "Yi",
        scalar: 1_208_925_819_614_629_174_706_176.0,
    },
];

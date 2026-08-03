use crate::{
    units::unit::Unit,
    AbacusError, Value, evaluation::tokenizer::registry::function::operators::FunctionOp,
};
use std::sync::Arc;

/// Ensure all arguments are compatible with the first argument's unit.
fn check_compatible_units<'a>(args: &'a [Value]) -> Result<&'a Arc<Unit>, AbacusError> {
    if args.is_empty() {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let first_unit = &args[0].unit;
    for val in args {
        if !val.unit.is_compatible_with(first_unit) {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    Ok(first_unit)
}

fn sum_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let sum: f64 = args.iter().map(|v| v.canonical).sum();

    Ok(Value {
        canonical: sum,
        unit: Arc::clone(first_unit),
    })
}

fn mean_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let sum: f64 = args.iter().map(|v| v.canonical).sum();

    Ok(Value {
        canonical: sum / (args.len() as f64),
        unit: Arc::clone(first_unit),
    })
}

fn min_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let min_val = args
        .iter()
        .map(|v| v.canonical)
        .fold(f64::INFINITY, f64::min);

    Ok(Value {
        canonical: min_val,
        unit: Arc::clone(first_unit),
    })
}

fn max_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let max_val = args
        .iter()
        .map(|v| v.canonical)
        .fold(f64::NEG_INFINITY, f64::max);

    Ok(Value {
        canonical: max_val,
        unit: Arc::clone(first_unit),
    })
}

fn range_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let min_val = args
        .iter()
        .map(|v| v.canonical)
        .fold(f64::INFINITY, f64::min);
    let max_val = args
        .iter()
        .map(|v| v.canonical)
        .fold(f64::NEG_INFINITY, f64::max);

    Ok(Value {
        canonical: max_val - min_val,
        unit: Arc::clone(first_unit),
    })
}

fn median_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let mut values: Vec<f64> = args.iter().map(|v| v.canonical).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let len = values.len();
    let median_val = if len % 2 == 0 {
        (values[len / 2 - 1] + values[len / 2]) / 2.0
    } else {
        values[len / 2]
    };

    Ok(Value {
        canonical: median_val,
        unit: Arc::clone(first_unit),
    })
}

fn mode_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    if args.is_empty() {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    // Find the most frequent canonical value (with small tolerance for floats)
    let mut max_count = 0;
    let mut mode_val = args[0].canonical;

    for (i, v1) in args.iter().enumerate() {
        let count = args
            .iter()
            .skip(i)
            .filter(|v2| (v1.canonical - v2.canonical).abs() < 1e-9)
            .count();
        if count > max_count {
            max_count = count;
            mode_val = v1.canonical;
        }
    }

    Ok(Value {
        canonical: mode_val,
        unit: Arc::clone(first_unit),
    })
}

fn var_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / (args.len() as f64);
    let variance = args
        .iter()
        .map(|v| (v.canonical - mean).powi(2))
        .sum::<f64>()
        / ((args.len() - 1) as f64); // sample variance

    // Variance unit is squared dimensions
    let squared_unit = Arc::new(Unit {
        scalar: first_unit.scalar * first_unit.scalar,
        offset: 0.0,
        dimensions: first_unit.dimensions * 2.0,
        display: first_unit.display.multiply(&first_unit.display),
    });

    Ok(Value {
        canonical: variance,
        unit: squared_unit,
    })
}

fn std_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / (args.len() as f64);
    let variance = args
        .iter()
        .map(|v| (v.canonical - mean).powi(2))
        .sum::<f64>()
        / ((args.len() - 1) as f64);
    let stdev = variance.sqrt();

    Ok(Value {
        canonical: stdev,
        unit: Arc::clone(first_unit),
    })
}

pub fn register_stats() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "sum",
            min_args: 1,
            max_args: usize::MAX,
            func: sum_fn,
        },
        FunctionOp {
            name: "mean",
            min_args: 1,
            max_args: usize::MAX,
            func: mean_fn,
        },
        FunctionOp {
            name: "min",
            min_args: 1,
            max_args: usize::MAX,
            func: min_fn,
        },
        FunctionOp {
            name: "max",
            min_args: 1,
            max_args: usize::MAX,
            func: max_fn,
        },
        FunctionOp {
            name: "range",
            min_args: 1,
            max_args: usize::MAX,
            func: range_fn,
        },
        FunctionOp {
            name: "median",
            min_args: 1,
            max_args: usize::MAX,
            func: median_fn,
        },
        FunctionOp {
            name: "mode",
            min_args: 1,
            max_args: usize::MAX,
            func: mode_fn,
        },
        FunctionOp {
            name: "var",
            min_args: 2,
            max_args: usize::MAX,
            func: var_fn,
        },
        FunctionOp {
            name: "variance",
            min_args: 2,
            max_args: usize::MAX,
            func: var_fn,
        },
        FunctionOp {
            name: "std",
            min_args: 2,
            max_args: usize::MAX,
            func: std_fn,
        },
        FunctionOp {
            name: "stdev",
            min_args: 2,
            max_args: usize::MAX,
            func: std_fn,
        },
    ]
}

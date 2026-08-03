use crate::{
    units::{unit::Unit, unit::UnitExpr, dimensions::Dimensions},
    AbacusError, Value, evaluation::tokenizer::registry::function::operators::FunctionOp,
};
use std::sync::Arc;

fn make_dimensionless(val: f64) -> Value {
    Value {
        canonical: val,
        unit: Arc::new(Unit {
            scalar: 1.0,
            offset: 0.0,
            dimensions: Dimensions::DIMENSIONLESS,
            display: UnitExpr::dimensionless(),
        }),
    }
}

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
        / ((args.len() - 1) as f64);

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

/// Helper for linear interpolation quantile calculation
fn calc_quantile(data: &[Value], q: f64) -> f64 {
    let mut values: Vec<f64> = data.iter().map(|v| v.canonical).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if values.len() == 1 {
        return values[0];
    }

    let pos = q * ((values.len() - 1) as f64);
    let idx = pos.floor() as usize;
    let frac = pos - (idx as f64);

    if idx >= values.len() - 1 {
        values[values.len() - 1]
    } else {
        values[idx] + frac * (values[idx + 1] - values[idx])
    }
}

/// quantile(data..., q) where q in [0, 1]
fn quantile_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let q_arg = &args[args.len() - 1];
    if !q_arg.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let q = q_arg.canonical;
    if q < 0.0 || q > 1.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let data = &args[..args.len() - 1];
    let first_unit = check_compatible_units(data)?;
    let val = calc_quantile(data, q);

    Ok(Value {
        canonical: val,
        unit: Arc::clone(first_unit),
    })
}

/// percentile(data..., p) where p in [0, 100]
fn percentile_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let p_arg = &args[args.len() - 1];
    if !p_arg.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let p = p_arg.canonical;
    if p < 0.0 || p > 100.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let data = &args[..args.len() - 1];
    let first_unit = check_compatible_units(data)?;
    let val = calc_quantile(data, p / 100.0);

    Ok(Value {
        canonical: val,
        unit: Arc::clone(first_unit),
    })
}

/// iqr(data...) -> Q3 - Q1
fn iqr_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let q3 = calc_quantile(args, 0.75);
    let q1 = calc_quantile(args, 0.25);

    Ok(Value {
        canonical: q3 - q1,
        unit: Arc::clone(first_unit),
    })
}

/// corr(x_range, y_range) or corr(x1..xN, y1..yN)
fn corr_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 4 || args.len() % 2 != 0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let n = args.len() / 2;
    let x_data = &args[..n];
    let y_data = &args[n..];

    let _x_unit = check_compatible_units(x_data)?;
    let _y_unit = check_compatible_units(y_data)?;

    let x_mean = x_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);
    let y_mean = y_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);

    let mut cov_sum = 0.0;
    let mut x_var_sum = 0.0;
    let mut y_var_sum = 0.0;

    for i in 0..n {
        let dx = x_data[i].canonical - x_mean;
        let dy = y_data[i].canonical - y_mean;
        cov_sum += dx * dy;
        x_var_sum += dx * dx;
        y_var_sum += dy * dy;
    }

    let denom = (x_var_sum * y_var_sum).sqrt();
    if denom == 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let r = cov_sum / denom;
    Ok(make_dimensionless(r))
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
        FunctionOp {
            name: "quantile",
            min_args: 2,
            max_args: usize::MAX,
            func: quantile_fn,
        },
        FunctionOp {
            name: "percentile",
            min_args: 2,
            max_args: usize::MAX,
            func: percentile_fn,
        },
        FunctionOp {
            name: "iqr",
            min_args: 1,
            max_args: usize::MAX,
            func: iqr_fn,
        },
        FunctionOp {
            name: "corr",
            min_args: 4,
            max_args: usize::MAX,
            func: corr_fn,
        },
        FunctionOp {
            name: "correlation",
            min_args: 4,
            max_args: usize::MAX,
            func: corr_fn,
        },
    ]
}

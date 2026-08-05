use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::operators::{FunctionOp, FunctionTarget},
    units::{dimensions::Dimensions, unit::Unit, unit::UnitExpr},
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

fn var_p_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    if args.is_empty() {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / (args.len() as f64);
    let variance = args
        .iter()
        .map(|v| (v.canonical - mean).powi(2))
        .sum::<f64>()
        / (args.len() as f64);

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

fn std_p_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    if args.is_empty() {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / (args.len() as f64);
    let variance = args
        .iter()
        .map(|v| (v.canonical - mean).powi(2))
        .sum::<f64>()
        / (args.len() as f64);
    let stdev = variance.sqrt();

    Ok(Value {
        canonical: stdev,
        unit: Arc::clone(first_unit),
    })
}

fn geomean_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    let mut log_sum = 0.0;
    for v in args {
        if v.canonical <= 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        log_sum += v.canonical.ln();
    }
    let gm = (log_sum / n).exp();

    Ok(Value {
        canonical: gm,
        unit: Arc::clone(first_unit),
    })
}

fn harmean_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    let mut inv_sum = 0.0;
    for v in args {
        if v.canonical == 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        inv_sum += 1.0 / v.canonical;
    }
    let hm = n / inv_sum;

    Ok(Value {
        canonical: hm,
        unit: Arc::clone(first_unit),
    })
}

fn cov_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 4 || args.len() % 2 != 0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let n = args.len() / 2;
    let x_data = &args[..n];
    let y_data = &args[n..];

    let x_unit = check_compatible_units(x_data)?;
    let y_unit = check_compatible_units(y_data)?;

    let x_mean = x_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);
    let y_mean = y_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);

    let mut cov_sum = 0.0;
    for i in 0..n {
        let dx = x_data[i].canonical - x_mean;
        let dy = y_data[i].canonical - y_mean;
        cov_sum += dx * dy;
    }

    let sample_cov = cov_sum / ((n - 1) as f64);
    let product_unit = Arc::new(Unit {
        scalar: x_unit.scalar * y_unit.scalar,
        offset: 0.0,
        dimensions: x_unit.dimensions + y_unit.dimensions,
        display: x_unit.display.multiply(&y_unit.display),
    });

    Ok(Value {
        canonical: sample_cov,
        unit: product_unit,
    })
}

fn cov_p_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 4 || args.len() % 2 != 0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let n = args.len() / 2;
    let x_data = &args[..n];
    let y_data = &args[n..];

    let x_unit = check_compatible_units(x_data)?;
    let y_unit = check_compatible_units(y_data)?;

    let x_mean = x_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);
    let y_mean = y_data.iter().map(|v| v.canonical).sum::<f64>() / (n as f64);

    let mut cov_sum = 0.0;
    for i in 0..n {
        let dx = x_data[i].canonical - x_mean;
        let dy = y_data[i].canonical - y_mean;
        cov_sum += dx * dy;
    }

    let pop_cov = cov_sum / (n as f64);
    let product_unit = Arc::new(Unit {
        scalar: x_unit.scalar * y_unit.scalar,
        offset: 0.0,
        dimensions: x_unit.dimensions + y_unit.dimensions,
        display: x_unit.display.multiply(&y_unit.display),
    });

    Ok(Value {
        canonical: pop_cov,
        unit: product_unit,
    })
}

fn skew_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let _first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    if n < 3.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / n;
    let mut m2 = 0.0;
    let mut m3 = 0.0;

    for v in args {
        let diff = v.canonical - mean;
        m2 += diff * diff;
        m3 += diff * diff * diff;
    }

    let var = m2 / n;
    if var == 0.0 {
        return Ok(make_dimensionless(0.0));
    }
    let skewness = (m3 / n) / var.powf(1.5);

    Ok(make_dimensionless(skewness))
}

fn kurt_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let _first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    if n < 4.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / n;
    let mut m2 = 0.0;
    let mut m4 = 0.0;

    for v in args {
        let diff = v.canonical - mean;
        m2 += diff * diff;
        m4 += diff.powi(4);
    }

    let var = m2 / n;
    if var == 0.0 {
        return Ok(make_dimensionless(0.0));
    }
    let excess_kurtosis = (m4 / n) / (var * var) - 3.0;

    Ok(make_dimensionless(excess_kurtosis))
}

fn mad_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    let mean = args.iter().map(|v| v.canonical).sum::<f64>() / n;
    let mad_val = args.iter().map(|v| (v.canonical - mean).abs()).sum::<f64>() / n;

    Ok(Value {
        canonical: mad_val,
        unit: Arc::clone(first_unit),
    })
}

fn rms_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let first_unit = check_compatible_units(args)?;
    let n = args.len() as f64;
    let mean_sq = args.iter().map(|v| v.canonical * v.canonical).sum::<f64>() / n;
    let rms_val = mean_sq.sqrt();

    Ok(Value {
        canonical: rms_val,
        unit: Arc::clone(first_unit),
    })
}

fn zscore_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() != 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let x = &args[0];
    let mean = &args[1];
    let std = &args[2];

    if !x.unit.is_compatible_with(&mean.unit) || !x.unit.is_compatible_with(&std.unit) {
        return Err(AbacusError::IncompatibleDimensions);
    }

    if std.canonical == 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let z = (x.canonical - mean.canonical) / std.canonical;
    Ok(make_dimensionless(z))
}

pub fn register_stats() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "sum",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(sum_fn),
        },
        FunctionOp {
            name: "mean",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(mean_fn),
        },
        FunctionOp {
            name: "geomean",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(geomean_fn),
        },
        FunctionOp {
            name: "harmean",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(harmean_fn),
        },
        FunctionOp {
            name: "min",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(min_fn),
        },
        FunctionOp {
            name: "max",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(max_fn),
        },
        FunctionOp {
            name: "range",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(range_fn),
        },
        FunctionOp {
            name: "median",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(median_fn),
        },
        FunctionOp {
            name: "mode",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(mode_fn),
        },
        FunctionOp {
            name: "var",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(var_fn),
        },
        FunctionOp {
            name: "var_s",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(var_fn),
        },
        FunctionOp {
            name: "var_p",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(var_p_fn),
        },
        FunctionOp {
            name: "variance",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(var_fn),
        },
        FunctionOp {
            name: "std",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(std_fn),
        },
        FunctionOp {
            name: "std_s",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(std_fn),
        },
        FunctionOp {
            name: "std_p",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(std_p_fn),
        },
        FunctionOp {
            name: "stdev",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(std_fn),
        },
        FunctionOp {
            name: "cov",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(cov_fn),
        },
        FunctionOp {
            name: "cov_s",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(cov_fn),
        },
        FunctionOp {
            name: "cov_p",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(cov_p_fn),
        },
        FunctionOp {
            name: "skew",
            min_args: 3,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(skew_fn),
        },
        FunctionOp {
            name: "skewness",
            min_args: 3,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(skew_fn),
        },
        FunctionOp {
            name: "kurt",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(kurt_fn),
        },
        FunctionOp {
            name: "kurtosis",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(kurt_fn),
        },
        FunctionOp {
            name: "mad",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(mad_fn),
        },
        FunctionOp {
            name: "rms",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(rms_fn),
        },
        FunctionOp {
            name: "zscore",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(zscore_fn),
        },
        FunctionOp {
            name: "standardize",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(zscore_fn),
        },
        FunctionOp {
            name: "quantile",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(quantile_fn),
        },
        FunctionOp {
            name: "percentile",
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(percentile_fn),
        },
        FunctionOp {
            name: "iqr",
            min_args: 1,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(iqr_fn),
        },
        FunctionOp {
            name: "corr",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(corr_fn),
        },
        FunctionOp {
            name: "correlation",
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(corr_fn),
        },
    ]
}

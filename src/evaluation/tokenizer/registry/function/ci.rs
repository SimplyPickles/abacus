use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        check_dimensionless,
        distributions::special::erfinv,
        distributions::student_t::compute_invt,
        operators::FunctionOp,
        stats::{compute_mean, compute_variance},
    },
    units::eval_result::EvalResult,
    units::interval::Interval,
};
use std::sync::Arc;

/// Helper to parse data array and optional trailing confidence level (e.g. 0.95)
fn parse_data_and_confidence(args: &[Value]) -> Result<(Vec<Value>, f64), AbacusError> {
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let last = &args[args.len() - 1];
    let (data_slice, conf) = if last.unit.is_dimensionless()
        && last.canonical > 0.0
        && last.canonical < 1.0
        && args.len() >= 3
    {
        (&args[..args.len() - 1], last.canonical)
    } else {
        (args, 0.95)
    };

    if data_slice.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let first_unit = &data_slice[0].unit;
    for v in data_slice {
        if !v.unit.is_compatible_with(first_unit) {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    Ok((data_slice.to_vec(), conf))
}

/// Helper to compute z* = sqrt(2) * erfinv(2p - 1)
fn z_critical(confidence: f64) -> Result<f64, AbacusError> {
    if confidence <= 0.0 || confidence >= 1.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let p = f64::midpoint(1.0, confidence);
    Ok(std::f64::consts::SQRT_2 * erfinv(2.0 * p - 1.0))
}

/// Helper to compute t* for given confidence and df
fn t_critical(confidence: f64, df: f64) -> Result<f64, AbacusError> {
    if confidence <= 0.0 || confidence >= 1.0 || df <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let p = f64::midpoint(1.0, confidence);
    let t = compute_invt(p, df);
    if t.is_nan() || t.is_infinite() {
        Err(AbacusError::IncompatibleFunctionArguments)
    } else {
        Ok(t)
    }
}

// ── TInterval(data..., [confidence]) or TInterval(mean, std_dev, n, [confidence]) ──
// TI-84 TInterval: 1-Sample T Confidence Interval
fn t_interval_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() >= 3
        && args.len() <= 4
        && args[2].unit.is_dimensionless()
        && args[2].canonical > 1.0
    {
        // Summary statistics mode: mean, std_dev, n [, confidence]
        let mean_val = &args[0];
        let std_val = &args[1];
        let n_val = &args[2];

        if !std_val.unit.is_compatible_with(&mean_val.unit) {
            return Err(AbacusError::IncompatibleDimensions);
        }

        let n = n_val.canonical;
        let std_dev = std_val.canonical;
        if n <= 1.0 || std_dev < 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }

        let conf = if args.len() == 4 {
            if !args[3].unit.is_dimensionless() {
                return Err(AbacusError::IncompatibleDimensions);
            }
            args[3].canonical
        } else {
            0.95
        };

        let se = std_dev / n.sqrt();
        let t_star = t_critical(conf, n - 1.0)?;
        let moe = t_star * se;

        let base = mean_val.amount();
        let step = moe / mean_val.unit.scalar;
        let lo = Value::new(base - step, Arc::clone(&mean_val.unit));
        let hi = Value::new(base + step, Arc::clone(&mean_val.unit));

        return Ok(EvalResult::Interval(Interval::new(lo, hi)?));
    }

    // Sample data mode: data... [, confidence]
    let (data, conf) = parse_data_and_confidence(args)?;
    let n = data.len() as f64;
    let unit = Arc::clone(&data[0].unit);

    let mean = compute_mean(&data);
    let std_dev = compute_variance(&data, 1.0).sqrt();
    let se = std_dev / n.sqrt();

    let t_star = t_critical(conf, n - 1.0)?;
    let moe = t_star * se;

    let lo = Value::new((mean - moe - unit.offset) / unit.scalar, Arc::clone(&unit));
    let hi = Value::new((mean + moe - unit.offset) / unit.scalar, unit);

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── ZInterval(mean, std_dev, n, [confidence]) ──
// TI-84 ZInterval: 1-Sample Z Confidence Interval
fn z_interval_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 3 || args.len() > 4 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean_val = &args[0];
    let std_val = &args[1];
    let n_val = &args[2];

    if !std_val.unit.is_compatible_with(&mean_val.unit) || !n_val.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let n = n_val.canonical;
    let std_dev = std_val.canonical;
    if n <= 0.0 || std_dev < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let conf = if args.len() == 4 {
        if !args[3].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        args[3].canonical
    } else {
        0.95
    };

    let se = std_dev / n.sqrt();
    let z_star = z_critical(conf)?;
    let moe = z_star * se;

    let base = mean_val.amount();
    let step = moe / mean_val.unit.scalar;
    let lo = Value::new(base - step, Arc::clone(&mean_val.unit));
    let hi = Value::new(base + step, Arc::clone(&mean_val.unit));

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── 1-PropZInt(x, n, [confidence]) ──
// TI-84 1-PropZInt: 1-Proportion Z Interval
fn one_prop_z_int_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    check_dimensionless(args)?;

    let x = args[0].canonical;
    let n = args[1].canonical;
    let conf = if args.len() == 3 {
        args[2].canonical
    } else {
        0.95
    };

    if n <= 0.0 || x < 0.0 || x > n {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let p = x / n;
    let z = z_critical(conf)?;
    let z2 = z * z;

    let denom = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denom;
    let moe = (z / denom) * ((p * (1.0 - p) / n) + (z2 / (4.0 * n * n))).sqrt();

    let lo = Value::dimensionless((center - moe).max(0.0));
    let hi = Value::dimensionless((center + moe).min(1.0));

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── 2-SampTInt(mean1, std1, n1, mean2, std2, n2, [confidence]) ──
// TI-84 2-SampTInt: 2-Sample T Interval for difference of means (mean1 - mean2)
fn two_samp_t_int_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 6 || args.len() > 7 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean1 = &args[0];
    let std1 = &args[1];
    let n1_val = &args[2];
    let mean2 = &args[3];
    let std2 = &args[4];
    let n2_val = &args[5];

    if !std1.unit.is_compatible_with(&mean1.unit)
        || !mean2.unit.is_compatible_with(&mean1.unit)
        || !std2.unit.is_compatible_with(&mean1.unit)
        || !n1_val.unit.is_dimensionless()
        || !n2_val.unit.is_dimensionless()
    {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let n1 = n1_val.canonical;
    let n2 = n2_val.canonical;
    let s1 = std1.canonical;
    let s2 = std2.canonical;

    if n1 <= 1.0 || n2 <= 1.0 || s1 < 0.0 || s2 < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let conf = if args.len() == 7 {
        if !args[6].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        args[6].canonical
    } else {
        0.95
    };

    let v1 = (s1 * s1) / n1;
    let v2 = (s2 * s2) / n2;
    let se = (v1 + v2).sqrt();

    // Welch-Satterthwaite degrees of freedom
    let num = (v1 + v2).powi(2);
    let den = (v1 * v1) / (n1 - 1.0) + (v2 * v2) / (n2 - 1.0);
    let df = num / den;

    let t_star = t_critical(conf, df)?;
    let moe = t_star * se;
    let diff = mean1.canonical - mean2.canonical;

    let lo = Value::new(
        (diff - moe - mean1.unit.offset) / mean1.unit.scalar,
        Arc::clone(&mean1.unit),
    );
    let hi = Value::new(
        (diff + moe - mean1.unit.offset) / mean1.unit.scalar,
        Arc::clone(&mean1.unit),
    );

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── 2-SampZInt(mean1, std1, n1, mean2, std2, n2, [confidence]) ──
// TI-84 2-SampZInt: 2-Sample Z Interval for difference of means (mean1 - mean2)
fn two_samp_z_int_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 6 || args.len() > 7 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mean1 = &args[0];
    let std1 = &args[1];
    let n1_val = &args[2];
    let mean2 = &args[3];
    let std2 = &args[4];
    let n2_val = &args[5];

    if !std1.unit.is_compatible_with(&mean1.unit)
        || !mean2.unit.is_compatible_with(&mean1.unit)
        || !std2.unit.is_compatible_with(&mean1.unit)
        || !n1_val.unit.is_dimensionless()
        || !n2_val.unit.is_dimensionless()
    {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let n1 = n1_val.canonical;
    let n2 = n2_val.canonical;
    let s1 = std1.canonical;
    let s2 = std2.canonical;

    if n1 <= 0.0 || n2 <= 0.0 || s1 < 0.0 || s2 < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let conf = if args.len() == 7 {
        if !args[6].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        args[6].canonical
    } else {
        0.95
    };

    let se = ((s1 * s1) / n1 + (s2 * s2) / n2).sqrt();
    let z_star = z_critical(conf)?;
    let moe = z_star * se;
    let diff = mean1.canonical - mean2.canonical;

    let lo = Value::new(
        (diff - moe - mean1.unit.offset) / mean1.unit.scalar,
        Arc::clone(&mean1.unit),
    );
    let hi = Value::new(
        (diff + moe - mean1.unit.offset) / mean1.unit.scalar,
        Arc::clone(&mean1.unit),
    );

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── 2-PropZInt(x1, n1, x2, n2, [confidence]) ──
// TI-84 2-PropZInt: 2-Proportion Z Interval for difference of proportions (p1 - p2)
fn two_prop_z_int_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 4 || args.len() > 5 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    check_dimensionless(args)?;

    let x1 = args[0].canonical;
    let n1 = args[1].canonical;
    let x2 = args[2].canonical;
    let n2 = args[3].canonical;
    let conf = if args.len() == 5 {
        args[4].canonical
    } else {
        0.95
    };

    if n1 <= 0.0 || n2 <= 0.0 || x1 < 0.0 || x1 > n1 || x2 < 0.0 || x2 > n2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let p1 = x1 / n1;
    let p2 = x2 / n2;
    let diff = p1 - p2;
    let se = ((p1 * (1.0 - p1)) / n1 + (p2 * (1.0 - p2)) / n2).sqrt();

    let z_star = z_critical(conf)?;
    let moe = z_star * se;

    let lo = Value::dimensionless(diff - moe);
    let hi = Value::dimensionless(diff + moe);

    Ok(EvalResult::Interval(Interval::new(lo, hi)?))
}

// ── moe(data..., [confidence]) ──
fn moe_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let (data, conf) = parse_data_and_confidence(args)?;
    let n = data.len() as f64;
    let unit = Arc::clone(&data[0].unit);

    let std_dev = compute_variance(&data, 1.0).sqrt();
    let se = std_dev / n.sqrt();

    let t_star = t_critical(conf, n - 1.0)?;
    let moe = t_star * se;

    Ok(Value::new(moe / unit.scalar, unit))
}

// ── tmoe(std_dev, n, [confidence]) ──
fn tmoe_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let std_val = &args[0];
    let n_val = &args[1];

    if !n_val.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let n = n_val.canonical;
    let std_dev = std_val.canonical;
    if n <= 1.0 || std_dev < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let conf = if args.len() == 3 {
        if !args[2].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        args[2].canonical
    } else {
        0.95
    };

    let se = std_dev / n.sqrt();
    let t_star = t_critical(conf, n - 1.0)?;
    let moe = t_star * se;

    Ok(Value::new(
        moe / std_val.unit.scalar,
        Arc::clone(&std_val.unit),
    ))
}

// ── zmoe(std_dev, n, [confidence]) ──
fn zmoe_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let std_val = &args[0];
    let n_val = &args[1];

    if !n_val.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let n = n_val.canonical;
    let std_dev = std_val.canonical;
    if n <= 0.0 || std_dev < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let conf = if args.len() == 3 {
        if !args[2].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        args[2].canonical
    } else {
        0.95
    };

    let se = std_dev / n.sqrt();
    let z_star = z_critical(conf)?;
    let moe = z_star * se;

    Ok(Value::new(
        moe / std_val.unit.scalar,
        Arc::clone(&std_val.unit),
    ))
}

pub fn register_ci() -> Vec<FunctionOp> {
    let mut ops = Vec::new();

    // ── TInterval aliases (TI-84 Option 8) ──
    for name in &["TInterval", "tinterval", "TInt", "t_interval", "ci", "tci"] {
        ops.push(FunctionOp::eval_result(name, 2, usize::MAX, t_interval_fn));
    }

    // ── ZInterval aliases (TI-84 Option 7) ──
    for name in &["ZInterval", "zinterval", "ZInt", "z_interval", "zci"] {
        ops.push(FunctionOp::eval_result(name, 3, 4, z_interval_fn));
    }

    // ── 1-PropZInt aliases (TI-84 Option A) ──
    for name in &[
        "1-PropZInt",
        "1_PropZInt",
        "1PropZInt",
        "1_prop_z_int",
        "propzint",
        "propci",
    ] {
        ops.push(FunctionOp::eval_result(name, 2, 3, one_prop_z_int_fn));
    }

    // ── 2-SampTInt aliases (TI-84 Option 0) ──
    for name in &[
        "2-SampTInt",
        "2_SampTInt",
        "2SampTInt",
        "2_samp_t_int",
        "samptint2",
    ] {
        ops.push(FunctionOp::eval_result(name, 6, 7, two_samp_t_int_fn));
    }

    // ── 2-SampZInt aliases (TI-84 Option 9) ──
    for name in &[
        "2-SampZInt",
        "2_SampZInt",
        "2SampZInt",
        "2_samp_z_int",
        "sampzint2",
    ] {
        ops.push(FunctionOp::eval_result(name, 6, 7, two_samp_z_int_fn));
    }

    // ── 2-PropZInt aliases (TI-84 Option B) ──
    for name in &[
        "2-PropZInt",
        "2_PropZInt",
        "2PropZInt",
        "2_prop_z_int",
        "propzint2",
    ] {
        ops.push(FunctionOp::eval_result(name, 4, 5, two_prop_z_int_fn));
    }

    // ── Margin of error helpers ──
    ops.push(FunctionOp::scalar("moe", 2, usize::MAX, moe_fn));
    ops.push(FunctionOp::scalar("tmoe", 2, 3, tmoe_fn));
    ops.push(FunctionOp::scalar("zmoe", 2, 3, zmoe_fn));

    ops
}

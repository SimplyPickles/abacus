use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        operators::{FunctionOp, FunctionTarget},
        stats::{compute_mean, parse_paired_data},
    },
    units::{eval_result::EvalResult, hash::Hash, unit::Unit, value::Value as AbacusValue},
};
use std::sync::Arc;

/// Parse x_data and y_data arrays from arguments by delegating to parse_paired_data.
fn parse_regression_inputs(args: &[Value]) -> Result<(&[Value], &[Value]), AbacusError> {
    let (x, y, _) = parse_paired_data(args)?;
    Ok((x, y))
}

/// Computes basic regression statistics (mean_x, mean_y, Sxx, Syy, Sxy, n)
struct RegStats {
    n: f64,
    mean_x: f64,
    mean_y: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
    x_unit: Arc<Unit>,
    y_unit: Arc<Unit>,
}

impl RegStats {
    fn compute(x_data: &[Value], y_data: &[Value]) -> Result<Self, AbacusError> {
        let n = x_data.len() as f64;
        if n < 2.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }

        let mean_x = compute_mean(x_data);
        let mean_y = compute_mean(y_data);

        let mut sxx = 0.0;
        let mut syy = 0.0;
        let mut sxy = 0.0;

        for (x, y) in x_data.iter().zip(y_data.iter()) {
            let dx = x.canonical - mean_x;
            let dy = y.canonical - mean_y;
            sxx += dx * dx;
            syy += dy * dy;
            sxy += dx * dy;
        }

        if sxx <= 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }

        Ok(Self {
            n,
            mean_x,
            mean_y,
            sxx,
            syy,
            sxy,
            x_unit: Arc::clone(&x_data[0].unit),
            y_unit: Arc::clone(&y_data[0].unit),
        })
    }

    fn slope(&self) -> f64 {
        self.sxy / self.sxx
    }

    fn intercept(&self) -> f64 {
        self.mean_y - self.slope() * self.mean_x
    }

    fn r(&self) -> f64 {
        if self.syy <= 0.0 {
            0.0
        } else {
            self.sxy / (self.sxx * self.syy).sqrt()
        }
    }

    fn r2(&self) -> f64 {
        let r_val = self.r();
        r_val * r_val
    }

    fn slope_unit(&self) -> Arc<Unit> {
        Arc::new(Unit {
            scalar: self.y_unit.scalar / self.x_unit.scalar,
            offset: 0.0,
            dimensions: self.y_unit.dimensions - self.x_unit.dimensions,
            display: self.y_unit.display.divide(&self.x_unit.display),
        })
    }
}

// ── linreg(x_data..., y_data...) -> EvalResult::Hash ──
fn linreg_hash_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;

    let mut hash = Hash::new();

    // slope
    let m = stats.slope();
    let slope_unit = stats.slope_unit();
    hash.insert(
        "slope",
        AbacusValue {
            canonical: m,
            unit: slope_unit,
        },
    );

    // intercept
    let b = stats.intercept();
    hash.insert(
        "intercept",
        AbacusValue {
            canonical: b,
            unit: Arc::clone(&stats.y_unit),
        },
    );

    // r2
    hash.insert("r2", Value::dimensionless(stats.r2()));

    // r
    hash.insert("r", Value::dimensionless(stats.r()));

    // se (if n > 2)
    if stats.n > 2.0 {
        let s_e = ((stats.syy - m * stats.sxy) / (stats.n - 2.0))
            .max(0.0)
            .sqrt();
        hash.insert(
            "se",
            AbacusValue {
                canonical: s_e,
                unit: Arc::clone(&stats.y_unit),
            },
        );
    }

    // mean_x & mean_y
    hash.insert(
        "mean_x",
        AbacusValue {
            canonical: stats.mean_x,
            unit: Arc::clone(&stats.x_unit),
        },
    );
    hash.insert(
        "mean_y",
        AbacusValue {
            canonical: stats.mean_y,
            unit: Arc::clone(&stats.y_unit),
        },
    );

    Ok(EvalResult::Hash(hash))
}

// ── linreg_slope(x_data..., y_data...) ──
fn linreg_slope_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;
    let m = stats.slope();
    let unit = stats.slope_unit();

    Ok(AbacusValue { canonical: m, unit })
}

// ── linreg_intercept(x_data..., y_data...) ──
fn linreg_intercept_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;
    let b = stats.intercept();

    Ok(AbacusValue {
        canonical: b,
        unit: stats.y_unit,
    })
}

// ── linreg_r2(x_data..., y_data...) ──
fn linreg_r2_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;
    Ok(Value::dimensionless(stats.r2()))
}

// ── linreg_r(x_data..., y_data...) ──
fn linreg_r_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;
    Ok(Value::dimensionless(stats.r()))
}

// ── linreg_se(x_data..., y_data...) ──
fn linreg_se_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    let (x_data, y_data) = parse_regression_inputs(args)?;
    let stats = RegStats::compute(x_data, y_data)?;

    if stats.n <= 2.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let m = stats.slope();
    let s_e = ((stats.syy - m * stats.sxy) / (stats.n - 2.0))
        .max(0.0)
        .sqrt();

    Ok(AbacusValue {
        canonical: s_e,
        unit: stats.y_unit,
    })
}

// ── linreg_predict(x_target, x_data..., y_data...) or linreg_predict(x_data..., y_data..., x_target) ──
fn linreg_predict_fn(args: &[Value]) -> Result<AbacusValue, AbacusError> {
    if args.len() < 5 || !(args.len() - 1).is_multiple_of(2) {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    // Determine if x_target is first or last argument
    let total = args.len();
    let _n = (total - 1) / 2;
    let last = total - 1;

    // Check if the last argument is x_target:
    // In postfix mode `(x_data..., y_data..., x_target)`, args[last] is compatible with args[0] (x_data[0]),
    // while args[last - 1] (y_data[n-1]) is NOT compatible with args[0] (unless X and Y have same units).
    let last_is_target = args[last].unit.is_compatible_with(&args[0].unit)
        && !args[last - 1].unit.is_compatible_with(&args[0].unit);

    let (x_target, reg_args) = if last_is_target {
        (&args[last], &args[..last])
    } else {
        (&args[0], &args[1..])
    };

    let (x_data, y_data) = parse_regression_inputs(reg_args)?;
    if !x_target.unit.is_compatible_with(&x_data[0].unit) {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let stats = RegStats::compute(x_data, y_data)?;
    let m = stats.slope();
    let b = stats.intercept();
    let y_pred = m * x_target.canonical + b;

    Ok(AbacusValue {
        canonical: y_pred,
        unit: stats.y_unit,
    })
}

pub fn register_regression() -> Vec<FunctionOp> {
    let mut ops = Vec::new();

    // linreg / LinReg -> Returns EvalResult::Hash containing slope, intercept, r2, r, se, mean_x, mean_y
    for name in &["linreg", "LinReg"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::EvalResult(linreg_hash_fn),
        });
    }

    // linreg_slope / LinRegSlope -> Returns scalar Value slope
    for name in &["linreg_slope", "LinRegSlope"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_slope_fn),
        });
    }

    // linreg_intercept / LinRegIntercept
    for name in &["linreg_intercept", "LinRegIntercept"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_intercept_fn),
        });
    }

    // linreg_r2 / LinRegR2 / r2
    for name in &["linreg_r2", "LinRegR2", "r2", "R2"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_r2_fn),
        });
    }

    // linreg_r / LinRegR
    for name in &["linreg_r", "LinRegR"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_r_fn),
        });
    }

    // linreg_se / LinRegSE
    for name in &["linreg_se", "LinRegSE"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_se_fn),
        });
    }

    // linreg_predict / LinRegPredict / predict
    for name in &["linreg_predict", "LinRegPredict", "predict"] {
        ops.push(FunctionOp {
            name,
            min_args: 5,
            max_args: usize::MAX,
            func: FunctionTarget::Scalar(linreg_predict_fn),
        });
    }

    ops
}

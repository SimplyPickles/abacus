use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::{beta_inc, erfinv, lgamma, make_dimensionless},
        operators::FunctionOp,
    },
};

fn compute_tpdf(df: f64, t: f64) -> f64 {
    let num_log = lgamma((df + 1.0) / 2.0);
    let den_log = (df * std::f64::consts::PI).ln() * 0.5 + lgamma(df / 2.0);
    let power = -((df + 1.0) / 2.0) * (1.0 + (t * t) / df).ln();
    (num_log - den_log + power).exp()
}

fn compute_tcdf(df: f64, t: f64) -> f64 {
    let x = df / (df + t * t);
    let ibeta = beta_inc(df / 2.0, 0.5, x);
    if t >= 0.0 {
        1.0 - 0.5 * ibeta
    } else {
        0.5 * ibeta
    }
}

/// tpdf(df, t)
fn tpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df = args[0].canonical;
    let t = args[1].canonical;

    if df <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    Ok(make_dimensionless(compute_tpdf(df, t)))
}

/// tcdf(df, t)
fn tcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df = args[0].canonical;
    let t = args[1].canonical;

    if df <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    Ok(make_dimensionless(compute_tcdf(df, t)))
}

/// invt(p, df)
fn invt_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let p = args[0].canonical;
    let df = args[1].canonical;

    if p <= 0.0 || p >= 1.0 || df <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let z = std::f64::consts::SQRT_2 * erfinv(2.0 * p - 1.0);
    let mut t = z;

    for _ in 0..20 {
        let cdf = compute_tcdf(df, t);
        let pdf = compute_tpdf(df, t);
        if pdf <= 0.0 {
            break;
        }
        let diff = cdf - p;
        if diff.abs() < 1e-12 {
            break;
        }
        t -= diff / pdf;
    }

    Ok(make_dimensionless(t))
}

pub fn register_student_t() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "tpdf",
            min_args: 2,
            max_args: 2,
            func: tpdf_fn,
        },
        FunctionOp {
            name: "tcdf",
            min_args: 2,
            max_args: 2,
            func: tcdf_fn,
        },
        FunctionOp {
            name: "invt",
            min_args: 2,
            max_args: 2,
            func: invt_fn,
        },
    ]
}

use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::{erfinv, gamma_p, lgamma, make_dimensionless},
        operators::FunctionOp,
    },
};

fn compute_chisqpdf(df: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let log_pdf =
        (df / 2.0 - 1.0) * x.ln() - x / 2.0 - (df / 2.0) * (2.0f64).ln() - lgamma(df / 2.0);
    log_pdf.exp()
}

fn compute_chisqcdf(df: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    gamma_p(df / 2.0, x / 2.0)
}

/// chisqpdf(df, x)
fn chisqpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df = args[0].canonical;
    let x = args[1].canonical;

    if df <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    Ok(make_dimensionless(compute_chisqpdf(df, x)))
}

/// chisqcdf(df, x)
fn chisqcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df = args[0].canonical;
    let x = args[1].canonical;

    if df <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    Ok(make_dimensionless(compute_chisqcdf(df, x)))
}

/// invchisq(p, df)
fn invchisq_fn(args: &[Value]) -> Result<Value, AbacusError> {
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

    // Wilson-Hilferty transformation initial guess
    let z = std::f64::consts::SQRT_2 * erfinv(2.0 * p - 1.0);
    let term = 1.0 - 2.0 / (9.0 * df) + z * (2.0 / (9.0 * df)).sqrt();
    let mut x = (df * term * term * term).max(1e-4);

    for _ in 0..25 {
        let cdf = compute_chisqcdf(df, x);
        let pdf = compute_chisqpdf(df, x);
        if pdf <= 0.0 {
            break;
        }
        let diff = cdf - p;
        if diff.abs() < 1e-12 {
            break;
        }
        x -= diff / pdf;
        if x < 1e-6 {
            x = 1e-6;
        }
    }

    Ok(make_dimensionless(x))
}

pub fn register_chi_square() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "chisqpdf",
            min_args: 2,
            max_args: 2,
            func: chisqpdf_fn,
        },
        FunctionOp {
            name: "chisqcdf",
            min_args: 2,
            max_args: 2,
            func: chisqcdf_fn,
        },
        FunctionOp {
            name: "invchisq",
            min_args: 2,
            max_args: 2,
            func: invchisq_fn,
        },
    ]
}

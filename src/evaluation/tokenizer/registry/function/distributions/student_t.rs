use crate::{
    evaluation::tokenizer::registry::function::{
        distributions::special::{beta_inc, lgamma, make_dimensionless},
        operators::FunctionOp,
    },
    AbacusError, Value,
};

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

    let num_log = lgamma((df + 1.0) / 2.0);
    let den_log = (df * std::f64::consts::PI).ln() * 0.5 + lgamma(df / 2.0);
    let power = -((df + 1.0) / 2.0) * (1.0 + (t * t) / df).ln();
    let pdf = (num_log - den_log + power).exp();

    Ok(make_dimensionless(pdf))
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

    let x = df / (df + t * t);
    let ibeta = beta_inc(df / 2.0, 0.5, x);

    let cdf = if t >= 0.0 {
        1.0 - 0.5 * ibeta
    } else {
        0.5 * ibeta
    };

    Ok(make_dimensionless(cdf))
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
    ]
}

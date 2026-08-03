use crate::{
    evaluation::tokenizer::registry::function::{
        distributions::special::{gamma_p, lgamma, make_dimensionless},
        operators::FunctionOp,
    },
    AbacusError, Value,
};

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

    let log_pdf = (df / 2.0 - 1.0) * x.ln() - x / 2.0 - (df / 2.0) * (2.0f64).ln() - lgamma(df / 2.0);
    Ok(make_dimensionless(log_pdf.exp()))
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

    let cdf = gamma_p(df / 2.0, x / 2.0);
    Ok(make_dimensionless(cdf))
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
    ]
}

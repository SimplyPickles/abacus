use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::{beta_inc, lgamma, make_dimensionless},
        operators::{FunctionOp, FunctionTarget},
    },
};

/// fpdf(df1, df2, x)
fn fpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df1 = args[0].canonical;
    let df2 = args[1].canonical;
    let x = args[2].canonical;

    if df1 <= 0.0 || df2 <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let log_num = (df1 / 2.0) * (df1 * x).ln() + (df2 / 2.0) * df2.ln();
    let log_den = lgamma(df1 / 2.0) + lgamma(df2 / 2.0) - lgamma((df1 + df2) / 2.0)
        + ((df1 + df2) / 2.0) * (df1 * x + df2).ln()
        + x.ln();

    let pdf = (log_num - log_den).exp();
    Ok(make_dimensionless(pdf))
}

/// fcdf(df1, df2, x)
fn fcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let df1 = args[0].canonical;
    let df2 = args[1].canonical;
    let x = args[2].canonical;

    if df1 <= 0.0 || df2 <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = df1 * x / (df1 * x + df2);
    let cdf = beta_inc(df1 / 2.0, df2 / 2.0, k);
    Ok(make_dimensionless(cdf))
}

pub fn register_f_dist() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "fpdf",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(fpdf_fn),
        },
        FunctionOp {
            name: "fcdf",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(fcdf_fn),
        },
    ]
}

use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        check_dimensionless,
        distributions::special::{beta_inc, lgamma},
        operators::FunctionOp,
    },
};

/// fpdf(df1, df2, x)
fn fpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    check_dimensionless(args)?;

    let df1 = args[0].canonical;
    let df2 = args[1].canonical;
    let x = args[2].canonical;

    if df1 <= 0.0 || df2 <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let log_num = (df1 / 2.0) * (df1 * x).ln() + (df2 / 2.0) * df2.ln();
    let log_den = lgamma(df1 / 2.0) + lgamma(df2 / 2.0) - lgamma(f64::midpoint(df1, df2))
        + f64::midpoint(df1, df2) * (df1 * x + df2).ln()
        + x.ln();

    let pdf = (log_num - log_den).exp();
    Ok(Value::dimensionless(pdf))
}

/// fcdf(df1, df2, x)
fn fcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    check_dimensionless(args)?;

    let df1 = args[0].canonical;
    let df2 = args[1].canonical;
    let x = args[2].canonical;

    if df1 <= 0.0 || df2 <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = df1 * x / (df1 * x + df2);
    let cdf = beta_inc(df1 / 2.0, df2 / 2.0, k);
    Ok(Value::dimensionless(cdf))
}

pub fn register_f_dist() -> Vec<FunctionOp> {
    vec![
        FunctionOp::scalar("fpdf", 3, 3, fpdf_fn),
        FunctionOp::scalar("fcdf", 3, 3, fcdf_fn),
    ]
}

use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        check_dimensionless,
        distributions::special::factorial,
        operators::FunctionOp,
    },
};

/// poissonpdf(lambda, k)
fn poissonpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    check_dimensionless(args)?;

    let lambda = args[0].canonical;
    let k_val = args[1].canonical;

    if lambda < 0.0 || k_val < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = k_val.round() as u64;
    let pdf = (lambda.powf(k as f64) * (-lambda).exp()) / factorial(k);
    Ok(Value::dimensionless(pdf))
}

/// poissoncdf(lambda, k)
fn poissoncdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    check_dimensionless(args)?;

    let lambda = args[0].canonical;
    let k_val = args[1].canonical;

    if lambda < 0.0 || k_val < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = k_val.round() as u64;
    let mut cdf = 0.0;
    for i in 0..=k {
        cdf += (lambda.powf(i as f64) * (-lambda).exp()) / factorial(i);
    }

    Ok(Value::dimensionless(cdf))
}

pub fn register_poisson() -> Vec<FunctionOp> {
    vec![
        FunctionOp::scalar("poissonpdf", 2, 2, poissonpdf_fn),
        FunctionOp::scalar("poissoncdf", 2, 2, poissoncdf_fn),
    ]
}

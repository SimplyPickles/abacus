use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless, operators::FunctionOp,
    },
};

fn factorial(n: u64) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * (x as f64))
}

/// poissonpdf(lambda, k)
fn poissonpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let lambda = args[0].canonical;
    let k_val = args[1].canonical;

    if lambda < 0.0 || k_val < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = k_val.round() as u64;
    let pdf = (lambda.powf(k as f64) * (-lambda).exp()) / factorial(k);
    Ok(make_dimensionless(pdf))
}

/// poissoncdf(lambda, k)
fn poissoncdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

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

    Ok(make_dimensionless(cdf))
}

pub fn register_poisson() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "poissonpdf",
            min_args: 2,
            max_args: 2,
            func: poissonpdf_fn,
        },
        FunctionOp {
            name: "poissoncdf",
            min_args: 2,
            max_args: 2,
            func: poissoncdf_fn,
        },
    ]
}

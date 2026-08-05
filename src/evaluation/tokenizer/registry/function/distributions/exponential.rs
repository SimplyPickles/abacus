use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless,
        operators::{FunctionOp, FunctionTarget},
    },
};

/// exppdf(lambda, x)
fn exppdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let lambda = args[0].canonical;
    let x = args[1].canonical;

    if lambda <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let pdf = lambda * (-lambda * x).exp();
    Ok(make_dimensionless(pdf))
}

/// expcdf(lambda, x)
fn expcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let lambda = args[0].canonical;
    let x = args[1].canonical;

    if lambda <= 0.0 || x < 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let cdf = 1.0 - (-lambda * x).exp();
    Ok(make_dimensionless(cdf))
}

/// invexp(p, lambda)
fn invexp_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let p = args[0].canonical;
    let lambda = args[1].canonical;

    if p <= 0.0 || p >= 1.0 || lambda <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let x = -(1.0 - p).ln() / lambda;
    Ok(make_dimensionless(x))
}

pub fn register_exponential() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "exppdf",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(exppdf_fn),
        },
        FunctionOp {
            name: "expcdf",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(expcdf_fn),
        },
        FunctionOp {
            name: "invexp",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(invexp_fn),
        },
    ]
}

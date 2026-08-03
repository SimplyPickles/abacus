use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless, operators::FunctionOp,
    },
};

/// unifpdf(a, b, x)
fn unifpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let a = args[0].canonical;
    let b = args[1].canonical;
    let x = args[2].canonical;

    if a >= b {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let pdf = if x >= a && x <= b { 1.0 / (b - a) } else { 0.0 };

    Ok(make_dimensionless(pdf))
}

/// unifcdf(a, b, x)
fn unifcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let a = args[0].canonical;
    let b = args[1].canonical;
    let x = args[2].canonical;

    if a >= b {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let cdf = if x < a {
        0.0
    } else if x > b {
        1.0
    } else {
        (x - a) / (b - a)
    };

    Ok(make_dimensionless(cdf))
}

/// invunif(p, a, b)
fn invunif_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let p = args[0].canonical;
    let a = args[1].canonical;
    let b = args[2].canonical;

    if p < 0.0 || p > 1.0 || a >= b {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let x = a + p * (b - a);
    Ok(make_dimensionless(x))
}

pub fn register_uniform() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "unifpdf",
            min_args: 3,
            max_args: 3,
            func: unifpdf_fn,
        },
        FunctionOp {
            name: "unifcdf",
            min_args: 3,
            max_args: 3,
            func: unifcdf_fn,
        },
        FunctionOp {
            name: "invunif",
            min_args: 3,
            max_args: 3,
            func: invunif_fn,
        },
    ]
}

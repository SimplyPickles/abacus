use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless,
        operators::{FunctionOp, FunctionTarget},
    },
};

/// geompdf(p, k)
fn geompdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let p = args[0].canonical;
    let k_val = args[1].canonical;

    if p <= 0.0 || p > 1.0 || k_val < 1.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = k_val.round() as i32;
    let pdf = (1.0 - p).powi(k - 1) * p;
    Ok(make_dimensionless(pdf))
}

/// geomcdf(p, k)
fn geomcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let p = args[0].canonical;
    let k_val = args[1].canonical;

    if p <= 0.0 || p > 1.0 || k_val < 1.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = k_val.round() as i32;
    let cdf = 1.0 - (1.0 - p).powi(k);
    Ok(make_dimensionless(cdf))
}

pub fn register_geometric() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "geompdf",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(geompdf_fn),
        },
        FunctionOp {
            name: "geomcdf",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(geomcdf_fn),
        },
    ]
}

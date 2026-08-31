use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::{make_dimensionless, n_cr},
        operators::{FunctionOp, FunctionTarget},
    },
};

/// binompdf(n, p, k)
fn binompdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let n_val = args[0].canonical;
    let p_val = args[1].canonical;
    let k_val = args[2].canonical;

    if n_val < 0.0 || !(0.0..=1.0).contains(&p_val) || k_val < 0.0 || k_val > n_val {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let n = n_val.round() as u64;
    let k = k_val.round() as u64;

    let pdf = n_cr(n, k) * p_val.powi(k as i32) * (1.0 - p_val).powi((n - k) as i32);
    Ok(make_dimensionless(pdf))
}

/// binomcdf(n, p, k)
fn binomcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let n_val = args[0].canonical;
    let p_val = args[1].canonical;
    let k_val = args[2].canonical;

    if n_val < 0.0 || !(0.0..=1.0).contains(&p_val) || k_val < 0.0 || k_val > n_val {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let n = n_val.round() as u64;
    let k = k_val.round() as u64;

    let mut cdf = 0.0;
    for i in 0..=k {
        cdf += n_cr(n, i) * p_val.powi(i as i32) * (1.0 - p_val).powi((n - i) as i32);
    }

    Ok(make_dimensionless(cdf))
}

pub fn register_binomial() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "binompdf",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(binompdf_fn),
        },
        FunctionOp {
            name: "binomcdf",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(binomcdf_fn),
        },
    ]
}

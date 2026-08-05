use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::{make_dimensionless, n_cr},
        operators::{FunctionOp, FunctionTarget},
    },
};

/// hypgeompdf(N, K, n, k)
fn hypgeompdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let big_n = args[0].canonical.round() as u64;
    let big_k = args[1].canonical.round() as u64;
    let n = args[2].canonical.round() as u64;
    let k = args[3].canonical.round() as u64;

    if big_k > big_n || n > big_n || k > n || k > big_k {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let pdf = (n_cr(big_k, k) * n_cr(big_n - big_k, n - k)) / n_cr(big_n, n);
    Ok(make_dimensionless(pdf))
}

/// hypgeomcdf(N, K, n, k)
fn hypgeomcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }

    let big_n = args[0].canonical.round() as u64;
    let big_k = args[1].canonical.round() as u64;
    let n = args[2].canonical.round() as u64;
    let k = args[3].canonical.round() as u64;

    if big_k > big_n || n > big_n || k > n || k > big_k {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mut cdf = 0.0;
    for i in 0..=k {
        if i <= big_k && (n - i) <= (big_n - big_k) {
            cdf += (n_cr(big_k, i) * n_cr(big_n - big_k, n - i)) / n_cr(big_n, n);
        }
    }

    Ok(make_dimensionless(cdf))
}

pub fn register_hypergeometric() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "hypgeompdf",
            min_args: 4,
            max_args: 4,
            func: FunctionTarget::Scalar(hypgeompdf_fn),
        },
        FunctionOp {
            name: "hypgeomcdf",
            min_args: 4,
            max_args: 4,
            func: FunctionTarget::Scalar(hypgeomcdf_fn),
        },
    ]
}

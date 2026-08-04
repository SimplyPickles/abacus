use crate::{
    evaluation::tokenizer::registry::function::{
        distributions::special::{make_dimensionless, n_cr},
        operators::FunctionOp,
    },
    AbacusError, Value,
};

fn n_pr(n: u64, r: u64) -> f64 {
    if r > n {
        return 0.0;
    }
    ((n - r + 1)..=n).fold(1.0, |acc, x| acc * (x as f64))
}

fn factorial_u64(n: u64) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * (x as f64))
}

/// factorial(n)
fn factorial_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let n_val = args[0].canonical;
    if n_val < 0.0 || !n_val.is_finite() || n_val > 170.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let n = n_val.round() as u64;
    Ok(make_dimensionless(factorial_u64(n)))
}

/// nCr(n, r) or comb(n, r)
fn n_cr_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    let n_val = args[0].canonical;
    let r_val = args[1].canonical;
    if n_val < 0.0 || r_val < 0.0 || r_val > n_val || !n_val.is_finite() || !r_val.is_finite() || n_val > 1000.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let n = n_val.round() as u64;
    let r = r_val.round() as u64;
    Ok(make_dimensionless(n_cr(n, r)))
}

/// nPr(n, r) or perm(n, r)
fn n_pr_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    let n_val = args[0].canonical;
    let r_val = args[1].canonical;
    if n_val < 0.0 || r_val < 0.0 || r_val > n_val || !n_val.is_finite() || !r_val.is_finite() || n_val > 170.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    let n = n_val.round() as u64;
    let r = r_val.round() as u64;
    Ok(make_dimensionless(n_pr(n, r)))
}

pub fn register_combinatorics() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "factorial",
            min_args: 1,
            max_args: 1,
            func: factorial_fn,
        },
        FunctionOp {
            name: "nCr",
            min_args: 2,
            max_args: 2,
            func: n_cr_fn,
        },
        FunctionOp {
            name: "comb",
            min_args: 2,
            max_args: 2,
            func: n_cr_fn,
        },
        FunctionOp {
            name: "nPr",
            min_args: 2,
            max_args: 2,
            func: n_pr_fn,
        },
        FunctionOp {
            name: "perm",
            min_args: 2,
            max_args: 2,
            func: n_pr_fn,
        },
    ]
}

use crate::{
    units::{dimensions::Dimensions, unit::Unit, unit::UnitExpr},
    AbacusError, Value, evaluation::tokenizer::registry::function::operators::FunctionOp,
};
use std::sync::Arc;

fn n_cr(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut c = 1.0;
    for i in 0..k {
        c = c * (n - i) as f64 / (i + 1) as f64;
    }
    c
}

fn make_dimensionless(val: f64) -> Value {
    Value {
        canonical: val,
        unit: Arc::new(Unit {
            scalar: 1.0,
            offset: 0.0,
            dimensions: Dimensions::DIMENSIONLESS,
            display: UnitExpr::dimensionless(),
        }),
    }
}

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

    if n_val < 0.0 || p_val < 0.0 || p_val > 1.0 || k_val < 0.0 || k_val > n_val {
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

    if n_val < 0.0 || p_val < 0.0 || p_val > 1.0 || k_val < 0.0 || k_val > n_val {
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
            func: binompdf_fn,
        },
        FunctionOp {
            name: "binomcdf",
            min_args: 3,
            max_args: 3,
            func: binomcdf_fn,
        },
    ]
}

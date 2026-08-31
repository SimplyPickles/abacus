use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless,
        operators::{FunctionOp, FunctionTarget},
    },
};
use std::sync::Arc;

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_u64(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd_u64(a, b)) * b
    }
}

/// clamp(x, min, max) — clamps x within [min, max]
fn clamp_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let x = &args[0];
    let min = &args[1];
    let max = &args[2];

    // Check unit compatibility or promote dimensionless bounds
    let min_canonical = if min.unit.is_compatible_with(&x.unit) {
        min.canonical
    } else if min.unit.is_dimensionless() && !x.unit.is_dimensionless() {
        let min_amount = min.amount();
        Value::new(min_amount, Arc::clone(&x.unit)).canonical
    } else {
        return Err(AbacusError::IncompatibleDimensions);
    };

    let max_canonical = if max.unit.is_compatible_with(&x.unit) {
        max.canonical
    } else if max.unit.is_dimensionless() && !x.unit.is_dimensionless() {
        let max_amount = max.amount();
        Value::new(max_amount, Arc::clone(&x.unit)).canonical
    } else {
        return Err(AbacusError::IncompatibleDimensions);
    };

    if min_canonical > max_canonical {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let clamped_canonical = x.canonical.clamp(min_canonical, max_canonical);

    Ok(Value {
        canonical: clamped_canonical,
        unit: Arc::clone(&x.unit),
    })
}

/// gcd(a, b) — Greatest Common Divisor
fn gcd_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    let a = args[0].canonical.abs().round() as u64;
    let b = args[1].canonical.abs().round() as u64;
    Ok(make_dimensionless(gcd_u64(a, b) as f64))
}

/// lcm(a, b) — Least Common Multiple
fn lcm_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    let a = args[0].canonical.abs().round() as u64;
    let b = args[1].canonical.abs().round() as u64;
    Ok(make_dimensionless(lcm_u64(a, b) as f64))
}

/// modulo_fn(a, b) — Unit-aware modulo calculation
pub fn compute_modulo(a: &Value, b: &Value) -> Result<Value, AbacusError> {
    if a.unit.is_affine() || b.unit.is_affine() {
        return Err(AbacusError::AffineUnitOperation("modulo"));
    }

    if a.unit.is_compatible_with(&b.unit) {
        let a_amount = a.amount();
        let b_amount = b.amount();
        if b_amount == 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let rem_amount = a_amount % b_amount;
        Ok(Value::new(rem_amount, Arc::clone(&a.unit)))
    } else if b.unit.is_dimensionless() && !a.unit.is_dimensionless() {
        let a_amount = a.amount();
        let b_amount = b.amount();
        if b_amount == 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let rem_amount = a_amount % b_amount;
        Ok(Value::new(rem_amount, Arc::clone(&a.unit)))
    } else if a.unit.is_dimensionless() && !b.unit.is_dimensionless() {
        let a_amount = a.amount();
        let b_amount = b.amount();
        if b_amount == 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let rem_amount = a_amount % b_amount;
        Ok(Value::new(rem_amount, Arc::clone(&b.unit)))
    } else {
        Err(AbacusError::IncompatibleDimensions)
    }
}

fn modulo_fn(args: &[Value]) -> Result<Value, AbacusError> {
    compute_modulo(&args[0], &args[1])
}

pub fn register_math_helpers() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "clamp",
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::Scalar(clamp_fn),
        },
        FunctionOp {
            name: "gcd",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(gcd_fn),
        },
        FunctionOp {
            name: "lcm",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(lcm_fn),
        },
        FunctionOp {
            name: "mod",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(modulo_fn),
        },
        FunctionOp {
            name: "modulo",
            min_args: 2,
            max_args: 2,
            func: FunctionTarget::Scalar(modulo_fn),
        },
    ]
}

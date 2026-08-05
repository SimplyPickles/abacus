use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless,
        operators::{FunctionOp, FunctionTarget},
    },
};
use std::sync::Arc;

/// ln(x) — Natural logarithm
fn ln_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let x = args[0].canonical;
    if x <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    Ok(make_dimensionless(x.ln()))
}

/// log10(x) — Base-10 logarithm
fn log10_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let x = args[0].canonical;
    if x <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    Ok(make_dimensionless(x.log10()))
}

/// log2(x) — Base-2 logarithm
fn log2_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let x = args[0].canonical;
    if x <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    Ok(make_dimensionless(x.log2()))
}

/// log(x) [base 10] or log(x, base)
fn log_fn(args: &[Value]) -> Result<Value, AbacusError> {
    for arg in args {
        if !arg.unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
    }
    let x = args[0].canonical;
    if x <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    if args.len() == 1 {
        Ok(make_dimensionless(x.log10()))
    } else {
        let base = args[1].canonical;
        if base <= 0.0 || base == 1.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        Ok(make_dimensionless(x.log(base)))
    }
}

/// exp(x) — Exponential e^x
fn exp_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let x = args[0].canonical;
    Ok(make_dimensionless(x.exp()))
}

/// abs(x) — Absolute value (preserves unit)
fn abs_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let val = &args[0];
    let canonical = val.canonical.abs();
    Ok(Value {
        canonical,
        unit: Arc::clone(&val.unit),
    })
}

/// floor(x) — Floor function (preserves unit)
fn floor_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let val = &args[0];
    let unit_amount = (val.canonical - val.unit.offset) / val.unit.scalar;
    let floored_amount = unit_amount.floor();
    Ok(Value::new(floored_amount, Arc::clone(&val.unit)))
}

/// ceil(x) — Ceiling function (preserves unit)
fn ceil_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let val = &args[0];
    let unit_amount = (val.canonical - val.unit.offset) / val.unit.scalar;
    let ceiled_amount = unit_amount.ceil();
    Ok(Value::new(ceiled_amount, Arc::clone(&val.unit)))
}

/// round(x) — Rounding function (preserves unit)
fn round_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let val = &args[0];
    let unit_amount = (val.canonical - val.unit.offset) / val.unit.scalar;
    let rounded_amount = unit_amount.round();
    Ok(Value::new(rounded_amount, Arc::clone(&val.unit)))
}

/// sign(x) — Signum (1.0, 0.0, -1.0)
fn sign_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let x = args[0].canonical;
    let s = if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    };
    Ok(make_dimensionless(s))
}

pub fn register_math() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "ln",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(ln_fn),
        },
        FunctionOp {
            name: "log10",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(log10_fn),
        },
        FunctionOp {
            name: "log2",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(log2_fn),
        },
        FunctionOp {
            name: "log",
            min_args: 1,
            max_args: 2,
            func: FunctionTarget::Scalar(log_fn),
        },
        FunctionOp {
            name: "exp",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(exp_fn),
        },
        FunctionOp {
            name: "abs",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(abs_fn),
        },
        FunctionOp {
            name: "floor",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(floor_fn),
        },
        FunctionOp {
            name: "ceil",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(ceil_fn),
        },
        FunctionOp {
            name: "round",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(round_fn),
        },
        FunctionOp {
            name: "sign",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(sign_fn),
        },
    ]
}

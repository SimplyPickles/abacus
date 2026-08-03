use crate::{
    AbacusError, Unit, Value,
    evaluation::tokenizer::registry::function::operators::FunctionOp,
    units::{dimensions::Dimensions, unit::UnitExpr},
};
use std::sync::Arc;

fn parse_angle_radians(val: &Value) -> Result<f64, AbacusError> {
    if val.unit.is_dimensionless() {
        Ok(val.canonical)
    } else {
        Err(AbacusError::IncompatibleDimensions)
    }
}

fn make_dimensionless(value: f64) -> Value {
    Value {
        canonical: value,
        unit: Arc::new(Unit {
            scalar: 1.0,
            offset: 0.0,
            dimensions: Dimensions::DIMENSIONLESS,
            display: UnitExpr::dimensionless(),
        }),
    }
}

fn make_angle_rad(radians: f64) -> Value {
    Value {
        canonical: radians,
        unit: Arc::new(Unit {
            scalar: 1.0,
            offset: 0.0,
            dimensions: Dimensions::DIMENSIONLESS,
            display: UnitExpr::single("rad"),
        }),
    }
}

fn sin_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.sin()))
}

fn cos_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.cos()))
}

fn tan_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.tan()))
}

fn asin_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.asin()))
}

fn acos_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.acos()))
}

fn atan_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.atan()))
}

fn atan2_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_compatible_with(&args[1].unit) {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let y = args[0].canonical;
    let x = args[1].canonical;
    Ok(make_angle_rad(y.atan2(x)))
}

fn sinh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.sinh()))
}

fn cosh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.cosh()))
}

fn tanh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(make_dimensionless(rad.tanh()))
}

fn asinh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.asinh()))
}

fn acosh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.acosh()))
}

fn atanh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    if !args[0].unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    Ok(make_angle_rad(args[0].canonical.atanh()))
}

pub fn register_trig() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "sin",
            min_args: 1,
            max_args: 1,
            func: sin_fn,
        },
        FunctionOp {
            name: "cos",
            min_args: 1,
            max_args: 1,
            func: cos_fn,
        },
        FunctionOp {
            name: "tan",
            min_args: 1,
            max_args: 1,
            func: tan_fn,
        },
        FunctionOp {
            name: "asin",
            min_args: 1,
            max_args: 1,
            func: asin_fn,
        },
        FunctionOp {
            name: "acos",
            min_args: 1,
            max_args: 1,
            func: acos_fn,
        },
        FunctionOp {
            name: "atan",
            min_args: 1,
            max_args: 1,
            func: atan_fn,
        },
        FunctionOp {
            name: "atan2",
            min_args: 2,
            max_args: 2,
            func: atan2_fn,
        },
        FunctionOp {
            name: "sinh",
            min_args: 1,
            max_args: 1,
            func: sinh_fn,
        },
        FunctionOp {
            name: "cosh",
            min_args: 1,
            max_args: 1,
            func: cosh_fn,
        },
        FunctionOp {
            name: "tanh",
            min_args: 1,
            max_args: 1,
            func: tanh_fn,
        },
        FunctionOp {
            name: "asinh",
            min_args: 1,
            max_args: 1,
            func: asinh_fn,
        },
        FunctionOp {
            name: "acosh",
            min_args: 1,
            max_args: 1,
            func: acosh_fn,
        },
        FunctionOp {
            name: "atanh",
            min_args: 1,
            max_args: 1,
            func: atanh_fn,
        },
    ]
}

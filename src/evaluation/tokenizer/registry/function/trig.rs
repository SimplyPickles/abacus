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
    Ok(Value::dimensionless(rad.sin()))
}

fn cos_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(Value::dimensionless(rad.cos()))
}

fn tan_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(Value::dimensionless(rad.tan()))
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
    Ok(Value::dimensionless(rad.sinh()))
}

fn cosh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(Value::dimensionless(rad.cosh()))
}

fn tanh_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let rad = parse_angle_radians(&args[0])?;
    Ok(Value::dimensionless(rad.tanh()))
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
        FunctionOp::scalar("sin", 1, 1, sin_fn),
        FunctionOp::scalar("cos", 1, 1, cos_fn),
        FunctionOp::scalar("tan", 1, 1, tan_fn),
        FunctionOp::scalar("asin", 1, 1, asin_fn),
        FunctionOp::scalar("acos", 1, 1, acos_fn),
        FunctionOp::scalar("atan", 1, 1, atan_fn),
        FunctionOp::scalar("atan2", 2, 2, atan2_fn),
        FunctionOp::scalar("sinh", 1, 1, sinh_fn),
        FunctionOp::scalar("cosh", 1, 1, cosh_fn),
        FunctionOp::scalar("tanh", 1, 1, tanh_fn),
        FunctionOp::scalar("asinh", 1, 1, asinh_fn),
        FunctionOp::scalar("acosh", 1, 1, acosh_fn),
        FunctionOp::scalar("atanh", 1, 1, atanh_fn),
    ]
}

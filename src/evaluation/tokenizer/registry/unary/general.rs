use crate::{
    units::dimensions::Dimensions,
    evaluation::tokenizer::registry::unary::operators::UnaryOp, AbacusError, Unit, Value,
};
use std::sync::Arc;

fn negate(a: Value) -> Result<Value, AbacusError> {
    if a.unit.is_affine() {
        return Err(AbacusError::AffineUnitOperation("negate"));
    }
    Ok(Value {
        canonical: -a.canonical,
        unit: a.unit,
    })
}

fn sqrt(a: Value) -> Result<Value, AbacusError> {
    if a.unit.is_affine() {
        return Err(AbacusError::AffineUnitOperation("sqrt"));
    }
    if a.canonical < 0.0 {
        return Err(AbacusError::IncompatibleDimensions);
    }

    let mut new_dims = [0i8; 8];
    for (i, &dim) in a.unit.dimensions.0.iter().enumerate() {
        if dim % 2 != 0 {
            return Err(AbacusError::IncompatibleDimensions);
        }
        new_dims[i] = dim / 2;
    }

    let mut new_display = a.unit.display.clone();
    if new_display.numerator.len() % 2 == 0 && new_display.denominator.len() % 2 == 0 {
        new_display.numerator.truncate(new_display.numerator.len() / 2);
        new_display.denominator.truncate(new_display.denominator.len() / 2);
    }

    let new_unit = Arc::new(Unit {
        scalar: a.unit.scalar.sqrt(),
        offset: 0.0,
        dimensions: Dimensions(new_dims),
        display: new_display,
    });

    Ok(Value {
        canonical: a.canonical.sqrt(),
        unit: new_unit,
    })
}

fn factorial(a: Value) -> Result<Value, AbacusError> {
    if !a.unit.is_dimensionless() {
        return Err(AbacusError::IncompatibleDimensions);
    }
    let n = a.canonical as u64;
    let result = (1..=n).fold(1u64, |acc, x| acc * x) as f64;
    Ok(Value {
        canonical: result,
        unit: a.unit,
    })
}


pub fn register_general() -> Vec<UnaryOp> {
    vec![
        // Negation shares precedence 2 with sqrt so that `sqrt -9` correctly
        // negates first, then takes the root. Both run before `^` (precedence 3)
        // so `-2^2` evaluates as -(2^2) = -4, matching mathematical convention.
        UnaryOp {
            alias: "-",
            func: negate,
            precedence: 2,
            prefix: true,
        },
        UnaryOp {
            alias: "sqrt",
            func: sqrt,
            precedence: 2,
            prefix: false,
        },
        UnaryOp {
            alias: "!",
            func: factorial,
            precedence: 3,
            prefix: false,
        },
    ]
}

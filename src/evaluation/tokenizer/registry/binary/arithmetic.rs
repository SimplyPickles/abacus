use crate::{
    evaluation::tokenizer::registry::binary::operators::BinaryOp, AbacusError, Value,
};


fn add(a: Value, b: Value) -> Result<Value, AbacusError> {
    a + b
}

fn sub(a: Value, b: Value) -> Result<Value, AbacusError> {
    a - b
}

fn mul(a: Value, b: Value) -> Result<Value, AbacusError> {
    a * b
}

fn div(a: Value, b: Value) -> Result<Value, AbacusError> {
    a / b
}


fn exp(a: Value, b: Value) -> Result<Value, AbacusError> {
    if a.unit.is_affine() || b.unit.is_affine() {
        return Err(AbacusError::AffineUnitOperation("multiply"));
    }

    if b.unit.is_dimensionless() {
        Ok(Value {
            canonical: a.canonical.powf(b.canonical),
            unit: a.unit,
        })
    } else {
        Err(AbacusError::IncompatibleDimensions)
    }
}

pub fn register_arithmetic() -> Vec<BinaryOp> {
    vec![
        BinaryOp {
            alias: "+",
            func: add,
            precedence: 0,
            right_associative: false,
        },
        BinaryOp {
            alias: "-",
            func: sub,
            precedence: 0,
            right_associative: false,
        },
        BinaryOp {
            alias: "*",
            func: mul,
            precedence: 1,
            right_associative: false,
        },
        BinaryOp {
            alias: "/",
            func: div,
            precedence: 1,
            right_associative: false,
        },
        // Exponentiation has higher precedence than multiplication and is
        // right-associative: 2^3^2 = 2^(3^2) = 512, not (2^3)^2 = 64.
        BinaryOp {
            alias: "^",
            func: exp,
            precedence: 3,
            right_associative: true,
        },
    ]
}

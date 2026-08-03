use crate::{AbacusError, Value};

#[derive(Debug)]
pub struct UnaryOp {
    pub alias: &'static str,
    pub func: fn(Value) -> Result<Value, AbacusError>,
    pub precedence: u8,
    pub prefix: bool,
}

impl UnaryOp {
    pub fn apply(&self, value: Value) -> Result<Value, AbacusError> {
        (self.func)(value)
    }
}


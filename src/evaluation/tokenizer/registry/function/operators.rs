use crate::{AbacusError, Value};

#[derive(Debug)]
pub struct FunctionOp {
    pub name: &'static str,
    pub min_args: usize,
    pub max_args: usize,
    pub func: fn(&[Value]) -> Result<Value, AbacusError>,
}

impl FunctionOp {
    pub fn apply(&self, args: &[Value]) -> Result<Value, AbacusError> {
        if args.len() < self.min_args || args.len() > self.max_args {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        (self.func)(args)
    }
}

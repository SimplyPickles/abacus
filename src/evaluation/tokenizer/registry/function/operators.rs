use crate::{AbacusError, Value, units::{hash::Hash, interval::EvalResult}};

#[derive(Debug, Clone)]
pub enum FunctionTarget {
    Scalar(fn(&[Value]) -> Result<Value, AbacusError>),
    EvalResult(fn(&[Value]) -> Result<EvalResult, AbacusError>),
    Hash(fn(&[Value]) -> Result<Hash, AbacusError>),
}

#[derive(Debug, Clone)]
pub struct FunctionOp {
    pub name: &'static str,
    pub min_args: usize,
    pub max_args: usize,
    pub func: FunctionTarget,
}

impl FunctionOp {
    pub fn scalar(
        name: &'static str,
        min_args: usize,
        max_args: usize,
        f: fn(&[Value]) -> Result<Value, AbacusError>,
    ) -> Self {
        Self {
            name,
            min_args,
            max_args,
            func: FunctionTarget::Scalar(f),
        }
    }

    pub fn eval_result(
        name: &'static str,
        min_args: usize,
        max_args: usize,
        f: fn(&[Value]) -> Result<EvalResult, AbacusError>,
    ) -> Self {
        Self {
            name,
            min_args,
            max_args,
            func: FunctionTarget::EvalResult(f),
        }
    }

    pub fn apply(&self, args: &[Value]) -> Result<EvalResult, AbacusError> {
        if args.len() < self.min_args || args.len() > self.max_args {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        match self.func {
            FunctionTarget::Scalar(f) => f(args).map(EvalResult::Scalar),
            FunctionTarget::EvalResult(f) => f(args),
            _ => Err(AbacusError::HashAsResult),
        }
    }
}

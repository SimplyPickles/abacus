use crate::{AbacusError, Value, units::eval_result::EvalResult};

#[derive(Debug, Clone)]
pub enum FunctionTarget {
    Scalar(fn(&[Value]) -> Result<Value, AbacusError>),
    EvalResult(fn(&[Value]) -> Result<EvalResult, AbacusError>),
    General(fn(&[EvalResult]) -> Result<EvalResult, AbacusError>),
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

    pub fn general(
        name: &'static str,
        min_args: usize,
        max_args: usize,
        f: fn(&[EvalResult]) -> Result<EvalResult, AbacusError>,
    ) -> Self {
        Self {
            name,
            min_args,
            max_args,
            func: FunctionTarget::General(f),
        }
    }

    pub fn apply(&self, raw_args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
        if raw_args.len() < self.min_args || raw_args.len() > self.max_args {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        match self.func {
            FunctionTarget::General(f) => f(raw_args),
            FunctionTarget::Scalar(f) => {
                let mut scalar_args = Vec::with_capacity(raw_args.len());
                for arg in raw_args {
                    let scalar = arg.as_scalar()?.clone();
                    scalar_args.push(scalar);
                }
                f(&scalar_args).map(EvalResult::Scalar)
            }
            FunctionTarget::EvalResult(f) => {
                let mut scalar_args = Vec::with_capacity(raw_args.len());
                for arg in raw_args {
                    let scalar = arg.as_scalar()?.clone();
                    scalar_args.push(scalar);
                }
                f(&scalar_args)
            }
        }
    }

    pub fn apply_scalar(&self, args: &[Value]) -> Result<EvalResult, AbacusError> {
        let eval_args: Vec<EvalResult> = args.iter().cloned().map(EvalResult::Scalar).collect();
        self.apply(&eval_args)
    }
}

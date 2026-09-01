use std::fmt;

// Error enum for evaluation/tokenization errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AbacusError {
    UnknownUnit(String),
    IncompatibleDimensions,
    IncompatibleFunctionArguments,
    AffineUnitOperation(&'static str),
    UnexpectedToken(String),
    UnexpectedEnd,
    UnclosedParen,
    UnclosedBracket,
    IntervalInFunction,
    InvalidDate(String),
    InvalidNumber(String),
    IncompatibleOperatorType(String),
    IncompatibleWithConversion(String),
    RecursionLimitExceeded,
    UndefinedVariable(String),
    EvaluationError(String),
    CurrencyRateError(String),
}

// Display implementation for `AbacusError`
impl fmt::Display for AbacusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownUnit(unit) => write!(f, "unknown unit: {unit}"),
            Self::IncompatibleDimensions => {
                write!(f, "cannot perform operation on incompatible dimensions")
            }
            Self::AffineUnitOperation(op) => write!(f, "cannot {op} affine units"),
            Self::IncompatibleFunctionArguments => {
                write!(f, "incompatible function arguments")
            }
            Self::UnexpectedToken(tok) => write!(f, "unexpected token: {tok}"),
            Self::UnexpectedEnd => write!(f, "unexpected end of expression"),
            Self::UnclosedParen => write!(f, "unclosed parenthesis"),
            Self::UnclosedBracket => write!(f, "unclosed bracket in interval"),
            Self::IntervalInFunction => {
                write!(f, "interval values cannot be passed as function arguments")
            }
            Self::InvalidDate(msg) => write!(f, "invalid date: {msg}"),
            Self::InvalidNumber(num) => write!(f, "invalid number format: {num}"),
            Self::IncompatibleOperatorType(msg) => write!(f, "{msg}"),
            Self::IncompatibleWithConversion(ty) => {
                write!(f, "values of type {ty} cannot be converted")
            }
            Self::RecursionLimitExceeded => {
                write!(f, "maximum recursion depth exceeded")
            }
            Self::UndefinedVariable(var) => {
                write!(f, "undefined variable: {var}")
            }
            Self::EvaluationError(msg) => {
                write!(f, "{msg}")
            }
            Self::CurrencyRateError(msg) => {
                write!(f, "currency rate error: {msg}")
            }
        }
    }
}

impl std::error::Error for AbacusError {}

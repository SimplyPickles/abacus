use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

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
        }
    }
}

impl std::error::Error for AbacusError {}

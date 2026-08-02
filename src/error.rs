use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbacusError {
    UnknownUnit(String),
    IncompatibleDimensions,
    AffineUnitOperation(&'static str),
}

impl fmt::Display for AbacusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownUnit(unit) => write!(f, "unknown unit: {unit}"),
            Self::IncompatibleDimensions => {
                write!(f, "cannot perform operation on incompatible dimensions")
            }
            Self::AffineUnitOperation(op) => write!(f, "cannot {op} affine units"),
        }
    }
}

impl std::error::Error for AbacusError {}

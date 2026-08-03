use crate::Value;

// Represents a token parsed from the input string
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Operators
    BinaryOp(&'static str), // e.g., '+', '-', '*', '/'
    UnaryOp(&'static str),  // e.g., 'sqrt' for square root or '!' for factorial

    ConversionOp,

    // Values
    Float(f64),   // parsed number
    Unit(String), // "kg", "m", "ft"

    // Grouping
    OpenParen,
    CloseParen,

    Val(Value), // final value
}

// pub fn register_tokens() ->  {

// }

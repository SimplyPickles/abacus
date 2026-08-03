use crate::Value;

// Represents a token parsed from the input string
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Operators
    BinaryOp(&'static str), // e.g., '+', '-', '*', '/'
    UnaryOp(&'static str),  // e.g., 'sqrt' for square root or '!' for factorial
    Function(&'static str), // e.g., 'sin', 'binompdf', 'mean'

    ConversionOp,

    // Values
    Float(f64),   // parsed number
    Unit(String), // "kg", "m", "ft"

    // Grouping & Delimiters
    OpenParen,
    CloseParen,
    Comma,
    Range, // `..` for range expressions inside functions, e.g. mean(1..10)

    Val(Value), // final value
}

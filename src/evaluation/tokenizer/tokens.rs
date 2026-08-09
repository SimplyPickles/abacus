use crate::{Date, Value};

// Represents a token parsed from the input string
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Operators
    BinaryOp(&'static str), // e.g., '+', '-', '*', '/'
    UnaryOp(&'static str),  // e.g., 'sqrt' for square root or '!' for factorial
    Function(&'static str), // e.g., 'sin', 'binompdf', 'mean'

    ConversionOp,
    RelTimeOp(&'static str), // "ago", "from_now", "before", "after"

    // Values
    Float(f64),    // parsed number
    Unit(&'a str), // "kg", "m", "ft" borrowed directly from input text

    // Grouping & Delimiters
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Range,               // `..` for range expressions inside functions, e.g. mean(1..10)
    DotProperty(String), // `.intercept`, `.slope`, `.r2` etc.

    Val(Value), // final value
    Date(Date), // parsed date
}

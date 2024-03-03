#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    FSlash,
    Asterisk,
}

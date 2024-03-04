#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,

    Ident(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    FSlash,
    Asterisk,
    Semicolon,
}

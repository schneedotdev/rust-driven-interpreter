#[derive(Debug, PartialEq)]
pub enum Token {
    // Identifiers
    Ident(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    FSlash,
    Asterisk,

    // Delimiters
    Semicolon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Comparison
    LessThan,
    GreaterThan,

    // Other
    Illegal,
}

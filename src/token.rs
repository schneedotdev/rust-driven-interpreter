#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Identifiers
    Ident(&'a str),

    // Literal
    Int(isize),

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
    Eq,
    NotEq,
    LessThanEq,
    GreaterThanEq,

    // Other
    Illegal,

    // Keywords
    Let,
    Function,
    Return,
    If,
    Else,
    True,
    False,
}

impl Token<'_> {
    pub fn return_keyword_or_ident(s: &str) -> Token<'_> {
        match s {
            "let" => Token::Let,
            "fn" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(s),
        }
    }
}

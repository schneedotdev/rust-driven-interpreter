#[derive(Debug, PartialEq)]
pub enum Token {
    // Identifiers
    Ident(String),

    // Literal
    Int(isize),
    UnsignedInt(usize),

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

    // Keywords
    Let,
    Function, 
    Return,
    If,
    Else,
    True,
    False
}


impl Token {
    pub fn return_keyword(s: &str) -> Option<Token> {
        Some(match s {
            "let" => Token::Let,
            "fn" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => return None
        })
    }
}

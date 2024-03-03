use crate::token::Token;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.input.find(|c| !c.is_whitespace()).and_then(|c| {
            Some(match c {
                '=' => Token::Assign,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' => Token::FSlash,
                '!' => Token::Bang,
                '*' => Token::Asterisk,
                _ => Token::Illegal,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_a_single_token() {
        let token = Lexer::new("+").next_token().unwrap();
        assert_eq!(token, Token::Plus);

        let token = Lexer::new("-").next_token().unwrap();
        assert_eq!(token, Token::Minus);
    }

    #[test]
    fn should_parse_single_character_tokens() {
        let mut lexer = Lexer::new("+=  -/* !");
        todo!("parsing single character tokens for an entire string")
    }
}

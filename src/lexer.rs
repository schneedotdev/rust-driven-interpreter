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
    fn should_lex_single_character_tokens() {
        let mut lexer = Lexer::new("+=  -/* !");
        
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Plus));
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Assign));
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Minus));
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::FSlash));
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Asterisk));
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Bang));
        let token = lexer.next_token();
        assert_eq!(token, None);
    }
}

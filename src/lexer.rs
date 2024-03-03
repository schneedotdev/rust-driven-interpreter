use std::str::Chars;

use crate::token::Token;

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
    fn should_parse_single_character_tokens() {
        let mut lexer = Lexer::new("+=  -/* !");
        todo!("test next token functionality");
    }
}

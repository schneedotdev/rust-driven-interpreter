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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
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
        let tokens = Lexer::new("+=  -/* !").collect::<Vec<Token>>();

        let expected_tokens = vec![
            Token::Plus,
            Token::Assign,
            Token::Minus,
            Token::FSlash,
            Token::Asterisk,
            Token::Bang,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}

use crate::token::Token;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.find(|c| !c.is_whitespace())?;

        Some(match c {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::FSlash,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            ';' => Token::Semicolon,
            _ if c.is_alphabetic() => {
                let mut token = c.to_string();
                while let Some(next_char) = self.input.next_if(|c| c.is_alphabetic()) {
                    token.push(next_char)
                }

                Token::Ident(token)
            }
            _ => Token::Illegal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_lex_single_character_tokens() {
        let tokens = Lexer::new("+=  -/* !").collect::<Vec<_>>();

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

    #[test]
    fn should_lex_identifiers() {
        let tokens = Lexer::new("le;t").collect::<Vec<_>>();

        let expected_tokens = vec![
            Token::Ident("le".to_string()),
            Token::Semicolon,
            Token::Ident("t".to_string()),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}

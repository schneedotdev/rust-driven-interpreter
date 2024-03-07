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

    fn group_while<F>(&mut self, start: char, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut group = start.to_string();
        while let Some(character) = self.input.next_if(|&c| condition(c)) {
            group.push(character)
        }

        group
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.find(|c| !c.is_whitespace())?;

        let token = match c {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::FSlash,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            _ if c.is_alphabetic() => {
                let value = self.group_while(c, char::is_alphabetic);
                Token::Ident(value)
            }
            _ if c.is_numeric() => {
                let value = self.group_while(c, char::is_numeric);

                match value.parse() {
                    Ok(val) => Token::Int(val),
                    Err(_) => Token::Illegal,
                }
            }
            _ => Token::Illegal,
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_lex_single_character_tokens() {
        let tokens = Lexer::new(",;=+-!/*(){}<>").collect::<Vec<_>>();

        let expected_tokens = vec![
            Token::Comma,
            Token::Semicolon,
            Token::Assign,
            Token::Plus,
            Token::Minus,
            Token::Bang,
            Token::FSlash,
            Token::Asterisk,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LessThan,
            Token::GreaterThan,
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

    #[test]
    fn should_lex_integers() {
        let tokens = Lexer::new("1234;4545").collect::<Vec<_>>();

        let expected_tokens = vec![Token::Int(1234), Token::Semicolon, Token::Int(4545)];

        assert_eq!(tokens, expected_tokens);
    }
}

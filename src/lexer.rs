use crate::token::Token;
use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn group_while<F>(&mut self, start: usize, c: char, condition: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let mut end = start + c.len_utf8();

        while let Some((i, c)) = self.chars.next_if(|(_, c)| condition(*c)) {
            end = i + c.len_utf8();
        }

        &self.input[start..end]
    }

    fn process_comparison_token(
        &mut self,
        single_char_token: Token<'a>,
        double_char_token: Token<'a>,
    ) -> Token<'a> {
        match self.chars.next_if(|(_, c)| c.eq(&'=')) {
            Some(_) => double_char_token,
            None => single_char_token,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, c) = self.chars.find(|(_, c)| !c.is_whitespace())?;

        let token = match c {
            '=' => self.process_comparison_token(Token::Assign, Token::Eq),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::FSlash,
            '!' => self.process_comparison_token(Token::Bang, Token::NotEq),
            '*' => Token::Asterisk,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '<' => self.process_comparison_token(Token::LessThan, Token::LessThanEq),
            '>' => self.process_comparison_token(Token::GreaterThan, Token::GreaterThanEq),
            _ if c.is_alphabetic() => {
                let s = self.group_while(i, c, char::is_alphabetic);
                Token::return_keyword_or_ident(s)
            }
            _ if c.is_numeric() => {
                match self.group_while(i, c, char::is_numeric).parse() {
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
            Token::Ident("le".into()),
            Token::Semicolon,
            Token::Ident("t".into()),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn should_lex_integers() {
        let tokens = Lexer::new("1234;4545").collect::<Vec<_>>();

        let expected_tokens = vec![Token::Int(1234), Token::Semicolon, Token::Int(4545)];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn should_identify_keyword_tokens() {
        let tokens = Lexer::new("let fn else true false if").collect::<Vec<_>>();

        let expected_tokens = vec![
            Token::Let,
            Token::Function,
            Token::Else,
            Token::True,
            Token::False,
            Token::If,
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn should_lex_multi_char_operators() {
        let tokens = Lexer::new("!= == >= <=").collect::<Vec<_>>();

        let expected_tokens = vec![
            Token::NotEq,
            Token::Eq,
            Token::GreaterThanEq,
            Token::LessThanEq,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}

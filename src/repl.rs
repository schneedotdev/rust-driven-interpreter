use std::io::{self, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn run() -> io::Result<()> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        for token in Lexer::new(input.as_str()) {
            println!("{:?}", token);
        }
    }
}

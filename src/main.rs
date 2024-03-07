use std::io;

use rust_interpreter::repl;

fn main() -> io::Result<()> {
    println!("Welcome, feel free to hack away!");

    repl::run()
}

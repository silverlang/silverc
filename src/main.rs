use std::io;
use std::io::prelude::*;

use lexer::Lexer;

use crate::lexer::Token;

mod lexer;
mod span;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Silver lexer output");
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        buffer.clear();
        handle.read_line(&mut buffer)?;

        buffer.remove(buffer.len() - 1);
        let buffer = buffer.replace("\\n", "\n");

        let lexer = Lexer::new(&buffer);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}

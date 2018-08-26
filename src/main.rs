use std::io::prelude::*;

mod lexer;
mod parser;
mod computer;

use lexer::*;
use parser::*;
use computer::*;

fn main() {
    let mut buffer = String::new();

    loop {
        print!(">");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            break;
        }

        match tokenize(&buffer) {
            Ok(tokens) => {
                let ast = parse(&tokens);
                // println!("{:#?}", ast);
                println!("{}", compute(&ast));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }

        buffer = String::new();
    }
}

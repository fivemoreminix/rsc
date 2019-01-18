extern crate structopt;

use std::io::prelude::*;

use structopt::StructOpt;

use rsc::lexer::*;
use rsc::parser::*;
use rsc::computer::*;

#[derive(StructOpt)]
#[structopt(about = "A scientific calculator for the terminal.")]
struct Opt {
    #[structopt(long = "ast", help = "Prints abstract syntax tree")]
    ast: bool,
    #[structopt(long = "vars", help = "Prints variable map")]
    vars: bool,
}

fn main() {
    let opt = Opt::from_args();

    let mut computer = Computer::new();

    loop {
        print!(">");
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            break;
        }

        match tokenize(&buffer, true) {
            Ok(tokens) => {
                match parse(&tokens) {
                    Ok(ast) => {
                        if opt.ast {
                            println!("{:#?}", ast);
                        }
                        
                        match computer.compute(&ast) {
                            Ok(num) => {
                                if opt.vars {
                                    println!("{:#?}", computer.variables);
                                }
                                println!("{}", num);
                            }
                            Err(err) => println!("Compute error: {:?}", err),
                        }
                    }
                    Err(err) => {
                        println!("Parser error: {:?}", err);
                    }
                }
            }
            Err(err) => {
                println!("Lexer error: {:?}", err);
            }
        }
    }
}

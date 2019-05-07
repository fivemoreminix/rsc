extern crate structopt;
extern crate colored;

use std::io::prelude::*;

use structopt::StructOpt;

use colored::Colorize;

use rsc::lexer::*;
use rsc::parser::*;
use rsc::computer::*;

#[derive(StructOpt)]
#[structopt(about = "A scientific calculator for the terminal.\nManual: https://github.com/asmoaesl/rsc/wiki")]
struct Opt {
    #[structopt(long = "ast", help = "Prints abstract syntax tree")]
    ast: bool,
    #[structopt(long = "vars", help = "Prints variable map")]
    vars: bool,
    #[structopt(long = "no-color", help = "Prevents colored text")]
    no_color: bool,
}

fn main() {
    let opt = Opt::from_args();

    let mut computer: Computer<f64> = Default::default();

    loop {
        print!("{}", if opt.no_color { ">".normal() } else { ">".blue() });
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            break;
        } else if &buffer[..] == "clear" {
            for _ in 0..100 {
                println!("");
            }
            continue;
        } else if buffer.starts_with(":") {
            continue;
        }

        match tokenize::<f64>(&buffer, true) {
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
                                if opt.no_color {
                                    println!("{}", num);
                                } else {
                                    println!("{}", num.to_string().yellow());
                                }
                            }
                            Err(err) => if opt.no_color {
                                println!("Computation error: {:?}", err)
                            } else {
                                println!("{}", format!("Computation error: {:?}", err).red())
                            },
                        }
                    }
                    Err(err) => if opt.no_color {
                        println!("Parser error: {:?}", err)
                    } else {
                        println!("{}", format!("Parser error: {:?}", err).red())
                    },
                }
            }
            Err(err) => if opt.no_color {
                println!("Lexer error: {:?}", err)
            } else {
                println!("{}", format!("Lexer error: {:?}", err).red())
            },
        }
    }
}

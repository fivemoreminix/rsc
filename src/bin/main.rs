extern crate colored;
extern crate structopt;

use std::io::prelude::*;

use structopt::StructOpt;

use colored::Colorize;

use rsc::{tokenize, parse, Interpreter};

#[derive(StructOpt)]
#[structopt(
    about = "A scientific calculator for the terminal.\nManual: https://github.com/asmoaesl/rsc/wiki"
)]
struct Opt {
    #[structopt()]
    expr: Option<String>,
    #[structopt(long = "ast", help = "Prints abstract syntax tree")]
    ast: bool,
    #[structopt(long = "vars", help = "Prints variable map")]
    vars: bool,
    #[structopt(long = "no-color", help = "Prevents colored text")]
    no_color: bool,
}

fn evaluate(input: &str, interpreter: &mut Interpreter, ast: bool, vars: bool, no_color: bool) {
    match tokenize(input) {
        Ok(tokens) => {
            println!("{:#?}", tokens);
            match parse(&tokens) {
                Ok(ast) => {
                    println!("{:#?}", ast);

                    println!("{:?}", interpreter.eval(&ast));
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut interpreter = Interpreter::default();

    if let Some(expr) = opt.expr {
        evaluate(&expr, &mut interpreter, opt.ast, opt.vars, opt.no_color);
        return;
    }

    loop {
        print!(
            "{}",
            if opt.no_color {
                ">".normal()
            } else {
                ">".blue()
            }
        );
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            break;
        } else if &buffer[..] == "clear" {
            for _ in 0..100 {
                println!();
            }
            continue;
        } else if buffer.starts_with(":") {
            continue;
        }

        evaluate(&buffer, &mut interpreter, opt.ast, opt.vars, opt.no_color);
    }
}

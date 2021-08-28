use std::io::prelude::*;
use structopt::StructOpt;
use colored::{Colorize, ColoredString};

use rsc::{tokenize, TokenizeError, parse, ParseError, Interpreter, InterpretError, Variant};

#[derive(StructOpt)]
#[structopt(
    about = "A scientific calculator for the terminal."
)]
struct Opt {
    #[structopt()]
    expr: Option<String>,
    #[structopt(short = "t", long = "tokens", help = "Prints the tokens")]
    tokens: bool,
    #[structopt(short = "e", long = "expr", help = "Prints the expression tree")]
    bexpr: bool,
    #[structopt(short = "v", long = "vars", help = "Prints variable map")]
    vars: bool,
    #[structopt(long = "no-color", help = "Prevents colored text")]
    no_color: bool,
}

#[inline(always)]
fn s_if(b: bool) -> &'static str {
    if b { "s" } else { "" }
}

fn display_interpret_error(err: &InterpretError) -> String {
    match err {
        InterpretError::TooFewArgs(id, n) =>
            format!("Function {:?} did not receive minimum of {} argument{}.", id, n, s_if(*n != 1)),
        InterpretError::TooManyArgs(id, n) =>
            format!("Function {:?} received more than the maximum {} argument{}.", id, n, s_if(*n != 1)),
        InterpretError::VarDoesNotExist(id) =>
            format!("No variable {:?} exists.", id),
        InterpretError::VarIsNotFunction(id) =>
            format!("The variable {:?} cannot be used like a function with arguments.", id),
        InterpretError::FunctionNameUsedLikeVar(id) =>
            format!("The function {:?} cannot be used without arguments.", id),
    }
}

fn evaluate(input: &str, interpreter: &mut Interpreter, btokens: bool, bexpr: bool, bvars: bool, bno_color: bool) {
    match tokenize(input) {
        Ok(tokens) => {
            if btokens {
                let fmt = format!("Tokens: {:?}", tokens);
                println!("{}", if bno_color { fmt } else { fmt.yellow().to_string() });
            }
            match parse(&tokens) {
                Ok(expr) => {
                    if bexpr {
                        let fmt = format!("Expr: {:#?}", expr);
                        println!("{}", if bno_color { fmt } else { fmt.yellow().to_string() });
                    }

                    match interpreter.eval(&expr) {
                        Ok(result) => println!(":{}", result),
                        Err(err) => {
                            let fmt = format!("{}", display_interpret_error(&err));
                            println!("{}", if bno_color { fmt } else { fmt.red().to_string() })
                        },
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
    if bvars {
        for (id, variant) in &interpreter.vars {
            let fmt;
            if let Variant::Num(n) = variant {
                fmt = format!("{} = {}", id, n);
            } else {
                fmt = format!("{}(..)", id);
            }
            println!("{}", if bno_color { fmt } else { fmt.yellow().to_string() });
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut interpreter = Interpreter::default();

    if let Some(expr) = opt.expr {
        evaluate(&expr, &mut interpreter, opt.tokens, opt.bexpr, opt.vars, opt.no_color);
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

        evaluate(&buffer, &mut interpreter, opt.tokens, opt.bexpr, opt.vars, opt.no_color);
    }
}

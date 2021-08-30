use colored::Colorize;
use std::io::prelude::*;
use structopt::StructOpt;

use rsc::{
    parse, tokenize, InterpretError, Interpreter, Num, ParseError, ParseErrorCode, TokenizeError,
    Variant,
};
use std::fmt::Display;
use std::ops::Range;

#[derive(StructOpt)]
#[structopt(about = "A scientific calculator for the terminal.")]
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

fn main() {
    let opt = Opt::from_args();

    let mut interpreter = Interpreter::default();

    if let Some(expr) = opt.expr {
        match tokenize(&expr) {
            Ok(tokens) => match parse(&tokens) {
                Ok(expr) => match interpreter.eval(&expr) {
                    Ok(result) => {
                        println!("{}", result);
                        return;
                    }
                    Err(e) => eprintln!("{:?}", e),
                },
                Err(ParseError { code, span }) => eprintln!("{:?} at {:?}", code, span),
            },
            Err(TokenizeError { code, span }) => eprintln!("{:?} at {:?}", code, span),
        }
        std::process::exit(1);
    }

    println!("RSC interactive expression interpreter.");
    println!("Try \"help\" for commands and examples.");

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
        } else if &buffer[..] == "help" {
            print_help(opt.no_color);
        } else if &buffer[..] == "vars" {
            print_vars(&interpreter, opt.no_color);
        } else if &buffer[..] == "clear" {
            for _ in 0..100 {
                println!();
            }
            continue;
        } else if buffer.starts_with(":") {
            continue;
        } else {
            evaluate(
                &buffer,
                &mut interpreter,
                opt.tokens,
                opt.bexpr,
                opt.vars,
                opt.no_color,
                ":",
            );
        }
    }
}

const COMMANDS: [(&str, &str); 5] = [
    ("quit|exit", "Close RSC"),
    ("help", "Show this help information"),
    ("vars", "Display all of the active variables"),
    ("clear", "Clear prior output"),
    (":", "Write notes"),
];

fn print_help(no_color: bool) {
    println!("Commands");
    for (name, desc) in COMMANDS {
        println!(
            "{:<10} {}",
            if no_color {
                name.green().clear()
            } else {
                name.green()
            },
            desc
        );
    }
    println!("\nExamples");
    println!("\t12.3(0.7)");
    println!("\t|-9| + 3!");
    println!("\tx = abs(5)");
    println!("\t-x^4");
}

fn get_variant_ord<N: Num>(v: &Variant<N>) -> usize {
    match v {
        Variant::Num(_) => 1,
        Variant::Function(_) => 0,
    }
}

fn print_vars<N: Num + Display>(interpreter: &Interpreter<N>, no_color: bool) {
    let mut vars: Vec<(&String, &Variant<N>)> = interpreter.vars.iter().collect();
    vars.sort_by(|(_, v1), (_, v2)| {
        // sort by type
        let v1_val = get_variant_ord(v1);
        let v2_val = get_variant_ord(v2);
        v1_val.cmp(&v2_val)
    });
    for (id, val) in vars {
        let fmt;
        match val {
            Variant::Num(n) => fmt = format!("{} = {}", &id.green(), n.clone()),
            Variant::Function(_) => fmt = format!("{}(..)", &id.green()),
        }
        println!(
            "{}",
            if no_color {
                fmt.red().clear().to_string()
            } else {
                fmt
            }
        );
    }
}

fn format_error(span: Range<usize>, message: &str) -> String {
    format!(
        " {}{} {}",
        " ".repeat(span.start),
        "^".repeat(span.len()).red(),
        message.red()
    )
}

fn evaluate<N: Num + Display>(
    input: &str,
    interpreter: &mut Interpreter<N>,
    btokens: bool,
    bexpr: bool,
    bvars: bool,
    bno_color: bool,
    success_prefix: &str,
) {
    match tokenize(input) {
        Ok(tokens) => {
            if btokens {
                let fmt = format!("Tokens: {:?}", tokens);
                println!(
                    "{}",
                    if bno_color {
                        fmt
                    } else {
                        fmt.yellow().to_string()
                    }
                );
            }
            match parse(&tokens) {
                Ok(expr) => {
                    if bexpr {
                        let fmt = format!("Expr: {:#?}", expr);
                        println!(
                            "{}",
                            if bno_color {
                                fmt
                            } else {
                                fmt.yellow().to_string()
                            }
                        );
                    }

                    match interpreter.eval(&expr) {
                        Ok(result) => {
                            println!("{}{}", success_prefix, result);
                        }
                        Err(err) => {
                            let fmt = format!("{}", display_interpret_error(&err));
                            println!(
                                "{}",
                                if bno_color {
                                    fmt
                                } else {
                                    fmt.red().to_string()
                                }
                            );
                        }
                    }
                }
                Err(ParseError { code, span }) => {
                    if code == ParseErrorCode::UnexpectedEOF {
                        println!(
                            "{}",
                            format_error(input.len()..input.len() + 1, &format!("{:?}", code))
                        );
                    } else {
                        println!("{}", format_error(span, &format!("{:?}", code)));
                    }
                }
            }
        }
        Err(TokenizeError { code, span }) => {
            println!("{}", format_error(span, &format!("{:?}", code)));
        }
    }
    if bvars {
        for (id, variant) in &interpreter.vars {
            let fmt;
            if let Variant::Num(n) = variant {
                fmt = format!("{} = {}", id, n);
            } else {
                fmt = format!("{}(..)", id);
            }
            println!(
                "{}",
                if bno_color {
                    fmt
                } else {
                    fmt.yellow().to_string()
                }
            );
        }
    }
}

#[inline(always)]
fn s_if(b: bool) -> &'static str {
    if b {
        "s"
    } else {
        ""
    }
}

fn display_interpret_error(err: &InterpretError) -> String {
    match err {
        InterpretError::TooFewArgs(id, n) => format!(
            "Function {:?} did not receive minimum of {} argument{}.",
            id,
            n,
            s_if(*n != 1)
        ),
        InterpretError::TooManyArgs(id, n) => format!(
            "Function {:?} received more than the maximum {} argument{}.",
            id,
            n,
            s_if(*n != 1)
        ),
        InterpretError::VarDoesNotExist(id) => format!("No variable or function {:?} exists.", id),
        InterpretError::VarIsNotFunction(id) => format!(
            "The variable {:?} cannot be used like a function with arguments.",
            id
        ),
        InterpretError::FunctionNameUsedLikeVar(id) => {
            format!("The function {:?} cannot be used without arguments.", id)
        }
    }
}

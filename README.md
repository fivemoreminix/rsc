RSC, the Calculator for Rust Code
=================================
![](https://img.shields.io/crates/l/rsc.svg) ![](https://img.shields.io/badge/status-stable-blue.svg)

**New**: crate updated to 3.0, read the [Changelog](CHANGELOG.md).

**RSC is a handwritten scientific calculator for interpreting equations inside strings.** RSC is designed to do a single
thing very well, enabling anyone to extend it with more features.

RSC intends to beat Wirth's Law. **Therefore, RSC will not receive many additions.** It will still receive updates with
relation to efficiency.

## Library
```rust
use rsc::{tokenize, parse, Interpreter};

// Maybe you write a wrapper function
fn evaluate(input: &str, interpreter: &mut Interpreter<f64>) -> Result<f64, ()> {
    // You have to call each function in the pipeline, but this gives you the most
    // control over error handling and performance.
    match tokenize(input) { // Step 1: splits input into symbols, words, and numbers
        Ok(tokens) => match parse(&tokens) { // Step 2: builds an Expr using tokens
            Ok(expr) => match interpreter.eval(&expr) { // Step 3: interprets the Expr
                Ok(result) => println!("{}", result),
                Err(interpret_error) => eprintln!("{:?}", interpret_error),
            },
            Err(parse_error) => eprintln!("{:?}", parse_error),
        },
        Err(tokenize_error) => eprintln!("{:?}", tokenize_error),
    }
}

fn main() {
    // Constructs an f64 interpreter with included variables
    let mut interpreter = Interpreter::default();
    
    evaluate("5^2", &mut interpreter); // prints "25"
    evaluate("x = 3", &mut interpreter); // prints "3"
    evaluate("x(3) + 1", &mut interpreter); // prints "10"
}
```

Variables are stored in the `Interpreter`:
```rust
use rsc::{tokenize, parse, Interpreter, Variant, InterpretError};

// assume you still had your evaluate function above

fn main() {
    // Create a completely empty interpreter for f64 calculations
    let mut i = Interpreter::<f64>::new();
    
    // Create some variables
    i.set_var(String::from("pi"), Variant::Num(std::f64::consts::PI));
    i.set_var(String::from("double"), Variant::Function(|name, args| {
        if args.len() < 1 {
            Err(InterpretError::TooFewArgs(name, 1))
        } else if args.len() > 1 {
            Err(InterpretError::TooManyArgs(name, 1))
        } else {
            Ok(args[0] * 2) // get the only argument and double it
        }
    }));
    
    evaluate("double(pi)", &mut i); // prints "6.283185307179586"
}
```

Because it can be redundant checking that functions received the correct number of arguments (if you wish to do so at all),
I made a helper function called `ensure_arg_count`. The above function redefined:

```rust
use rsc::ensure_arg_count;

i.set_var(String::from("double"), Variant::Function(|name, args| {
    // return Err if args are not within the min and max count
    ensure_arg_count(1, 1, args.len(), name)?;
    Ok(args[0] * 2)
}));
```

## Executable
### First you might need to build RSC as an executable
```shell
cargo build --release --features=executable
```
The `executable` feature is required to tell the crate to bring in certain dependencies only for the executable version, for colors in the terminal and argument parsing.

### Usage
```shell
RSC interactive expression interpreter.
Try "help" for commands and examples.
>sqrt(15+3)
:4.242640687119285
>:square root
>sqrt(15, 3)
Function "sqrt" received more than the maximum 1 argument.
> |-5|
:5
>abs(-5)
:5
>sqrt(4)(2)
        ^ UnexpectedToken(Token { value: Symbol(LP), span: 7..8 })
>(sqrt(4))(2)
:4
>x = 1.24
:1.24
>x(4)
:4.96
>vars
factorial(..)
sqrt(..)
abs(..)
x = 1.24
e = 2.718281828459045
pi = 3.141592653589793
tau = 6.283185307179586
```

Expressions can be passed to rsc directly:
```shell
$ rsc "12/sqrt(128)"
1.0606601717798212
```

There are various flags you can pass. Try:
```shell
rsc -tev
```

```shell
$ rsc -h
rsc 3.0.0
A scientific calculator for the terminal.

USAGE:
    rsc [FLAGS] [expr]

FLAGS:
    -e, --expr        Prints the expression tree
    -h, --help        Prints help information
        --no-color    Prevents colored text
    -t, --tokens      Prints the tokens
    -v, --vars        Prints variable map
    -V, --version     Prints version information

ARGS:
    <expr>
```

## Notes About Performance
 * The lexer is iterative and can easily be optimized.
 * The parser is an LL(2) recursive-descent parser, and that's the simplest, most brute-force parsing solution I came up with. It's easy to understand and maintain, but not the most efficient. The parser is currently the slowest of the 3 phases.
 * The `Interpreter::eval` function uses recursion for simplicity. Removing the recursion could prevent unnecessary pushing and popping of the frame pointer, and enable better caching, providing better performance.
 * Performance improvement PRs are very much welcomed and probably easy!

## Stability
RSC will not have any major changes to its syntax. It will remain consistent for a long time. It is up to forkers to make different flavors of RSC. It will also forever keep the same open-source permissions.

## License
RSC is MIT licensed. RSC will always remain free to modify and use without attribution.

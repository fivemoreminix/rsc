# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.0 - 2024-03-31
### Added
#### In the executable
 * Help command list
 * `vars` command shows active variables and functions.

### In the library
 * Implement the `Num` trait for all signed integer primitives: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, and floats: `f32` and `f64`.

### Changed
 * Rewrote *everything*.
 * Lexer and `Token` architecture. `Token` now includes data about where it was located in the input, and how many characters it spans, which is useful for errors.
 * Parser and `Expr` usage. Overall code cleanup for the parser. Now uses a lookahead of 2 to solve ambiguity in parsing. See grammar.
 * `ParseError` is now descriptive, including the position and length of the problem, and even sometimes providing the offending token.
 * `Computer<T>` became `Interpreter`. A lot of changes were made to the interpreter, compared to the old `Computer` that you should check out when migrating.
 * Some semantic expressions like absolute value `|x|` and factorial `x!` are now translated to `abs(x)` and `factorial(x)`, respectively.
 * The entire system still remains generic over which type of number is used, but I have simplified and extended the trait `Num` which a type must still implement to be used.
 * Update peekmore dependency 1.0.0 -> 1.3.0. 

### Removed
 * `ans` variable.
 * Global `eval` function and `EvalError` tagged enum. The "simplistic" interface was really quite complex and made things pretty complicated.

### Fixed
 * Some bugs in the grammar that caused seemingly ordinary expressions to produce false results.
 * Determining at runtime whether `x(5)` is a function `x` with an argument `5` or a variable `x` times `5`.
 * Functions were accidentally defined as the trait `Fn`, embarrassingly. I've updated functions, so they are now actually usable.

## 2.0 - 2019-06-21
### Added
* Real named functions! Functions are no longer tokens, and can now be created in a `Computer`, similar to variables.
```rust
let mut map = HashMap::<String, &'a Fn(f64) -> f64>::new();
map.insert("sqrt".to_owned(), &|n| n.sqrt());
```
* RSC is fully generic, now! Types that can support addition, subtraction, and a couple functions necessary in the `Num` trait can be lexed, parsed, and computed with no changes to the RSC source code.
* Getting the previous answer with the new `ans` variable. `ans` does not exist until you've run a calculation on a Computer already.
* Factorial: `5! = 120`

## [1.2.1] - 2017-06-20
### Removed
* Tests from lib.rs removed so it can compile on stable compiler branches.

*Versions prior to 1.2.1 had no changelog recordings, unfortunately.*

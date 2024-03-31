use crate::Num;
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpVal {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Exclaim,
}
use OpVal::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SymbolVal {
    LP,
    RP,
    Comma,
    Pipe,
}
use SymbolVal::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue<'input, N: Num> {
    Num(N),
    Id(&'input str),
    Op(OpVal),
    Symbol(SymbolVal),
}
use TokenValue::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'input, N: Num> {
    pub value: TokenValue<'input, N>,
    pub span: Range<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenizeErrorCode<'input> {
    InvalidNumber(&'input str),
    UnrecognizedChar(char),
}
use TokenizeErrorCode::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenizeError<'input> {
    pub code: TokenizeErrorCode<'input>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct TokenizeOptions {
    identifiers_contain_numbers: bool,
}

pub fn tokenize<N: Num>(input: &str) -> Result<Vec<Token<N>>, TokenizeError> {
    tokenize_with_options(input, TokenizeOptions::default())
}

pub fn tokenize_with_options<N: Num>(
    input: &str,
    options: TokenizeOptions,
) -> Result<Vec<Token<N>>, TokenizeError> {
    let mut tokens = Vec::with_capacity(16);
    let mut chars = input.chars().enumerate().peekable();

    macro_rules! push_token {
        ($token:expr, $pos:expr, $len:expr) => {
            tokens.push(Token {
                value: $token,
                span: Range {
                    start: $pos,
                    end: $pos + $len,
                },
            })
        };
    }

    while let Some((cpos, c)) = chars.next() {
        match c {
            '+' => push_token!(Op(Add), cpos, 1),
            '-' => push_token!(Op(Sub), cpos, 1),
            '*' => push_token!(Op(Mul), cpos, 1),
            '/' => push_token!(Op(Div), cpos, 1),
            '%' => push_token!(Op(Mod), cpos, 1),
            '^' => push_token!(Op(Pow), cpos, 1),
            '=' => push_token!(Op(Eq), cpos, 1),
            '!' => push_token!(Op(Exclaim), cpos, 1),

            '(' => push_token!(Symbol(LP), cpos, 1),
            ')' => push_token!(Symbol(RP), cpos, 1),
            ',' => push_token!(Symbol(Comma), cpos, 1),
            '|' => push_token!(Symbol(Pipe), cpos, 1),

            _ => {
                if c.is_ascii_digit() || c == '.' {
                    let start = cpos;
                    let mut end = start + 1;
                    while let Some((_, nc)) = chars.peek() {
                        if nc.is_ascii_digit() || *nc == '.' {
                            chars.next(); // Consume nc
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    if let Ok(num) = input[start..end].parse::<N>() {
                        push_token!(Num(num), start, end - start);
                    } else {
                        return Err(TokenizeError {
                            code: InvalidNumber(&input[start..end]),
                            span: start..end,
                        });
                    }
                } else if c == '_' || c.is_alphabetic() {
                    let start = cpos;
                    let mut end = start + 1;
                    while let Some((_, nc)) = chars.peek() {
                        // If it is any of _ A-z (or digits if option)
                        if *nc == '_'
                            || nc.is_alphanumeric()
                            || (options.identifiers_contain_numbers && nc.is_ascii_digit())
                        {
                            chars.next(); // Consume next character
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    push_token!(Id(&input[start..end]), start, end - start);
                } else if !c.is_whitespace() {
                    return Err(TokenizeError {
                        code: UnrecognizedChar(c),
                        span: cpos..cpos + 1,
                    });
                }
            }
        }
    }
    Ok(tokens)
}

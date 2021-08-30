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
pub enum TokenValue<'input> {
    Num(f64),
    Id(&'input str),
    Op(OpVal),
    Symbol(SymbolVal),
}
use TokenValue::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'input> {
    pub value: TokenValue<'input>,
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::with_capacity(16);
    let mut chars = input.chars().enumerate().peekable();

    macro_rules! push_token {
        ($token:expr, $pos:expr, $len:expr) => {
            tokens.push(Token { value: $token, span: Range { start: $pos, end: $pos + $len } })
        };
    }

    while let Some((cpos, c)) = chars.next() {
        match c {
            '+' => push_token!(Op(Add),     cpos, 1),
            '-' => push_token!(Op(Sub),     cpos, 1),
            '*' => push_token!(Op(Mul),     cpos, 1),
            '/' => push_token!(Op(Div),     cpos, 1),
            '%' => push_token!(Op(Mod),     cpos, 1),
            '^' => push_token!(Op(Pow),     cpos, 1),
            '=' => push_token!(Op(Eq),      cpos, 1),
            '!' => push_token!(Op(Exclaim), cpos, 1),

            '(' => push_token!(Symbol(LP), cpos, 1),
            ')' => push_token!(Symbol(RP), cpos, 1),
            ',' => push_token!(Symbol(Comma), cpos, 1),
            '|' => push_token!(Symbol(Pipe), cpos, 1),

            _ => {
                if c.is_digit(10) || c == '.' {
                    let start = cpos;
                    let mut end = start+1;
                    while let Some((_, nc)) = chars.peek() {
                        if nc.is_digit(10) || *nc == '.' {
                            chars.next(); // Consume nc
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    if let Ok(num) = input[start..end].parse::<f64>() {
                        push_token!(Num(num), start, end-start);
                    } else {
                        return Err(TokenizeError { code: InvalidNumber(&input[start..end]), span: start..end });
                    }
                } else if c.is_alphabetic() {
                    let start = cpos;
                    let mut end = start+1;
                    while let Some((_, nc)) = chars.peek() {
                        if nc.is_alphanumeric() {
                            chars.next(); // Consume nc
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    push_token!(Id(&input[start..end]), start, end-start);
                } else if !c.is_whitespace() {
                    return Err(TokenizeError { code: UnrecognizedChar(c), span: cpos..cpos+1 });
                }
            }
        }
    }
    Ok(tokens)
}

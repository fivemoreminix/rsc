#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}
use self::Operator::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Function {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
}
use self::Function::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumber(String),
    InvalidIdentifier(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::<Token>::new();

    let chars: Vec<char> = input.chars().collect();

    let mut i = 0usize;
    while i < chars.len() {
        match chars[i] {
            '+' => tokens.push(Token::Operator(Plus)),
            '-' => tokens.push(Token::Operator(Minus)),
            '*' | '•' | '×' => tokens.push(Token::Operator(Star)),
            '/' | '÷' => tokens.push(Token::Operator(Slash)),
            '%' => tokens.push(Token::Operator(Percent)),
            '^' => tokens.push(Token::Operator(Caret)),
            '(' => tokens.push(Token::Operator(LParen)),
            ')' => tokens.push(Token::Operator(RParen)),
            '√' => tokens.push(Token::Function(Sqrt)),
            c => {
                if c.is_whitespace() {
                    i += 1;
                    continue;
                } else if c.is_digit(10) || c == '.' {
                    let mut number_string = c.to_string();
                    
                    i += 1;
                    while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                        number_string.push(chars[i]);
                        i += 1;
                    }

                    let number;
                    match number_string.parse::<f64>() {
                        Ok(num) => number = num,
                        _ => return Err(LexerError::InvalidNumber(number_string)),
                    }

                    tokens.push(Token::Number(number));

                    continue; // we i += 1 at end of while
                } else if c.is_alphabetic() {
                    let mut full_identifier = c.to_string();

                    i += 1; // step over first character of identifier
                    while i < chars.len() && chars[i].is_alphabetic() {
                        full_identifier.push(chars[i]);
                        i += 1;
                    }

                    match &full_identifier.to_lowercase()[..] {
                        "sqrt" => tokens.push(Token::Function(Sqrt)),
                        "sin" => tokens.push(Token::Function(Sin)),
                        "cos" => tokens.push(Token::Function(Cos)),
                        "tan" => tokens.push(Token::Function(Tan)),
                        "log" => tokens.push(Token::Function(Log)),
                        _ => return Err(LexerError::InvalidIdentifier(full_identifier)),
                    }

		            continue;
                } else {
                    return Err(LexerError::InvalidCharacter(c));
                }
            }
        }
        i += 1;
    }
    
    Ok(tokens)
}

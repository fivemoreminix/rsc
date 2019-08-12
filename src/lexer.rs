//! For making notable symbols and words out of text.

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
    Pipe,
    Equals,
    Factorial,
}
use self::Operator::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a, T> {
    Number(T),
    Operator(Operator),
    Identifier(&'a str),
}

/// # Error Lookup Table
/// | Error ID         | Description                                                                                           |
/// |------------------|-------------------------------------------------------------------------------------------------------|
/// | InvalidCharacter | If the input contains any characters not recognized by the lexer to be numbers or characters, ex: 'ƒ' |
/// | InvalidNumber    | A number entered invalidly: '2.34.2' or '..3'                                                         |
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

/// Turn a string into a vector of tokens. This function generally takes the most time,
/// compared to parsing and computing. It is best to run this function as few times as
/// reasonably possible.
/// ```
/// use rsc::tokenize;
/// let tokens = tokenize("2 + 2").unwrap();
/// assert_eq!(tokens.as_slice(), &[Token::Number(2.0), Token::Operator(Operator::Plus), Token::Number(2.0)]);
/// ```
pub fn tokenize<'a, T>(src: &'a str) -> Result<Vec<Token<'a, T>>, LexerError>
where
    T: std::str::FromStr,
{
    let mut tokens = Vec::<Token<T>>::new();

    let mut chars = src.chars().enumerate().peekable();

    while let Some((i, c)) = chars.next() {
        match c {
            '+' => tokens.push(Token::Operator(Plus)),
            '-' => tokens.push(Token::Operator(Minus)),
            '*' => tokens.push(Token::Operator(Star)),
            '/' => tokens.push(Token::Operator(Slash)),
            '%' => tokens.push(Token::Operator(Percent)),
            '^' => tokens.push(Token::Operator(Caret)),
            '(' => tokens.push(Token::Operator(LParen)),
            ')' => tokens.push(Token::Operator(RParen)),
            '|' => tokens.push(Token::Operator(Pipe)),
            '=' => tokens.push(Token::Operator(Equals)),
            '!' => tokens.push(Token::Operator(Factorial)),
            _ => {
                if c.is_whitespace() {
                    continue;
                } else if c.is_digit(10) || c == '.' {
                    let start_idx = i;
                    let mut end_idx = i;

                    while let Some((_, c)) = chars.peek() {
                        if c.is_digit(10) || c == &'.' {
                            chars.next(); // consume the character
                            end_idx += 1;
                        } else {
                            break;
                        }
                    }

                    match (&src[start_idx..=end_idx]).parse::<T>() {
                        Ok(num) => tokens.push(Token::Number(num)),
                        _ => return Err(LexerError::InvalidNumber(src[start_idx..=end_idx].to_owned())),
                    }
                } else if c.is_alphabetic() {
                	let start_idx = i;
                    let mut end_idx = i;

                    while let Some((_, c)) = chars.peek() {
                        if c.is_alphabetic() || c == &'_' {
                            chars.next();
                            end_idx += 1;
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Identifier(&src[start_idx..=end_idx]));
                } else {
                    return Err(LexerError::InvalidCharacter(c));
                }
            }
        }
    }

    Ok(tokens)
}

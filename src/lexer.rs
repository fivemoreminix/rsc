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

/// All functions assume the next factor immediately following to be their argument.
/// Functions cannot contain more than a single argument. This may be changed in the future.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Function {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
    Abs,
}
use self::Function::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<T> {
    Number(T),
    Operator(Operator),
    Function(Function),
    Identifier(String),
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
pub fn tokenize<T>(input: &str, case_sensitive: bool) -> Result<Vec<Token<T>>, LexerError>
where
    T: std::str::FromStr,
{
    let mut tokens = Vec::<Token<T>>::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
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
            c => {
                if c.is_whitespace() {
                    continue;
                } else if c.is_digit(10) || c == '.' {
                    let mut number_string = c.to_string(); // Like creating a new string and pushing the character.

                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) || c == '.' {
                            number_string.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    match number_string.parse::<T>() {
                        Ok(num) => tokens.push(Token::Number(num)),
                        _ => return Err(LexerError::InvalidNumber(number_string)),
                    }
                } else if c.is_alphabetic() {
                    let mut full_identifier = c.to_string();

                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() || c == '_' {
                            full_identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    match &(if case_sensitive {
                        full_identifier
                    } else {
                        full_identifier.to_lowercase()
                    })[..]
                    {
                        // Functions
                        "sqrt" => tokens.push(Token::Function(Sqrt)),
                        "sin" => tokens.push(Token::Function(Sin)),
                        "cos" => tokens.push(Token::Function(Cos)),
                        "tan" => tokens.push(Token::Function(Tan)),
                        "log" => tokens.push(Token::Function(Log)),
                        "abs" => tokens.push(Token::Function(Abs)),

                        id => tokens.push(Token::Identifier(id.to_owned())),
                    }
                } else {
                    return Err(LexerError::InvalidCharacter(c));
                }
            }
        }
    }

    tokens.shrink_to_fit();
    Ok(tokens)
}

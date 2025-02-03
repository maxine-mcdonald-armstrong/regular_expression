use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct LexicalError {
    remaining_string: String,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "A lexical error occurred at {}", self.remaining_string)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    Char(char),
    Choice,
    Closure,
    LeftPrecedence,
    RightPrecedence,
    EmptyString,
}

fn match_token<'a>(
    token_map: &HashMap<String, Token>,
    input_string: &'a str,
) -> Result<(Token, &'a str), LexicalError> {
    for (string, token) in token_map {
        if input_string.starts_with(string) {
            return Ok((*token, &input_string[string.len()..]));
        }
    }
    Err(LexicalError {
        remaining_string: String::from(input_string),
    })
}

fn lex_string(
    token_map: &HashMap<String, Token>,
    input_string: &str,
) -> Result<Vec<Token>, LexicalError> {
    let mut token_stream: Vec<Token> = Vec::new();
    let mut remaining_input_string = input_string;
    while !remaining_input_string.is_empty() {
        let token: Token;
        (token, remaining_input_string) = match_token(token_map, remaining_input_string)?;
        token_stream.push(token);
    }
    Ok(token_stream)
}

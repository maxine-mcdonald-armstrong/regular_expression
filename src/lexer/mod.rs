use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct CharacterParsingError {
    unmatchable_char: char,
}

impl Display for CharacterParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "The following character of the input string could not be matched to a token rule: {}",
            self.unmatchable_char
        )
    }
}

#[derive(Debug, PartialEq)]
struct ReservedTokenOverwriteError {
    overwritten_token: Token,
}

impl Display for ReservedTokenOverwriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "A user-defined character has overwritten a reserved token {}. Please check the documentation to see which characters are reserved.",
            self.overwritten_token
        )
    }
}

#[derive(Debug, PartialEq)]
enum LexicalError {
    CharacterParsingError(CharacterParsingError),
}

struct TokenMap {
    token_map: HashMap<String, Token>,
}

impl TokenMap {
    fn verify_reserved_tokens_exist(&self) {}

    fn new(token_map: HashMap<String, Token>) -> Result<TokenMap, CharacterParsingError> {
        Ok(TokenMap{token_map: token_map})
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ReservedToken {
    Choice,
    Closure,
    LeftPrecedence,
    RightPrecedence,
    EmptyString,
}

impl Display for ReservedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ReservedToken::Choice => write!(f, "Choice \"|\""),
            ReservedToken::Closure => write!(f, "Closure \"*\""),
            ReservedToken::LeftPrecedence => write!(f, "Left Precedence \"(\""),
            ReservedToken::RightPrecedence => write!(f, "Right Precedence \")\""),
            ReservedToken::EmptyString => write!(f, "Empty String \"\\e\""),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    Char(char),
    ReservedToken(ReservedToken),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Char(c) => write!(f, "Char \"{}\"", c),
            Token::ReservedToken(t) => write!(f, "Reserved Token {}", t),
        }
    }
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
    Err(LexicalError::CharacterParsingError(CharacterParsingError {
        unmatchable_char: input_string.chars().next().unwrap(),
    }))
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

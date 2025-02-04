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
    overwritten_string: String,
    overwritten_token: Token,
}

impl Display for ReservedTokenOverwriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "A user-defined character {} has overwritten a reserved token {}. Please check the documentation to see which characters are reserved.",
            self.overwritten_string, self.overwritten_token
        )
    }
}

#[derive(Debug, PartialEq)]
enum LexicalError {
    CharacterParsingError(CharacterParsingError),
    ReservedTokenOverwriteError(ReservedTokenOverwriteError),
}

#[derive(Debug)]
struct TokenMap {
    token_map: HashMap<String, Token>,
}

impl TokenMap {
    fn verify_reserved_tokens_exist(
        token_map: &HashMap<String, Token>,
    ) -> Result<(), LexicalError> {
        let reserved_token_map = generate_reserved_token_map();
        for (reserved_key, reserved_token) in reserved_token_map {
            match token_map[&reserved_key] {
                Token::ReservedToken(_) => continue,
                _ => {
                    return Err(LexicalError::ReservedTokenOverwriteError(
                        ReservedTokenOverwriteError {
                            overwritten_string: reserved_key,
                            overwritten_token: reserved_token,
                        },
                    ))
                }
            }
        }
        Ok(())
    }

    fn new(token_map: HashMap<String, Token>) -> Result<TokenMap, LexicalError> {
        TokenMap::verify_reserved_tokens_exist(&token_map)?;
        Ok(TokenMap {
            token_map: token_map,
        })
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

fn generate_reserved_token_map() -> HashMap<String, Token> {
    HashMap::from([
        (
            String::from("|"),
            Token::ReservedToken(ReservedToken::Choice),
        ),
        (
            String::from("*"),
            Token::ReservedToken(ReservedToken::Closure),
        ),
        (
            String::from("("),
            Token::ReservedToken(ReservedToken::LeftPrecedence),
        ),
        (
            String::from(")"),
            Token::ReservedToken(ReservedToken::RightPrecedence),
        ),
        (
            String::from("\\e"),
            Token::ReservedToken(ReservedToken::EmptyString),
        ),
    ])
}

fn generate_token_map(alphabet: &str) -> Result<TokenMap, LexicalError> {
    let mut token_map = generate_reserved_token_map();
    for c in alphabet.chars() {
        token_map.insert(String::from(c), Token::Char(c));
    }
    TokenMap::new(token_map)
}

fn match_token<'a>(
    token_map: &TokenMap,
    input_string: &'a str,
) -> Result<(Token, &'a str), LexicalError> {
    for (string, token) in &token_map.token_map {
        if input_string.starts_with(string) {
            return Ok((*token, &input_string[string.len()..]));
        }
    }
    Err(LexicalError::CharacterParsingError(CharacterParsingError {
        unmatchable_char: input_string.chars().next().unwrap(),
    }))
}

fn lex_string(token_map: &TokenMap, input_string: &str) -> Result<Vec<Token>, LexicalError> {
    let mut token_stream: Vec<Token> = Vec::new();
    let mut remaining_input_string = input_string;
    while !remaining_input_string.is_empty() {
        let token: Token;
        (token, remaining_input_string) = match_token(token_map, remaining_input_string)?;
        token_stream.push(token);
    }
    Ok(token_stream)
}

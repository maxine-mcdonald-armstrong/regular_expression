//! Generates a [`Token`] stream from a regular expression string.
//!
//! Enforces the following properties at runtime:
//! - Every reserved token is representable by some char.
//! - The input string does not contain any chars not mapped to [`Token`]s.

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(test)]
mod tests;

/// Runtime error representing that the input string is not parsable to a [`Token`].
#[derive(Debug, PartialEq)]
pub struct CharacterParsingError {
    pub(crate) unmatchable_char: char,
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

/// Runtime error representing that the input alphabet overwrote some reserved [`Token`].
#[derive(Debug, PartialEq)]
pub struct ReservedTokenOverwriteError {
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

/// Runtime error representing that the input alphabet does not satisfy the prefix property.
///
/// This error is currently never returned, as only chars are supported in the user-defined
/// alphabet, and all reserved [`Token`]s are single-char. It is being kept in the code during
/// development as there are open issues which may result in changes to the input alphabet,
/// allowing it to contain arbitrary-length strings.
#[derive(Debug, PartialEq)]
pub struct PrefixPropertyViolationError {
    contained_string: String,
    containing_string: String,
}

impl Display for PrefixPropertyViolationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "The proposed alphabet contains keys which violate the prefix property: {} contains {}.",
            self.contained_string, self.containing_string
        )
    }
}

/// Wraps all lexer-based errors.
#[derive(Debug, PartialEq)]
pub enum LexicalError {
    CharacterParsing(CharacterParsingError),
    ReservedTokenOverwrite(ReservedTokenOverwriteError),
    PrefixPropertyViolation(PrefixPropertyViolationError),
}

/// Wraps a [`HashMap<String, Token>`], providing runtime guarantees.
///
/// Constructor [`TokenMap::new`] ensures certain properties are met at runtime.
#[derive(Debug)]
pub(crate) struct TokenMap {
    token_map: HashMap<String, Token>,
}

impl TokenMap {
    /// Verifies that reserved tokens exist and prefix property exists.
    fn verify_reserved_tokens_exist(
        token_map: &HashMap<String, Token>,
    ) -> Result<(), LexicalError> {
        let reserved_token_map = generate_reserved_token_map();
        for (reserved_key, reserved_token) in reserved_token_map {
            match token_map[&reserved_key] {
                Token::ReservedToken(_) => continue,
                _ => {
                    return Err(LexicalError::ReservedTokenOverwrite(
                        ReservedTokenOverwriteError {
                            overwritten_string: reserved_key,
                            overwritten_token: reserved_token,
                        },
                    ))
                }
            }
        }
        for key in token_map.keys() {
            let mut prefix = String::new();
            for c in key.chars() {
                String::push(&mut prefix, c);
                if prefix.len() == key.len() {
                    continue;
                }
                if token_map.contains_key(&prefix) {
                    return Err(LexicalError::PrefixPropertyViolation(
                        PrefixPropertyViolationError {
                            containing_string: String::from(key),
                            contained_string: prefix,
                        },
                    ));
                }
            }
        }
        Ok(())
    }

    /// Sanitising constructor.
    fn new(token_map: HashMap<String, Token>) -> Result<TokenMap, LexicalError> {
        TokenMap::verify_reserved_tokens_exist(&token_map)?;
        Ok(TokenMap { token_map })
    }
}

/// [`Token`]s that don't represent matches to characters in the user-defined alphabet.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ReservedToken {
    Choice,
    Closure,
    LeftPrecedence,
    RightPrecedence,
}

impl Display for ReservedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ReservedToken::Choice => write!(f, "Choice \"|\""),
            ReservedToken::Closure => write!(f, "Closure \"*\""),
            ReservedToken::LeftPrecedence => write!(f, "Left Precedence \"(\""),
            ReservedToken::RightPrecedence => write!(f, "Right Precedence \")\""),
        }
    }
}

/// [`Token`]s which are consumed by the parser.
///
/// These represent the units of the lexed text. Currently there is a direct correlation between
/// char and [`Token`], but in the future this abstraction may help the parser by wrapping
/// multi-char [`Token`]s.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Token {
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

/// Generates a [`HashMap<String, Token>`] representing [`ReservedToken`]s.
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
    ])
}

/// Generates a [`HashMap<String, Token>`] from an input [`str`] alphabet.
pub(crate) fn generate_token_map(alphabet: &str) -> Result<TokenMap, LexicalError> {
    let mut token_map = generate_reserved_token_map();
    for c in alphabet.chars() {
        token_map.insert(String::from(c), Token::Char(c));
    }
    TokenMap::new(token_map)
}

/// Matches the first [`Token`] from token_map in the input_string.
fn match_token<'a>(
    token_map: &TokenMap,
    input_string: &'a str,
) -> Result<(Token, &'a str), LexicalError> {
    for (string, token) in &token_map.token_map {
        if input_string.starts_with(string) {
            return Ok((*token, &input_string[string.len()..]));
        }
    }
    Err(LexicalError::CharacterParsing(CharacterParsingError {
        unmatchable_char: input_string.chars().next().unwrap(),
    }))
}

/// Generates a [`Vec<Token>`] representing the input_string from the token_map.
pub(crate) fn lex_string(
    token_map: &TokenMap,
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

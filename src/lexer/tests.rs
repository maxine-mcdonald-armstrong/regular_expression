use std::string;

use super::*;

fn generate_token_map(alphabet: &'static str) -> HashMap<String, Token> {
    let mut token_map = HashMap::from([
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
    ]);
    for c in alphabet.chars() {
        token_map.insert(String::from(c), Token::Char(c));
    }
    token_map
}

#[test]
fn test_token_match_ascii() {
    let token_map = generate_token_map("ab");
    let test_input = "a(b*)a|\\e";
    let expected_output = Ok(vec![
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::LeftPrecedence),
        Token::Char('b'),
        Token::ReservedToken(ReservedToken::Closure),
        Token::ReservedToken(ReservedToken::RightPrecedence),
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::ReservedToken(ReservedToken::EmptyString),
    ]);
    let lexed_string = lex_string(&token_map, test_input);
    assert_eq!(expected_output, lexed_string);
}

#[test]
fn test_invalid_input_ascii() {
    let token_map = generate_token_map("ab");
    let test_input = "aA";
    let expected_output = Err(LexicalError::CharacterParsingError(CharacterParsingError {
        unmatchable_char: 'A',
    }));
    let lexed_string = lex_string(&token_map, test_input);
    assert_eq!(expected_output, lexed_string);
}

#[test]
fn test_token_match_utf8() {
    let token_map = generate_token_map("ab⟹🦀");
    let test_input = "a|b|(🦀⟹)*";
    let expected_output = Ok(vec![
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::Char('b'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::ReservedToken(ReservedToken::LeftPrecedence),
        Token::Char('🦀'),
        Token::Char('⟹'),
        Token::ReservedToken(ReservedToken::RightPrecedence),
        Token::ReservedToken(ReservedToken::Closure),
    ]);
    let lexed_string = lex_string(&token_map, test_input);
    assert_eq!(expected_output, lexed_string);
}

#[test]
fn test_invalid_input_utf8() {
    let token_map = generate_token_map("ab⟹🦀");
    let test_input = "a🦀A🦀";
    let expected_output = Err(LexicalError::CharacterParsingError(CharacterParsingError {
        unmatchable_char: 'A',
    }));
    let lexed_string = lex_string(&token_map, test_input);
    assert_eq!(expected_output, lexed_string);
}

#[test]
fn test_empty_input() {
    let token_map = generate_token_map("");
    let test_input = "";
    let expected_output = Ok(vec![]);
    let lexed_string = lex_string(&token_map, test_input);
    assert_eq!(expected_output, lexed_string);
}

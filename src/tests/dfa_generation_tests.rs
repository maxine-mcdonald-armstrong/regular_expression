use std::collections::{HashMap, HashSet};

use super::*;

#[test]
fn test_empty_string() {
    let input_expression = "";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::new(),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_character() {
    let input_expression = "a";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_invalid_character() {
    let input_expression = "z";
    let input_alphabet = "ab";
    let expected_output = DfaGenerationError::Lexical(lexer::LexicalError::CharacterParsing(
        lexer::CharacterParsingError {
            unmatchable_char: 'z',
        },
    ));
    let output = generate_dfa(input_expression, input_alphabet).unwrap_err();
    assert_eq!(output, expected_output);
}

#[test]
fn test_closure_empty_string() {
    let input_expression = "()*";
    let input_alphabet = "";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::new(),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_invalid_closure_empty_string() {
    let input_expression = "*";
    let input_alphabet = "";
    let expected_output = DfaGenerationError::Syntactic(parser::SyntacticError::UnexpectedToken(
        parser::UnexpectedTokenError {
            token: lexer::Token::ReservedToken(lexer::ReservedToken::Closure),
            expected_tokens: vec![
                lexer::Token::Char('.'),
                lexer::Token::ReservedToken(lexer::ReservedToken::LeftPrecedence),
            ],
        },
    ));
    let output = generate_dfa(input_expression, input_alphabet).unwrap_err();
    assert_eq!(output, expected_output);
}

#[test]
fn test_closure_closure_empty_string() {
    let input_expression = "(()*)*";
    let input_alphabet = "";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::new(),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_invalid_closure_closure_empty_string() {
    let input_expression = "()**";
    let input_alphabet = "";
    let expected_output = DfaGenerationError::Syntactic(parser::SyntacticError::UnexpectedToken(
        parser::UnexpectedTokenError {
            token: lexer::Token::ReservedToken(lexer::ReservedToken::Closure),
            expected_tokens: vec![
                lexer::Token::ReservedToken(lexer::ReservedToken::Choice),
                lexer::Token::ReservedToken(lexer::ReservedToken::RightPrecedence),
                lexer::Token::Char('.'),
            ],
        },
    ));
    let output = generate_dfa(input_expression, input_alphabet).unwrap_err();
    assert_eq!(output, expected_output);
}

#[test]
fn test_closure_character() {
    let input_expression = "a*";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 0)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_closure_closure_character() {
    let input_expression = "(a*)*";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 0)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_closure_character() {
    let input_expression = "a*b*";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([0, 1]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 0), ('b', 1)])),
            (1, HashMap::from([('b', 1)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_character_and_closure_character() {
    let input_expression = "ab*";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 1)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_closure_character_and_character() {
    let input_expression = "a*b";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 0), ('b', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_closure_concatenation() {
    let input_expression = "(ab)*";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 0)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_multiple_concatenated_closures() {
    // This test needs to check many possible outputs, because DFA generated depends on
    // a random access to a HashMap and thus could follow one of many branches first.
    let input_expression = "ab*c*d";
    let input_alphabet = "abcd";
    let expected_output_a = dfa::Dfa {
        n_states: 4,
        start_state: 0,
        accepting_states: HashSet::from([3]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 1), ('c', 2), ('d', 3)])),
            (2, HashMap::from([('c', 2), ('d', 3)])),
        ]),
    };
    let expected_output_b = dfa::Dfa {
        n_states: 4,
        start_state: 0,
        accepting_states: HashSet::from([2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 1), ('d', 2), ('c', 3)])),
            (3, HashMap::from([('c', 3), ('d', 2)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert!(output == expected_output_a || output == expected_output_b);
}

#[test]
fn test_concatenated_choice_closure() {
    let input_expression = "a(a|b)*b";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 3,
        start_state: 0,
        accepting_states: HashSet::from([2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('a', 1), ('b', 2)])),
            (2, HashMap::from([('a', 1), ('b', 2)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_choice_character_character() {
    let input_expression = "a|b";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1), ('b', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_choice_character_empty_string() {
    let input_expression = "|a";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([0, 1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_invalid_choice_character_empty_string() {
    let input_expression = "a|";
    let input_alphabet = "ab";
    let expected_output = DfaGenerationError::Syntactic(
        parser::SyntacticError::MissingExpectedToken(parser::MissingExpectedTokenError {
            expected_tokens: vec![
                lexer::Token::Char('.'),
                lexer::Token::ReservedToken(lexer::ReservedToken::LeftPrecedence),
            ],
        }),
    );
    let output = generate_dfa(input_expression, input_alphabet).unwrap_err();
    assert_eq!(output, expected_output);
}

#[test]
fn test_choice_empty_string_empty_string() {
    let input_expression = "|()";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::new(),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_invalid_choice_empty_string_empty_string() {
    let input_expression = "|";
    let input_alphabet = "ab";
    let expected_output = DfaGenerationError::Syntactic(
        parser::SyntacticError::MissingExpectedToken(parser::MissingExpectedTokenError {
            expected_tokens: vec![
                lexer::Token::Char('.'),
                lexer::Token::ReservedToken(lexer::ReservedToken::LeftPrecedence),
            ],
        }),
    );
    let output = generate_dfa(input_expression, input_alphabet).unwrap_err();
    assert_eq!(output, expected_output);
}

#[test]
fn test_choice_character_concatenation() {
    let input_expression = "a|ab";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 3,
        start_state: 0,
        accepting_states: HashSet::from([1, 2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 2)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_choice_concatenation_concatenation() {
    // This test needs to check two possible outputs, because DFA generated depends on
    // a random access to a HashMap and thus could follow either of the "ab" or "ba"
    // branch first.
    let input_expression = "ab|ba";
    let input_alphabet = "ab";
    let expected_output_a = dfa::Dfa {
        n_states: 4,
        start_state: 0,
        accepting_states: HashSet::from([3]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1), ('b', 2)])),
            (1, HashMap::from([('b', 3)])),
            (2, HashMap::from([('a', 3)])),
        ]),
    };
    let expected_output_b = dfa::Dfa {
        n_states: 4,
        start_state: 0,
        accepting_states: HashSet::from([3]),
        transition_function: HashMap::from([
            (0, HashMap::from([('b', 1), ('a', 2)])),
            (1, HashMap::from([('a', 3)])),
            (2, HashMap::from([('b', 3)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert!(output == expected_output_a || output == expected_output_b);
}

#[test]
fn test_concatenated_choice_character() {
    let input_expression = "(a|b)a";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 3,
        start_state: 0,
        accepting_states: HashSet::from([2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1), ('b', 1)])),
            (1, HashMap::from([('a', 2)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_character_choice() {
    let input_expression = "a(a|b)";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 3,
        start_state: 0,
        accepting_states: HashSet::from([2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('a', 2), ('b', 2)])),
        ]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_redundant_parentheses() {
    let input_expression = "((a))";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_char_and_empty_string() {
    let input_expression = "a()";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenated_empty_string_and_char() {
    let input_expression = "()a";
    let input_alphabet = "ab";
    let expected_output = dfa::Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let output = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(output, expected_output);
}

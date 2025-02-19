use super::*;

#[test]
fn test_char_dfa_accepts_valid_char() {
    let input_expression = "a";
    let input_string = "a";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_char_dfa_rejects_invalid_char() {
    let input_expression = "a";
    let input_string = "b";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_char_dfa_rejects_non_alphabet_char() {
    let input_expression = "a";
    let input_string = "z";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_char_dfa_rejects_empty_string() {
    let input_expression = "a";
    let input_string = "";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_accepts_empty_string() {
    let input_expression = "a*";
    let input_string = "";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_accepts_single_char() {
    let input_expression = "a*";
    let input_string = "a";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_rejects_single_invalid_char() {
    let input_expression = "a*";
    let input_string = "b";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_accepts_repeated_char() {
    let input_expression = "a*";
    let input_string = "aaaaa";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_rejects_invalid_char_after_repetition() {
    let input_expression = "a*";
    let input_string = "aaaaab";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_closure_dfa_rejects_invalid_char_in_repetition() {
    let input_expression = "a*";
    let input_string = "aaaaabaaa";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_choice_dfa_accepts_left() {
    let input_expression = "a|b";
    let input_string = "a";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_choice_dfa_accepts_right() {
    let input_expression = "a|b";
    let input_string = "b";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_choice_dfa_rejects_concatenation() {
    let input_expression = "a|b";
    let input_string = "ab";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_choice_dfa_rejects_empty_string() {
    let input_expression = "a|b";
    let input_string = "";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

#[test]
fn test_complex_dfa_accepts_valid_input() {
    let input_expression = "a(a|b)*b|b(a|b)*a";
    let input_string = "abbabababb";
    let input_alphabet = "ab";
    let expected_output = true;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}
#[test]
fn test_complex_dfa_rejects_invalid_input() {
    let input_expression = "a(a|b)*b|b(a|b)*a";
    let input_string = "abbabababba";
    let input_alphabet = "ab";
    let expected_output = false;
    let dfa = generate_dfa(input_expression, input_alphabet).unwrap();
    assert_eq!(dfa.evaluate(input_string), expected_output);
}

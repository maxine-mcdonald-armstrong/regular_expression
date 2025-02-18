use super::*;
use crate::annotator::annotate_ast;
use crate::parser::Expression;

// Example given in https://tinman.cs.gsu.edu/~raj/4510/f24/RegExp2DFA.pdf.
//
// This function tests unexposed internal functionality, but is appropriate in order to
// verify that the algorithm has been correctly implemented.
#[test]
fn test_calculates_matches_next() {
    let input = Expression::Concatenation(vec![
        Expression::Closure(Box::from(Expression::Choice(vec![
            Expression::Char('a'),
            Expression::Char('b'),
        ]))),
        Expression::Char('a'),
        Expression::Char('b'),
        Expression::Char('b'),
    ]);
    let expected_output = vec![
        HashSet::from([0, 1, 2]),
        HashSet::from([0, 1, 2]),
        HashSet::from([3]),
        HashSet::from([4]),
        HashSet::from([5]),
        HashSet::new(),
    ];
    let annotated_input = annotate_ast(input).unwrap();
    let mut matches_next = vec![HashSet::<usize>::new(); 6];
    calculate_matches_next(&annotated_input.expression, &mut matches_next);
    assert_eq!(matches_next, expected_output);
}

#[test]
fn test_generates_dfa() {
    let input = Expression::Concatenation(vec![
        Expression::Closure(Box::from(Expression::Choice(vec![
            Expression::Char('a'),
            Expression::Char('b'),
        ]))),
        Expression::Char('a'),
        Expression::Char('b'),
        Expression::Char('b'),
    ]);
    let expected_output = Dfa {
        n_states: 4,
        start_state: 0,
        accepting_states: HashSet::from([3]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1), ('b', 0)])),
            (1, HashMap::from([('a', 1), ('b', 2)])),
            (2, HashMap::from([('a', 1), ('b', 3)])),
            (3, HashMap::from([('a', 1), ('b', 0)])),
        ]),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

// The following tests, testing DFA generation, are implementation-sensitive and thus
// only test simple expressions and don't provide much confidence in the DFA generation
// implementation.
#[test]
fn test_char_expression_dfa() {
    let input = Expression::Char('a');
    let expected_output = Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1)]))]),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_empty_string_expression_dfa() {
    let input = Expression::EmptyString;
    let expected_output = Dfa {
        n_states: 1,
        start_state: 0,
        accepting_states: HashSet::from([0]),
        transition_function: HashMap::new(),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_concatenation_expression_dfa() {
    let input = Expression::Concatenation(vec![Expression::Char('a'), Expression::Char('b')]);
    let expected_output = Dfa {
        n_states: 3,
        start_state: 0,
        accepting_states: HashSet::from([2]),
        transition_function: HashMap::from([
            (0, HashMap::from([('a', 1)])),
            (1, HashMap::from([('b', 2)])),
        ]),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_not_nullable_choice_expression_dfa() {
    let input = Expression::Choice(vec![Expression::Char('a'), Expression::Char('b')]);
    let expected_output = Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1), ('b', 1)]))]),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

#[test]
fn test_nullable_choice_expression_dfa() {
    let input = Expression::Choice(vec![
        Expression::EmptyString,
        Expression::Char('a'),
        Expression::Char('b'),
    ]);
    let expected_output = Dfa {
        n_states: 2,
        start_state: 0,
        accepting_states: HashSet::from([0, 1]),
        transition_function: HashMap::from([(0, HashMap::from([('a', 1), ('b', 1)]))]),
    };
    let annotated_input = annotate_ast(input).unwrap();
    let output = generate_dfa(annotated_input);
    assert_eq!(output, expected_output);
}

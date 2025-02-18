use super::*;
use crate::annotator::annotate_ast;
use crate::parser::Expression;

// Example given in https://tinman.cs.gsu.edu/~raj/4510/f24/RegExp2DFA.pdf.
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

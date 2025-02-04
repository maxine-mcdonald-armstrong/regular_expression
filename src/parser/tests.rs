use super::*;

#[test]
fn test_char() {
    let test_input = vec![Token::Char('a')];
    assert_eq!(parse(test_input).unwrap(), Expression::Char('a'));
}

#[test]
fn test_empty_string() {
    let test_input = Vec::new();
    assert_eq!(parse(test_input).unwrap(), Expression::EmptyString);
}

#[test]
fn test_precedence() {
    let test_input = vec![
        Token::ReservedToken(ReservedToken::LeftPrecedence),
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::RightPrecedence),
    ];
    assert_eq!(parse(test_input).unwrap(), Expression::Char('a'));
}

#[test]
fn test_closure() {
    let test_input = vec![
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Closure),
    ];
    let expected_output = Expression::Closure(Box::from(Expression::Char('a')));
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

#[test]
fn test_closure_fails_on_empty_string() {
    let test_input = vec![Token::ReservedToken(ReservedToken::Closure)];
    let expected_output = SyntacticError::UnexpectedTokenError(UnexpectedTokenError {
        token: Token::ReservedToken(ReservedToken::Closure),
    });
    assert_eq!(parse(test_input).unwrap_err(), expected_output);
}

#[test]
fn test_concatenation() {
    let test_input = vec![Token::Char('a'), Token::Char('b'), Token::Char('c')];
    let expected_output = Expression::Concatenation(vec![
        Box::from(Expression::Char('a')),
        Box::from(Expression::Char('b')),
        Box::from(Expression::Char('c')),
    ]);
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

#[test]
fn test_concatenation_on_closure() {
    let test_input = vec![
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Closure),
        Token::Char('b'),
    ];
    let expected_output = Expression::Concatenation(vec![
        Box::from(Expression::Closure(Box::from(Expression::Char('a')))),
        Box::from(Expression::Char('b')),
    ]);
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

#[test]
fn test_choice() {
    let test_input = vec![
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::Char('b'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::Char('c'),
    ];
    let expected_output = Expression::Choice(vec![
        Box::from(Expression::Char('a')),
        Box::from(Expression::Char('b')),
        Box::from(Expression::Char('c')),
    ]);
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

#[test]
fn test_choice_on_concatenation() {
    let test_input = vec![
        Token::Char('a'),
        Token::Char('b'),
        Token::ReservedToken(ReservedToken::Choice),
        Token::Char('b'),
        Token::Char('c'),
    ];
    let expected_output = Expression::Choice(vec![
        Box::from(Expression::Concatenation(vec![
            Box::from(Expression::Char('a')),
            Box::from(Expression::Char('b')),
        ])),
        Box::from(Expression::Concatenation(vec![
            Box::from(Expression::Char('b')),
            Box::from(Expression::Char('c')),
        ])),
    ]);
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

#[test]
fn test_choice_on_empty_string() {
    let test_input = vec![
        Token::ReservedToken(ReservedToken::Choice),
        Token::ReservedToken(ReservedToken::Choice),
        Token::Char('a'),
        Token::ReservedToken(ReservedToken::Choice),
    ];
    let expected_output = Expression::Choice(vec![
        Box::from(Expression::EmptyString),
        Box::from(Expression::EmptyString),
        Box::from(Expression::Char('a')),
        Box::from(Expression::EmptyString),
    ]);
    assert_eq!(parse(test_input).unwrap(), expected_output);
}

use super::*;

fn unwrap_terminal(expression: AnnotatedExpressionContext) -> Option<AnnotatedExpressionContext> {
    match &expression.expression.expression {
        AnnotatedExpressionType::Concatenation(subexpressions) => {
            if subexpressions.len() != 2 {
                panic!(
                    "Expected Concat(_, Terminal); Got {:?}.",
                    expression.expression.expression
                )
            }
            let index = expression.leaves.len() - 1;
            if subexpressions[1]
                != Rc::from(AnnotatedExpression {
                    expression: AnnotatedExpressionType::Terminal(index),
                    is_nullable: false,
                    matches_start: HashSet::from([index]),
                    matches_end: HashSet::from([index]),
                })
            {
                panic!(
                    "Expected Concat(_, Terminal); Got {:?}.",
                    expression.expression.expression
                )
            }
            Some(AnnotatedExpressionContext {
                expression: Rc::clone(&subexpressions[0]),
                leaves: (&expression.leaves)[..index].to_vec(),
            })
        }
        _ => panic!(
            "Expected Concat(_, Terminal); Got {:?}.",
            expression.expression.expression
        ),
    }
}

#[test]
fn test_char_expression_annotation() {
    let input = Expression::Char('a');
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::Char('a', 0),
        is_nullable: false,
        matches_start: HashSet::from([0]),
        matches_end: HashSet::from([0]),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
    assert_eq!(output.expression, output.leaves[0]);
}

#[test]
fn test_empty_string_expression_annotation() {
    let input = Expression::EmptyString;
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::EmptyString(0),
        is_nullable: true,
        matches_start: HashSet::new(),
        matches_end: HashSet::new(),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
    assert_eq!(output.expression, output.leaves[0]);
}

#[test]
fn test_concatenation_annotation() {
    let input = Expression::Concatenation(vec![Expression::Char('a'), Expression::Char('b')]);
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::Concatenation(vec![
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char('a', 0),
                is_nullable: false,
                matches_start: HashSet::from([0]),
                matches_end: HashSet::from([0]),
            }),
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char('b', 1),
                is_nullable: false,
                matches_start: HashSet::from([1]),
                matches_end: HashSet::from([1]),
            }),
        ]),
        is_nullable: false,
        matches_start: HashSet::from([0]),
        matches_end: HashSet::from([1]),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
}

#[test]
fn test_nullable_choice_annotation() {
    let input = Expression::Choice(vec![Expression::EmptyString, Expression::Char('a')]);
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::Choice(vec![
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::EmptyString(0),
                is_nullable: true,
                matches_start: HashSet::new(),
                matches_end: HashSet::new(),
            }),
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char('a', 1),
                is_nullable: false,
                matches_start: HashSet::from([1]),
                matches_end: HashSet::from([1]),
            }),
        ]),
        is_nullable: true,
        matches_start: HashSet::from([1]),
        matches_end: HashSet::from([1]),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
}

#[test]
fn test_not_nullable_choice_annotation() {
    let input = Expression::Choice(vec![Expression::Char('a'), Expression::Char('b')]);
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::Choice(vec![
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char('a', 0),
                is_nullable: false,
                matches_start: HashSet::from([0]),
                matches_end: HashSet::from([0]),
            }),
            Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char('b', 1),
                is_nullable: false,
                matches_start: HashSet::from([1]),
                matches_end: HashSet::from([1]),
            }),
        ]),
        is_nullable: false,
        matches_start: HashSet::from([0, 1]),
        matches_end: HashSet::from([0, 1]),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
}

#[test]
fn test_closure_annotation() {
    let input = Expression::Closure(Box::from(Expression::Char('a')));
    let expected_annotated_expression = AnnotatedExpression {
        expression: AnnotatedExpressionType::Closure(Rc::from(AnnotatedExpression {
            expression: AnnotatedExpressionType::Char('a', 0),
            is_nullable: false,
            matches_start: HashSet::from([0]),
            matches_end: HashSet::from([0]),
        })),
        is_nullable: true,
        matches_start: HashSet::from([0]),
        matches_end: HashSet::from([0]),
    };
    let output = unwrap_terminal(annotate_ast(input).unwrap()).unwrap();
    assert_eq!(*output.expression, expected_annotated_expression);
}

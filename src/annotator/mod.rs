//! Annotates an AST of [`Expression`] nodes for transformation.

use crate::parser::Expression;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    rc::Rc,
    vec,
};

#[cfg(test)]
mod tests;

/// Represents the expression tree recursively.
#[derive(Clone, Debug, PartialEq)]
pub enum AnnotatedExpressionType<T> {
    Char(char, usize),
    EmptyString(usize),
    /// Represents the end of the regular expression. This is only necessary for
    /// creating the DFA.
    Terminal(usize),
    Closure(Rc<T>),
    Concatenation(Vec<Rc<T>>),
    Choice(Vec<Rc<T>>),
}

/// Represents an expression annotated as necessary for creating a DFA.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedExpression {
    /// The expression itself.
    pub expression: AnnotatedExpressionType<AnnotatedExpression>,
    /// Represents whether this expression matches the empty string.
    pub is_nullable: bool,
    /// Represents the leaf nodes of this expression which could match the start of a
    /// string accepted by this expression.
    pub matches_start: HashSet<usize>,
    /// Represents the leaf nodes which could match the end of a string accepted by
    /// this expression.
    pub matches_end: HashSet<usize>,
}

#[derive(Debug)]
/// Stores an expression and a vector of the leaves of that expression, allowing
/// indexed access.
pub struct AnnotatedExpressionContext {
    pub expression: Rc<AnnotatedExpression>,
    pub leaves: Vec<Rc<AnnotatedExpression>>,
}

/// Raised if the number of leaf nodes exceeds the capacity of a vector.
#[derive(Debug, PartialEq)]
pub struct NodeOverflowError {
    size: usize,
}

impl Display for NodeOverflowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "The generated AST has too many leaf nodes, >{}.",
            self.size
        )
    }
}

/// Raised if the input expression is invalid.
#[derive(Debug, PartialEq)]
pub struct InvalidExpressionError {}

impl Display for InvalidExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "The generated AST includes an invalid expression.",)
    }
}

/// Wraps all annotator-based errors.
#[derive(Debug, PartialEq)]
pub enum AnnotationError {
    NodeOverflow(NodeOverflowError),
    InvalidExpression(InvalidExpressionError),
}

/// Recursively annotates an input expression.
fn annotate_expression(
    expression: Expression,
    next_index: &mut usize,
    leaves: Vec<Rc<AnnotatedExpression>>,
) -> Result<AnnotatedExpressionContext, AnnotationError> {
    match expression {
        Expression::Char(c) => {
            let next_expression = Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Char(c, *next_index),
                is_nullable: false,
                matches_start: HashSet::from([*next_index]),
                matches_end: HashSet::from([*next_index]),
            });
            *next_index += 1;
            let mut next_leaves = leaves;
            next_leaves.push(Rc::clone(&next_expression));
            match next_index {
                0 => Err(AnnotationError::NodeOverflow(NodeOverflowError {
                    size: *next_index - 1,
                })),
                _ => Ok(AnnotatedExpressionContext {
                    expression: Rc::clone(&next_expression),
                    leaves: next_leaves,
                }),
            }
        }
        Expression::EmptyString => {
            let next_expression = Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::EmptyString(*next_index),
                is_nullable: true,
                matches_start: HashSet::new(),
                matches_end: HashSet::new(),
            });
            *next_index += 1;
            let mut next_leaves = leaves;
            next_leaves.push(Rc::clone(&next_expression));
            match next_index {
                0 => Err(AnnotationError::NodeOverflow(NodeOverflowError {
                    size: *next_index - 1,
                })),
                _ => Ok(AnnotatedExpressionContext {
                    expression: Rc::clone(&next_expression),
                    leaves: next_leaves,
                }),
            }
        }
        Expression::Closure(sub_expression) => {
            let internal_expression = annotate_expression(*sub_expression, next_index, leaves)?;
            let next_expression = Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Closure(Rc::clone(
                    &internal_expression.expression,
                )),
                is_nullable: true,
                matches_start: internal_expression.expression.matches_start.clone(),
                matches_end: internal_expression.expression.matches_start.clone(),
            });
            Ok(AnnotatedExpressionContext {
                expression: next_expression,
                leaves: internal_expression.leaves,
            })
        }
        Expression::Choice(sub_expressions) => {
            let mut internal_expressions = vec![];
            let mut is_nullable = false;
            let mut start_positions = HashSet::new();
            let mut end_positions = HashSet::new();
            let mut next_leaves = leaves;
            for internal_expression in sub_expressions {
                let next_expression =
                    annotate_expression(internal_expression, next_index, next_leaves)?;
                internal_expressions.push(Rc::clone(&next_expression.expression));
                is_nullable = is_nullable || next_expression.expression.is_nullable;
                start_positions.extend(next_expression.expression.matches_start.iter().copied());
                end_positions.extend(next_expression.expression.matches_end.iter().copied());
                next_leaves = next_expression.leaves;
            }
            let next_expression = Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Choice(internal_expressions),
                is_nullable,
                matches_start: start_positions,
                matches_end: end_positions,
            });
            Ok(AnnotatedExpressionContext {
                expression: next_expression,
                leaves: next_leaves,
            })
        }
        Expression::Concatenation(sub_expressions) => {
            let l = sub_expressions.len();
            if l == 0 {
                return Err(AnnotationError::InvalidExpression(
                    InvalidExpressionError {},
                ));
            }
            let mut internal_expressions = vec![];
            let mut is_nullable = true;
            let mut next_leaves = leaves;
            let mut matches_start = HashSet::new();
            let mut matches_end = HashSet::new();
            for (i, internal_expression) in sub_expressions.into_iter().enumerate() {
                let next_expression =
                    annotate_expression(internal_expression, next_index, next_leaves)?;
                internal_expressions.push(Rc::clone(&next_expression.expression));
                if i == 0 || is_nullable {
                    matches_start.extend(next_expression.expression.matches_start.clone());
                }
                is_nullable = is_nullable && next_expression.expression.is_nullable;
                if i == l - 1 || is_nullable {
                    matches_end.extend(next_expression.expression.matches_end.clone());
                } else {
                    matches_end = HashSet::new();
                }
                next_leaves = next_expression.leaves;
            }
            let next_expression = Rc::from(AnnotatedExpression {
                expression: AnnotatedExpressionType::Concatenation(internal_expressions),
                is_nullable,
                matches_start,
                matches_end,
            });
            Ok(AnnotatedExpressionContext {
                expression: next_expression,
                leaves: next_leaves,
            })
        }
    }
}

/// Annotates a complete input expression, adding a terminal node at the end.
pub fn annotate_ast(root_node: Expression) -> Result<AnnotatedExpressionContext, AnnotationError> {
    let expression = annotate_expression(root_node, &mut 0, vec![])?;
    if expression.leaves.len() + 1 == 0 {
        return Err(AnnotationError::NodeOverflow(NodeOverflowError {
            size: expression.leaves.len(),
        }));
    }
    let terminal = Rc::from(AnnotatedExpression {
        expression: AnnotatedExpressionType::Terminal(expression.leaves.len()),
        is_nullable: false,
        matches_start: HashSet::from([expression.leaves.len()]),
        matches_end: HashSet::from([expression.leaves.len()]),
    });
    let mut combined_matches_start = HashSet::new();
    let mut combined_matches_end = HashSet::new();
    if expression.expression.is_nullable {
        combined_matches_start.extend(terminal.matches_start.iter().copied());
        combined_matches_end.extend(expression.expression.matches_end.iter().copied());
    }
    combined_matches_start.extend(expression.expression.matches_start.iter().copied());
    combined_matches_end.extend(terminal.matches_end.iter().copied());
    let combined_expression = Rc::from(AnnotatedExpression {
        expression: AnnotatedExpressionType::Concatenation(vec![
            Rc::clone(&expression.expression),
            Rc::clone(&terminal),
        ]),
        is_nullable: false,
        matches_start: combined_matches_start,
        matches_end: combined_matches_end,
    });
    let mut next_leaves = expression.leaves;
    next_leaves.push(Rc::clone(&terminal));
    Ok(AnnotatedExpressionContext {
        expression: Rc::clone(&combined_expression),
        leaves: next_leaves,
    })
}

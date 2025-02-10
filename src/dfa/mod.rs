use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::parser::Expression;

#[cfg(test)]
mod tests;

struct DfaNode {
    index: usize,
    is_accepting: bool,
    transitions: HashMap<char, usize>,
}

struct Dfa {
    states: Vec<DfaNode>,
}

fn nullable(expression: Expression) -> bool {
    match expression {
        Expression::EmptyString => true,
        Expression::Char(_) => false,
        Expression::Closure(subexpression) => nullable(*subexpression),
        Expression::Choice(subexpression) => subexpression.iter().map(|e| nullable(e)).reduce(|acc, e| acc || e)?,
    }
}

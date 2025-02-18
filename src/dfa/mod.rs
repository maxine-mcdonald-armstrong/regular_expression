use crate::annotator::{AnnotatedExpression, AnnotatedExpressionContext, AnnotatedExpressionType};
use std::collections::{BTreeSet, HashMap, HashSet};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Dfa {
    states: Vec<BTreeSet<usize>>,
    alphabet: HashSet<char>,
    start_state: usize,
    accepting_states: HashSet<usize>,
    /// The transition function from state X alphabet -> state
    transition_function: HashMap<usize, HashMap<char, usize>>,
}

/// calculate_matches_next(e)[i] is a set of the leaf nodes which will match the first
/// character of the string remaining after matching node i.
fn calculate_matches_next(
    expression: &AnnotatedExpression,
    matches_next: &mut Vec<HashSet<usize>>,
) {
    match &expression.expression {
        AnnotatedExpressionType::Concatenation(sub_expressions) => {
            for sub_expression in sub_expressions {
                calculate_matches_next(sub_expression, matches_next);
            }
            for i in 0..sub_expressions.len() - 1 {
                let prev_expression = &sub_expressions[i];
                let next_expression = &sub_expressions[i + 1];
                for j in &prev_expression.matches_end {
                    matches_next[*j].extend(next_expression.matches_start.iter().copied());
                }
            }
        }
        AnnotatedExpressionType::Closure(sub_expression) => {
            calculate_matches_next(sub_expression, matches_next);
            for i in &sub_expression.matches_end {
                matches_next[*i].extend(sub_expression.matches_start.iter().copied());
            }
        }
        AnnotatedExpressionType::Choice(sub_expressions) => {
            for sub_expression in sub_expressions {
                calculate_matches_next(sub_expression, matches_next);
            }
        }
        _ => (),
    }
}

fn generate_dfa(expression: AnnotatedExpressionContext, alphabet: HashSet<char>) -> Dfa {
    let mut matches_next = vec![HashSet::<usize>::new(); expression.leaves.len()];
    let mut unmarked_states_map = HashMap::new();
    let mut marked_states_map = HashMap::new();
    let mut dfa = Dfa {
        states: Vec::new(),
        alphabet,
        start_state: 0,
        accepting_states: HashSet::new(),
        transition_function: HashMap::new(),
    };
    calculate_matches_next(&expression.expression, &mut matches_next);
    let initial_state = BTreeSet::from_iter(expression.expression.matches_start.iter().copied());
    dfa.states.push(initial_state.clone());
    if initial_state.contains(&(expression.leaves.len() - 1)) {
        dfa.accepting_states.insert(0);
    }
    unmarked_states_map.insert(initial_state, 0);
    let mut next_state_index = 1;
    while !unmarked_states_map.is_empty() {
        let unmarked_state = unmarked_states_map.keys().next().unwrap().clone();
        let unmarked_state_index = unmarked_states_map.remove(&unmarked_state).unwrap();
        marked_states_map.insert(unmarked_state.clone(), unmarked_state_index);
        let mut grouped_by_char: HashMap<char, Vec<usize>> = HashMap::new();
        for leaf_index in unmarked_state {
            if let AnnotatedExpressionType::Char(c, i) = &expression.leaves[leaf_index].expression {
                for k in &matches_next[*i] {
                    grouped_by_char.entry(*c).or_default().push(*k);
                }
            }
        }
        for (c, char_leaves) in grouped_by_char {
            let mut target_state = BTreeSet::new();
            let target_state_index;
            for leaf in char_leaves {
                target_state.insert(leaf);
            }
            if target_state.is_empty() {
                continue;
            }
            if !unmarked_states_map.contains_key(&target_state)
                && !marked_states_map.contains_key(&target_state)
            {
                unmarked_states_map
                    .entry(target_state.clone())
                    .or_insert(next_state_index);
                dfa.states.push(target_state.clone());
                target_state_index = next_state_index;
                next_state_index += 1;
            } else if unmarked_states_map.contains_key(&target_state) {
                target_state_index = unmarked_states_map[&target_state];
            } else {
                target_state_index = marked_states_map[&target_state];
            }
            if target_state.contains(&(expression.leaves.len() - 1)) {
                dfa.accepting_states.insert(target_state_index);
            }
            dfa.transition_function
                .entry(unmarked_state_index)
                .or_default()
                .insert(c, target_state_index);
        }
    }
    dfa
}

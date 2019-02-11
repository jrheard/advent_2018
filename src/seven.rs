use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::iter::FromIterator;
use std::rc::Rc;

use hashbrown::HashMap;
use hashbrown::HashSet;

#[derive(Debug, PartialEq)]
struct StepConstraint {
    first: char,
    then: char,
}

impl StepConstraint {
    pub fn new(s: &str) -> StepConstraint {
        let (first, then) =
            scan!("Step {} must be finished before step {} can begin." <- s).unwrap();
        StepConstraint { first, then }
    }
}

#[derive(Clone, Debug)]
struct Node {
    step: char,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(step: char) -> Node {
        Node {
            step: step,
            children: RefCell::new(vec![]),
        }
    }
}

fn find_step_in_graph(root_node: Rc<Node>, step: char) -> Option<Rc<Node>> {
    if root_node.step == step {
        return Some(root_node);
    } else {
        for node in *root_node.children.borrow() {
            if let Some(ret) = find_step_in_graph(node, step) {
                return Some(ret);
            }
        }
    }

    None
}

fn construct_dependency_graph(step_constraints: &[StepConstraint]) -> Node {
    // Make a map of step -> [steps that depend on this step].
    let mut step_parents = HashMap::new();
    for constraint in step_constraints {
        let depended_on_by = step_parents.entry(constraint.then).or_insert(vec![]);
        depended_on_by.push(constraint.first);
    }

    // Find the one step that isn't depended on by anything. That'll be our root step.
    let mut all_steps = HashSet::new();
    for constraint in step_constraints {
        all_steps.insert(constraint.first);
        all_steps.insert(constraint.then);
    }

    let steps_with_dependencies = HashSet::from_iter(step_parents.keys().cloned());
    let steps_with_no_dependencies: Vec<char> = all_steps
        .difference(&steps_with_dependencies)
        .cloned()
        .collect();

    assert_eq!(steps_with_no_dependencies.iter().count(), 1);

    // Found it!
    let root_step = steps_with_no_dependencies[0];
    let root_node = Rc::new(Node::new(root_step));

    let mut constraint_deque = VecDeque::from_iter(step_constraints.iter());

    while let Some(constraint) = constraint_deque.pop_front() {
        if let Some(node_rc) = find_step_in_graph(root_node, constraint.first) {
            // The first step of this constraint has an entry in the dependency graph!

            let child = if let Some(child_rc) = find_step_in_graph(root_node, constraint.then) {
                // The second step of this constraint also has an entry in the graph,
                // so let's just Rc::clone it and that'll be this constraint's child node.
                Rc::clone(&child_rc)
            } else {
                // The second step of this constraint doesn't have an entry in this graph,
                // so let's make one.
                Rc::new(Node::new(constraint.then))
            };

            node_rc.children.borrow_mut().push(child);
        } else {
            // Not ready to process this constraint yet, replace it at the end of the queue and try again later.
            constraint_deque.push_back(constraint);
        }
    }

    Rc::try_unwrap(root_node).unwrap()
}

/// The instructions specify a series of steps and requirements about
/// which steps must be finished before others can begin (your puzzle input).
/// Each step is designated by a single letter.
pub fn seven_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/7_sample.txt").unwrap();

    let steps: Vec<StepConstraint> = contents.lines().map(StepConstraint::new).collect();

    dbg!(construct_dependency_graph(&steps));

    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_constraint_new() {
        assert_eq!(
            StepConstraint::new("Step C must be finished before step A can begin."),
            StepConstraint {
                first: 'C',
                then: 'A'
            }
        )
    }
}

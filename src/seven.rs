use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::iter::FromIterator;
use std::rc::Rc;

use hashbrown::HashMap;
use hashbrown::HashSet;
use serde_scan::scan;

const SENTINEL_ROOT_NODE_VALUE: char = '☃';

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

#[derive(Clone, Debug, PartialEq)]
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

fn find_step_in_graph(node: Rc<Node>, step: char) -> Option<Rc<Node>> {
    if node.step == step {
        return Some(node);
    } else {
        for child in node.children.borrow().iter() {
            if let Some(ret) = find_step_in_graph(Rc::clone(child), step) {
                return Some(ret);
            }
        }
    }

    None
}

fn construct_dependency_graph(step_constraints: &[StepConstraint]) -> Rc<Node> {
    // Make a map of step -> [steps that depend on this step].
    let mut step_parents = HashMap::new();
    for constraint in step_constraints {
        let depended_on_by = step_parents.entry(constraint.then).or_insert(vec![]);
        depended_on_by.push(constraint.first);
    }

    // Find the nodes that aren't depended on by anything.
    let mut all_steps = HashSet::new();
    for constraint in step_constraints {
        all_steps.insert(constraint.first);
        all_steps.insert(constraint.then);
    }

    let steps_with_dependencies = HashSet::from_iter(step_parents.keys().cloned());
    let steps_with_no_dependencies = all_steps.difference(&steps_with_dependencies);

    let root_node = Rc::new(Node {
        step: SENTINEL_ROOT_NODE_VALUE,
        children: RefCell::new(
            steps_with_no_dependencies
                .map(|&step| Rc::new(Node::new(step)))
                .collect(),
        ),
    });

    // We've assembled our root node, we're good to go, let's start populating the graph.
    let mut constraint_deque = VecDeque::from_iter(step_constraints.iter());

    while let Some(constraint) = constraint_deque.pop_front() {
        if let Some(node_rc) = find_step_in_graph(Rc::clone(&root_node), constraint.first) {
            // The first step of this constraint has an entry in the dependency graph!

            let child = if let Some(child_rc) =
                find_step_in_graph(Rc::clone(&root_node), constraint.then)
            {
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

    root_node
}

struct GraphWalker {
    buffer: Vec<Rc<Node>>,
}

impl GraphWalker {
    fn new(root_node: Rc<Node>) -> GraphWalker {
        let mut walker = GraphWalker {
            buffer: vec![root_node],
        };
        walker.pop_node(SENTINEL_ROOT_NODE_VALUE);

        walker
    }

    fn peek(&self) -> char {
        self.buffer[0].step
    }

    fn available_steps(&self) -> Vec<char> {
        self.buffer.iter().map(|node| node.step).collect()
    }

    fn pop_node(&mut self, step: char) {
        let index = self
            .buffer
            .iter()
            .position(|node| node.step == step)
            .unwrap();
        let node = self.buffer.remove(index);

        for child in node.children.borrow().iter() {
            // If a node's parents have all been processed, then that node's step is ready to go!
            if Rc::strong_count(child) == 1 {
                self.buffer.push(Rc::clone(child));
            }
        }

        // "If more than one step is ready, choose the step which is first alphabetically."
        self.buffer.sort_by_key(|node| node.step);
    }
}

fn dependency_graph_resolution_order(root_node: Rc<Node>) -> String {
    let mut ret = String::new();
    let mut walker = GraphWalker::new(root_node);

    while !walker.buffer.is_empty() {
        let step = walker.peek();
        ret.push(step);
        walker.pop_node(step);
    }

    ret
}

/// The instructions specify a series of steps and requirements about
/// which steps must be finished before others can begin (your puzzle input).
/// Each step is designated by a single letter.
/// Your first goal is to determine the order in which the steps should be completed.
pub fn seven_a() -> String {
    let contents = fs::read_to_string("src/inputs/7.txt").unwrap();
    let steps: Vec<StepConstraint> = contents.lines().map(StepConstraint::new).collect();
    let graph = construct_dependency_graph(&steps);

    dependency_graph_resolution_order(graph)
}

/// Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3,
/// and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds.
fn step_duration(step: char) -> u32 {
    61 + u32::from((step as u8) - b'A')
}

#[derive(Debug)]
struct ElfJob {
    step: char,
    time_left: u32,
}

#[derive(Debug)]
struct ElfPool {
    num_elves: usize,
    jobs: Vec<ElfJob>,
}

/// A pool of helpful elves.
impl ElfPool {
    fn new(num_elves: usize) -> ElfPool {
        ElfPool {
            num_elves,
            jobs: vec![],
        }
    }

    /// Advance time one second. Return a vector of any steps that were completed during this second.
    fn advance_time(&mut self) -> Vec<char> {
        let mut ret = vec![];

        for job in &mut self.jobs {
            job.time_left -= 1;

            if job.time_left == 0 {
                ret.push(job.step);
            }
        }

        self.jobs.retain(|job| job.time_left > 0);

        ret
    }

    /// Start an elf working on a given step of the sleigh's assembly.
    fn add_job(&mut self, step: char) {
        assert_ne!(self.jobs.len(), self.num_elves);

        self.jobs.push(ElfJob {
            step,
            time_left: step_duration(step),
        });
    }

    fn steps_in_progress(&self) -> HashSet<char> {
        HashSet::from_iter(self.jobs.iter().map(|job| job.step))
    }
}

/// Now, you need to account for multiple people working on steps simultaneously.
/// If multiple steps are available, workers should still begin them in alphabetical order.
pub fn seven_b() -> i32 {
    let contents = fs::read_to_string("src/inputs/7.txt").unwrap();
    let steps: Vec<StepConstraint> = contents.lines().map(StepConstraint::new).collect();
    let graph = construct_dependency_graph(&steps);

    let mut pool = ElfPool::new(5);
    let mut walker = GraphWalker::new(graph);
    let mut seconds = 0;

    // While the sleigh is not yet put together:
    while !walker.buffer.is_empty() || !pool.jobs.is_empty() {
        // Figure out which steps are available but aren't yet being worked on.
        let all_available_steps: HashSet<char> = HashSet::from_iter(walker.available_steps());
        let steps_in_progress = pool.steps_in_progress();
        let steps_not_being_worked_on = all_available_steps.difference(&steps_in_progress);

        // Add jobs until all of the elves are busy or we can't add more jobs.
        for &step in steps_not_being_worked_on {
            if pool.jobs.len() < pool.num_elves {
                pool.add_job(step);
            }
        }

        // Advance time one second and see if any jobs are done.
        let done_steps = pool.advance_time();
        for step in done_steps {
            walker.pop_node(step);
        }

        seconds += 1;
    }

    seconds
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(seven_a(), "ABGKCMVWYDEHFOPQUILSTNZRJX");
        assert_eq!(seven_b(), 898);
    }

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

    #[test]
    fn test_step_duration() {
        assert_eq!(step_duration('A'), 61);
        assert_eq!(step_duration('Z'), 86);
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
use binary_heap_plus::BinaryHeap;
use std::cell::RefCell;

use solutions::utils;

static INPUT: &str = "data/input7";


#[derive(Debug)]
struct StepNode {
    requirements: HashSet<String>,
    dependency: Vec<String>,
}

impl StepNode {
    fn new() -> StepNode {
        StepNode {
            requirements: HashSet::new(),
            dependency: Vec::new(),
        }
    }
}


fn create_step_graph(instructions: Vec<&str>) -> HashMap<String, RefCell<StepNode>> {
    let mut step_graph = HashMap::new();
    for instruction in instructions {
        let fields = instruction.split_whitespace().collect::<Vec<_>>();
        let requirement = fields[1].to_owned();
        let dependency = fields[7].to_owned();

        {
            let required_node = step_graph
                .entry(requirement.clone())
                .or_insert(RefCell::new(StepNode::new()));

            required_node.borrow_mut().dependency.push(dependency.clone());
        }

        {
            let dependent_node = step_graph
                .entry(dependency.clone())
                .or_insert(RefCell::new(StepNode::new()));

            dependent_node.borrow_mut().requirements.insert(requirement.clone());
        }
    }

    step_graph
}


// Problem 1
// ==================================================

/// Do a topological sort over the step graph, breaking ties by lexicographic order.
/// We do this as follows:
///     1. Find all sinks (=no requirements), take the smallest, say S, by lexicographic order
///     2. Remove S from the requirements of the steps it points to and remove S from the graph
///     3. If the graph is not empty, go back to step 1. (Old sinks are still sinks but
///        satisfying S might have created more sinks which are smaller.)
///     The sink set is maintained in a priority queue (by lexicographic order).
fn find_instruction_order(mut step_graph: HashMap<String, RefCell<StepNode>>) -> String {
    let mut instruction_order = String::new();

    // find initial set of sinks by iterating through all nodes. we push values
    // manually because collect::<..> doesn't seem to work for the min heap variant.
    let mut sink_set = BinaryHeap::new_min();
    for sink_node_id in step_graph
        .iter()
        .filter(|&(_, node)| node.borrow().requirements.is_empty())
        .map(|(id, _)| id.clone()) {
        sink_set.push(sink_node_id);
    }

    while !step_graph.is_empty() {
        // this pop should always return something if the graph is not empty
        // because a non-empty DAG always has some sinks, else there's a cycle.
        if let Some(sink_node_id) = sink_set.pop() {
            // add this node_id to the instruction order
            instruction_order.push_str(&sink_node_id);
            // remove this node as a requirement from the nodes that depend on it
            // and check if new sinks are formed when removing this node
            {
                let removed_sink_node = step_graph.get(&sink_node_id).unwrap();
                for dependent_node_id in &removed_sink_node.borrow().dependency {
                    let dependent_node = step_graph.get(dependent_node_id).unwrap();
                    dependent_node.borrow_mut().requirements.remove(&sink_node_id);

                    if dependent_node.borrow().requirements.is_empty() {
                        sink_set.push(dependent_node_id.clone());
                    }
                }
            }

            // remove the processed sink from the graph
            step_graph.remove(&sink_node_id);
        } else {
            panic!("The graph seems to have a cycle?")
        }
    }

    instruction_order
}

// Problem 2
// ==================================================

// Interface
// ==================================================

pub fn solution1() -> () {
    let instructions = utils::file_to_string(INPUT);
    let instructions = instructions.lines().collect::<Vec<_>>();
    let step_graph = create_step_graph(instructions);
    let instruction_order = find_instruction_order(step_graph);
    println!("Instructions should be executed as '{}'", instruction_order);
}


pub fn solution2() -> () {}


pub fn solve_day() {
    solution1();
    solution2();
}


// Test the sample puzzle inputs
// ==================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_samples1() {
        let test_instructions =
            "Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.";

        let instructions = test_instructions.lines().collect::<Vec<_>>();
        let step_graph = create_step_graph(instructions);

        assert_eq!(find_instruction_order(step_graph), "CABDFE");
    }

    #[test]
    fn test_samples2() {}
}

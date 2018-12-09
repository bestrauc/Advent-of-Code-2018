use std::collections::HashMap;
use std::collections::HashSet;
use binary_heap_plus::{BinaryHeap, MinComparator};
use std::cell::RefCell;
use std::cmp;

use solutions::utils;

static INPUT: &str = "data/input7";


#[derive(Debug)]
struct StepNode {
    requirements: HashSet<String>,
    dependency: Vec<String>,
    start_time: u32,
}

impl StepNode {
    fn new() -> StepNode {
        StepNode {
            requirements: HashSet::new(),
            dependency: Vec::new(),
            start_time: 0,
        }
    }
}

/// since we used String (unnecessarily) for the step IDs, we convert
/// them to char here and get the duration for the first char
fn get_duration_for_id(id: &str) -> u32 {
    let str_bytes = id.bytes().collect::<Vec<_>>();

    ((str_bytes[0] - 64) as u32)
//    60 + ((str_bytes[0] - 64) as u32)
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
/// the predecessor node has accumulated some total processing time,
/// which potentially increases the time needed to finish the node
/// (this is similar to the classical shortest path step `dist + t < current_dist`)
fn update_node_processing_time(predecessor: &StepNode, node: &mut StepNode, predecessor_time: u32) {
    if predecessor.start_time + predecessor_time > node.start_time {
        node.start_time = predecessor.start_time + predecessor_time;
    }
}

/// Do a topological sort over the step graph, breaking ties by lexicographic order.
/// We do this as follows:
///     1. Find all sinks (=no requirements), take the smallest, say S, by lexicographic order
///     2. Remove S from the requirements of the steps it points to and remove S from the graph
///     3. If the graph is not empty, go back to step 1. (Old sinks are still sinks but
///        satisfying S might have created more sinks which are smaller.)
///     The sink set is maintained in a priority queue (by lexicographic order).
fn find_instruction_order(step_graph: HashMap<String, RefCell<StepNode>>,
                          num_workers: usize) -> (u32, String) {
    // find initial set of sinks by iterating through all nodes. we push values
    // manually because collect::<..> doesn't seem to work for the min heap variant.
    let mut sink_set = BinaryHeap::new_min();
    for sink_node_id in step_graph
        .iter()
        .filter(|&(_, node)| node.borrow().requirements.is_empty())
        .map(|(id, _)| id.clone()) {
        sink_set.push(sink_node_id);
    }

    while !sink_set.is_empty() {
        let sink_node_id = sink_set.pop().unwrap();

        let removed_sink_node = step_graph.get(&sink_node_id).unwrap();

        // remove this node as a requirement from the nodes that depend on it
        for dependent_node_id in &removed_sink_node.borrow().dependency {
            let dependent_node = step_graph.get(dependent_node_id).unwrap();
            dependent_node.borrow_mut().requirements.remove(&sink_node_id);

            // check if new sinks are formed when removing this node
            // if yes, add them to our priority queue, with updated times
            if dependent_node.borrow().requirements.is_empty() {
                update_node_processing_time(&removed_sink_node.borrow(),
                                            &mut dependent_node.borrow_mut(),
                                            get_duration_for_id(&sink_node_id));

                sink_set.push(dependent_node_id.clone());
            }
        }
    }

    let mut sorted_graph_nodes = step_graph
        .iter()
        .map(|(key, node)| (key, node.borrow().start_time + get_duration_for_id(key)))
        .collect::<Vec<_>>();

    sorted_graph_nodes.sort_by_key(|(_, duration)| *duration);
    println!("{:?}", sorted_graph_nodes);


    let duration = sorted_graph_nodes.last().unwrap().1;
    let mut completion_order = String::new();
    for (key, _) in sorted_graph_nodes {
        completion_order.push_str(key);
    }


    (duration, completion_order)
}

// Problem 2
// ==================================================

// Interface
// ==================================================

pub fn solution1() -> () {
    let instructions = utils::file_to_string(INPUT);
    let instructions = instructions.lines().collect::<Vec<_>>();
    let step_graph = create_step_graph(instructions);
    let (processing_time, instruction_order) = find_instruction_order(step_graph, 1);
    println!("Instructions should be executed as '{}' in {}s", instruction_order, processing_time);
}


pub fn solution2() -> () {
    let instructions = utils::file_to_string(INPUT);
    let instructions = instructions.lines().collect::<Vec<_>>();
    let step_graph = create_step_graph(instructions);
    let (processing_time, instruction_order) = find_instruction_order(step_graph, 2);
    println!("Instructions should be executed as '{}' in {}s", instruction_order, processing_time);
}


pub fn solve_day() {
    solution1();
    solution2();
}


// Test the sample puzzle inputs
// ==================================================
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INSTRUCTIONS: &str =
        "Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.";

    #[test]
    fn test_samples1() {
        let instructions = TEST_INSTRUCTIONS.lines().collect::<Vec<_>>();
        let step_graph = create_step_graph(instructions);
        let (_, instruction_order) =
            find_instruction_order(step_graph, 1);

        assert_eq!(instruction_order, "CABDFE");
    }

    #[test]
    fn test_samples2() {
        let instructions = TEST_INSTRUCTIONS.lines().collect::<Vec<_>>();
        let step_graph = create_step_graph(instructions);
        let (instruction_time, instruction_order2) =
            find_instruction_order(step_graph, 2);

        assert_eq!(instruction_order2, "CABFDE");
        assert_eq!(instruction_time, 375);
    }
}

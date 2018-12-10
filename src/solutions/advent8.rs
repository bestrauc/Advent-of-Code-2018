use solutions::utils;

static INPUT: &str = "data/input8";

// Problem 1
// ==================================================

/// The return type is(subtree length, subtree metadata sum).
/// The subtree length is used to pass recursion information upwards.
fn sum_metadata(tree_spec: &[usize]) -> (usize, usize) {
    let children = tree_spec[0];
    let metadata = tree_spec[1];

    let mut child_offset = 2;
    let mut metadata_sum = 0;

    // get metadata count and length of child subtree.
    // the recursion will terminate eventually because
    // eventually we find a leaf without children, which
    // will end the recursion. (this recursion could only
    // go on indefinitely if the tree input was wrong)
    for _ in 0..children {
        let (child_len, child_sum) = sum_metadata(&tree_spec[child_offset..]);
        child_offset += child_len;
        metadata_sum += child_sum;
    }

    let own_len = child_offset + metadata;

    // after the children, sum own metadata
    // (this will start at 2 if no children)
    metadata_sum += tree_spec[child_offset..(child_offset + metadata)].iter().sum::<usize>();

    (own_len, metadata_sum)
}

// Problem 2
// ==================================================

fn indexed_sum_metadata(tree_spec: &[usize]) -> (usize, usize) {
    let children = tree_spec[0];
    let metadata = tree_spec[1];

    let mut child_offset = 2;
    let mut child_metadata_sums = Vec::new();

    // get metadata count and length of child subtree.
    // the recursion will terminate eventually because
    // eventually we find a leaf without children, which
    // will end the recursion. (this recursion could only
    // go on indefinitely if the tree input was wrong)
    for _ in 0..children {
        let (child_len, child_sum) = indexed_sum_metadata(&tree_spec[child_offset..]);
        child_offset += child_len;
        child_metadata_sums.push(child_sum);
    }

    let own_len = child_offset + metadata;

    let mut metadata_sum = 0;
    // if we have no children, just sum metadata normally
    if child_metadata_sums.is_empty() {
        metadata_sum += tree_spec[child_offset..(child_offset + metadata)].iter().sum::<usize>();
    } else {
        // given a child metadata sum vector, we now
        // use our own metadata vector to index it
        for i in child_offset..(child_offset + metadata) {
            let metadata_index = tree_spec[i];
            if 0 < metadata_index && metadata_index <= child_metadata_sums.len() {
                metadata_sum += child_metadata_sums[metadata_index - 1];
            }
        }
    }

    (own_len, metadata_sum)
}

// Interface
// ==================================================

pub fn solution1() -> () {
    let tree_spec = utils::file_to_string(INPUT);
    let tree_spec = tree_spec
        .trim_right()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (tree_len, tree_metadata_sum) = sum_metadata(&tree_spec[..]);
    println!("Checksum of tree (Len {}) is {}", tree_len, tree_metadata_sum);
}


pub fn solution2() -> () {
    let tree_spec = utils::file_to_string(INPUT);
    let tree_spec = tree_spec
        .trim_right()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (_, tree_metadata_sum) = indexed_sum_metadata(&tree_spec[..]);
    println!("Indexed checksum of the tree is {}", tree_metadata_sum);
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

    static TREE_SPEC: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_samples1() {
        let tree_spec = TREE_SPEC
            .trim_right()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (tree_len, tree_metadata_sum) = sum_metadata(&tree_spec[..]);
        println!("{} - {}", tree_len, tree_metadata_sum);
        assert_eq!(tree_len, tree_spec.len());
        assert_eq!(tree_metadata_sum, 138);
    }

    #[test]
    fn test_samples2() {
        let tree_spec = TREE_SPEC
            .trim_right()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (tree_len, tree_metadata_sum) = indexed_sum_metadata(&tree_spec[..]);
        println!("{} - {}", tree_len, tree_metadata_sum);
        assert_eq!(tree_len, tree_spec.len());
        assert_eq!(tree_metadata_sum, 66);
    }
}

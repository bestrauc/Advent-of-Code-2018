use std::collections::HashMap;
use std::collections::HashSet;

use utils;

static INPUT: &str = "data/input2";

// Problem 1
// ==================================================

// we operate over chars, not unicode code points, but
// it doesn't matter since we mostly have ascii here
fn count_frequencies(id: &str) -> HashMap<char, u16> {
    let mut char_count_map = HashMap::new();
    for c in id.chars() {
        *char_count_map.entry(c).or_insert(0) += 1;
    }

    char_count_map
}

fn add_counts(id: &str, twos: &mut u16, threes: &mut u16) {
    let char_counts = count_frequencies(id);
    let char_counts: HashSet<&u16> = char_counts.values().collect();

    if char_counts.contains(&2) {
        *twos +=1;
    }

    if char_counts.contains(&3) {
        *threes +=1;
    }
}

fn compute_checksum(ids: Vec<&str>) -> u16 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        add_counts(id, &mut twos, &mut threes);
    };

    twos*threes
}


// Problem 2
// ==================================================

fn match_common_string(id1: &str, id2: &str) -> Option<String> {
    let mut common_string = String::new();

    let mut mismatch_count = 0;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 == c2 {
            common_string.push(c1);
        }
        else {
            mismatch_count += 1;
            if mismatch_count > 1 {
                return None;
            }
        }
    }

    Some(common_string)
}

fn get_common_string(ids: Vec<&str>) -> String {
    for i in 0..ids.len() {
        for j in (i+1)..ids.len() {
            if let Some(common) = match_common_string(ids[i], ids[j]) {
                return common
            }
        }
    }

    String::from("Impossible!")
}

// Interface
// ==================================================

pub fn solution1() -> u16 {
    let ids = utils::file_to_string(INPUT);
    let ids: Vec<&str> = ids.lines().collect();

    compute_checksum(ids)
}


pub fn solution2() -> String {
    let ids = utils::file_to_string(INPUT);
    let ids: Vec<&str> = ids.lines().collect();

    get_common_string(ids)
}


// Test the sample puzzle inputs
// ================================================== 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_samples1() {
        let test_ids = vec!["abcdef", "bababc", "abbcde", "abcccd", 
                            "aabcdd", "abcdee", "ababab"];

        let checksum = compute_checksum(test_ids);
        assert_eq!(checksum, 12);
    }

    #[test]
    fn test_samples2() {
        let test_ids = vec!["abcde", "fghij", "klmno", "pqrst", 
                            "fguij", "axcye", "wvxyz"];

        let common_string = get_common_string(test_ids);
        assert_eq!(common_string, "fgij");
    }

}

use std::collections::HashSet;

use solutions::utils;

static INPUT: &str = "data/input5";

// Problem 1
// ==================================================

fn reduce_polymer(polymer: &str) -> String {
    let mut reduced = Vec::new();

    for c in polymer.chars() {
        match reduced.last().cloned() {
            Some(ref last_c) => {
                if (c.is_ascii_uppercase() && (c.to_ascii_lowercase() == *last_c)) ||
                    (c.is_ascii_lowercase() && (c.to_ascii_uppercase() == *last_c)) {
                    reduced.pop().unwrap();
                } else {
                    reduced.push(c);
                }
            }
            None => {
                reduced.push(c)
            }
        };
    }

    reduced.into_iter().collect()
}

// Problem 2
// ==================================================

fn filtered_polymers(polymer: String) -> Vec<(char, String)> {
    let character_set = polymer
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<HashSet<char>>();

    let filtered_polys = character_set.iter().map(|ref_c| {
        let filtered_polymer = polymer
            .chars()
            .filter(|c| c.to_ascii_uppercase() != *ref_c)
            .collect::<String>();

            (*ref_c, filtered_polymer)
    }).collect::<Vec<_>>();

    filtered_polys
}

// Interface
// ==================================================

pub fn solution1() -> () {
    let base_polymer = utils::file_to_string(INPUT);
    let base_polymer = base_polymer.trim_right();
    let reduced_polymer = reduce_polymer(base_polymer);
    println!("Reduced polymer has length {}:\n{}", reduced_polymer.len(), reduced_polymer);
}


pub fn solution2() -> () {
    let base_polymer = utils::file_to_string(INPUT);
    let base_polymer = base_polymer.trim_right().to_owned();
    let filtered_polys = filtered_polymers(base_polymer);
    let char_filter_lens = filtered_polys
        .iter()
        .map(|(c, filtered_poly)| {
            let reduced_filtered_poly = reduce_polymer(&filtered_poly);
            (c, reduced_filtered_poly.len())
        })
        .collect::<Vec<_>>();

    let (filter_char, filter_len) = char_filter_lens
        .iter()
        .min_by_key(|(_, filter_len)| filter_len).unwrap();

    println!("Smallest length of {} was after filtering '{}/{}'!",
             filter_len, filter_char, filter_char.to_ascii_lowercase());
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

    #[test]
    fn test_samples1() {
        let sample1 = "abBA";
        assert_eq!(reduce_polymer(sample1), "");

        let sample2 = "abAB";
        assert_eq!(reduce_polymer(sample2), "abAB");

        let sample3 = "aabAAB";
        assert_eq!(reduce_polymer(sample3), "aabAAB");

        let sample4 = "dabAcCaCBAcCcaDA";
        assert_eq!(reduce_polymer(sample4), "dabCBAcaDA");

        let sample5 = "lsCTtcCctTuthHhHzEVLlmMtmTJjFfFfJmMtTdDOfkawnNIilLYyXxHHhfpPYyZmMDVvtTcCdR";
        let sample5_result = "lsutzEVtmTJOfkawHfZR";
        assert_eq!(reduce_polymer(sample5), sample5_result);
    }

    #[test]
    fn test_samples2() {}
}

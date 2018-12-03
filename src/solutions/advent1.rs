use std::collections::HashSet;

use utils;

static INPUT: &str = "data/input1";

// Problem 1
// ==================================================

fn get_freq_vector() -> Vec<i32> {
    let file_contents = utils::file_to_string(INPUT);

    let freq_vector: Vec<_> = file_contents.lines().map(|line| {
        line.parse::<i32>().unwrap()
    }).collect();

    freq_vector
}


fn sum_frequencies(freqs: &Vec<i32>) -> i32 {
    freqs.iter().sum()
}


// Problem 2
// ==================================================

// Implement an infinite iterator adapter
struct Looper<T> 
where
    T: Iterator
{
    active_iter: T,
    iter_base: T,
}


impl<T> Iterator for Looper<T>
where
    T: Iterator + Clone
{
     type Item = <T as Iterator>::Item;

     fn next(&mut self) -> Option<Self::Item> {
         // return the next value or restart iterator
         match self.active_iter.next() {
             Some(item) => Some(item),
             None => {
                 // reset iterator and restart
                 self.active_iter = self.iter_base.clone();
                 self.next()
             }
         }
     }
}


fn looper<T: Iterator + Clone>(iter: T) -> Looper<T> {
    Looper {
        active_iter: iter.clone(),
        iter_base: iter,
    }
}


fn get_repeated_freq(freqs: &Vec<i32>) -> i32 {
    let mut seen_freqs = HashSet::new();
    let mut freq_state = 0;
    seen_freqs.insert(freq_state);

    // iterate until solution found
    for value in looper(freqs.iter()) {
        freq_state += value;
        
        if seen_freqs.contains(&freq_state) {
            return freq_state;
        }

        seen_freqs.insert(freq_state);
    }
    
    panic!("Why are we here?");
}

// Interface
// ==================================================

pub fn solution1() -> i32 {
    let freq_vector = get_freq_vector();
    let result = sum_frequencies(&freq_vector);
    result
}


pub fn solution2() -> i32 {
    let freq_vector = get_freq_vector();
    let result = get_repeated_freq(&freq_vector);
    result
}


// Test the sample puzzle inputs
// ================================================== 
#[cfg(test)]
mod test {
    #[test]
    fn test_samples1() {

    }

    #[test]
    fn test_samples2() {

    }
}

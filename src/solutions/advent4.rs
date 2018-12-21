use std::collections::HashMap;

use chrono::{NaiveDateTime, Timelike};

use solutions::utils;

static INPUT: &str = "data/input4";

// Problem 1
// =================================================

#[derive(Debug)]
enum LineAction {
    GuardStarts(String),
    Sleep,
    Wake,
}

fn split_components(line: &str) -> (NaiveDateTime, LineAction) {
    // split something like "[1518-05-12 00:46] wakes up"
    // first part will be "[1518-05-12 00:46", so cut off start
    let mut linesplit = line.split(']');
    let timestamp_string = linesplit.next().unwrap();
    let timestamp_string: &str = &timestamp_string[1..];
    let timestamp = NaiveDateTime::parse_from_str(timestamp_string,
                                                  "%Y-%m-%d %H:%M").unwrap();

    let guard_action: &str = linesplit.next().unwrap();
    let line_action =
        if guard_action.contains("asleep") {
            LineAction::Sleep
        } else if guard_action.contains("wakes") {
            LineAction::Wake
        } else {
            let guard_id: Vec<_> = guard_action.split_whitespace().collect();
            let guard_id = guard_id[1];
            LineAction::GuardStarts(String::from(guard_id))
        };

    (timestamp, line_action)
}

fn split_and_sort(lines: Vec<&str>) -> Vec<(NaiveDateTime, LineAction)> {
    let mut line_actions: Vec<_> = lines.iter().map(|line| split_components(line)).collect();
    line_actions.sort_by_key(|&(timestamp, _)| timestamp);

    line_actions
}


fn count_sleeps(line_actions: &Vec<(NaiveDateTime, LineAction)>) -> HashMap<String, Vec<u32>> {
    let mut sleep_counter: HashMap<String, Vec<u32>> = HashMap::new();

    let mut active_guard: Option<&str> = None;
    let mut asleep_time: Option<&NaiveDateTime> = None;
    for (time, action) in line_actions {
        match action {
            LineAction::GuardStarts(guard_id) => {
                active_guard = Some(guard_id);
            }
            LineAction::Wake => {
                let sleep_start = asleep_time.unwrap().minute();
                let sleep_end = time.minute() + 1;
                let guard_counter = sleep_counter.entry(active_guard.unwrap().to_string())
                    .or_insert(vec![0; 60]);

                for minute in sleep_start..sleep_end {
                    guard_counter[minute as usize] += 1;
                }
            }
            LineAction::Sleep => {
                asleep_time = Some(time);
            }
        };
    }

    sleep_counter
}

// Problem 2
// ==================================================

// Interface
// ==================================================

pub fn solution1() -> () {
    let input = utils::file_to_string(INPUT);
    let sorted_actions = split_and_sort(input.lines().collect());
    let sleep_counter = count_sleeps(&sorted_actions);

    let max_total_sleep_counter: (&String, &Vec<u32>) = sleep_counter
        .iter()
        .max_by_key(|(_, v)| -> u32 {
            v.iter().sum()
        }).unwrap();

    let max_sleep = max_total_sleep_counter.1.iter().max().unwrap();
    let max_sleep_index = max_total_sleep_counter.1.iter()
        .position(|m| m == max_sleep).unwrap();
    let sleep_sum = max_total_sleep_counter.1.iter().sum::<u32>();

    println!("Guard with maximum sleep time:\n{} - Sleep minutes: {} - Sleepiest minute: {}",
             max_total_sleep_counter.0, sleep_sum, max_sleep_index);
}


pub fn solution2() -> () {
    let input = utils::file_to_string(INPUT);
    let sorted_actions = split_and_sort(input.lines().collect());
    let sleep_counter = count_sleeps(&sorted_actions);

    let max_sleep_minute_counter = sleep_counter
        .iter()
        .max_by_key(|(_, v)| {
            v.iter().max().unwrap()
    }).unwrap();

    let max_sleep = max_sleep_minute_counter.1.iter().max().unwrap();
    let max_sleep_index = max_sleep_minute_counter.1.iter()
        .position(|m| m == max_sleep).unwrap();

    println!("Guard with sleepiest minute:\n{} - Minute: {} - Days asleep: {}",
             max_sleep_minute_counter.0, max_sleep_index, max_sleep);

}

/// Problem 1 and problem 2 are pretty much identical on this day,
/// only that problem 2 uses a sleepy minute counting for maximum
/// columns (sleep minutes) instead of maximum rows (minutes asleep).
pub fn solve_day() {
    solution1();
    solution2();
}


// Test the sample puzzle inputs
// ================================================== 
#[cfg(test)]
mod test {
    #[test]
    fn test_samples1() {}

    #[test]
    fn test_samples2() {}
}

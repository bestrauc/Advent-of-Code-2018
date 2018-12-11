use std::collections::VecDeque;

use linked_list::{Cursor, LinkedList};

use solutions::utils;


// Problem 1
// ==================================================

fn print_game_state(active_marble: usize, circle: &VecDeque<usize>) {
    for marble in circle {
        if *marble == active_marble {
            print!("({})", marble);
        } else {
            print!(" {} ", marble);
        }
    }

    println!();
}

fn print_game_state2(active_marble: &Cursor<usize>, circle: &mut LinkedList<usize>) {
    let mut circle_iter = circle.cursor();
    while let Some(marble) = circle_iter.next() {
        print!(" {} ", marble);
    }

    println!();
}

fn play_marble_game_old(players: usize, last_marble: usize) -> usize {
    let mut player_scores = vec![0; players];
    let mut circle = VecDeque::new();
    let mut current_marble = 0;
    circle.push_back(0usize);

    for (marble, player) in (1..last_marble+1).zip((0..players).cycle()) {
//        print_game_state(circle[current_marble], &circle);
        if marble % 23 == 0 {
            // add marble to player score
            player_scores[player] += marble;

            // add the removed marble (7 marbles counterclockwise) to the score
            let remove_pos = (current_marble + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(remove_pos).unwrap();
            player_scores[player] += removed_marble;

            // the vector shrunk and now the marble previously clockwise is at remove_pos
            current_marble = remove_pos;
        } else {
            let insert_pos = (current_marble + 2) % circle.len();
            circle.insert(insert_pos, marble);
            current_marble = insert_pos;
        }
    }

    *player_scores.iter().max().unwrap()
}

fn print_circle(test: LinkedList<usize>) -> LinkedList<usize> {
    println!("{:?}", test);
    test
}

fn play_marble_game(players: usize, last_marble: usize) -> usize {
    let mut player_scores = vec![0; players];
    let mut circle = LinkedList::new();
    let mut current_marble = 0;
    circle.push_back(0usize);

    for (marble, player) in (1..last_marble + 1).zip((0..players).cycle()) {
//        print_game_state(circle[current_marble], &circle);
        if marble % 23 == 0 {
            // add marble to player score
            player_scores[player] += marble;

            // add the removed marble (7 marbles counterclockwise) to the score
            let remove_pos = (current_marble + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(remove_pos).unwrap();
            player_scores[player] += removed_marble;

            // the vector shrunk and now the marble previously clockwise is at remove_pos
            current_marble = remove_pos;
        } else {
            let insert_pos = (current_marble + 2) % circle.len();
            circle.insert(insert_pos, marble);
            current_marble = insert_pos;
        }
    }

    *player_scores.iter().max().unwrap()
}

// Problem 2
// ==================================================

// Interface
// ==================================================

pub fn solution1() -> () {
    // don't need to read here since input is just two numbers
    let players = 452;
    let last_marble = 71250;
    let score = play_marble_game(players, last_marble);
    println!("Score for game with {} players and {} marbles is {}!", players, last_marble, score);
}


pub fn solution2() -> () {
    let players = 452;
    let last_marble = 71250*100;
    let score = play_marble_game(players, last_marble);
    println!("Score for game with {} players and {} marbles is {}!", players, last_marble, score);
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
//        let samples = [(10, 1618), (13, 7999), (17, 1104), (21, 6111), (30, 5807)];
//        let results = [8317, 146373, 2764, 54718, 37305];

        let samples = [(9, 25)];
        let results = [32];

        for ((players, marbles), score) in samples.iter().zip(results.iter()) {
            let game_score = play_marble_game(*players, *marbles);
            assert_eq!(game_score, *score);
        }
    }

    #[test]
    fn test_samples2() {

    }
}

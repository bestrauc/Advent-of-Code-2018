use std::collections::VecDeque;

// Problem 1 and 2
// ==================================================

fn play_marble_game(players: usize, last_marble: usize) -> usize {
    let mut player_scores = vec![0; players];
    let mut circle = VecDeque::with_capacity(last_marble);
    circle.push_back(0usize);

    // we maintain the invariant that the active marble is always 
    // the second-to-last element in the deque (so inserting the new
    // marble is only a push_back)

    for (marble, player) in (1..last_marble+1).zip((0..players).cycle()) {
        if marble % 23 == 0 {
            // add marble to player score
            player_scores[player] += marble;

            // rotate the circle so that the to-be-removed marble is third-to-last
            // so after we remove it the second-to-last marble is the current marble
            for _ in 0..6 {
                let last_marble = circle.pop_back().unwrap();
                circle.push_front(last_marble);
            }

            let n = circle.len();
            // removing the marble causes 2 elements to shift, but that should be ok
            let removed_marble = circle.remove(n-3).unwrap();
            player_scores[player] += removed_marble;
        } else {
            // append the new marble
            circle.push_back(marble);
            // ensure that the new marble (which is the new current marble)
            // is second-to-last by rotating the first marble to last place
            let first_marble = circle.pop_front().unwrap();
            circle.push_back(first_marble);
        }
    }

    *player_scores.iter().max().unwrap()
}

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
        let samples = [(10, 1618), (13, 7999), (17, 1104), (21, 6111), (30, 5807)];
        let results = [8317, 146373, 2764, 54718, 37305];

        for ((players, marbles), score) in samples.iter().zip(results.iter()) {
            let game_score = play_marble_game(*players, *marbles);
            assert_eq!(game_score, *score);
        }
    }
}

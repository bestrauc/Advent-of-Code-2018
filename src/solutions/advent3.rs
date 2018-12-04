use utils;

static INPUT: &str = "data/input3";

// Problem 1
// ==================================================

struct Square {
    id: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Square {
    /// Constructs a new square from a claim string
    /// as given by the AOC problem: "#ID @ x,y: wxh"
    pub fn from_claim_str(claim: &str) -> Square {
        let claim_fields: Vec<&str> = claim.split_whitespace().collect();
        let id = String::from(claim_fields[0]);
        let pos = claim_fields[2];
        let pos = &pos[..(pos.len() - 1)];
        let dim = claim_fields[3];

        let tmp: Vec<u32> = pos.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
        let (x, y) = (tmp[0], tmp[1]);
        let tmp: Vec<u32> = dim.split('x').map(|s| s.parse::<u32>().unwrap()).collect();
        let (w, h) = (tmp[0], tmp[1]);

        Square {id, x, y, w, h }
    }
}

fn compute_square_cover(squares: &Vec<Square>) -> Vec<Vec<u32>> {
    let mut count_matrix: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for square in squares {
        for x in (square.x)..(square.x + square.w) {
            for y in (square.y)..(square.y + square.h) {
                count_matrix[x as usize][y as usize] += 1;
            }
        }
    }

    count_matrix
}

fn count_duplicates_squares(squares: Vec<Square>) -> u32 {
    let count_matrix = compute_square_cover(&squares);
    let val_count = count_matrix.iter()
        .flatten()
        .map(|&count| (count > 1) as u32)
        .sum();

    val_count
}

// Problem 2
// ==================================================

fn get_intact_square(squares: Vec<Square>) -> String {
    let mut count_matrix = compute_square_cover(&squares);

    let mut found_square = None;
    'start: for square in squares {
        for x in (square.x)..(square.x + square.w) {
            for y in (square.y)..(square.y + square.h) {
                // advance to the next square if this claim overlaps already
                if count_matrix[x as usize][y as usize] > 1 {
                    continue 'start;
                }
            }
        }

        found_square = Some(square);
        break;
    }

    // Unwrap should succeed because the input has one right answer
    found_square.unwrap().id
}

// Interface
// ==================================================

pub fn solution1() -> u32 {
    let file_input = utils::file_to_string(INPUT);
    let squares: Vec<_> = file_input.lines()
        .map(|line| Square::from_claim_str(line))
        .collect();

    count_duplicates_squares(squares)
}


pub fn solution2() -> String {
    let file_input = utils::file_to_string(INPUT);
    let squares: Vec<_> = file_input.lines()
        .map(|line| Square::from_claim_str(line))
        .collect();

    get_intact_square(squares)
}


// Test the sample puzzle inputs
// ================================================== 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_samples1() {}

    #[test]
    fn test_samples2() {}

    #[test]
    fn test_square_from_claim() {
        let test_str = "#123 @ 3,2: 5x4";
        let square = Square::from_claim_str(test_str);

        assert_eq!(square.x, 3);
        assert_eq!(square.y, 2);
        assert_eq!(square.w, 5);
        assert_eq!(square.h, 4);
    }
}

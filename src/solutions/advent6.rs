use std::collections::HashMap;

use solutions::utils;

static INPUT: &str = "data/input6";

#[derive(Debug, Copy, Clone)]
struct Pos {
    id: usize,
    x: i32,
    y: i32,
}

impl Pos {
    fn from_str(id: usize, coord_str: &str) -> Pos {
        let coords: Vec<_> = coord_str.split(',').map(|field| field.trim()).collect();
        Pos {
            id,
            x: coords[0].parse::<i32>().unwrap(),
            y: coords[1].parse::<i32>().unwrap(),
        }
    }
}


fn abs_diff(a: i32, b: i32) -> u32 {
    if a < b {
        return (b-a) as u32;
    } else {
        return (a-b) as u32;
    }
}


/// Manhattan distance
fn dist(pos1: &Pos, pos2: &Pos) -> u32 {
    abs_diff(pos1.x,pos2.x) + abs_diff(pos1.y, pos2.y)
}


/// Return distances to from Pos to all Location coordinates, as a vector like
/// [(locationPos1, dist(pos, locationPos1), (locationPos2, dist(pos, locationPos2), ..]
fn get_coord_distance_map(pos: &Pos, coords: &Vec<Pos>) -> Vec<(Pos, u32)> {
    let dists_to_coords = coords.iter()
        .map(|ipos| (ipos.clone(), dist(ipos, pos)))
        .collect::<Vec<_>>();

    dists_to_coords
}

// Problem 1
// ==================================================

fn get_closest_pos(pos: &Pos, coords: &Vec<Pos>) -> Option<usize> {
    let dists_to_coords = get_coord_distance_map(pos, coords);

    let closest_pos = dists_to_coords
        .iter()
        .min_by_key(|(_, pos_dist)| pos_dist)
        .unwrap();

    let closest_id = closest_pos.0.id;
    let min_dist = closest_pos.1;

    let min_distance_count = dists_to_coords
        .iter()
        .filter(|(_, pos_dist)| *pos_dist == min_dist)
        .count();

    // don't count ares that are equidistant to multiple coordinates
    if min_distance_count > 1 {
        None
    } else {
        Some(closest_id)
    }
}

/// We find the largest area like this: for each coordinate, we find the
/// closest location (or None if multiple exist) and give this location
/// a +1 count. If the location borders on the edge of the region we check
/// it extends to infinity and gets negative counts (hack to ignore this area).
fn get_largest_area(coords: &Vec<Pos>) -> i32 {
    let max_x = coords.iter().map(|pos| pos.x).max().unwrap()+1;
    let max_y = coords.iter().map(|pos| pos.y).max().unwrap()+1;

    println!("Checking areas of size {} x {}", max_x, max_y);
    let mut area_sizes = HashMap::new();

    for x in 0..max_x {
        for y in 0..max_y {
            let current_pos = Pos{id: 0, x, y};
            if let Some(closest_pos_id) =  get_closest_pos(&current_pos, coords) {
                // areas that have closest points that are on the border of the field
                // we are looking at get a penalty, because they stretch into infinity
                // and should be ignored. A hacky solution, but it does the job here.
                if x==0 || y==0 || x == (max_x-1) || y == (max_y-1) {
                    *area_sizes.entry(closest_pos_id).or_insert(0i32) -= 100;
                }

                *area_sizes.entry(closest_pos_id).or_insert(0i32) += 1;
            }
        }
    }

    println!("{:?}", area_sizes);
    *area_sizes.values().max().unwrap()
}

// Problem 2
// ==================================================

/// The safe area is supposed to consist only of those coordinates
/// which have a sum of manhattan distances to all locations which
/// is smaller than 10000. We simply count the occurrences of
/// coordinates which satisfy that, which is enought to find the safe area.
fn get_safe_area(coords: &Vec<Pos>) -> u32 {
    let max_x = coords.iter().map(|pos| pos.x).max().unwrap()+1;
    let max_y = coords.iter().map(|pos| pos.y).max().unwrap()+1;

    println!("Checking areas of size {} x {}", max_x, max_y);

    let mut compact_area_size = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            let current_pos = Pos{id: 0, x, y};
            let coord_dists = get_coord_distance_map(&current_pos, coords);
            let current_pos_dist_sum = coord_dists
                .iter()
                .map(|(_, d)| d)
                .sum::<u32>();

            if current_pos_dist_sum < 10000 {
                compact_area_size += 1;
            }

        }
    }

    compact_area_size
}


// Interface
// ==================================================

pub fn solution1() -> () {
    let coordinates = utils::file_to_string(INPUT);
    let coordinates: Vec<_> = coordinates.lines().enumerate()
        .map(|(num, line)| Pos::from_str(num, line)).collect();

    let max_area = get_largest_area(&coordinates);
    println!("Largest area: {}", max_area);

}


pub fn solution2() -> () {
    let coordinates = utils::file_to_string(INPUT);
    let coordinates: Vec<_> = coordinates.lines().enumerate()
        .map(|(num, line)| Pos::from_str(num, line)).collect();

    let min_all_dists = get_safe_area(&coordinates);
    println!("The safe area has size {}", min_all_dists);
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
        let coordinates = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        let coordinates: Vec<_> = coordinates.iter().enumerate()
            .map(|(num, line)| Pos::from_str(num, line)).collect();
        assert_eq!(get_largest_area(&coordinates), 17);
    }

    #[test]
    fn test_samples2() {

    }
}

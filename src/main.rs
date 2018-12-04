mod utils;
mod solutions;

use solutions::*;

fn solve_advent1() {
    let solution1 = advent1::solution1();
    println!("End frequency: {}", solution1);

    let solution2 = advent1::solution2();
    println!("First repeated frequency: {}", solution2);
}


fn solve_advent2() {
    let solution1 = advent2::solution1();
    println!("Checksum: {}", solution1);

    let solution2 = advent2::solution2();
    println!("Common string: {}", solution2);
}


fn solve_advent3() {
    let solution1 = advent3::solution1();
    println!("Covered squares: {}", solution1);

    let solution2 = advent3::solution2();
    println!("Intact claim: {}", solution2);
}


fn main() {
    solve_advent3();
}

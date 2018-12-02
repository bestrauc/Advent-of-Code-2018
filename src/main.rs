mod utils;
mod advent1;

fn main() {
    let solution1 = advent1::solution1();
    println!("End frequency: {}", solution1);

    let solution2 = advent1::solution2();
    println!("First repeated frequency: {}", solution2);
}

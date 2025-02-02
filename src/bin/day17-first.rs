use std::fs;

use aoc_2024::day17;

fn main() {
    let input = fs::read_to_string("data/day17/input").unwrap();
    let mut computer: day17::Computer = input.parse().unwrap();
    let stdout = computer.run();
    println!("{}", stdout);
}

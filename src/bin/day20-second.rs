use std::fs;

use aoc_2024::day20;

fn main() {
    let input = fs::read_to_string("data/day20/input").unwrap();
    let racetrack: day20::Racetrack = input.parse().unwrap();
    println!("{}", racetrack.count_top_improvements(20, 100));
}

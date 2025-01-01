use aoc_2024::day10;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day10/input").unwrap();
    let map: day10::Map = input.parse().unwrap();
    let total_score = map.total_rating();
    println!("{}", total_score);
}

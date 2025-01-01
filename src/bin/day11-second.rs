use aoc_2024::day11;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day11/input").unwrap();
    let stones = day11::parse_input(&input);
    let final_stones = day11::simulate_stones(&stones, 75);
    println!("{}", final_stones);
}

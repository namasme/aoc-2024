use aoc_2024::day4;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day4/input").unwrap();
    let puzzle: day4::Puzzle = input.parse().unwrap();
    let occurrences = puzzle.find_pattern_occurrences(day4::WordPattern::new("XMAS".to_string()));
    println!("{}", occurrences.len());
}

use aoc_2024::day8;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day8/input").unwrap();
    let grid: day8::Grid = input.parse().unwrap();
    let unique_antinodes = grid.unique_antinodes();
    println!("{}", unique_antinodes.len());
}

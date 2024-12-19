use aoc_2024::day3;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day3/input").unwrap();
    let corrupted_program = day3::CorruptedProgram::parse(&input);
    let result = corrupted_program.result();
    println!("{}", result);
}

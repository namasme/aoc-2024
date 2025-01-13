use std::fs;

use aoc_2024::day13;

fn main() {
    let input = fs::read_to_string("data/day13/input").unwrap();
    let machines = day13::parse_input(&input);
    let total_tokens: usize = machines
        .iter()
        .filter_map(|machine| machine.required_tokens_adjusted())
        .sum();
    println!("{}", total_tokens);
}

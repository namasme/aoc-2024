use aoc_2024::day5;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day5/input").unwrap();
    let (rules, updates) = day5::parse_input(&input);
    let checksum: u64 = updates
        .iter()
        .filter(|update| !update.is_valid(&rules))
        .map(|update| update.sorted(&rules).middle_page())
        .sum();

    println!("{}", checksum);
}

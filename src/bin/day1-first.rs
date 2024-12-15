use aoc_2024::day1;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day1/input").unwrap();
    let (mut left, mut right) = day1::parse_input(&input);
    let total_distance = total_distance(&mut left, &mut right);
    println!("{}", total_distance);
}

fn total_distance(
    left: &mut Vec<day1::LocationID>,
    right: &mut Vec<day1::LocationID>,
) -> day1::LocationID {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

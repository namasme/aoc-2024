use std::fs;

use aoc_2024::day15;

fn main() {
    let input = fs::read_to_string("data/day15/input").unwrap();
    let (mut warehouse, moves) = day15::parse_input(&input);
    warehouse.apply(&moves);
    let gps_sum = warehouse.gps_sum();
    println!("{}", gps_sum);
}

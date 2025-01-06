use std::fs;

use aoc_2024::day12;

fn main() {
    let input = fs::read_to_string("data/day12/input").unwrap();
    let farm: day12::Farm = input.parse().unwrap();
    println!("{}", farm.total_bulk_discount_price());
}

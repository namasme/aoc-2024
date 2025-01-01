use aoc_2024::day9;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day9/input").unwrap();
    let disk: day9::Disk = input.parse().unwrap();
    let checksum = disk.compute_whole_checksum();
    println!("{}", checksum);
}

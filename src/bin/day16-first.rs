use std::fs;

use aoc_2024::day16;

fn main() {
    let input = fs::read_to_string("data/day16/input").unwrap();
    let maze: day16::Maze = input.parse().unwrap();
    let lowest_score = maze.lowest_score();
    println!("{}", lowest_score);
}

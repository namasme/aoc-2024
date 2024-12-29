use aoc_2024::day6;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day6/input").unwrap();
    let (grid, guard) = day6::parse_input(&input);
    let looping_obstacles = grid.count_loops(guard);
    println!("{:?}", looping_obstacles);
}

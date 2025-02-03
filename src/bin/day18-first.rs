use std::fs;

use aoc_2024::day18;

const GRID_SIZE: usize = 71;
const BYTES_READ: usize = 1024;

fn main() {
    let input = fs::read_to_string("data/day18/input").unwrap();
    let bytes = day18::parse_input(&input);
    let grid = day18::Grid::new(GRID_SIZE, GRID_SIZE, &bytes[..BYTES_READ]);
    let shortest_distance = grid.shortest_distance();
    println!("{}", shortest_distance);
}

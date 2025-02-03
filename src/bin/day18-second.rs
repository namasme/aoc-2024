use std::fs;

use aoc_2024::day18;

const GRID_SIZE: usize = 71;

fn main() {
    let input = fs::read_to_string("data/day18/input").unwrap();
    let bytes = day18::parse_input(&input);
    let grid = day18::Grid::new(GRID_SIZE, GRID_SIZE, &[]);
    let first_blocking_byte = grid.first_blocking_byte(&bytes);
    println!("{},{}", first_blocking_byte.x, first_blocking_byte.y);
}

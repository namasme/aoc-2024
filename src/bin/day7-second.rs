use aoc_2024::day7;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day7/input").unwrap();
    let equations = day7::parse_input(&input);
    let allowed_operations = HashSet::from([
        day7::Operation::Add,
        day7::Operation::Mul,
        day7::Operation::Concat,
    ]);
    let calibration_result = day7::compute_calibration_result(&equations, &allowed_operations);
    println!("{}", calibration_result);
}

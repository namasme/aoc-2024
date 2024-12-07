use std::fs;

fn main() {
    let input = fs::read_to_string("data/day1/input").unwrap();
    let (mut left, mut right) = parse_input(&input);
    let total_distance = total_distance(&mut left, &mut right);
    println!("{}", total_distance);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once("   ").unwrap();
            (
                parts.0.parse::<i32>().unwrap(),
                parts.1.parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

fn total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

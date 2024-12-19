use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day3/input").unwrap();
    let muls = parse_input(&input);
    let result = compute_result(&muls);
    println!("{}", result);
}

fn compute_result(muls: &[(Value, Value)]) -> Value {
    muls.iter().map(|(x, y)| x * y).sum()
}

type Value = u32;

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let (_, [x, y]) = cap.extract();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

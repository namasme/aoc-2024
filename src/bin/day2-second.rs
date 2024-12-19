use aoc_2024::day2;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day2/input").unwrap();
    let reports = day2::parse_input(&input);
    let safe_reports_count = reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count();
    println!("{}", safe_reports_count);
}

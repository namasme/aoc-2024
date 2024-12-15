use aoc_2024::day1;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day1/input").unwrap();
    let (left, right) = day1::parse_input(&input);
    let similarity_score = compute_similarity_score(&left, &right);
    println!("{}", similarity_score);
}

fn compute_similarity_score(left: &[day1::LocationID], right: &[day1::LocationID]) -> usize {
    let left_counter = frequencies(left);
    let right_counter = frequencies(right);

    left_counter
        .iter()
        .map(|(&location, &count)| {
            let &right_count = right_counter.get(&location).unwrap_or(&0);

            (location as usize) * count * right_count
        })
        .sum()
}

fn frequencies(list: &[day1::LocationID]) -> HashMap<day1::LocationID, usize> {
    let mut counter = HashMap::new();

    for location in list {
        counter
            .entry(*location)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counter
}

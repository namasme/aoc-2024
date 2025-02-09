use std::collections::HashSet;
use std::fs;

use aoc_2024::day19::{self, Design, Pattern};

fn main() {
    let input = fs::read_to_string("data/day19/input").unwrap();
    let (patterns, designs) = day19::parse_input(&input);
    let total_possible_designs = count_possible_designs(
        &designs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );
    println!("{}", total_possible_designs);
}

fn count_possible_designs(designs: &[Design], patterns: &[Pattern]) -> usize {
    designs
        .iter()
        .filter(|d| is_satisfiable(d, patterns))
        .count()
}

fn is_satisfiable(design: Design, patterns: &[Pattern]) -> bool {
    let mut partial_solutions = vec![(design, 0)];
    let mut known_suffixes: HashSet<Design> = HashSet::new();

    while let Some((current, pattern_idx_min)) = partial_solutions.pop() {
        if current.is_empty() {
            return true;
        } else if known_suffixes.contains(current) {
            continue;
        }

        if let Some(idx) = day19::find_first_prefix(current, &patterns[pattern_idx_min..]) {
            // idx is obtained from the tail of patterns after pattern_idx_min
            let global_idx = pattern_idx_min + idx;
            let pattern = &patterns[global_idx];
            let suffix = &current[pattern.len()..];
            // we can always try again with the next one it that doesn't work
            if global_idx + 1 < patterns.len() {
                partial_solutions.push((current, global_idx + 1));
            }
            partial_solutions.push((suffix, 0));
        } else {
            // cannot apply any pattern so it's the end of the road
            // but let's record it so we don't try again
            known_suffixes.insert(current);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_satisfiable() {
        let patterns: Vec<_> = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let cases = vec![
            ("brwrr", true),
            ("bggr", true),
            ("ubwu", false),
            ("bwurrg", true),
            ("bbrgwb", false),
            ("bwu", true),
        ];

        for (design, expected) in cases {
            assert_eq!(
                expected,
                is_satisfiable(design, &patterns),
                "{} {:?}",
                design,
                patterns
            )
        }
    }
}

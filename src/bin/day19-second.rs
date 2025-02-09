use std::collections::HashMap;
use std::fs;

use aoc_2024::day19::{self, Design, Pattern};

fn main() {
    let input = fs::read_to_string("data/day19/input").unwrap();
    let (patterns, designs) = day19::parse_input(&input);
    let total_possible_arrangements = count_total_arrangements(
        &designs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );
    println!("{}", total_possible_arrangements);
}

fn count_total_arrangements(designs: &[Design], patterns: &[Pattern]) -> u64 {
    designs
        .iter()
        .map(|d| count_distinct_arrangements(d, patterns))
        .sum()
}

fn count_distinct_arrangements(design: Design, patterns: &[Pattern]) -> u64 {
    let mut partial_solutions = vec![(design, 0, vec![])];
    let mut pending: HashMap<Design, u64> = HashMap::new();
    let mut known: HashMap<Design, u64> = HashMap::from([("", 1)]);

    while let Some((current, pattern_idx_min, ancestors)) = partial_solutions.pop() {
        if let Some(current_arrangements) = known.get(current) {
            for ancestor in ancestors {
                *pending.entry(ancestor).or_insert(0) += current_arrangements;
            }

            continue;
        }

        let first_match_idx = if pattern_idx_min < patterns.len() {
            day19::find_first_prefix(current, &patterns[pattern_idx_min..])
        } else {
            None
        };

        if let Some(idx) = first_match_idx {
            let global_idx = pattern_idx_min + idx;
            let pattern = &patterns[global_idx];
            let suffix = &current[pattern.len()..];
            let mut extended_ancestors = ancestors.clone();
            extended_ancestors.push(current);
            // we can always try again with the next one it that doesn't work
            partial_solutions.push((current, global_idx + 1, ancestors));
            partial_solutions.push((suffix, 0, extended_ancestors));
        } else {
            // cannot apply any pattern so it's the end of the road
            // we now know exactly how many distinct arrangements yield current
            // so let's remove it from the pending values
            let current_arrangements = pending.remove(&current).unwrap_or(0);
            // and record it for future lookups
            known.insert(current, current_arrangements);
        }
    }

    known[&design]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_arrangements() {
        let patterns: Vec<_> = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let cases = vec![
            ("brwrr", 2),
            ("bggr", 1),
            ("gbbr", 4),
            ("rrbgbr", 6),
            ("bwurrg", 1),
            ("brgr", 2),
            ("ubwu", 0),
            ("bbrgwb", 0),
        ];

        for (design, expected) in cases {
            assert_eq!(
                expected,
                count_distinct_arrangements(design, &patterns),
                "{} {:?}",
                design,
                patterns
            )
        }
    }
}

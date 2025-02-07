use std::fs;

fn main() {
    let input = fs::read_to_string("data/day19/input").unwrap();
    let (patterns, designs) = parse_input(&input);
    let total_possible_designs = count_possible_designs(
        &designs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );
    println!("{}", total_possible_designs);
}

type Pattern<'a> = &'a str;
type Design<'a> = &'a str;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns_block, designs_block) = input.split_once("\n\n").unwrap();
    let patterns: Vec<_> = patterns_block.split(", ").map(|s| s.to_string()).collect();
    let designs = designs_block.lines().map(|s| s.to_string()).collect();

    (minimal_patterns(patterns), designs)
}

fn count_possible_designs(designs: &[Design], patterns: &[Pattern]) -> usize {
    designs
        .iter()
        .filter(|d| is_satisfiable(d, patterns))
        .count()
}

fn is_satisfiable(design: Design, patterns: &[Pattern]) -> bool {
    let mut partial_solutions = vec![(design, 0)];

    while let Some((current, pattern_idx_min)) = partial_solutions.pop() {
        if current.is_empty() {
            return true;
        }

        if let Some(idx) = find_first_prefix(current, &patterns[pattern_idx_min..]) {
            // idx is obtained from the tail of patterns after pattern_idx_min
            let global_idx = pattern_idx_min + idx;
            let pattern = &patterns[global_idx];
            let suffix = &current[pattern.len()..];
            // we can always try again with the next one it that doesn't work
            if global_idx + 1 < patterns.len() {
                partial_solutions.push((current, global_idx + 1));
            }
            partial_solutions.push((suffix, 0));
        }

        // cannot apply any pattern so it's the end of the road
    }

    false
}

fn minimal_patterns(patterns: Vec<String>) -> Vec<String> {
    let mut sorted_patterns = patterns;
    sorted_patterns.sort_by_key(|p| p.len());

    let mut refined_patterns = vec![];

    for (idx, pattern) in sorted_patterns.iter().enumerate() {
        if !is_satisfiable(
            &pattern,
            &sorted_patterns[..idx]
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        ) {
            refined_patterns.push(pattern.to_string());
        }
    }

    refined_patterns
}

fn find_first_prefix(partial_design: &str, patterns: &[&str]) -> Option<usize> {
    if patterns.is_empty() {
        return None;
    }

    patterns
        .iter()
        .enumerate()
        .find(|(_, pattern)| partial_design.starts_with(*pattern))
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("data/day19/test_input").unwrap();
        let expected_patterns = ["r", "b", "g", "wr", "bwu"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected_designs = vec![
            "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!((expected_patterns, expected_designs), parse_input(&input));
    }

    #[test]
    fn test_find_first_prefix() {
        let cases = vec![
            ("bcd", vec!["a", "b", "bc", "bcd"], Some(1)),
            ("bcd", vec!["b", "bc", "bcd"], Some(0)),
            ("bcd", vec!["ab", "abc", "abcd"], None),
            ("bcd", vec!["a", "ab", "abc", "bcd"], Some(3)),
            ("bwu", vec!["g", "bwu", "rb", "gb", "br"], Some(1)),
        ];

        for (design, patterns, expected) in cases {
            assert_eq!(expected, find_first_prefix(design, &patterns))
        }
    }

    #[test]
    fn test_is_satisfiable() {
        let mut patterns: Vec<_> = vec!["r", "b", "g", "wr", "bwu"];
        patterns.sort();
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

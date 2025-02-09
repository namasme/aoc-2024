pub type Pattern<'a> = &'a str;
pub type Design<'a> = &'a str;

pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns_block, designs_block) = input.split_once("\n\n").unwrap();
    let patterns: Vec<_> = patterns_block.split(", ").map(|s| s.to_string()).collect();
    let designs = designs_block.lines().map(|s| s.to_string()).collect();

    (patterns, designs)
}

pub fn find_first_prefix(partial_design: Design, patterns: &[Pattern]) -> Option<usize> {
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
        let expected_patterns = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
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
}

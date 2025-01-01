use std::collections::HashMap;

type Stone = u64;

pub fn parse_input(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect()
}

pub fn simulate_stones(stones: &[Stone], blinks: usize) -> usize {
    stones
        .iter()
        .map(|&stone| simulate_stone(stone, blinks))
        .sum()
}

fn simulate_stone(stone: Stone, blinks: usize) -> usize {
    let mut frequencies = HashMap::from([(stone, 1)]);

    for _ in 0..blinks {
        let mut updated_frequencies = HashMap::new();

        for (stone, count) in frequencies {
            for next_stone in blink(stone) {
                *updated_frequencies.entry(next_stone).or_insert(0) += count;
            }
        }

        frequencies = updated_frequencies;
    }

    frequencies.iter().map(|(_, count)| count).sum()
}

fn blink(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        return vec![1];
    }

    let serialized = stone.to_string();
    if serialized.len() % 2 == 0 {
        let (left, right) = serialized.split_at(serialized.len() / 2);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }

    vec![stone * 2024]
}

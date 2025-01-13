use std::str::FromStr;

use crate::common::bezout;
use crate::spatial::{Point2D, Point2DCast};

const PART_2_PRIZE_DELTA: usize = 10000000000000;

pub fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .filter_map(|machine_block| machine_block.parse().ok())
        .collect()
}

pub type Position = Point2D<Coordinate>;
type Move = Point2D<Coordinate>;
type Coordinate = usize;

pub struct Machine {
    pub button_a: Move,
    pub button_b: Move,
    pub prize: Position,
}

impl Machine {
    pub fn required_tokens(&self) -> Option<usize> {
        let eq_a = PositiveDiophantineEquation {
            a: self.button_a.x,
            b: self.button_b.x,
            c: self.prize.x,
        };
        let eq_b = PositiveDiophantineEquation {
            a: self.button_a.y,
            b: self.button_b.y,
            c: self.prize.y,
        };

        eq_a.intersect(&eq_b)
            .map(|solution| 3 * solution.x + solution.y)
    }

    pub fn required_tokens_adjusted(&self) -> Option<usize> {
        let eq_a = PositiveDiophantineEquation {
            a: self.button_a.x,
            b: self.button_b.x,
            c: self.prize.x + PART_2_PRIZE_DELTA,
        };
        let eq_b = PositiveDiophantineEquation {
            a: self.button_a.y,
            b: self.button_b.y,
            c: self.prize.y + PART_2_PRIZE_DELTA,
        };

        eq_a.intersect(&eq_b)
            .map(|solution| 3 * solution.x + solution.y)
    }

    fn parse_line(button_line: &str) -> Move {
        let (_, coordinates_block) = button_line.split_once(": ").unwrap();
        let (x_block, y_block) = coordinates_block.split_once(", ").unwrap();
        let x = x_block[2..].parse().unwrap();
        let y = y_block[2..].parse().unwrap();

        Move { x, y }
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(machine_block: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = machine_block.lines().collect();
        let button_a = Machine::parse_line(lines[0]);
        let button_b = Machine::parse_line(lines[1]);
        let prize = Machine::parse_line(lines[2]);

        Ok(Machine {
            button_a,
            button_b,
            prize,
        })
    }
}

/// Represents the equation α * a + β * b = c / a, b, c >= 0
/// Solves for α, β >= 0
struct PositiveDiophantineEquation {
    a: Coordinate,
    b: Coordinate,
    c: Coordinate,
}

impl PositiveDiophantineEquation {
    fn intersect(&self, other: &PositiveDiophantineEquation) -> Option<Position> {
        // Does NOT handle all cases (e.g. gcd == 0), but works for the given input ¯\_(ツ)_/¯
        let gcd = bezout(self.a, other.a).0;
        let lambda = (self.a * other.a) / gcd;

        let denominator =
            ((lambda * self.b) / self.a) as isize - ((lambda * other.b) / other.a) as isize;
        if denominator == 0 {
            return None;
        }

        let numerator =
            ((lambda * self.c) / self.a) as isize - ((lambda * other.c) / other.a) as isize;
        let mu = self.b * self.a / bezout(self.a, self.b).0;

        let y = numerator / denominator;
        let x = ((self.c * mu) as isize - (mu * self.b) as isize * y) / (mu * self.a) as isize;

        // Last validation in case some integer division arithmetic went wrong
        Point2D::new(x, y).cast().ok().filter(|candidate| {
            self.a * candidate.x + self.b * candidate.y == self.c
                && other.a * candidate.x + other.b * candidate.y == other.c
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let cases = [
            ((1, 1, 0), (1, 1, 1), None),
            ((1, 1, 1), (2, 2, 2), None), // Not enough information
            ((3, 1, 5), (3, 2, 6), None), // x = 4/3
            ((1, 1, 6), (3, 2, 12), Some(Position::new(0, 6))),
            ((69, 41, 5242), (48, 88, 3944), Some(Position::new(73, 5))),
            ((20, 61, 4553), (52, 42, 5658), Some(Position::new(66, 53))),
            ((20, 61, 4553), (52, 42, 5658), Some(Position::new(66, 53))),
            ((21, 26, 690), (16, 68, 1104), Some(Position::new(18, 12))),
        ];

        for (eq_a, eq_b, expected) in cases {
            let result = PositiveDiophantineEquation {
                a: eq_a.0,
                b: eq_a.1,
                c: eq_a.2,
            }
            .intersect(&PositiveDiophantineEquation {
                a: eq_b.0,
                b: eq_b.1,
                c: eq_b.2,
            });
            assert_eq!(result, expected);
        }
    }
}

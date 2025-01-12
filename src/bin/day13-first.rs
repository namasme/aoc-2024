use std::default::Default;
use std::fs;
use std::str::FromStr;

use aoc_2024::common::bezout;
use aoc_2024::spatial::Point2D;

fn main() {
    let input = fs::read_to_string("data/day13/input").unwrap();
    let machines = parse_input(&input);
    let total_tokens: usize = machines
        .iter()
        .filter_map(|machine| machine.required_tokens())
        .sum();
    println!("{}", total_tokens);
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .filter_map(|machine_block| machine_block.parse().ok())
        .collect()
}

type Position = Point2D<Coordinate>;
type Move = Point2D<Coordinate>;
type Coordinate = usize;

struct Machine {
    button_a: Move,
    button_b: Move,
    prize: Position,
}

impl Machine {
    fn required_tokens(&self) -> Option<usize> {
        PositiveDiophantineEquation {
            a: self.button_a.x,
            b: self.button_b.x,
            c: self.prize.x,
        }
        .solutions()
        .filter(|(alpha, beta)| alpha * self.button_a.y + beta * self.button_b.y == self.prize.y)
        .map(|(alpha, beta)| (3 * alpha + beta) as usize)
        .min()
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
    fn solutions(&self) -> PDESolutions {
        let (line, range) = self.find_line();

        PDESolutions {
            line: Line2D {
                base: Position::new(line.base.x as Coordinate, line.base.y as Coordinate),
                delta: line.delta,
            },
            range: range as usize,
            current_k: 0,
        }
    }

    fn find_line(&self) -> (Line2D<isize>, isize) {
        let (gcd, alpha, beta) = bezout(self.a, self.b);

        // No solution
        if self.c % gcd != 0 {
            return Default::default();
        }

        let lcm = self.a * self.b / gcd;
        // We can express c = remainder + n * lcm(a, b)
        // This is useful because lcm(a, b) is by definition a multiple of both a and b,
        // so we can compensate "deficits" in one with the other.
        // E.g. 11 * 3 - 4 * 5 = 6 * 3 - 1 * 5 = 1 * 3 + 2 * 5 = 13
        let remainder = self.c % lcm;
        let n = (self.c / lcm) as isize;
        // The remainder can only be expressed by means of the Bézout coefficients, however.
        // The result of this is what we will have to offset via _lcm conversions_.
        let multiplier = (remainder / gcd) as isize;
        let alpha_delta = (lcm / self.a) as isize;
        let beta_delta = (lcm / self.b) as isize;
        // At this point we know
        // (alpha * multiplier, beta * multiplier) · (a, b) = remainder
        // (alpha_delta, -beta_delta) · (a, b) = 0 => (A + alpha_delta, B - beta_delta) · (a, b) = (A, B) · (a, b)
        // So we calculate the smallest k such that alpha * multiplier + k * alpha_delta >= 0
        let adjustment_factor = Self::smallest_k(alpha * multiplier, alpha_delta);
        // The base solution for α is thus
        //
        // alpha * multiplier + adjustment_factor * alpha_delta >= 0.
        //
        // Then we have
        //
        // (alpha * multiplier + adjustment_factor * alpha_delta, beta * multiplier - adjusment_factor * beta_delta) · (a, b) =
        //                                                                   (alpha * multiplier, beta * multiplier) · (a, b) =
        //                                                                                                                      remainder
        //
        // The _unused_ lcms are then added to β to satisfy the original equation:
        //
        // (alpha * multiplier + adjustment_factor * alpha_delta, beta * multiplier + (n - adjustment_factor) * beta_delta) · (a, b) =
        //                                                                                  remainder + (0, n * beta_delta) · (a, b) =
        //                                                                                            remainder + n * b * beta_delta
        //
        // Where n * b * beta_delta = (c / lcm(a, b)) * b * (lcm(a, b) / b) = c - remainder due to integer division
        let base = Point2D::new(
            alpha * multiplier + adjustment_factor * alpha_delta,
            beta * multiplier + (n - adjustment_factor) * beta_delta,
        );
        // Finally we need to find out how many times we can convert lcms between alpha and beta before beta becomes negative.
        // This will happen whenever
        //
        // β - k' * beta_delta < 0, k' > 0
        //
        // So we can use the same function to find
        //
        // min{k: β + k * beta_delta >= 0} => β + (k - 1) * beta_delta < 0
        //
        // And finally k' = -(k - 1) = 1 - k.
        let range = 1 - (Self::smallest_k(base.y, beta_delta));

        (
            Line2D {
                base,
                delta: Point2D::new(alpha_delta, -beta_delta),
            },
            range,
        )
    }

    /// smallest_k calculates the smallest k / a + k * b >= 0
    /// Not a great name but unclear what to call it
    /// Relies on integer division properties depending on the sign of its inputs.
    fn smallest_k(a: isize, b: isize) -> isize {
        if a > 0 {
            return -(a / b);
        } else {
            return (b.abs() - a - 1) / b;
        }
    }
}

struct PDESolutions {
    line: Line2D<Coordinate>,
    current_k: usize,
    range: usize,
}

#[derive(Default)]
struct Line2D<T> {
    base: Point2D<T>,
    delta: Point2D<isize>,
}

impl Iterator for PDESolutions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_k >= self.range {
            return None;
        }

        let solution = (
            (self.line.base.x as isize + self.current_k as isize * self.line.delta.x) as usize,
            (self.line.base.y as isize + self.current_k as isize * self.line.delta.y) as usize,
        );
        self.current_k += 1;
        Some(solution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solutions() {
        let cases = vec![
            ((3, 5, 1), vec![]),
            ((3, 5, 3), vec![(1, 0)]),
            ((3, 5, 5), vec![(0, 1)]),
            ((3, 5, 17), vec![(4, 1)]),
            ((3, 5, 32), vec![(4, 4), (9, 1)]),
            ((3, 5, 47), vec![(4, 7), (9, 4), (14, 1)]),
            ((1, 1, 3), vec![(0, 3), (1, 2), (2, 1), (3, 0)]),
            ((4, 8, 2), vec![]),
        ];

        for ((a, b, target), expected) in cases {
            let result: Vec<_> = PositiveDiophantineEquation { a, b, c: target }
                .solutions()
                .collect();
            //let result = Machine::positive_solutions(a, b, target);
            assert_eq!(result, expected);
        }
    }
}

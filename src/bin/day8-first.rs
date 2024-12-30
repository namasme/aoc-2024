use aoc_2024::common::pairs;
use aoc_2024::spatial::Point2D;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/day8/input").unwrap();
    let grid: Grid = input.parse().unwrap();
    let unique_antinodes = grid.unique_antinodes();
    println!("{}", unique_antinodes.len());
}

type Frequency = char;
type Coordinate = i16;
type Position = Point2D<Coordinate>;

struct Grid {
    rows: usize,
    columns: usize,
    antennas: HashMap<Frequency, Vec<Position>>,
}

impl Grid {
    fn unique_antinodes(&self) -> HashSet<Position> {
        self.antennas
            .iter()
            .flat_map(|(_, positions)| {
                pairs(positions)
                    .into_iter()
                    .flat_map(|(first, second)| self.generate_antinodes(first, second))
            })
            .collect::<HashSet<Position>>()
    }

    fn generate_antinodes(&self, first: Position, second: Position) -> Vec<Position> {
        let delta = second - first;
        let mut antinodes = vec![first - delta, second + delta];

        if delta.x % 3 == 0 && delta.y % 3 == 0 {
            let small_delta = Position::new(delta.x / 3, delta.y / 3);
            antinodes.push(first + small_delta);
            antinodes.push(second - small_delta);
        }

        antinodes
            .into_iter()
            .filter(|antinode| self.is_valid(antinode))
            .collect()
    }

    fn is_valid(&self, antinode: &Position) -> bool {
        antinode.x >= 0
            && antinode.x < self.columns as Coordinate
            && antinode.y >= 0
            && antinode.y < self.rows as Coordinate
    }
}

impl FromStr for Grid {
    type Err = (); // cannot fail
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input.lines().count();
        let columns = input.find('\n').unwrap_or(input.len());
        let mut antennas = HashMap::new();

        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                let frequency = c;
                let antinodes = antennas.entry(frequency).or_insert(Vec::new());
                antinodes.push(Position::new(x as Coordinate, y as Coordinate));
            }
        }

        Ok(Grid {
            rows,
            columns,
            antennas,
        })
    }
}

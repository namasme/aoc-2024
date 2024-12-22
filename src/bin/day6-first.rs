use aoc_2024::spatial::{Direction, Orientation, Point2D};
use std::collections::HashSet;
use std::fs;
use std::iter::successors;

fn main() {
    let input = fs::read_to_string("data/day6/input").unwrap();
    let (grid, guard) = parse_input(&input);
    let unique_positions = guard.unique_positions(&grid);
    println!("{}", unique_positions.len());
}

fn parse_input(input: &str) -> (Grid, Guard) {
    let rows = input.lines().count();
    let columns = input.find('\n').unwrap();
    let mut obstacles = HashSet::new();
    let mut start = Point2D::new(0, 0);

    for (row, line) in input.lines().enumerate() {
        for (column, tile) in line.chars().enumerate() {
            match tile {
                '#' => {
                    obstacles.insert(Point2D::new(column as Coordinate, row as Coordinate));
                }
                '^' => {
                    start = Point2D::new(column as Coordinate, row as Coordinate);
                }
                _ => {}
            }
        }
    }

    let grid = Grid {
        rows,
        columns,
        obstacles,
    };
    let guard = Guard {
        position: start,
        facing: Direction::Up,
    };

    (grid, guard)
}

type Coordinate = i16;
type Position = Point2D<Coordinate>;

struct Grid {
    rows: usize,
    columns: usize,
    obstacles: HashSet<Position>,
}

impl Grid {
    fn is_valid(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.columns as Coordinate
            && position.y >= 0
            && position.y < self.rows as Coordinate
    }

    fn is_empty(&self, position: &Position) -> bool {
        !self.obstacles.contains(position)
    }
}

#[derive(Clone, Copy)]
struct Guard {
    position: Position,
    facing: Direction,
}

impl Guard {
    fn unique_positions(&self, grid: &Grid) -> HashSet<Position> {
        successors(Some(*self), |guard| guard.advance(grid))
            .map(|guard| guard.position)
            .collect()
    }

    fn advance(&self, grid: &Grid) -> Option<Guard> {
        let delta: Position = Position::from(self.facing);
        let mirrored_delta = Position::new(delta.x, -delta.y);
        let candidate_next = self.position + mirrored_delta;

        if !grid.is_valid(&candidate_next) {
            return None;
        } else if !grid.is_empty(&candidate_next) {
            return Some(Self {
                position: self.position,
                facing: self.facing.rotate(Orientation::Clockwise),
            });
        } else {
            return Some(Self {
                position: candidate_next,
                facing: self.facing,
            });
        }
    }
}

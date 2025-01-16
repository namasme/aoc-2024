use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use aoc_2024::spatial::{Direction, Point2D};

fn main() {
    let input = fs::read_to_string("data/day15/input").unwrap();
    let (mut warehouse, moves) = parse_input(&input);
    warehouse.apply(&moves);
    let gps_sum = warehouse.gps_sum();
    println!("{}", gps_sum);
}

fn parse_input(input: &str) -> (Warehouse, Vec<Move>) {
    let (warehouse_block, moves_block) = input.split_once("\n\n").unwrap();
    let warehouse = warehouse_block.parse().unwrap();
    let moves = moves_block
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect();

    (warehouse, moves)
}

struct Warehouse {
    width: usize,
    height: usize,
    robot: Position,
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
}
type Move = Direction;
type Position = Point2D<Coordinate>;
type Coordinate = i8;

impl Warehouse {
    fn apply(&mut self, moves: &[Move]) {
        for move_ in moves {
            self.apply_move(*move_);
        }
    }

    fn gps_sum(&self) -> u64 {
        self.boxes.iter().map(|box_| self.gps(*box_)).sum()
    }

    fn apply_move(&mut self, move_: Move) {
        let delta = Position::from(move_);
        let mut current = self.robot + delta;

        while self.is_valid(current) && self.boxes.contains(&current) {
            current = current + delta;
        }

        if !self.is_valid(current) || self.walls.contains(&current) {
            // we bumped into a wall, so we don't move
            return;
        }

        // The boxes are pushed up until here
        self.boxes.insert(current);
        // This will now be occupied by the robot
        self.boxes.remove(&(self.robot + delta));
        self.robot = self.robot + delta;
    }

    fn gps(&self, box_: Position) -> u64 {
        let top = self.height as u64 - box_.y as u64;
        let left = 1 + (box_.x as u64);
        100 * top + left
    }

    fn is_valid(&self, position: Position) -> bool {
        position.x >= 0
            && position.x < self.width as Coordinate
            && position.y >= 0
            && position.y < self.height as Coordinate
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = None;
        let width = s.find('\n').unwrap() - 2;
        let height = s.lines().count() - 2;

        for (y, line) in s.lines().rev().skip(1).take(height).enumerate() {
            for (x, c) in line.chars().skip(1).take(width).enumerate() {
                let position = Position::new(x as Coordinate, y as Coordinate);
                match c {
                    '#' => {
                        walls.insert(position);
                    }
                    '@' => {
                        robot = Some(position);
                    }
                    'O' => {
                        boxes.insert(position);
                    }
                    _ => {}
                }
            }
        }

        Ok(Warehouse {
            width,
            height,
            robot: robot.unwrap(),
            walls,
            boxes,
        })
    }
}

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use aoc_2024::spatial::Point2D;

fn main() {
    let input = fs::read_to_string("data/day14/input").unwrap();
    let grid = Grid::parse(&input, 101, 103);
    let safety_factor = grid.safety_factor(100);
    println!("{}", safety_factor);
}

struct Grid {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

struct Robot {
    position: Position,
    velocity: Velocity,
}

type Position = Point2D<Coordinate>;
type Velocity = Point2D<Coordinate>;
type Coordinate = i64;
type Time = u64;

impl Grid {
    fn parse(input: &str, width: usize, height: usize) -> Grid {
        let robots = input.lines().map(|line| line.parse().unwrap()).collect();
        Grid {
            robots,
            width,
            height,
        }
    }

    fn safety_factor(&self, after: Time) -> u64 {
        let mut quadrants_count = HashMap::new();
        let delta = (
            (self.width / 2) as Coordinate,
            (self.height / 2) as Coordinate,
        );

        for robot in self.robots.iter() {
            let final_position = self.simulate(&robot, after);
            let quadrant = (
                (final_position.x - delta.0).signum(),
                (final_position.y - delta.1).signum(),
            );

            if quadrant.0 == 0 || quadrant.1 == 0 {
                continue;
            }

            quadrants_count
                .entry(quadrant)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        quadrants_count
            .iter()
            .inspect(|(quadrant, count)| println!("quadrant {:?}: {}", quadrant, count))
            .map(|(_, count)| count)
            .product()
    }

    fn simulate(&self, robot: &Robot, after: Time) -> Position {
        self.torus_add(robot.position, robot.velocity * after as Coordinate)
    }

    fn torus_add(&self, p: Position, q: Position) -> Position {
        Position::new(
            (p.x + q.x).rem_euclid(self.width as Coordinate),
            (p.y + q.y).rem_euclid(self.height as Coordinate),
        )
    }
}

impl Robot {
    fn parse_vector(s: &str) -> Position {
        let (x, y) = s[2..].split_once(',').unwrap();

        Position::new(x.parse().unwrap(), y.parse().unwrap())
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position_block, velocity_block) = s.split_once(' ').unwrap();
        let position = Robot::parse_vector(position_block);
        let velocity = Robot::parse_vector(velocity_block);
        Ok(Robot { position, velocity })
    }
}

use std::fs;
use std::str::FromStr;

use aoc_2024::graph::{Dijkstra, WeightedGraph};
use aoc_2024::parse::TextGrid;
use aoc_2024::spatial::{Direction, Orientation, Point2D};

fn main() {
    let input = fs::read_to_string("data/day16/input").unwrap();
    let maze: Maze = input.parse().unwrap();
    let lowest_score = maze.lowest_score();
    println!("{}", lowest_score);
}

struct Maze {
    content: TextGrid,
    start: Position,
    end: Position,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Reindeer {
    position: Position,
    facing: Direction,
}

type Position = Point2D<Coordinate>;
type Coordinate = usize;
type Score = u64;

impl Maze {
    fn lowest_score(&self) -> Score {
        let initial_reindeer = Reindeer {
            position: self.start,
            facing: Direction::Right,
        };

        self.distance(&[initial_reindeer], |reindeer| {
            reindeer.position == self.end
        })
        .unwrap()
    }

    fn is_empty(&self, position: Position) -> bool {
        self.content
            .char_at(position)
            .map(|c| c != '#')
            .unwrap_or(false)
    }

    fn is_valid(&self, position: Position) -> bool {
        position.x > 0
            && position.x < self.content.width - 1
            && position.y > 0
            && position.y < self.content.width - 1
    }
}

impl WeightedGraph<Reindeer, Score> for Maze {
    fn neighbours(&self, reindeer: &Reindeer) -> Vec<(Score, Reindeer)> {
        let forward = Some(Reindeer {
            position: reindeer.position.advance(reindeer.facing),
            facing: reindeer.facing,
        })
        .filter(|candidate| self.is_valid(candidate.position) && self.is_empty(candidate.position));
        let turn_cw = Reindeer {
            position: reindeer.position,
            facing: reindeer.facing.rotate(Orientation::Clockwise),
        };
        let turn_ccw = Reindeer {
            position: reindeer.position,
            facing: reindeer.facing.rotate(Orientation::Counterclockwise),
        };

        let forward_candidate = forward.map(|forward_candidate| (1, forward_candidate));
        let turn_candidates = [turn_cw, turn_ccw]
            .into_iter()
            .map(|turn_candidate| (1000, turn_candidate));

        forward_candidate
            .into_iter()
            .chain(turn_candidates)
            .collect()
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content: TextGrid = s.parse()?;
        let start = content.iter().find(|&(_, c)| c == 'S').unwrap().0;
        let end = content.iter().find(|&(_, c)| c == 'E').unwrap().0;

        Ok(Self {
            content,
            start,
            end,
        })
    }
}

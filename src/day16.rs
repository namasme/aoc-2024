use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

use crate::graph::{Dijkstra, WeightedGraph};
use crate::parse::TextGrid;
use crate::spatial::{Direction, Orientation, Point2D};

pub struct Maze {
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
    pub fn shortest_paths_tiles(&self) -> usize {
        let initial_reindeer = Reindeer {
            position: self.start,
            facing: Direction::Right,
        };
        let is_end = |reindeer: &Reindeer| reindeer.position == self.end;
        let traversal = self.traverse(&[initial_reindeer], is_end);

        // Find all the nodes in the graph that correspond to the end position
        // and have the minimum score
        let min_score: Score = traversal.shortest_distance(is_end).unwrap();
        let goals = traversal
            .distances
            .iter()
            .filter(|(reindeer, &score)| is_end(reindeer) && score == min_score)
            .map(|(reindeer, _)| reindeer);

        // For each of those, find all the shortest paths that lead to it
        // and keep track of the unique positions they visit
        let tiles: HashSet<Position> = goals
            .flat_map(|&end| {
                traversal
                    .shortest_paths(end)
                    .into_iter()
                    .flat_map(|path| path)
                    .map(|reindeer| reindeer.position)
            })
            .collect();

        tiles.len()
    }

    pub fn lowest_score(&self) -> Score {
        let initial_reindeer = Reindeer {
            position: self.start,
            facing: Direction::Right,
        };
        let is_end = |reindeer: &Reindeer| reindeer.position == self.end;

        let traversal = self.traverse(&[initial_reindeer], is_end);

        traversal.shortest_distance(is_end).unwrap()
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

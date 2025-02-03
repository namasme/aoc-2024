use std::cmp::Ordering;
use std::collections::HashSet;

use crate::common::{binary_search, Match};
use crate::graph::Dijkstra;
use crate::graph::WeightedGraph;
use crate::spatial::{Point2D, Point2DCast};

type BytePosition = Point2D<Coordinate>;
type Coordinate = usize;

pub fn parse_input(input: &str) -> Vec<BytePosition> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            BytePosition {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

pub struct Grid {
    width: usize,
    height: usize,
    blocks: HashSet<BytePosition>,
}

impl Grid {
    pub fn shortest_distance(&self) -> usize {
        let origin = BytePosition::new(0, 0);
        let target = BytePosition::new(self.width - 1, self.height - 1);
        let result = self.traverse(&vec![origin], |&node| node == target);

        result.distances[&target]
    }

    // Kinda funky API but oh well
    pub fn first_blocking_byte(&self, bytes: &[BytePosition]) -> BytePosition {
        let target = self.target();
        let result = binary_search(0, bytes.len(), |candidate| {
            let grid = Grid::new(self.width, self.height, &bytes[..candidate]);
            let result = grid.traverse(&[self.origin()], |&node| node == target);
            let is_reachable = result.distances.contains_key(&target);

            // bool -> Ordering mapping here is arbitrary, we are only interested in changes
            if is_reachable {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });

        match result {
            Match::After(idx) => bytes[idx - 1],
            _ => panic!("not found"),
        }
    }

    pub fn new(width: usize, height: usize, bytes: &[BytePosition]) -> Self {
        Self {
            width,
            height,
            blocks: bytes.iter().copied().collect(),
        }
    }

    fn origin(&self) -> BytePosition {
        BytePosition::new(0, 0)
    }

    fn target(&self) -> BytePosition {
        BytePosition::new(self.width - 1, self.height - 1)
    }

    fn is_valid(&self, position: Point2D<isize>) -> bool {
        position.x >= 0
            && (position.x as usize) < self.width
            && position.y >= 0
            && (position.y as usize) < self.height
    }
}

impl WeightedGraph<BytePosition, Coordinate> for Grid {
    fn neighbours(&self, node: &BytePosition) -> Vec<(Coordinate, BytePosition)> {
        // UGH I hate usize bounds and having to manually cast to do anything meaningful
        node.cast()
            .unwrap()
            .neighbours()
            .into_iter()
            .filter(|neighbour| {
                self.is_valid(*neighbour) && !self.blocks.contains(&neighbour.cast().unwrap())
            })
            .map(|neighbour| (1, neighbour.cast().unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_input() {
        let input = "0,0\n1,1\n2,2\n";
        let expected = vec![
            BytePosition { x: 0, y: 0 },
            BytePosition { x: 1, y: 1 },
            BytePosition { x: 2, y: 2 },
        ];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn test_shortest_distance() {
        let input = fs::read_to_string("data/day18/test_input").unwrap();
        let bytes = parse_input(&input);
        let grid_size = 7;
        let bytes_read = 12;
        let grid = Grid::new(grid_size, grid_size, &bytes[..bytes_read]);

        assert_eq!(22, grid.shortest_distance());
    }

    #[test]
    fn test_first_blocking_byte() {
        let input = fs::read_to_string("data/day18/test_input").unwrap();
        let bytes = parse_input(&input);
        let grid_size = 7;
        let grid = Grid::new(grid_size, grid_size, &[]);

        assert_eq!(Point2D::new(6, 1), grid.first_blocking_byte(&bytes));
    }
}

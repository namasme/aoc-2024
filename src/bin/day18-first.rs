use std::collections::HashSet;
use std::fs;

use aoc_2024::graph::Dijkstra;
use aoc_2024::graph::WeightedGraph;
use aoc_2024::spatial::{Point2D, Point2DCast};

const GRID_SIZE: usize = 71;
const BYTES_READ: usize = 1024;

fn main() {
    let input = fs::read_to_string("data/day18/input").unwrap();
    let bytes = parse_input(&input);
    let grid = Grid::new(GRID_SIZE, GRID_SIZE, &bytes[..BYTES_READ]);
    let shortest_distance = grid.shortest_distance();
    println!("{}", shortest_distance);
}

type BytePosition = Point2D<Coordinate>;
type Coordinate = usize;

fn parse_input(input: &str) -> Vec<BytePosition> {
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

struct Grid {
    width: usize,
    height: usize,
    blocks: HashSet<BytePosition>,
}

impl Grid {
    fn shortest_distance(&self) -> usize {
        let origin = BytePosition::new(0, 0);
        let target = BytePosition::new(self.width - 1, self.height - 1);
        let dijkstra = self.traverse(&vec![origin], |&node| node == target);

        dijkstra.distances[&target]
    }

    fn new(width: usize, height: usize, bytes: &[BytePosition]) -> Self {
        Self {
            width,
            height,
            blocks: bytes.iter().copied().collect(),
        }
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
}

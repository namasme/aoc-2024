use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

pub trait WeightedGraph<Node, Distance> {
    fn neighbours(&self, node: &Node) -> Vec<(Distance, Node)>;
}

pub trait Dijkstra<Node, Distance> {
    fn traverse<P: FnMut(&Node) -> bool>(
        &self,
        seeds: &[Node],
        is_goal: P,
    ) -> DijkstraTraversal<Node, Distance>;
}

pub struct DijkstraTraversal<Node, Distance> {
    pub seeds: Vec<Node>,
    pub distances: HashMap<Node, Distance>,
    pub predecessors: HashMap<Node, Vec<Node>>,
}

impl<Node, Distance, T> Dijkstra<Node, Distance> for T
where
    T: WeightedGraph<Node, Distance>,
    Node: Copy + Eq + Hash,
    Distance: Add<Output = Distance> + Copy + Default + Ord + PartialOrd,
{
    fn traverse<P: FnMut(&Node) -> bool>(
        &self,
        seeds: &[Node],
        mut is_goal: P,
    ) -> DijkstraTraversal<Node, Distance> {
        let mut distances: HashMap<Node, Distance> = HashMap::new();
        let mut unvisited: BinaryHeap<DijkstraDistanceNode<Node, Distance>> = BinaryHeap::new();
        let mut predecessors: HashMap<Node, Vec<Node>> = HashMap::new();

        for seed in seeds.iter().copied() {
            unvisited.push(DijkstraDistanceNode::from(Default::default(), seed));
            distances.insert(seed, Default::default());
        }

        while let Some(current) = unvisited.pop() {
            if is_goal(&current.node) {
                return DijkstraTraversal {
                    seeds: seeds.to_vec(),
                    distances,
                    predecessors,
                };
            }

            let current_best_distance = distances[&current.node];

            if current_best_distance < current.cumulative_distance {
                continue;
            }

            for (edge_distance, neighbour) in self.neighbours(&current.node) {
                let candidate_neighbour_distance = current.cumulative_distance + edge_distance;
                let neighbour_best_distance = distances
                    .entry(neighbour)
                    .or_insert(candidate_neighbour_distance);

                // We already know of a better way of getting to neighbour, so we can stop here
                if *neighbour_best_distance < candidate_neighbour_distance {
                    continue;
                }

                let other_predecessors = predecessors.entry(neighbour).or_insert(vec![]);
                // We found a different but equivalent way of getting to neighbour
                // Let's add the current node to the list of predecessors for it
                if *neighbour_best_distance == candidate_neighbour_distance {
                    let first_visit = other_predecessors.is_empty();
                    other_predecessors.push(current.node);

                    // We had actually _not_ found a path to neighbour yet, so we
                    // also need to add it to the queue to continue exploring from it
                    if first_visit {
                        unvisited.push(DijkstraDistanceNode::from(
                            candidate_neighbour_distance,
                            neighbour,
                        ));
                    }

                    continue;
                }

                // We found a better way of getting to neighbour, so let's continue the search from here
                unvisited.push(DijkstraDistanceNode::from(
                    candidate_neighbour_distance,
                    neighbour,
                ));
                *neighbour_best_distance = candidate_neighbour_distance;
                *other_predecessors = vec![current.node];
            }
        }

        DijkstraTraversal {
            seeds: seeds.to_vec(),
            distances,
            predecessors,
        }
    }
}

impl<Node: Copy + Eq + Hash, Distance: Copy + Ord> DijkstraTraversal<Node, Distance> {
    // TODO: could return an Iterator instead but this does the trick for now
    pub fn shortest_paths(&self, target: Node) -> Vec<Vec<Node>> {
        let mut pending = vec![vec![target]];
        let mut paths = vec![];

        while let Some(mut incomplete_path) = pending.pop() {
            let current = incomplete_path.last().unwrap();

            if self.seeds.contains(current) {
                incomplete_path.reverse();
                paths.push(incomplete_path);
                continue;
            }

            let predecessors = &self.predecessors[current];
            for predecessor in predecessors {
                let mut new_path = incomplete_path.clone();
                new_path.push(*predecessor);
                pending.push(new_path);
            }
        }

        paths
    }

    pub fn shortest_distance<P: FnMut(&Node) -> bool>(&self, mut is_goal: P) -> Option<Distance> {
        self.distances
            .iter()
            .filter(|(node, _)| is_goal(node))
            .map(|(_, distance)| distance)
            .min()
            .copied()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DijkstraDistanceNode<Node, Distance> {
    cumulative_distance: Distance,
    node: Node,
}

impl<Node, Distance> DijkstraDistanceNode<Node, Distance> {
    pub fn from(cumulative_distance: Distance, node: Node) -> DijkstraDistanceNode<Node, Distance> {
        DijkstraDistanceNode {
            cumulative_distance,
            node,
        }
    }
}

/// Need a custom implementation because Node may not necessarily implement Ord
impl<Node, Distance> Ord for DijkstraDistanceNode<Node, Distance>
where
    Node: Eq + PartialEq,
    Distance: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.cumulative_distance.cmp(&self.cumulative_distance)
    }
}

impl<Node, Distance> PartialOrd for DijkstraDistanceNode<Node, Distance>
where
    Node: Eq + PartialEq,
    Distance: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    use crate::spatial::Point2D;

    struct Grid {
        width: u8,
        height: u8,
    }

    impl Grid {
        fn is_valid(&self, node: &Point2D<i8>) -> bool {
            node.x >= 0 && node.y >= 0 && node.x < self.width as i8 && node.y < self.height as i8
        }
    }

    impl WeightedGraph<Point2D<i8>, u8> for Grid {
        fn neighbours(&self, node: &Point2D<i8>) -> Vec<(u8, Point2D<i8>)> {
            node.neighbours()
                .into_iter()
                .filter(|neighbour| self.is_valid(neighbour))
                .map(|neighbour| (1, neighbour))
                .collect()
        }
    }

    #[test]
    fn grid_full_traversal() {
        let grid = Grid {
            width: 3,
            height: 4,
        };
        let full_traversal = grid.traverse(&[Point2D::new(0, 0)], |_| false);
        let target = Point2D::new(2, 2);

        assert_eq!(4, full_traversal.distances[&target]);
        assert_eq!(
            vec![Point2D::new(1, 2), Point2D::new(2, 1)],
            full_traversal.predecessors[&target]
        );
    }

    struct AdjacencyListGraph<Node> {
        edges: HashMap<Node, Vec<Node>>,
    }

    impl WeightedGraph<u8, u8> for AdjacencyListGraph<u8> {
        fn neighbours(&self, node: &u8) -> Vec<(u8, u8)> {
            self.edges
                .get(node)
                .unwrap_or(&vec![])
                .iter()
                .copied()
                .map(|neighbour| (1, neighbour))
                .collect()
        }
    }

    #[test]
    fn adjacency_list_full_traversal() {
        //   - 1 -
        //  /     \
        // 0 - 2 - 4 - 5
        //  \     /
        //   - 3 -
        let graph = AdjacencyListGraph {
            edges: HashMap::from([
                (0, vec![1, 2, 3]),
                (1, vec![0, 4]),
                (2, vec![0, 4]),
                (3, vec![0, 4]),
                (4, vec![1, 2, 3, 5]),
            ]),
        };
        let full_traversal = graph.traverse(&[0], |_| false);
        let target = 5;

        assert_eq!(3, full_traversal.distances[&target]);
        assert_eq!(vec![1, 2, 3], full_traversal.predecessors[&4]);
    }

    #[test]
    fn adjacency_list_shortest_paths() {
        //   - 1 -
        //  /     \
        // 0 - 2 - 4 - 5
        //  \     /
        //   - 3 -
        let graph = AdjacencyListGraph {
            edges: HashMap::from([
                (0, vec![1, 2, 3]),
                (1, vec![0, 4]),
                (2, vec![0, 4]),
                (3, vec![0, 4]),
                (4, vec![1, 2, 3, 5]),
            ]),
        };
        let start = 0;
        let target = 5;
        let full_traversal = graph.traverse(&[start], |_| false);
        let paths = full_traversal.shortest_paths(target);

        let expected_paths = vec![vec![0, 1, 4, 5], vec![0, 2, 4, 5], vec![0, 3, 4, 5]];
        assert_eq!(expected_paths.len(), paths.len());
        for expected_path in expected_paths {
            assert!(paths.contains(&expected_path));
        }
    }

    struct Maze {
        grid: Grid,
        blocks: HashSet<Point2D<i8>>,
    }

    impl Maze {
        fn from(raw_maze: &str, block_tile: char) -> Self {
            let mut obstacles = HashSet::new();

            for (row, line) in raw_maze.lines().rev().enumerate() {
                for (column, tile) in line.chars().enumerate() {
                    let position = Point2D::new(column as i8, row as i8);
                    match tile {
                        _ if tile == block_tile => {
                            obstacles.insert(position);
                        }
                        _ => {}
                    }
                }
            }
            let grid = Grid {
                width: raw_maze.lines().next().unwrap().len() as u8,
                height: raw_maze.lines().count() as u8,
            };

            Self {
                grid,
                blocks: obstacles,
            }
        }
    }

    impl WeightedGraph<Point2D<i8>, u8> for Maze {
        fn neighbours(&self, node: &Point2D<i8>) -> Vec<(u8, Point2D<i8>)> {
            node.neighbours()
                .into_iter()
                .filter(|neighbour| {
                    self.grid.is_valid(neighbour) && !self.blocks.contains(neighbour)
                })
                .map(|neighbour| (1, neighbour))
                .collect()
        }
    }

    #[test]
    fn reachability() {
        let raw_maze = r"
...#...
.##..##
.#..#..
...#..#
###..##
.##.###
#.#....
"
        .trim();
        let maze = Maze::from(raw_maze, '#');
        let start = Point2D::new(0, 0);
        let target = Point2D::new((maze.grid.width - 1) as i8, (maze.grid.height - 1) as i8);
        let result = maze.traverse(&[start], |&node| node == target);
        let is_reachable = result.distances.contains_key(&target);

        assert!(!is_reachable);
    }
}

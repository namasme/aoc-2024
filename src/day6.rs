use crate::common::CycleDetection;
use crate::spatial::{Direction, Orientation, Point2D};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Grid, Guard) {
    let rows = input.lines().count();
    let columns = input.find('\n').unwrap();
    let mut obstacles = HashSet::new();
    let mut start = Point2D::new(0, 0);

    // Iterate the rows in reverse order to have the origin at the bottom left
    // so directions works as expected, i.e. up is positive y, not negative.
    for (row, line) in input.lines().rev().enumerate() {
        for (column, tile) in line.chars().enumerate() {
            let position = Point2D::new(column as Coordinate, row as Coordinate);
            match tile {
                '#' => {
                    obstacles.insert(position);
                }
                '^' => {
                    start = position;
                }
                _ => {}
            }
        }
    }

    let grid = Grid::new(rows, columns, obstacles);
    let guard = Guard {
        position: start,
        facing: Direction::Up,
    };

    (grid, guard)
}

type Coordinate = i16;
type Position = Point2D<Coordinate>;

pub struct Grid {
    rows: usize,
    columns: usize,
    obstacles: HashSet<Position>,
    collisions_cache: RefCell<HashMap<Collision, Option<Collision>>>,
}

impl Grid {
    fn new(rows: usize, columns: usize, obstacles: HashSet<Position>) -> Self {
        Self {
            rows,
            columns,
            obstacles,
            collisions_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn unique_positions(&self, start: &Guard) -> HashSet<Position> {
        self.iter_guard(start).map(|guard| guard.position).collect()
    }

    pub fn count_loops(&self, start: Guard) -> usize {
        let mut visited = HashSet::new();
        let mut loops_detected = HashSet::new();

        for guard in self.iter_guard(&start) {
            if guard.position == start.position || visited.contains(&guard.position) {
                continue;
            }

            visited.insert(guard.position);
            if self.creates_loop(guard) {
                loops_detected.insert(guard.position);
            }
        }

        loops_detected.len()
    }

    fn iter_guard<'a>(&'a self, guard: &Guard) -> GuardIter<'a> {
        GuardIter {
            guard: Some(*guard),
            grid: self,
        }
    }

    fn creates_loop(&self, guard: Guard) -> bool {
        let extended_grid = ExtendedGrid {
            base_grid: self,
            additional_obstacle: guard.position,
        };
        let initial_collision = Collision {
            position: guard.position,
            face: -guard.facing,
        };
        let collisions = extended_grid.iter_collisions(initial_collision);

        collisions.detect_cycle().is_some()
    }

    /// Returns the next obstacle the guard collides with, starting from the given obstacle and face direction.
    fn next_collision(&self, collision: Collision) -> Option<Collision> {
        // Check if the result is already cached
        if let Some(&result) = self.collisions_cache.borrow().get(&collision) {
            return result;
        }

        // Compute the result lazily
        let result = self.compute_next_collision(Guard::from(collision));

        // Cache the result
        self.collisions_cache.borrow_mut().insert(collision, result);

        result
    }

    /// Computes the next obstacle collision
    fn compute_next_collision(&self, guard: Guard) -> Option<Collision> {
        let delta: Position = Position::from(guard.facing);
        let mut current_position = guard.position + delta;

        // Simulate the guard moving in the given direction until it hits an obstacle or exits the grid
        while self.is_valid(&current_position) {
            if self.obstacles.contains(&current_position) {
                return Some(Collision {
                    position: current_position,
                    face: -guard.facing,
                });
            }
            current_position = current_position + delta;
        }

        // No obstacle collision; guard exits the grid
        None
    }

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
pub struct Guard {
    position: Position,
    facing: Direction,
}

impl Guard {
    fn advance(&self, grid: &Grid) -> Option<Guard> {
        let delta: Position = Position::from(self.facing);
        let candidate_next = self.position + delta;

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

struct GuardIter<'a> {
    guard: Option<Guard>, // TODO: make this Option to ensure the iterator is fused
    grid: &'a Grid,
}

impl<'a> Iterator for GuardIter<'a> {
    type Item = Guard;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.guard;
        self.guard = self.guard?.advance(self.grid);

        current
    }
}

/// A wrapper over a Grid on which an additional obstacle has been placed
struct ExtendedGrid<'a> {
    base_grid: &'a Grid,
    additional_obstacle: Position,
}

impl<'a> ExtendedGrid<'a> {
    fn iter_collisions(&'a self, collision: Collision) -> CollisionsIter<'a> {
        CollisionsIter {
            extended_grid: &self,
            collision: Some(collision),
        }
    }

    fn next_collision(&self, collision: Collision) -> Option<Collision> {
        let guard = Guard::from(collision);
        let base_result = self.base_grid.next_collision(collision);

        // Guard cannot possibly bump into the additional obstacle so we can
        // return the base result as is
        if !self.may_collision_with_additional(guard) {
            return base_result;
        }

        Some(
            base_result
                // If the base result is NOT between the guard and the
                // additional obstacle then the guard will bump into the
                // additional obstacle first
                .filter(|next_collision| {
                    next_collision
                        .position
                        .is_between(&guard.position, &self.additional_obstacle)
                })
                // If the guard would not normally bump into any obstacles, or
                // it has been filtered out in the previous step, then the
                // collision is with the additional obstacle
                .unwrap_or(Collision {
                    position: self.additional_obstacle,
                    face: -guard.facing,
                }),
        )
    }

    /// Whether the given guard may end up bumping into the additional obstacle.
    /// This does not mean it necessarily will, because it might be stopped by
    /// an earlier obstacle.
    fn may_collision_with_additional(&self, guard: Guard) -> bool {
        let step = Position::from(guard.facing);
        let delta = self.additional_obstacle - guard.position;
        delta.is_parallel(&step) // guard and obstacle are in the same axis
            && delta.dot(&step) > 0 // guard is facing towards the obstacle, not away from it
    }
}

#[derive(Clone)]
struct CollisionsIter<'a> {
    extended_grid: &'a ExtendedGrid<'a>,
    collision: Option<Collision>,
}

impl<'a> Iterator for CollisionsIter<'a> {
    type Item = Collision;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.extended_grid.next_collision(self.collision?);
        self.collision = next;
        next
    }
}

/// Represents a collision with an obstacle. Field position is the position of
/// the obstacle the guard collided with, and face is the face of the obstacle
/// on which the collision takes place.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Collision {
    position: Position,
    face: Direction,
}

impl From<Collision> for Guard {
    fn from(collision: Collision) -> Self {
        Self {
            position: collision.position + Position::from(collision.face),
            facing: collision.face.rotate(Orientation::Counterclockwise),
        }
    }
}

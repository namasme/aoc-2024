use std::fs;

use aoc_2024::day15;
use aoc_2024::spatial::Direction;

fn main() {
    let input = fs::read_to_string("data/day15/input").unwrap();
    let (warehouse, moves) = day15::parse_input(&input);
    let mut wide_warehouse = WideWarehouse::from(warehouse);
    wide_warehouse.apply(&moves);
    let gps_sum = wide_warehouse.gps_sum();
    println!("{}", gps_sum);
}

struct WideWarehouse(day15::Warehouse);
#[derive(Clone, Copy, Eq, PartialEq)]
struct WideBox(day15::Position);

impl WideWarehouse {
    fn apply(&mut self, moves: &[day15::Move]) {
        for move_ in moves {
            self.apply_move(*move_);
        }
    }

    fn gps_sum(&self) -> u64 {
        self.0.boxes.iter().map(|box_| self.gps(*box_)).sum()
    }

    fn gps(&self, box_: day15::Position) -> u64 {
        let top = self.0.height as u64 - box_.y as u64;
        // 2 + because the left edge is now two #'s wide
        let left = 2 + (box_.x as u64);
        100 * top + left
    }

    fn apply_move(&mut self, move_: day15::Move) {
        if move_ == day15::Move::Right || move_ == day15::Move::Left {
            self.apply_horizontal_move(move_);
        } else {
            self.apply_vertical_move(move_);
        }
    }

    fn apply_horizontal_move(&mut self, move_: day15::Move) {
        let delta = day15::Position::from(move_);
        let mut current = self.0.robot + delta;

        while self.0.is_valid(current) && self.get_box_at(current).is_some() {
            // We can fast-forward because boxes are wide
            current = current + delta * 2;
        }

        if self.is_blocking(current) {
            // We bumped into a wall, so we don't move
            return;
        }

        // We found an empty spot so boxes can be pushed
        // Now we need to trace back our steps and move the boxes one by one
        current = current - delta;
        while current != self.0.robot {
            // If there was a box here
            if self.0.boxes.remove(&current) {
                self.0.boxes.insert(current + delta); // It needs to be pushed
            }

            current = current - delta;
        }

        self.0.robot = self.0.robot + delta;
    }

    fn apply_vertical_move(&mut self, move_: day15::Move) {
        let delta = day15::Position::from(move_);
        // For this kind of move we need to consider a _frontier_, which is a
        // set of points in a horizontal line (possibly with holes).
        // This represents the contact points between boxes and/or walls from a push in the given direction.
        // E.g. in this situation:
        //
        // .[]..[].
        // ..[][]..
        // ...[]...
        // ....@...
        //
        // the frontier will first include the robot, then the two box tiles
        // above it, then the four box tiles above it, and finally the last four box tiles.
        //
        // In a more complicated scenario, something like this may occur (pushing upwards):
        //
        // ........    ...[]...    ...[]...
        // .#.[]...    .#[][]..    .#[][]..
        // ..[][]..    .[]..[].    .[]..[].
        // .[]..[]. -> ..[][].. -> ..[][]..
        // ..[][]..    ...[]...    ...[]...
        // ...[]...    ....@...    ....@...
        // ....@...    ........    ........
        //
        //  In the first move, the frontier will move along the border until it gets to the topmost box.
        //  In the second move, the frontier will also move along but will hit a wall on the leftmost box,
        //  thus stopping the push.
        //
        let mut frontier = vec![self.0.robot];
        let mut boxes_to_push = vec![];

        // The frontier being empty represents we haven't found any more boxes
        // to push or walls to block, so we can proceed to push the accumulated ones.
        while !frontier.is_empty() {
            // Naïvely move forward at every point of the current frontier
            let new_frontier: Vec<_> = frontier.iter().map(|&position| position + delta).collect();

            // If we bump into something, we stop
            if new_frontier
                .iter()
                .any(|&position| self.is_blocking(position))
            {
                return;
            }

            // Otherwise let's figure out which boxes will be pushed
            let frontier_boxes = self.get_boxes_at(&new_frontier);

            frontier = frontier_boxes
                .iter()
                .flat_map(|box_| box_.tiles())
                .collect();
            // Keep track of the boxes we'll need to push in the end
            boxes_to_push.extend(frontier_boxes);
        }

        // Process in reverse so we don't remove a box that we just pushed
        for box_ in boxes_to_push.into_iter().rev() {
            self.0.boxes.remove(&box_.left_edge());
            self.0.boxes.insert(box_.left_edge() + delta);
        }

        self.0.robot = self.0.robot + delta;
    }

    /// Finds the boxes that have at least one tile in the given positions list,
    /// without duplicates an in left-to-right order.
    fn get_boxes_at(&self, positions: &[day15::Position]) -> Vec<WideBox> {
        let mut boxes = vec![];

        for position in positions {
            if let Some(box_) = self.get_box_at(*position) {
                // Did we already include that box?
                let is_duplicate = boxes
                    .last()
                    .map(|known_box| *known_box == box_)
                    .unwrap_or(false);

                if !is_duplicate {
                    boxes.push(box_);
                }
            }
        }

        boxes
    }

    /// Whether the given position blocks a movement, either
    /// because it is out-of-bounds or because there's a wall in it.
    fn is_blocking(&self, position: day15::Position) -> bool {
        !self.0.is_valid(position) || self.0.walls.contains(&position)
    }

    fn get_box_at(&self, position: day15::Position) -> Option<WideBox> {
        // The boxes are represented by the coordinates of their leftmost tile.
        // The given position may either be the left tile, so it would be a member of boxes,
        // or the right one, in which case the position to the left would be.
        if self.0.boxes.contains(&position) {
            return Some(WideBox(position));
        }

        let left = position + day15::Position::from(Direction::Left);

        if self.0.boxes.contains(&left) {
            return Some(WideBox(left));
        }

        None
    }

    fn to_wide_coordinates(position: day15::Position) -> day15::Position {
        day15::Position::new(2 * position.x, position.y)
    }
}

impl WideBox {
    fn left_edge(&self) -> day15::Position {
        self.0
    }

    fn tiles(&self) -> [day15::Position; 2] {
        [self.0, self.0 + day15::Position::from(Direction::Right)]
    }
}

impl From<day15::Warehouse> for WideWarehouse {
    fn from(warehouse: day15::Warehouse) -> Self {
        Self(day15::Warehouse {
            width: 2 * warehouse.width,
            height: warehouse.height,
            robot: WideWarehouse::to_wide_coordinates(warehouse.robot),
            walls: warehouse
                .walls
                .into_iter()
                // This is a bit ugly but we just need the same functionality so whatever
                .flat_map(|wall| WideBox(WideWarehouse::to_wide_coordinates(wall)).tiles())
                .collect(),
            boxes: warehouse
                .boxes
                .into_iter()
                .map(WideWarehouse::to_wide_coordinates)
                .collect(),
        })
    }
}

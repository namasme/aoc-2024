use std::collections::HashSet;
use std::str::FromStr;

use crate::parse::TextGrid;
use crate::spatial::{Direction, Orientation, Point2D};

pub struct Farm {
    text_grid: TextGrid,
}

type Position = Point2D<Coordinate>;
type Coordinate = i16;

impl Farm {
    pub fn total_fence_price(&self) -> u64 {
        self.as_regions().iter().map(Region::fence_price).sum()
    }

    pub fn total_bulk_discount_price(&self) -> u64 {
        self.as_regions()
            .iter()
            .map(Region::bulk_discount_price)
            .sum()
    }

    fn as_regions(&self) -> Vec<Region> {
        let mut known: HashSet<Position> = HashSet::new();
        let mut regions = vec![];

        for row in 0..self.text_grid.height {
            for column in 0..self.text_grid.width {
                let position = Position::new(row as Coordinate, column as Coordinate);

                // Already part of a region, no need to explore
                if known.contains(&position) {
                    continue;
                }

                let region = self.as_region(position);

                known.extend(region.plots.iter().cloned());
                regions.push(region);
            }
        }

        regions
    }

    /// DFS from start position to explore the entire contiguous region
    fn as_region(&self, start: Position) -> Region {
        let plant_label = self.text_grid.char_at(start).unwrap();
        let mut pending = vec![start];
        let mut plots = HashSet::from([start]);
        let mut seen = HashSet::from([start]);

        while let Some(current) = pending.pop() {
            let neighbours: Vec<_> = current
                .neighbours()
                .into_iter()
                .filter(|neighbour| self.is_valid(neighbour))
                .filter(|neighbour| !seen.contains(neighbour))
                .filter(|neighbour| self.is_in_region(neighbour, plant_label))
                .collect();

            pending.extend(neighbours.iter().cloned());
            plots.extend(neighbours.iter().cloned());
            seen.extend(neighbours.into_iter());
        }

        Region { plots }
    }

    fn is_in_region(&self, position: &Position, label: char) -> bool {
        self.text_grid
            .char_at(*position)
            .map(|c| c == label)
            .unwrap_or(false)
    }

    fn is_valid(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < (self.text_grid.width as Coordinate)
            && position.y >= 0
            && position.y < (self.text_grid.height as Coordinate)
    }
}

struct Region {
    plots: HashSet<Position>,
}

impl Region {
    fn fence_price(&self) -> u64 {
        self.perimeter() * self.area()
    }

    fn bulk_discount_price(&self) -> u64 {
        self.sides() * self.area()
    }

    fn perimeter(&self) -> u64 {
        self.plots
            .iter()
            .map(|plot| {
                plot.neighbours()
                    .into_iter()
                    .filter(|neighbour| !self.plots.contains(neighbour))
                    .count() as u64
            })
            .sum()
    }

    fn area(&self) -> u64 {
        self.plots.len() as u64
    }

    fn sides(&self) -> u64 {
        // Regions are not solid and so may have multiple borders inside.
        // For each of those we need to count the number of sides, and add them together.
        self.borders().iter().map(|border| border.sides()).sum()
    }

    fn borders(&self) -> Vec<Border> {
        let mut borders: Vec<Border> = vec![];

        for plot in self.plots.iter() {
            for direction in Direction::all() {
                let boundary_position = BoundaryPosition {
                    plot: *plot,
                    normal: direction,
                };

                // Cannot check before the loop even though the condition does
                // not depend on direction because borders might have new items
                // from the previous iteration
                if borders
                    .iter()
                    .any(|border| border.contains(&boundary_position))
                {
                    // We have already seen this border, skip neighbour
                    continue;
                }

                let neighbour = *plot + Position::from(direction);
                if self.plots.contains(&neighbour) {
                    // Neighbour is also in the region, so we are not in the boundary
                    continue;
                }

                let border = self.complete_border(boundary_position);
                borders.push(border);
            }
        }

        borders
    }

    /// Given a point in the boundary of a region, collect all the vertices in that border
    fn complete_border(&self, boundary_position: BoundaryPosition) -> Border {
        // Move to the next start point since we might be starting from the middle of a side
        let initial_side_start = self.next_side(boundary_position);

        let mut vertices = vec![initial_side_start];
        let mut side_start = initial_side_start;

        loop {
            // Move to the next side
            side_start = self.next_side(side_start);

            if side_start == initial_side_start {
                // We already saw this side in the beginning, so we have cycled the entire border
                break;
            } else {
                vertices.push(side_start);
            }
        }

        Border { vertices }
    }

    /// Given a point in the boundary of a region, move to the next vertex in that border
    fn next_side(&self, boundary_position: BoundaryPosition) -> BoundaryPosition {
        let mut is_member_inside = true;
        let mut is_member_outside = false;
        let mut current = boundary_position;
        let delta = Position::from(boundary_position.normal.rotate(Orientation::Clockwise));

        while is_member_inside && !is_member_outside {
            current.plot = current.plot + delta;
            is_member_inside = self.plots.contains(&current.plot);
            is_member_outside = self.plots.contains(&(current.outside()));
        }

        if !is_member_inside {
            // We have a situation like this:
            //
            //               <
            // ### -> <## -> ###
            // <      #      #
            //
            // So the next start should be:
            //
            // ^##
            // #
            BoundaryPosition {
                plot: current.plot - delta,
                normal: boundary_position.normal.rotate(Orientation::Clockwise),
            }
        } else {
            // The inside plot is a member of the region, so it must be the outside one does too.
            // We have a situation like this:
            //
            //   #      #      #
            //   # ->   # ->   #
            // ^##    #^#    ##^
            //
            // So the next start should be:
            //
            //   #
            //   #
            // ##<
            //
            // This does not actually verify the BoundaryPosition invariant,
            // but the next visited position does.
            BoundaryPosition {
                plot: current.plot,
                normal: boundary_position
                    .normal
                    .rotate(Orientation::Counterclockwise),
            }
        }
    }
}

struct Border {
    vertices: Vec<BoundaryPosition>,
}

impl Border {
    fn sides(&self) -> u64 {
        self.vertices.len() as u64
    }

    fn contains(&self, boundary_position: &BoundaryPosition) -> bool {
        // Loop through the vertices pairwise, and check if the given position is part of the resulting side

        let sides = self
            .vertices
            .iter()
            .cycle()
            .zip(self.vertices.iter().cycle().skip(1));

        sides
            .take(self.vertices.len()) // sides is an infinite iterator
            .any(|(side_start, side_end)| {
                boundary_position
                    .plot
                    .is_between(&side_start.plot, &side_end.plot)
                    && side_start.normal == boundary_position.normal
            })
    }
}

/// Represents a point in a border's boundary
/// It includes a point inside the region, and the direction
/// of the normal vector at that point.
/// It should verify that the point inside plus the normal vector
/// is NOT a member of the border.
#[derive(Clone, Copy, Eq, PartialEq)]
struct BoundaryPosition {
    plot: Position,
    normal: Direction,
}

impl BoundaryPosition {
    fn outside(&self) -> Position {
        self.plot + Position::from(self.normal)
    }
}

impl FromStr for Farm {
    type Err = <TextGrid as FromStr>::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Reverse lines so directions work as expected with coordinates
        let reversed_lines = input.lines().rev().collect::<Vec<_>>().join("\n");
        Ok(Farm {
            text_grid: reversed_lines.parse()?,
        })
    }
}

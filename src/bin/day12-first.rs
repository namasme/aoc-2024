use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use aoc_2024::parse::TextGrid;
use aoc_2024::spatial::Point2D;
use aoc_2024::spatial::Point2DCast;

fn main() {
    let input = fs::read_to_string("data/day12/input").unwrap();
    let farm: Farm = input.parse().unwrap();
    println!("{}", farm.total_fence_price());
}

struct Farm {
    text_grid: TextGrid,
}

type Position = Point2D<Coordinate>;
type Coordinate = i16;

impl Farm {
    fn total_fence_price(&self) -> u64 {
        self.as_regions().iter().map(Region::fence_price).sum()
    }

    fn as_regions(&self) -> Vec<Region> {
        let mut known: HashSet<Position> = HashSet::new();
        let mut regions = vec![];

        for row in 0..self.text_grid.height {
            for column in 0..self.text_grid.width {
                let position = Position::new(row as Coordinate, column as Coordinate);

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

    fn as_region(&self, start: Position) -> Region {
        let plant_label = self.text_grid.char_at(start.cast().unwrap()).unwrap();
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
        position
            .cast()
            .ok()
            .and_then(|position| self.text_grid.char_at(position))
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

#[derive(Debug)]
struct Region {
    plots: HashSet<Position>,
}

impl Region {
    fn fence_price(&self) -> u64 {
        self.perimeter() * self.area()
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
}

impl FromStr for Farm {
    type Err = <TextGrid as FromStr>::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Farm {
            text_grid: input.parse()?,
        })
    }
}

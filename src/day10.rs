use crate::spatial::Point2D;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

type Coordinate = i16;
type Position = Point2D<Coordinate>;
type Level = u8;

pub struct Map {
    rows: usize,
    columns: usize,
    levels: HashMap<Position, Level>,
}

impl Map {
    pub fn total_score(&self) -> usize {
        self.levels
            .iter()
            .map(|(start, _)| self.trailhead_score(start))
            .sum()
    }

    pub fn total_rating(&self) -> usize {
        self.levels
            .iter()
            .map(|(start, _)| self.trailhead_rating(start))
            .sum()
    }

    fn trailhead_score(&self, start: &Position) -> usize {
        if self.levels[start] != 0 {
            return 0;
        }

        let mut heads = vec![(0, *start)];
        let mut visited: HashSet<Position> = HashSet::new();
        let mut nines = 0;

        while let Some((level, position)) = heads.pop() {
            if level == 9 {
                nines += 1;
                continue;
            }

            let to_visit: Vec<_> = position
                .neighbours()
                .into_iter()
                .filter(|neighbour| {
                    self.is_valid(neighbour)
                        && self.levels[neighbour] == level + 1
                        && !visited.contains(neighbour)
                })
                .collect();

            heads.extend(
                to_visit
                    .iter()
                    .cloned()
                    .map(|neighbour| (level + 1, neighbour)),
            );
            visited.extend(to_visit);
        }

        nines
    }

    fn trailhead_rating(&self, start: &Position) -> usize {
        if self.levels[start] != 0 {
            return 0;
        }

        let mut heads = vec![(0, *start)];
        let mut nines = 0;

        while let Some((level, position)) = heads.pop() {
            if level == 9 {
                nines += 1;
                continue;
            }

            let to_visit: Vec<_> = position
                .neighbours()
                .into_iter()
                .filter(|neighbour| self.is_valid(neighbour) && self.levels[neighbour] == level + 1)
                .collect();

            heads.extend(
                to_visit
                    .iter()
                    .cloned()
                    .map(|neighbour| (level + 1, neighbour)),
            );
        }

        nines
    }

    fn is_valid(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.columns as Coordinate
            && position.y >= 0
            && position.y < self.rows as Coordinate
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input.lines().count();
        let columns = input.find('\n').unwrap();
        let levels = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    Some((
                        Position::new(x as Coordinate, y as Coordinate),
                        c.to_digit(10)? as Level,
                    ))
                })
            })
            .collect();

        Ok(Map {
            rows,
            columns,
            levels,
        })
    }
}

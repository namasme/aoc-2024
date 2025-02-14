use std::collections::HashMap;
use std::iter::once;
use std::str::FromStr;

use crate::parse::TextGrid;
use crate::spatial::Point2D;

pub struct Racetrack {
    track: TextGrid,
    start: Position,
    end: Position,
}

type Position = Point2D<Coordinate>;
type Coordinate = usize;

impl Racetrack {
    fn from(track: TextGrid) -> Self {
        let mut start = None;
        let mut end = None;

        for (position, character) in track.iter() {
            match character {
                'S' => start = Some(position),
                'E' => end = Some(position),
                _ => {}
            }
        }

        Racetrack {
            track,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    pub fn count_top_improvements(&self, radius: Coordinate, lower_bound: Coordinate) -> u64 {
        let reverse_index = self.reverse_index();

        self.iter()
            .map(|step| {
                self.improvements(&reverse_index, step, radius)
                    .into_iter()
                    .filter(|&(_, improvement)| improvement >= lower_bound)
                    .count()
            })
            .sum::<usize>() as u64
    }

    fn improvements(
        &self,
        reverse_index: &HashMap<Position, usize>,
        position: Position,
        radius: Coordinate,
    ) -> HashMap<Position, usize> {
        position
            .l1_ball(radius)
            .into_iter()
            .filter(|&neighbour| self.is_valid(neighbour) && !self.is_wall(neighbour))
            .filter_map(|neighbour| {
                let cheat_length = position.manhattan_distance(&neighbour);
                // + cheat_length because we need to compare the gain against
                // what we would have advanced otherwise
                let baseline = reverse_index[&position] + cheat_length;
                let improvement = reverse_index.get(&neighbour)?.checked_sub(baseline)?;
                Some((neighbour, improvement))
            })
            .collect()
    }

    fn reverse_index(&self) -> HashMap<Position, usize> {
        let path = self
            .iter()
            .enumerate()
            .map(|(index, step)| (step, index + 1));

        once((self.start, 0)).chain(path).collect()
    }

    fn iter<'a>(&'a self) -> RacetrackPath {
        RacetrackPath {
            racetrack: &self,
            step: RacetrackStep {
                previous: self.start,
                current: Some(self.start),
            },
        }
    }

    fn is_wall(&self, position: Position) -> bool {
        self.track.char_at(position).unwrap() == '#'
    }

    fn is_valid(&self, position: Position) -> bool {
        position.x > 0
            && position.x < self.track.width - 1
            && position.y > 0
            && position.y < self.track.height - 1
    }
}

struct RacetrackPath<'a> {
    racetrack: &'a Racetrack,
    step: RacetrackStep,
}

#[derive(Debug, Clone, Copy)]
struct RacetrackStep {
    previous: Position,
    current: Option<Position>,
}

impl<'a> Iterator for RacetrackPath<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.step.current;

        if let Some(current) = item {
            let next = if current == self.racetrack.end {
                None
            } else {
                current
                    .neighbours()
                    .into_iter()
                    .filter(|&neighbour| {
                        self.racetrack.is_valid(neighbour)
                            && !self.racetrack.is_wall(neighbour)
                            && neighbour != self.step.previous
                    })
                    .next()
            };
            self.step.previous = current;
            self.step.current = next;
        }

        item
    }
}

impl FromStr for Racetrack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Racetrack::from(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn all_improvements(
        racetrack: &Racetrack,
        radius: Coordinate,
    ) -> HashMap<usize, Vec<(Position, Position)>> {
        let reverse_index = racetrack.reverse_index();
        let mut improvements = HashMap::new();

        for step in racetrack.iter() {
            let step_improvements = racetrack.improvements(&reverse_index, step, radius);
            for (position, improvement) in step_improvements {
                let entry = improvements.entry(improvement).or_insert_with(Vec::new);
                entry.push((step, position));
            }
        }

        improvements
    }

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("data/day20/test_input").unwrap();
        let racetrack_ = input.parse();
        assert!(racetrack_.is_ok());

        let racetrack: Racetrack = racetrack_.unwrap();
        assert_eq!(15, racetrack.track.width);
        assert_eq!(15, racetrack.track.height);
        assert_eq!(Position::new(1, 3), racetrack.start);
        assert_eq!(Position::new(5, 7), racetrack.end);
    }

    #[test]
    fn test_path_iterator() {
        let input = fs::read_to_string("data/day20/test_input").unwrap();
        let racetrack: Racetrack = input.parse().unwrap();
        let steps = racetrack.iter().collect::<Vec<_>>();

        assert_eq!(85, steps.len());
        assert_eq!(racetrack.start, steps[0]);
        assert_eq!(racetrack.end, steps[steps.len() - 1]);
    }

    #[test]
    fn test_count_top_improvements() {
        let input = fs::read_to_string("data/day20/test_input").unwrap();
        let racetrack: Racetrack = input.parse().unwrap();

        assert_eq!(10, racetrack.count_top_improvements(2, 10));
    }

    #[test]
    fn test_all_improvements_part_1() {
        let input = fs::read_to_string("data/day20/test_input").unwrap();
        let racetrack: Racetrack = input.parse().unwrap();
        let all_improvements = all_improvements(&racetrack, 2);
        let expected_counts = HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);

        for (improvement, expected_cheats) in expected_counts {
            assert_eq!(
                expected_cheats,
                all_improvements[&improvement].len(),
                "expected {} cheats to improve by {}",
                expected_cheats,
                improvement
            );
        }
    }

    #[test]
    fn test_all_improvements_part_2() {
        let input = fs::read_to_string("data/day20/test_input").unwrap();
        let racetrack: Racetrack = input.parse().unwrap();
        let all_improvements = all_improvements(&racetrack, 20);
        let expected_counts = HashMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]);

        for (improvement, expected_cheats) in expected_counts {
            assert_eq!(
                expected_cheats,
                all_improvements[&improvement].len(),
                "expected {} cheats to improve by {}",
                expected_cheats,
                improvement
            );
        }
    }
}

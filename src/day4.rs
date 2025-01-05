use crate::parse::TextGrid;
use crate::spatial::Point2D;
use crate::spatial::Point2DCast;
use std::iter;
use std::str::FromStr;

pub struct Puzzle {
    text_grid: TextGrid,
}

pub trait PatternSpotter {
    type Occurrence;

    fn find_pattern_occurrences(&self, text_grid: &TextGrid) -> Vec<Self::Occurrence>;
}

pub struct WordPattern {
    word: String,
}

impl WordPattern {
    pub fn new(word: String) -> Self {
        Self { word }
    }

    fn match_word(&self, text_grid: &TextGrid, start: &Position, direction: &Direction) -> bool {
        let ray = iter::successors(Some(*start), |&current| {
            Some(current + direction.as_delta())
        })
        .map(|position| {
            position
                .cast()
                .ok()
                .and_then(|position| text_grid.char_at(position))
        });

        self.word
            .chars()
            .zip(ray)
            .map(|(word_char, puzzle_char)| puzzle_char.map(|p| p == word_char).unwrap_or(false))
            .all(|matches| matches)
    }
}

impl PatternSpotter for WordPattern {
    type Occurrence = (Position, Direction);

    fn find_pattern_occurrences(&self, text_grid: &TextGrid) -> Vec<Self::Occurrence> {
        let first = self.word.as_bytes()[0] as char;
        text_grid
            .iter()
            .map(|(position, letter)| {
                if letter != first {
                    return vec![];
                }

                Direction::all()
                    .iter()
                    .filter(|direction| {
                        position
                            .cast()
                            .ok()
                            .map(|position| self.match_word(text_grid, &position, direction))
                            .unwrap_or(false)
                    })
                    .map(|direction| (position.cast().unwrap(), *direction))
                    .collect()
            })
            .flatten()
            .collect()
    }
}

pub struct CrossMASPattern {}

impl PatternSpotter for CrossMASPattern {
    type Occurrence = Point2D<Coordinate>;

    fn find_pattern_occurrences(&self, text_grid: &TextGrid) -> Vec<Self::Occurrence> {
        text_grid
            .iter()
            .filter_map(|(position, letter)| {
                if letter != 'A' {
                    return None;
                }

                if self.match_cross_mas(text_grid, position.cast().ok()?) {
                    Some(position.cast().unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl CrossMASPattern {
    pub fn new() -> Self {
        Self {}
    }

    fn match_cross_mas(&self, text_grid: &TextGrid, start: Position) -> bool {
        let corners: String = vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW]
            .iter()
            .filter_map(|direction| text_grid.char_at((start + direction.as_delta()).cast().ok()?))
            .collect();

        if corners.len() < 4 {
            return false;
        }

        match corners.as_str() {
            "MMSS" => true,
            "MSSM" => true,
            "SSMM" => true,
            "SMMS" => true,
            _ => false,
        }
    }
}

type Coordinate = i16;
type Position = Point2D<Coordinate>;

#[derive(Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn all() -> [Self; 8] {
        [
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }

    fn as_delta(&self) -> Point2D<Coordinate> {
        match self {
            Direction::N => Point2D::new(0, -1),
            Direction::NE => Point2D::new(1, -1),
            Direction::E => Point2D::new(1, 0),
            Direction::SE => Point2D::new(1, 1),
            Direction::S => Point2D::new(0, 1),
            Direction::SW => Point2D::new(-1, 1),
            Direction::W => Point2D::new(-1, 0),
            Direction::NW => Point2D::new(-1, -1),
        }
    }
}

impl Puzzle {
    pub fn find_pattern_occurrences<T: PatternSpotter>(
        &self,
        pattern_spotter: T,
    ) -> Vec<T::Occurrence> {
        pattern_spotter.find_pattern_occurrences(&self.text_grid)
    }
}

impl FromStr for Puzzle {
    type Err = <TextGrid as FromStr>::Err;

    fn from_str(input: &str) -> Result<Self, <TextGrid as FromStr>::Err> {
        Ok(Self {
            text_grid: input.parse()?,
        })
    }
}

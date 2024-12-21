use crate::spatial::Point2D;
use std::iter;

pub struct Puzzle {
    text_grid: TextGrid,
}

pub struct TextGrid {
    content: String,
    width: usize,
}

impl TextGrid {
    fn idx_to_coordinates(&self, idx: usize) -> Position {
        let column = idx % (self.width + 1);
        let row = idx / (self.width + 1);

        Point2D::new(column as Coordinate, row as Coordinate)
    }

    fn coordinates_to_index(&self, position: Position) -> Option<usize> {
        if position.x < 0 || position.y < 0 || (position.x as usize) >= self.width {
            return None;
        }

        let candidate = (self.width + 1) * (position.y as usize) + (position.x as usize);
        if candidate >= self.content.len() {
            return None;
        }

        return Some(candidate);
    }

    fn char_at(&self, position: Position) -> Option<char> {
        self.coordinates_to_index(position)
            .map(|idx| self.content.as_bytes()[idx] as char)
    }

    fn parse(input: String) -> Self {
        let width = input.find('\n').unwrap();

        Self {
            content: input,
            width,
        }
    }
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
        .map(|position| text_grid.char_at(position));

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
            .content
            .char_indices()
            .map(|(idx, letter)| {
                if letter != first {
                    return vec![];
                }

                let start = text_grid.idx_to_coordinates(idx);
                vec![
                    Direction::N,
                    Direction::NE,
                    Direction::E,
                    Direction::SE,
                    Direction::S,
                    Direction::SW,
                    Direction::W,
                    Direction::NW,
                ]
                .iter()
                .filter(|direction| self.match_word(text_grid, &start, direction))
                .map(|direction| (start, *direction))
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
            .content
            .char_indices()
            .filter_map(|(idx, letter)| {
                if letter != 'A' {
                    return None;
                }

                let start = text_grid.idx_to_coordinates(idx);

                if self.match_cross_mas(text_grid, start) {
                    Some(start)
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
        let ne = text_grid.char_at(start + Direction::NE.as_delta());
        let se = text_grid.char_at(start + Direction::SE.as_delta());
        let sw = text_grid.char_at(start + Direction::SW.as_delta());
        let nw = text_grid.char_at(start + Direction::NW.as_delta());

        let corners: String = vec![ne, se, sw, nw]
            .iter()
            .filter_map(Option::as_ref)
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

    pub fn parse(input: String) -> Self {
        Self {
            text_grid: TextGrid::parse(input),
        }
    }
}

use aoc_2024::spatial::Point2D;
use std::fs;
use std::iter;

fn main() {
    let input = fs::read_to_string("data/day4/input").unwrap();
    let puzzle = Puzzle::parse(input);
    let occurrences = puzzle.find_word_occurrences("XMAS");
    println!("{}", occurrences.len());
}

struct Puzzle {
    content: String,
    width: usize,
}

type Coordinate = i16;
type Position = Point2D<Coordinate>;

#[derive(Clone, Copy, Debug)]
enum Direction {
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

type Occurrence = (Position, Direction);

impl Puzzle {
    fn find_word_occurrences(&self, word: &str) -> Vec<Occurrence> {
        let first = word.as_bytes()[0] as char;
        self.content
            .char_indices()
            .map(|(idx, letter)| {
                if letter != first {
                    return vec![];
                }

                let start = self.idx_to_coordinates(idx);
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
                .filter(|direction| self.match_word(word, &start, direction))
                .map(|direction| (start, *direction))
                .collect()
            })
            .flatten()
            .collect()
    }

    fn match_word(&self, word: &str, start: &Position, direction: &Direction) -> bool {
        let ray = iter::successors(Some(*start), |&current| {
            Some(current + direction.as_delta())
        })
        .map(|position| self.char_at(position));

        word.chars()
            .zip(ray)
            .map(|(word_char, puzzle_char)| puzzle_char.map(|p| p == word_char).unwrap_or(false))
            .all(|matches| matches)
    }

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

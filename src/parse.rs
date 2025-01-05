use crate::spatial::Point2D;
use std::str::FromStr;

pub struct TextGrid {
    content: String,
    width: usize,
    height: usize,
}

impl TextGrid {
    pub fn char_at(&self, position: Point2D<usize>) -> Option<char> {
        self.coordinates_to_index(position)
            .map(|idx| self.content.as_bytes()[idx] as char)
    }

    pub fn iter(&self) -> TextGridIter<'_> {
        TextGridIter {
            text_grid: self,
            idx: 0,
        }
    }

    fn index_to_coordinates(&self, idx: usize) -> Point2D<usize> {
        // Adding one because of the trailing newline character
        let column = idx % (self.width + 1);
        let row = idx / (self.width + 1);

        Point2D::new(column as usize, row as usize)
    }

    fn coordinates_to_index(&self, position: Point2D<usize>) -> Option<usize> {
        if (position.x as usize) >= self.width || (position.y as usize) >= self.height {
            return None;
        }

        let candidate = (self.width + 1) * (position.y as usize) + (position.x as usize);

        Some(candidate)
    }
}

impl FromStr for TextGrid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').unwrap();
        let height = input.lines().count();

        Ok(Self {
            content: input.to_string(),
            width,
            height,
        })
    }
}

pub struct TextGridIter<'a> {
    text_grid: &'a TextGrid,
    idx: usize,
}

impl<'a> Iterator for TextGridIter<'a> {
    type Item = (Point2D<usize>, char);

    fn next(&mut self) -> Option<Self::Item> {
        let chars = self.text_grid.content.as_bytes();

        // Skip newline prefix
        while self.idx < chars.len() && chars[self.idx] == b'\n' {
            self.idx += 1;
        }

        if self.idx >= chars.len() {
            return None;
        }

        let current_idx = self.idx;
        self.idx += 1;

        Some((
            self.text_grid.index_to_coordinates(current_idx),
            chars[current_idx] as char,
        ))
    }
}

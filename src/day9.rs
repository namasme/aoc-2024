use crate::common::Range;
use std::cmp::min;
use std::str::FromStr;

pub struct Disk {
    files: Vec<File>,
}

impl Disk {
    pub fn compute_whole_checksum(&self) -> usize {
        let mut gaps = self.gaps();
        let mut checksum = 0;

        for file in self.files.iter().rev() {
            let suitable_gap = gaps
                .iter_mut()
                .find(|gap| gap.0.size >= file.size() && gap.0.start < file.location.start);

            if let Some(gap) = suitable_gap {
                checksum += file.partial_checksum(Range {
                    start: gap.0.start,
                    size: file.size(),
                });
                *gap = gap.claim(file.size());
            } else {
                checksum += file.partial_checksum(Range {
                    start: file.location.start,
                    size: file.size(),
                });
            }
        }

        checksum
    }

    pub fn compute_checksum(&self) -> usize {
        let mut gaps = self.gaps();
        gaps.reverse();
        let mut checksum = 0;

        for file in self.files.iter().rev() {
            let mut total_moved = 0;

            while total_moved < file.size() && !gaps.is_empty() {
                if gaps.last().unwrap().0.start > file.location.start {
                    // The gap is actually to the right of the file original location,
                    // so we can't keep moving blocks
                    break;
                }

                let gap = gaps.pop().unwrap();

                let to_move = min(file.size() - total_moved, gap.0.size);
                checksum += file.partial_checksum(Range {
                    start: gap.0.start,
                    size: to_move,
                });
                total_moved += to_move;

                if gap.0.size > to_move {
                    gaps.push(gap.claim(to_move));
                }
            }

            if total_moved < file.size() {
                checksum += file.partial_checksum(Range {
                    start: file.location.start,
                    size: file.size() - total_moved,
                });
            }
        }

        checksum
    }

    fn gaps(&self) -> Vec<Gap> {
        self.files
            .iter()
            .filter_map(|file| {
                Some(Gap(Range {
                    start: file.location.end(),
                    size: file.gap_size?,
                }))
            })
            .collect()
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut files = vec![];
        let mut current_start = 0;
        let digits: Vec<_> = input
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|digit| Ok(digit as usize))
                    .unwrap_or(Err(()))
            })
            .collect();
        let with_gaps = digits.chunks_exact(2);

        for (id, file_gap) in with_gaps.enumerate() {
            files.push(File {
                id,
                location: Range {
                    start: current_start,
                    size: file_gap[0]?,
                },
                gap_size: Some(file_gap[1]?),
            });

            current_start += file_gap[0]? + file_gap[1]?;
        }

        if digits.len() != 2 * files.len() + 1 {
            return Err(()); // there should be a trailing file
        }

        // Parse the last file, which doesn't have a gap
        let last_file_size = digits[digits.len() - 1];
        let id = files.len();
        files.push(File {
            id,
            location: Range {
                start: current_start,
                size: last_file_size?,
            },
            gap_size: None,
        });

        Ok(Disk { files })
    }
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    location: Range,
    gap_size: Option<usize>,
}

impl File {
    fn partial_checksum(&self, range: Range) -> usize {
        if range.size == 0 {
            return 0;
        }

        self.id * (range.start * range.size + ((range.size - 1) * range.size) / 2)
    }

    fn size(&self) -> usize {
        self.location.size
    }
}

#[derive(Debug, Clone, Copy)]
struct Gap(Range);

impl Gap {
    fn claim(&self, space: usize) -> Self {
        Self(Range {
            start: self.0.start + space,
            size: self.0.size - space,
        })
    }
}

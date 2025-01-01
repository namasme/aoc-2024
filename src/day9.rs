use std::ops::Range;
use std::str::FromStr;

pub type DiskRange = Range<usize>;
pub trait FileCompactor {
    fn checksum(&self) -> usize;
    fn compact_file(&self, file: &File, gaps: &mut Vec<Gap>) -> Vec<DiskRange>;
}

pub struct Disk {
    pub files: Vec<File>,
}

impl Disk {
    pub fn gaps(&self) -> Vec<Gap> {
        self.files
            .windows(2)
            .map(|files| Gap(files[0].location.end..files[1].location.start))
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
                location: (current_start..current_start + file_gap[0]?),
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
            location: current_start..current_start + last_file_size?,
        });

        Ok(Disk { files })
    }
}

#[derive(Clone)]
pub struct File {
    id: usize,
    pub location: DiskRange,
}

impl File {
    pub fn partial_checksum(&self, range: &DiskRange) -> usize {
        if range.len() == 0 {
            return 0;
        }

        self.id * (range.start * range.len() + ((range.len() - 1) * range.len()) / 2)
    }

    pub fn size(&self) -> usize {
        self.location.len()
    }
}

#[derive(Debug, Clone)]
pub struct Gap(pub DiskRange);

impl Gap {
    pub fn claim(&self, space: usize) -> Self {
        Self(self.0.start + space..self.0.end)
    }
}

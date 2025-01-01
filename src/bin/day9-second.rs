use aoc_2024::day9;
use aoc_2024::day9::FileCompactor;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day9/input").unwrap();
    let disk: day9::Disk = input.parse().unwrap();
    let compactor = WholeFileCompactor { disk };
    let checksum = compactor.checksum();
    println!("{}", checksum);
}

struct WholeFileCompactor {
    disk: day9::Disk,
}

impl day9::FileCompactor for WholeFileCompactor {
    fn checksum(&self) -> usize {
        let mut gaps = self.disk.gaps();
        self.disk
            .files
            .iter()
            .rev()
            .map(|file| {
                self.compact_file(file, &mut gaps)
                    .iter()
                    .map(|range| file.partial_checksum(range))
                    .sum::<usize>()
            })
            .sum()
    }

    fn compact_file(&self, file: &day9::File, gaps: &mut Vec<day9::Gap>) -> Vec<day9::DiskRange> {
        let mut ranges = vec![];
        let suitable_gap = gaps
            .iter_mut()
            .find(|gap| gap.0.len() >= file.size() && gap.0.start < file.location.start);

        if let Some(gap) = suitable_gap {
            ranges.push(gap.0.start..gap.0.start + file.size());
            *gap = gap.claim(file.size());
        } else {
            ranges.push(file.location.clone());
        }

        ranges
    }
}

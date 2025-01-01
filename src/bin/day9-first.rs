use aoc_2024::day9;
use aoc_2024::day9::FileCompactor;
use std::cmp::min;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day9/input").unwrap();
    let disk: day9::Disk = input.parse().unwrap();
    let compactor = BlockFileCompactor { disk };
    let checksum = compactor.checksum();
    println!("{}", checksum);
}

struct BlockFileCompactor {
    disk: day9::Disk,
}

impl day9::FileCompactor for BlockFileCompactor {
    fn checksum(&self) -> usize {
        let mut gaps: Vec<_> = self.disk.gaps().into_iter().rev().collect();
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
        let mut total_moved = 0;

        while total_moved < file.size() && !gaps.is_empty() {
            if gaps.last().unwrap().0.start > file.location.start {
                // The gap is actually to the right of the file original location,
                // so we can't keep moving blocks
                break;
            }

            let gap = gaps.pop().unwrap();

            let to_move = min(file.size() - total_moved, gap.0.len());
            ranges.push(gap.0.start..gap.0.start + to_move);
            total_moved += to_move;

            if gap.0.len() > to_move {
                gaps.push(gap.claim(to_move));
            }
        }

        if total_moved < file.size() {
            ranges.push(file.location.start..file.location.start + file.size() - total_moved);
        }

        ranges
    }
}

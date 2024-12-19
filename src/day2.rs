use std::cmp::Ordering;

type Level = isize;
pub struct Report {
    levels: Vec<Level>,
}

impl Report {
    pub fn is_safe_with_dampener(&self) -> bool {
        if Report::infer_ordering_slice(&self.levels[1..])
            .map(|expected_ordering| Report::is_safe_slice(&self.levels[1..], expected_ordering))
            .unwrap_or(false)
        {
            return true;
        }

        for skip_idx in 1..self.levels.len() - 1 {
            let prefix = &self.levels[..skip_idx];
            let suffix = &self.levels[skip_idx + 1..];
            let is_safe = Report::infer_ordering_slice(prefix)
                .or(Report::infer_ordering_slice(suffix))
                .map(|expected_ordering| {
                    let prefix_safe = Report::is_safe_slice(prefix, expected_ordering);
                    let suffix_safe = Report::is_safe_slice(suffix, expected_ordering);

                    prefix_safe
                        && Report::is_safe_step(
                            &prefix[skip_idx - 1],
                            &suffix[0],
                            expected_ordering,
                        )
                        && suffix_safe
                })
                .unwrap_or(false);

            if is_safe {
                return true;
            }
        }

        Report::infer_ordering_slice(&self.levels[..self.levels.len() - 1])
            .map(|expected_ordering| {
                Report::is_safe_slice(&self.levels[..self.levels.len() - 1], expected_ordering)
            })
            .unwrap_or(false)
    }

    fn is_safe_slice(slice: &[Level], expected_ordering: Ordering) -> bool {
        slice
            .windows(2)
            .map(|window| Report::is_safe_step(&window[0], &window[1], expected_ordering))
            .all(|x| x)
    }

    fn infer_ordering_slice(levels: &[Level]) -> Option<Ordering> {
        if levels.len() < 2 {
            return None;
        }

        match levels[0].cmp(&levels[1]) {
            Ordering::Equal => None,
            ordering => Some(ordering),
        }
    }

    pub fn is_safe_step(current: &Level, next: &Level, expected_ordering: Ordering) -> bool {
        current.cmp(next) == expected_ordering && (current - next).abs() <= 3
    }

    pub fn is_safe(&self) -> bool {
        Report::infer_ordering_slice(&self.levels)
            .map(|expected_ordering| Report::is_safe_slice(&self.levels, expected_ordering))
            .unwrap_or(false)
    }

    fn from_str(line: &str) -> Report {
        let levels = line
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();
        Report { levels }
    }
}

pub fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(Report::from_str).collect()
}

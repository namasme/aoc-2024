use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day2/input").unwrap();
    let reports = parse_input(&input);
    let safe_reports_count = reports.iter().filter(|report| report.is_safe()).count();
    println!("{}", safe_reports_count);
}

type Level = isize;
struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let expected_order = self.levels[0].cmp(&self.levels[1]);
        if expected_order == Ordering::Equal {
            return false;
        }

        self.levels
            .windows(2)
            .map(|window| {
                window[0].cmp(&window[1]) == expected_order && (window[0] - window[1]).abs() <= 3
            })
            .all(|x| x)
    }

    fn from_str(line: &str) -> Report {
        let levels = line
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();
        Report { levels }
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(Report::from_str).collect()
}

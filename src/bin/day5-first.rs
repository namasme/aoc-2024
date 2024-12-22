use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day5/input").unwrap();
    let (rules, updates) = parse_input(&input);
    let checksum: u64 = updates
        .iter()
        .filter(|update| update.is_valid(&rules))
        .map(PageUpdate::middle_page)
        .sum();

    println!("{}", checksum);
}

fn parse_input(input: &str) -> (HashSet<OrderingRule>, Vec<PageUpdate>) {
    let (rules_block, updates_block) = input.split_once("\n\n").unwrap();

    let rules = rules_block.lines().map(OrderingRule::parse).collect();
    let updates = updates_block.lines().map(PageUpdate::parse).collect();

    (rules, updates)
}

type Page = u8;

#[derive(Eq, Hash, PartialEq)]
struct OrderingRule {
    before: Page,
    after: Page,
}

impl OrderingRule {
    fn parse(line: &str) -> Self {
        let (before, after) = line.split_once('|').unwrap();

        OrderingRule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        }
    }
}

struct PageUpdate {
    pages: Vec<Page>,
}

impl PageUpdate {
    fn middle_page(&self) -> u64 {
        self.pages[self.pages.len() / 2] as u64
    }

    fn is_valid(&self, rules: &HashSet<OrderingRule>) -> bool {
        !self.pages.iter().enumerate().any(|(idx, page)| {
            self.pages
                .iter()
                .take(idx)
                .any(|previous_page| PageUpdate::goes_before(rules, *page, *previous_page))
        })
    }

    fn goes_before(rules: &HashSet<OrderingRule>, page: Page, candidate: Page) -> bool {
        rules.contains(&OrderingRule {
            before: page,
            after: candidate,
        })
    }

    fn parse(line: &str) -> Self {
        PageUpdate {
            pages: line.split(',').map(|page| page.parse().unwrap()).collect(),
        }
    }
}

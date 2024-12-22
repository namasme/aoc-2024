use std::collections::HashSet;

pub fn parse_input(input: &str) -> (HashSet<OrderingRule>, Vec<PageUpdate>) {
    let (rules_block, updates_block) = input.split_once("\n\n").unwrap();

    let rules = rules_block.lines().map(OrderingRule::parse).collect();
    let updates = updates_block.lines().map(PageUpdate::parse).collect();

    (rules, updates)
}

type Page = u8;

#[derive(Eq, Hash, PartialEq)]
pub struct OrderingRule {
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

pub struct PageUpdate {
    pages: Vec<Page>,
}

impl PageUpdate {
    pub fn sorted(&self, rules: &HashSet<OrderingRule>) -> Self {
        let mut sorted_pages = vec![];

        for page in self.pages.iter() {
            PageUpdate::add_page_sorted(rules, &mut sorted_pages, *page);
        }

        Self {
            pages: sorted_pages,
        }
    }

    fn add_page_sorted(
        rules: &HashSet<OrderingRule>,
        sorted_pages: &mut Vec<Page>,
        target_page: Page,
    ) {
        for (idx, sorted_page) in sorted_pages.iter().enumerate() {
            if PageUpdate::goes_before(rules, target_page, *sorted_page) {
                sorted_pages.insert(idx, target_page);
                return;
            }
        }

        sorted_pages.push(target_page);
    }

    pub fn middle_page(&self) -> u64 {
        self.pages[self.pages.len() / 2] as u64
    }

    pub fn is_valid(&self, rules: &HashSet<OrderingRule>) -> bool {
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

use std::{fs, num::ParseIntError, str::FromStr};

enum RuleSide {
    Left,
    Right,
    None,
}
struct RuleParseError;

#[derive(Debug)]
struct Rule {
    left: u32,
    right: u32,
}

impl Rule {
    fn new(left: u32, right: u32) -> Self {
        return Self { left, right };
    }
    fn get_side(&self, page: u32) -> RuleSide {
        if self.left == page {
            return RuleSide::Left;
        }
        if self.left == page {
            return RuleSide::Right;
        }
        RuleSide::None
    }
}

impl From<ParseIntError> for RuleParseError {
    fn from(_value: ParseIntError) -> Self {
        Self {}
    }
}

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        let left_str = split.next().ok_or(RuleParseError)?;
        let right_str = split.next().ok_or(RuleParseError)?;

        let left: u32 = left_str.parse()?;
        let right: u32 = right_str.parse()?;

        Ok(Rule::new(left, right))
    }
}

fn main() {
    let input_file_path = "./src/day5/input.txt";
    let contents = fs::read_to_string(input_file_path).unwrap();

    let test = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let mut rules: Vec<Rule> = vec![];

    let mut count = 0;
    let mut middle_sum = 0;

    let data: Vec<_> = test
        .lines()
        .map(|line| {
            let line = line.trim();
            if let Ok(rule) = Rule::from_str(line) {
                rules.push(rule);
            } else if !line.is_empty() {
                let update: Vec<u32> = line
                    .split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();

                let mut broken_rules: Vec<&Rule> = vec![];

                for page in &update {
                    broken_rules.retain(|Rule { left, right }| right != page);
                    for rule in &rules {
                        let side = rule.get_side(*page);
                        if let RuleSide::Left = side {
                            if update.contains(&rule.right) {
                                broken_rules.push(rule);
                            }
                        }
                    }
                }

                if broken_rules.is_empty() {
                    count += 1;
                    let middle_index = update.len() / 2;
                    let middle = update.get(middle_index).unwrap();
                    middle_sum += middle;
                } else {
                    println!(
                        "wrong update {:?} - broken rules - {:?}",
                        update, broken_rules
                    );
                }
            }
        })
        .collect();

    // Part 1
    println!("{:?} - sum {:?}", count, middle_sum);

    // Part 2
    println!("{:?} - sum {:?}", count, middle_sum);
}

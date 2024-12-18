use std::{fs, num::ParseIntError, str::FromStr};

enum RuleSide {
    Left,
    Right,
    None,
}
struct RuleParseError;

#[derive(Debug, PartialEq)]
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
97,13,75,29,47
47,24,53,97,93,29";

    let mut rules: Vec<Rule> = vec![];

    let mut count = 0;
    let mut fixed_count = 0;
    let mut middle_sum = 0;
    let mut fixed_middle_sum = 0;

    let data: Vec<_> = contents
        .lines()
        .map(|line| {
            let line = line.trim();
            if let Ok(rule) = Rule::from_str(line) {
                rules.push(rule);
            } else if !line.is_empty() {
                let mut update: Vec<u32> = line
                    .split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();

                // AHHH my solution only fixed some of the broken rules...so instead iterate enough
                // times to fix all broken 
                // This is BAD :)
                for _ in 1..1000 {

                    let broken_rules: Vec<&Rule> = get_broken_rules(&update, &rules);

                    if broken_rules.is_empty() {
                        count += 1;
                        let middle_index = update.len() / 2;
                        let middle = update.get(middle_index).unwrap();
                        middle_sum += middle;
                    } else {
                        println!(
                            "\nwrong update {:?} - broken rules - {:?}",
                            update, broken_rules
                        );
                        let mut fixing_broken_rules = broken_rules.clone();
                        for &rule in &broken_rules {
                            if !fixing_broken_rules.contains(&rule) {
                                continue;
                            }
                            let pos_left = update.iter().position(|&p| p == rule.left).unwrap();
                            let swap_right = broken_rules.iter().filter(|&&b_r| b_r.left == rule.left).flat_map(|&b_r| {
                                // urgh
                                fixing_broken_rules.retain(|&i_b_r| i_b_r != b_r);

                                update.iter().position(|&page| page == b_r.right)
                            }).min();
                            println!("TEST {:?}", swap_right);
                            if let Some(swap_right) = swap_right {
                                update.swap(pos_left, swap_right);
                            }
                        }
                        let after_fix_broken_rules: Vec<&Rule> = get_broken_rules(&update, &rules);
                        if after_fix_broken_rules.is_empty() {
                            fixed_count += 1;
                            let middle_index = update.len() / 2;
                            let middle = update.get(middle_index).unwrap();
                            fixed_middle_sum += middle;
                            println!("fixed update {:?}", update);
                            break;
                        } else {
                            println!(
                                "\ncouldnt correct update {:?} - remaining after fix broken rules - {:?}",
                                update, after_fix_broken_rules
                            );
                        }
                    }
                }
            }
        })
        .collect();

    // Part 1
    println!("{:?} - sum {:?}", count, middle_sum);

    // Part 2
    println!("{:?} - fixed sum {:?}", fixed_count, fixed_middle_sum);
}

fn get_broken_rules<'a>(update: &[u32], rules: &'a [Rule]) -> Vec<&'a Rule> {
    let mut broken_rules: Vec<&Rule> = vec![];
    for page in update {
        broken_rules.retain(|Rule { left, right }| right != page);
        for rule in rules {
            let side = rule.get_side(*page);
            if let RuleSide::Left = side {
                if update.contains(&rule.right) {
                    broken_rules.push(rule);
                }
            }
        }
    }
    broken_rules
}

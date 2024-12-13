use std::{fs, num::ParseIntError, str::FromStr, string::ParseError};

fn main() {
    let input_file_path = "./src/day5/input.txt";
    let contents = fs::read_to_string(input_file_path).unwrap();

    let _test = "47|53
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

    struct Rule {
        left: u32,
        right: u32,
    }

    impl Rule {
        fn new(left: u32, right: u32) -> Self {
            return Self { left, right };
        }
    }

    struct RuleParseError;
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

    let rules: Vec<Rule>;
    let updates: Vec<Vec<u32>>;

    let mut data: Vec<_> = contents
        .lines()
        .map(|line| {
            let line = line.trim();
            _ = Rule::from_str(line)
                .and_then(|rule| {
                    rules.push(rule);
                    Ok(rule)
                })
                .or_else(|_| {
                    let split = line.split(',');
                });
        })
        .collect();

    let mut count = 0;

    // Part 1
    println!("{:?}", count);

    // Part 2
    println!("{:?}", count);
}

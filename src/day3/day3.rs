use regex::{self, Regex};
use std::fs;

fn calc_sum(re: &Regex, haystack: &str) -> u32 {
    re.captures_iter(haystack)
        .map(|c| {
            // TODO: is there way to do this without so much unwrap chaining?
            let a: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}
fn main() {
    let enable_re = Regex::new(r"do\(\)((?s).*?)don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input_file_path = "./src/day3/input.txt";
    let mut contents = fs::read_to_string(input_file_path).unwrap();

    // Part 1
    let sum = calc_sum(&mul_re, &contents);
    println!("{:?}", sum);

    contents.insert_str(0, "do()");
    contents.push_str("don't()");

    let sum: u32 = enable_re
        .captures_iter(&contents)
        .map(|c| {
            let inner_contents = c.get(1).unwrap().as_str();
            calc_sum(&mul_re, inner_contents)
        })
        .sum();

    // Part 2
    println!("{:?}", sum);
}

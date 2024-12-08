use regex::{self, Regex};
use std::fs;

fn main() {
    let enable_re = Regex::new(r"do\(\)((?s).*?)don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input_file_path = "./src/day4.txt";
    let mut contents = fs::read_to_string(input_file_path).unwrap();

    // Part 1
    //println!("{:?}", sum);

    // Part 2
    //println!("{:?}", sum);
}

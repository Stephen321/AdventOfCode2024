use regex::{self, Regex};
use std::fs;

fn main() {
    let enable_re = Regex::new(r"do\(\)((?s).*?)don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let input_file_path = "./src/day4/input.txt";
    let mut contents = fs::read_to_string(input_file_path).unwrap();

    let test = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let mut data: Vec<_> = test
        .lines()
        .map(|line| {
            let line = line.trim();
            return line.chars();
        })
        .collect();

    data.remove(0);
    let rows = data.len();
    let cols = data[0].count();
    println!("rows {:?} cols {:?}", rows, cols);
    print!("{test}");

    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.enumerate() {}
    }

    let total = 0;
    // Part 1
    //println!("{:?}", total);

    // Part 2
    //println!("{:?}", total);
}

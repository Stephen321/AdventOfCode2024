use std::fs;

fn main() {
    let input_file_path = "./src/day1/input.txt";
    let contents = fs::read_to_string(input_file_path);

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in contents.unwrap().lines() {
        // TODO: why does this have to be mut
        let mut split = line.split_whitespace();
        left_list.push(split.next().unwrap().parse().unwrap());
        right_list.push(split.next().unwrap().parse().unwrap());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let distance: u32 = left_list
        .iter()
        .zip(&right_list)
        .map(|(left, right)| (left - right).abs() as u32)
        .sum();

    // Part 1
    println!("{:?}", distance);

    // TODO: use map + sum too?
    let mut similarity = 0;
    for left in left_list {
        similarity += left * right_list.iter().filter(|&right| left == *right).count() as i32;
    }

    // Part 2
    println!("{:?}", similarity);
}

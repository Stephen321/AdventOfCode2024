use std::{fs, usize};

fn part_1(contents: &str) -> u32 {
    let mut safe_reports = 0u32;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let increasing = levels[1] > levels[0];
        let mut safe = true;
        for (index, level) in levels.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let diff = level - levels[index - 1];
            if increasing {
                if diff.is_negative() {
                    safe = false;
                    break;
                }
            } else {
                if diff.is_positive() {
                    safe = false;
                    break;
                }
            }
            match diff.abs() {
                1 | 2 | 3 => continue,
                _ => {
                    safe = false;
                    break;
                }
            }
        }
        if safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let increasing = levels[1] > levels[0];
    for (index, level) in levels.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let diff = level - levels[index - 1];
        if increasing {
            if diff.is_negative() {
                return false;
            }
        } else if diff.is_positive() {
            return false;
        }
        match diff.abs() {
            1..=3 => continue,
            _ => {
                return false;
            }
        }
    }
    true
}

fn part_2(contents: &str) -> u32 {
    let mut safe_reports = 0u32;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        if is_safe(&levels) {
            safe_reports += 1
        } else {
            for index in 0usize..levels.len() {
                let mut levels_clone = levels.clone();
                levels_clone.remove(index);
                if is_safe(&levels_clone) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

fn main() {
    let input_file_path = "./src/day2/input.txt";
    let contents = fs::read_to_string(input_file_path).unwrap();

    let safe_reports = part_1(&contents);
    println!("Part 1 = {:?}", safe_reports);

    let safe_reports = part_2(&contents);
    println!("Part 2 = {:?}", safe_reports);
}

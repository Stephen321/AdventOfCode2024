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

fn is_safe(levels: &Vec<i32>, has_removed: bool) -> bool {
    let increasing = levels[1] > levels[0];
    let mut safe: bool = true;

    for (index, level) in levels.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let diff = level - levels[index - 1];
        if increasing {
            if diff.is_negative() {
                if has_removed {
                    return false;
                } else {
                    let mut temp_levels = levels.clone();
                    temp_levels.remove(index);
                    return is_safe(&temp_levels, true);
                }
            }
        } else if diff.is_positive() {
            if has_removed {
                return false;
            } else {
                let mut temp_levels = levels.clone();
                temp_levels.remove(index);
                return is_safe(&temp_levels, true);
            }
        }
        match diff.abs() {
            1..=3 => continue,
            _ => {
                if has_removed {
                    return false;
                } else {
                    let mut temp_levels = levels.clone();
                    temp_levels.remove(index);
                    return is_safe(&temp_levels, true);
                }
            }
        }
    }
    safe
}

fn part_2(contents: &str) -> u32 {
    let mut safe_reports = 0u32;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        if is_safe(&levels, false) {
            safe_reports += 1;
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

use std::fs;

fn main() {
    let input_file_path = "./src/day2/input.txt";
    let contents = fs::read_to_string(input_file_path);

    let mut safe_reports = 0u32;
    for line in contents.unwrap().lines() {
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
    println!("{:?}", safe_reports);
}

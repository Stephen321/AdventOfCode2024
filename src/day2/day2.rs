fn main() {
    let lines: Vec<&str> = vec![
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];
    for line in lines {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();
        let increasing = levels[1] > levels[0];
        for (index, level) in levels.iter().enumerate() {
            if index == 0 {
                continue;
            }
        }
        println!("{:?}", levels);
    }
}

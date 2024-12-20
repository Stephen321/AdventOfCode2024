use std::{collections::HashSet, fs, process::exit, usize};

fn change_dir(dir: usize) -> usize {
    if dir == 3 {
        return 0;
    }
    dir + 1
}

fn get_char(data: &[String], row: usize, col: usize) -> Option<char> {
    data.get(row).and_then(|row| row.chars().nth(col))
}

fn find_start_pos(data: &[String]) -> Option<(usize, usize)> {
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            if char == '^' {
                return Some((row, col));
            }
        }
    }
    None
}

fn travel(data: &[String]) -> (bool, HashSet<(usize, usize)>) {
    const OFFSETS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let (mut next_row, mut next_col) = find_start_pos(data).unwrap();

    //println!("start row and col are {:?}-{:?}", next_row, next_col);
    let mut direction = 0usize;

    let mut travelled: HashSet<(usize, usize)> = HashSet::new();
    travelled.insert((next_row, next_col));

    let mut change_positions: Vec<(usize, usize)> = vec![]; //(0, 0); 8];

    loop {
        let (offset_row, offset_col) = OFFSETS[direction];
        let before_row = next_row;
        let before_col = next_col;
        next_row = next_row.wrapping_add_signed(offset_row);
        next_col = next_col.wrapping_add_signed(offset_col);
        match get_char(data, next_row, next_col) {
            Some('#') => {
                next_row = before_row;
                next_col = before_col;
                direction = change_dir(direction);

                change_positions.push((next_row, next_col));
                //println!("{:?}", change_positions);
                // minimum for a loop is 4 and we need at least 8 to determine a loop of 4 repeats
                let length = change_positions.len();
                if length >= 8 {
                    // only loop over even numbers as loop can't have odd amount of obstructions

                    let half_length = length / 2;
                    for i in (4..half_length).step_by(2) {
                        let mut chunks = change_positions.rchunks(i);
                        let c1 = chunks.next().unwrap();
                        let c2 = chunks.next().unwrap();
                        if c1 == c2 {
                            return (true, travelled);
                        }
                    }
                }
            }
            Some(_) => _ = travelled.insert((next_row, next_col)),
            None => break,
        };
    }
    (false, travelled)
}

fn main() {
    let input_file_path = "./src/day6/input.txt";
    let contents = fs::read_to_string(input_file_path).unwrap();

    let test = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let mut data: Vec<_> = contents
        .lines()
        .map(|line| {
            let line = line.trim();
            line.to_owned()
        })
        .collect();

    let rows = data.len();
    let cols = data[0].len();
    println!("rows {:?} cols {:?}", rows, cols);

    let (_, travelled) = travel(&data);

    let mut map = String::from('\n');
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            if travelled.contains(&(row, col)) {
                map.push('X');
            } else {
                map.push(char);
            }
        }
        map.push('\n');
    }
    println!("{}", map);

    // Part 1
    println!("{:?}", travelled.len());

    // Part 2
    let mut loop_causing_obs: Vec<(usize, usize)> = vec![];
    for row in 0..rows {
        for col in 0..cols {
            let char = get_char(&data, row, col);
            if let Some(ref c) = get_char(&data, row, col) {
                if ['^', '#'].contains(c) {
                    continue;
                }
            }
            //println!("checking {} {}...", row, col);
            let mut data_modified = data.clone();
            data_modified[row].replace_range(col..col + 1, "#");
            let (looping, _) = travel(&data_modified);
            if looping {
                //println!("Looping");
                loop_causing_obs.push((row, col));
            }
        }
    }
    println!("{:?}", data);
    //println!("{:?}", loop_causing_obs);
    println!(
        "loop causing obstruction count {:?}",
        loop_causing_obs.len()
    );
}

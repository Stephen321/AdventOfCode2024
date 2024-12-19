use std::{collections::HashSet, fs};

fn change_dir(dir: usize) -> usize {
    if dir == 3 {
        return 0;
    }
    dir + 1
}

fn get_char(data: &[&str], row: usize, col: usize) -> Option<char> {
    data.get(row).and_then(|row| row.chars().nth(col))
}

fn find_start_pos(data: &[&str]) -> Option<(usize, usize)> {
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            if char == '^' {
                return Some((row, col));
            }
        }
    }
    return None;
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

    let offsets: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let data: Vec<_> = contents
        .lines()
        .map(|line| {
            let line = line.trim();
            line
        })
        .collect();

    let rows = data.len();
    let cols = data[0].len();
    println!("rows {:?} cols {:?}", rows, cols);

    let (mut next_row, mut next_col) = find_start_pos(&data).unwrap();
    println!("start row and col are {:?}-{:?}", next_row, next_col);

    let mut direction = 0usize;

    let mut travelled: HashSet<(usize, usize)> = HashSet::new();
    travelled.insert((next_row, next_col));

    loop {
        let (offset_row, offset_col) = offsets[direction];
        let before_row = next_row;
        let before_col = next_col;
        next_row = next_row.wrapping_add_signed(offset_row);
        next_col = next_col.wrapping_add_signed(offset_col);
        match get_char(&data, next_row, next_col) {
            Some('#') => {
                next_row = before_row;
                next_col = before_col;
                direction = change_dir(direction)
            }
            Some(_) => _ = travelled.insert((next_row, next_col)),
            None => break,
        };
    }

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
}

use std::fs;

fn get_char(data: &[String], row: i32, col: i32, rows: i32, cols: i32) -> Option<char> {
    if row < 0 || row > rows - 1 {
        return None;
    }
    if col < 0 || col > cols - 1 {
        return None;
    }
    let chars = &data[row as usize];
    Some(chars.chars().nth(col as usize).unwrap())
}

fn check_for_word(data: &[String], rows: i32, cols: i32, char: char, row: i32, col: i32) -> u32 {
    if char != 'X' {
        return 0;
    }

    let offsets = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut words_found = 0;
    for (offset_row, offset_col) in offsets {
        let mut check_row = row;
        let mut check_col = col;
        let mut letters_found = 0;
        for next_char in ['M', 'A', 'S'] {
            check_row += offset_row;
            check_col += offset_col;
            if let Some(c) = get_char(data, check_row, check_col, rows, cols) {
                if c == next_char {
                    letters_found += 1;
                }
            }
        }
        if letters_found == 3 {
            words_found += 1;
        }
    }
    words_found
}

fn main() {
    let input_file_path = "./src/day4/input.txt";
    let mut contents = fs::read_to_string(input_file_path).unwrap();

    let _test = "
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

    let mut data: Vec<_> = contents
        .lines()
        .map(|line| {
            let line = line.trim();
            line.to_owned()
        })
        .collect();

    data.remove(0);
    let rows = data.len() as i32;
    let cols = data[0].len() as i32;
    println!("rows {:?} cols {:?}", rows, cols);
    //print!("{test}");

    let mut count: u32 = 0;
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            count += check_for_word(data.as_slice(), rows, cols, char, row as i32, col as i32);
        }
    }

    // Part 1
    println!("{:?}", count);

    // Part 2
    //println!("{:?}", total);
}

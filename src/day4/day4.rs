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

fn check_offset_m_s_count(
    data: &[String],
    rows: i32,
    cols: i32,
    row: i32,
    col: i32,
    offsets: [(i32, i32); 2],
) -> (i32, i32) {
    let mut m_count = 0;
    let mut s_count = 0;
    for (offset_row, offset_col) in offsets {
        let check_row = row + offset_row;
        let check_col = col + offset_col;
        match get_char(data, check_row, check_col, rows, cols) {
            Some('M') => m_count += 1,
            Some('S') => s_count += 1,
            _ => (),
        }
    }
    (m_count, s_count)
}

fn check_for_x_mas(data: &[String], rows: i32, cols: i32, char: char, row: i32, col: i32) -> bool {
    if char != 'A' {
        return false;
    }

    let up_offsets = [(1, -1), (-1, 1)];
    let down_offsets = [(-1, -1), (1, 1)];

    if let (1, 1) = check_offset_m_s_count(data, rows, cols, row, col, up_offsets) {
        if let (1, 1) = check_offset_m_s_count(data, rows, cols, row, col, down_offsets) {
            return true;
        }
    }
    false
}

fn check_for_xmas(data: &[String], rows: i32, cols: i32, char: char, row: i32, col: i32) -> u32 {
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
    let contents = fs::read_to_string(input_file_path).unwrap();

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

    // TODO: only for test day
    //data.remove(0);

    let rows = data.len() as i32;
    let cols = data[0].len() as i32;
    println!("rows {:?} cols {:?}", rows, cols);
    //print!("{test}");

    let mut count: u32 = 0;
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            count += check_for_xmas(data.as_slice(), rows, cols, char, row as i32, col as i32);
        }
    }

    // Part 1
    println!("{:?}", count);

    // Part 2
    let mut count: u32 = 0;
    for (row, chars) in data.iter().enumerate() {
        for (col, char) in chars.char_indices() {
            if check_for_x_mas(data.as_slice(), rows, cols, char, row as i32, col as i32) {
                count += 1;
            }
        }
    }
    println!("{:?}", count);
}

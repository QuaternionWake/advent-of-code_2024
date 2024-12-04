use std::fs;

const ROWS: usize = 140;
const COLS: usize = 140;

const WORD_LEN: usize = 4;
const WORD: [char; WORD_LEN] = ['X', 'M', 'A', 'S'];
const WORD_REV: [char; WORD_LEN] = ['S', 'A', 'M', 'X'];

pub fn run() {
    let input_path = "inputs/day_4.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let mut field: [[char; COLS]; ROWS] = [['\0'; COLS]; ROWS];
    for (i, line) in input.lines().take(ROWS).enumerate() {
        for (j, char) in line.chars().take(COLS).enumerate() {
            field[i][j] = char;
        }
    }

    let mut xmas_count = 0;
    for row in field {
        for chunk in row.windows(4) {
            if chunk == WORD || chunk == WORD_REV {
                xmas_count += 1;
            }
        }
    }

    for i in 0..=ROWS - WORD_LEN {
        for j in 0..COLS {
            if vertical_slice(&field, i, j) == WORD || vertical_slice(&field, i, j) == WORD_REV {
                xmas_count += 1;
            }
        }
    }

    for i in 0..=ROWS - WORD_LEN {
        for j in 0..=COLS - WORD_LEN {
            if prim_diag_slice(&field, i, j) == WORD || prim_diag_slice(&field, i, j) == WORD_REV {
                xmas_count += 1;
            }
        }
    }

    for i in 0..=ROWS - WORD_LEN {
        for j in WORD_LEN - 1..COLS {
            if sec_diag_slice(&field, i, j) == WORD || sec_diag_slice(&field, i, j) == WORD_REV {
                xmas_count += 1;
            }
        }
    }

    println!("Number of XMASes found: {xmas_count}");
}

fn vertical_slice(field: &[[char; COLS]; ROWS], i: usize, j: usize) -> [char; WORD_LEN] {
    [
        field[i][j],
        field[i + 1][j],
        field[i + 2][j],
        field[i + 3][j],
    ]
}

fn prim_diag_slice(field: &[[char; COLS]; ROWS], i: usize, j: usize) -> [char; WORD_LEN] {
    [
        field[i][j],
        field[i + 1][j + 1],
        field[i + 2][j + 2],
        field[i + 3][j + 3],
    ]
}

fn sec_diag_slice(field: &[[char; COLS]; ROWS], i: usize, j: usize) -> [char; WORD_LEN] {
    [
        field[i][j],
        field[i + 1][j - 1],
        field[i + 2][j - 2],
        field[i + 3][j - 3],
    ]
}

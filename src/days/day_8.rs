use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

const ROWS: usize = 50;
const COLS: usize = 50;

pub fn run() {
    let input = crate::load_input(8);

    let mut field: [[char; COLS]; ROWS] = [['\0'; COLS]; ROWS];
    for (i, line) in input.lines().take(ROWS).enumerate() {
        for (j, char) in line.chars().take(COLS).enumerate() {
            field[i][j] = char;
        }
    }

    let mut antennas: HashMap<char, Vec<_>> = HashMap::new();
    for (i, line) in field.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_alphanumeric() {
                antennas
                    .entry(*char)
                    .and_modify(|v| v.push(Pos::new(j as isize, i as isize)))
                    .or_insert(vec![Pos::new(j as isize, i as isize)]);
            }
        }
    }

    let mut antinode_field: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];

    for freq_antennas in antennas.values() {
        for i in 0..freq_antennas.len() {
            for j in i + 1..freq_antennas.len() {
                place_antinodes(&mut antinode_field, freq_antennas[i], freq_antennas[j]);
            }
        }
    }

    let antinode_count = antinode_field.as_flattened().iter().filter(|&&b| b).count();

    println!("Number of antinodes: {antinode_count}");

    for freq_antennas in antennas.values() {
        for i in 0..freq_antennas.len() {
            for j in i + 1..freq_antennas.len() {
                place_resonant_antinodes(&mut antinode_field, freq_antennas[i], freq_antennas[j]);
            }
        }
    }

    let resonant_antinode_count = antinode_field.as_flattened().iter().filter(|&&b| b).count();

    println!("Number of antinodes: {resonant_antinode_count}");
}

fn place_antinodes(antinode_field: &mut [[bool; COLS]; ROWS], antenna_1: Pos, antenna_2: Pos) {
    let antenna_distance = antenna_2 - antenna_1;
    let antinode_1 = antenna_1 - antenna_distance;
    let antinode_2 = antenna_2 + antenna_distance;

    if (0..COLS).contains(&(antinode_1.x as usize)) && (0..ROWS).contains(&(antinode_1.y as usize))
    {
        antinode_field[antinode_1.y as usize][antinode_1.x as usize] = true;
    }
    if (0..COLS).contains(&(antinode_2.x as usize)) && (0..ROWS).contains(&(antinode_2.y as usize))
    {
        antinode_field[antinode_2.y as usize][antinode_2.x as usize] = true;
    }
}

fn place_resonant_antinodes(antinode_field: &mut [[bool; 50]; 50], antenna_1: Pos, antenna_2: Pos) {
    let antenna_distance = antenna_2 - antenna_1;

    let mut antinode_pos = antenna_2;
    while (0..COLS).contains(&(antinode_pos.x as usize))
        && (0..ROWS).contains(&(antinode_pos.y as usize))
    {
        antinode_field[antinode_pos.y as usize][antinode_pos.x as usize] = true;
        antinode_pos += antenna_distance;
    }

    antinode_pos = antenna_1;
    while (0..COLS).contains(&(antinode_pos.x as usize))
        && (0..ROWS).contains(&(antinode_pos.y as usize))
    {
        antinode_field[antinode_pos.y as usize][antinode_pos.x as usize] = true;
        antinode_pos -= antenna_distance;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Pos {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

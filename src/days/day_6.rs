use std::{fs, ops::Add};

const COLS: usize = 130;
const ROWS: usize = 130;

pub fn run() {
    let input_path = "inputs/day_6.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let mut field: [[char; COLS]; ROWS] = [['\n'; COLS]; ROWS];
    for (i, line) in input.lines().take(ROWS).enumerate() {
        for (j, char) in line.chars().take(COLS).enumerate() {
            field[i][j] = char;
        }
    }

    let start_guard_pos = field
        .iter()
        .enumerate()
        .find_map(|(i, l)| {
            l.iter()
                .enumerate()
                .find_map(|(j, char)| (*char == '^').then_some(Pos::new(j as isize, i as isize)))
        })
        .expect("could not find guard");
    let start_guard_orient = Orient::Up;

    let mut guard_pos = start_guard_pos;
    let mut guard_orient = start_guard_orient;
    field[guard_pos.y as usize][guard_pos.x as usize] = 'X';
    while let Some((new_pos, new_orient)) = take_step(&field, guard_pos, guard_orient) {
        field[new_pos.y as usize][new_pos.x as usize] = 'X';
        guard_pos = new_pos;
        guard_orient = new_orient;
    }

    let tiles_visited = field.as_flattened().iter().filter(|&&c| c == 'X').count();

    // Print the path

    // for line in field {
    //     println!("{}", line.iter().collect::<String>());
    // }

    println!("Tiles visited: {tiles_visited}");

    let spot_candidates: Vec<_> = field
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| (*c == 'X').then_some(Pos::new(j as isize, i as isize)))
        })
        .collect();

    let mut potential_spot_count = 0;
    'outer: for (i, candidate) in spot_candidates.iter().enumerate() {
        eprint!("\r");
        eprint!("Checking spot {} of {}", i, spot_candidates.len());

        let mut previous_states = vec![(start_guard_pos, start_guard_orient)];

        let mut guard_pos = start_guard_pos;
        let mut guard_orient = start_guard_orient;
        let mut field = field; // apparently arrays implement copy?????????
        field[candidate.y as usize][candidate.x as usize] = '#';
        while let Some((new_pos, new_orient)) = take_step(&field, guard_pos, guard_orient) {
            if previous_states.contains(&(new_pos, new_orient)) {
                potential_spot_count += 1;
                continue 'outer;
            }
            field[new_pos.y as usize][new_pos.x as usize] = 'X';

            previous_states.push((new_pos, new_orient));
            guard_pos = new_pos;
            guard_orient = new_orient;
        }
    }
    eprint!("\r");

    println!("Potential obstruction spots: {potential_spot_count}");
}

fn take_step(field: &[[char; COLS]; ROWS], pos: Pos, orient: Orient) -> Option<(Pos, Orient)> {
    let next_pos = pos + orient.to_pos();
    if !(0..ROWS as isize).contains(&next_pos.y) {
        return None;
    }
    if !(0..COLS as isize).contains(&next_pos.x) {
        return None;
    }
    if field[next_pos.y as usize][next_pos.x as usize] == '#' {
        Some((pos, orient.rot_cw()))
    } else {
        Some((next_pos, orient))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Orient {
    Up,
    Right,
    Down,
    Left,
}

impl Orient {
    fn rot_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn to_pos(&self) -> Pos {
        match self {
            Self::Up => Pos::new(0, -1),
            Self::Right => Pos::new(1, 0),
            Self::Down => Pos::new(0, 1),
            Self::Left => Pos::new(-1, 0),
        }
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

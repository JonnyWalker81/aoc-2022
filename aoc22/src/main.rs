use core::fmt;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

type Point = (usize, usize);

#[derive(Debug, Default)]
enum Cell {
    #[default]
    Empty,
    Open,
    Wall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Empty => write!(f, " "),
            Cell::Open => write!(f, "."),
            Cell::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Default)]
struct Board {
    board: HashMap<Point, Cell>,
}

#[derive(Debug, Default)]
enum Path {
    #[default]
    Noop,
    Move(usize),
    Dir(Direction),
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    Noop,
    Left,
    Right,
    Up,
    Down,
}

// impl Direction {
//     fn rotate(&self) -> Self {
//         match *self {
//             Direction::Left => {
//                 match
//             }
//         }
//     }
// }

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unrecognized: {}", item)
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let (b, path) = input.split_once("\n\n").unwrap();
    let mut board = Board::default();
    for (r, l) in b.lines().enumerate() {
        for (c, i) in l.chars().enumerate() {
            let cell = match i {
                ' ' => continue,
                '.' => Cell::Open,
                '#' => Cell::Wall,
                _ => panic!("unrecognized cell: {}", i),
            };

            board.board.insert((r, c), cell);
        }
    }

    let mut dig = String::new();
    let mut moves = vec![];
    for p in path.trim().chars() {
        if p.is_digit(10) {
            dig.push(p);
        } else {
            moves.push(Path::Move(dig.parse().unwrap()));
            moves.push(Path::Dir(p.into()));
            dig.clear();
        }
    }

    if !dig.is_empty() {
        moves.push(Path::Move(dig.parse().unwrap()));
    }

    println!("{moves:?}");
    let mut first: usize = board
        .board
        .iter()
        .filter(|(k, _)| k.0 == 0)
        .map(|(k, _)| k.1)
        .min()
        .unwrap();

    println!("First: (0, {first})");

    let mut cur = Direction::Right;

    let mut dir = ();
    for m in &moves {
        let min_r = board
            .board
            .iter()
            .filter(|(p, _)| p.0 == cur.1)
            .map(|(p, _)| p.0)
            .min()
            .unwrap();
        let min_c = board
            .board
            .iter()
            .filter(|(p, _)| p.0 == cur.0)
            .map(|(p, _)| p.1)
            .min()
            .unwrap();

        let max_r = board
            .board
            .iter()
            .filter(|(p, _)| p.0 == cur.1)
            .map(|(p, _)| p.0)
            .max()
            .unwrap();
        let max_c = board
            .board
            .iter()
            .filter(|(p, _)| p.0 == cur.0)
            .map(|(p, _)| p.1)
            .min()
            .unwrap();

        let mv = match dir {
            Direction::Right => (0, 1),
            Di
        }
    }

    // println!("{board:?}");
    print_board(&board);
    Ok(())
}

fn print_board(board: &Board) {
    let max_r = board.board.iter().map(|(k, _)| k.0).max().unwrap();
    let max_c = board.board.iter().map(|(k, _)| k.1).max().unwrap();

    for r in 0..max_r {
        for c in 0..max_c {
            if let Some(c) = board.board.get(&(r, c)) {
                print!("{c}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

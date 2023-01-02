use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

type Point = (i32, i32);

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
struct Bounds {
    min_r: i32,
    min_c: i32,
    max_r: i32,
    max_c: i32,
}

#[derive(Debug, Default)]
struct Board {
    board: HashMap<Point, Cell>,
    bounds: HashMap<Point, Bounds>,
}

#[derive(Debug, Clone, Default)]
enum Path {
    #[default]
    Noop,
    Move(usize),
    Dir(Direction),
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
enum Direction {
    #[default]
    Noop,
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Direction::Noop => write!(f, " "),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
        }
    }
}

impl Direction {
    fn rotate(&self, rotate: &Direction) -> Self {
        match *self {
            Direction::Right => match rotate {
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                _ => panic!("shold not get here"),
            },
            Direction::Left => match rotate {
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                _ => panic!("shold not get here"),
            },
            Direction::Up => match rotate {
                Direction::Right => Direction::Right,
                Direction::Left => Direction::Left,
                _ => panic!("shold not get here"),
            },
            Direction::Down => match rotate {
                Direction::Right => Direction::Left,
                Direction::Left => Direction::Right,
                _ => panic!("shold not get here"),
            },
            _ => panic!("shold not get here"),
        }
    }
}

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unrecognized: {}", item),
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

            board.board.insert((r as i32, c as i32), cell);
        }
    }

    // process to get min/max for each Point in Board
    for ((r, c), _) in &board.board {
        let min_r = board
            .board
            .iter()
            .filter(|(k, _)| &k.1 == c)
            .map(|(k, _)| k.0)
            .min()
            .unwrap();

        let min_c = board
            .board
            .iter()
            .filter(|(k, _)| &k.0 == r)
            .map(|(k, _)| k.1)
            .min()
            .unwrap();

        let max_r = board
            .board
            .iter()
            .filter(|(k, _)| &k.1 == c)
            .map(|(k, _)| k.0)
            .max()
            .unwrap();

        let max_c = board
            .board
            .iter()
            .filter(|(k, _)| &k.0 == r)
            .map(|(k, _)| k.1)
            .max()
            .unwrap();

        board.bounds.insert(
            (*r, *c),
            Bounds {
                min_r,
                max_r,
                min_c,
                max_c,
            },
        );
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

    // println!("{moves:?}");
    let first: i32 = board
        .board
        .iter()
        .filter(|(k, _)| k.0 == 0)
        .map(|(k, _)| k.1)
        .min()
        .unwrap();

    println!("First: (0, {first})");

    let mut cur = Direction::Right;

    let mut history = HashSet::new();
    let mut loc = (0, first);
    for m in &moves {
        match m {
            Path::Move(s) => {
                for _ in 0..*s {
                    let mv = match cur {
                        Direction::Up => (-1, 0),
                        Direction::Down => (1, 0),
                        Direction::Left => (0, -1),
                        Direction::Right => (0, 1),
                        _ => (0, 0),
                    };

                    let mut np = (loc.0 + mv.0, loc.1 + mv.1);
                    // check bounds
                    if let Some(bounds) = board.bounds.get(&loc) {
                        if np.0 < bounds.min_r {
                            np.0 = bounds.max_r;
                        }

                        if np.0 > bounds.max_r {
                            np.0 = bounds.min_r;
                        }

                        if np.1 < bounds.min_c {
                            np.1 = bounds.max_c;
                        }

                        if np.1 > bounds.max_c {
                            np.1 = bounds.min_c;
                        }
                        if let Some(b) = board.board.get(&np) {
                            if let Cell::Wall = b {
                                break;
                            }
                        }

                        loc = np;
                        history.insert(loc);
                    }
                }
            }
            Path::Dir(d) => {
                cur = cur.rotate(d);
            }
            _ => {}
        }
    }

    // println!("{board:?}");
    // print_board(&board, &history);
    // println!("History: {history:#?}");
    println!("Facing: {cur:?}");
    println!("Loc: {loc:?}");

    let facing_value = match cur {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => panic!("should not get here."),
    };

    let password = (1000 * (loc.0 + 1)) + (4 * (loc.1 + 1)) + facing_value;
    println!("Password: {password}");

    Ok(())
}

const SIZE: i32 = 50;

fn part2(input: &str) -> Result<()> {
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

            board.board.insert((r as i32, c as i32), cell);
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

    let first: i32 = board
        .board
        .iter()
        .filter(|(k, _)| k.0 == 0)
        .map(|(k, _)| k.1)
        .min()
        .unwrap();

    println!("First: (0, {first})");

    let mut cur = Direction::Right;

    let mut history = HashMap::new();
    let mut loc = (0, first);
    for m in &moves {
        match m {
            Path::Move(s) => {
                for _ in 0..*s {
                    let mv = match cur {
                        Direction::Up => (-1, 0),
                        Direction::Down => (1, 0),
                        Direction::Left => (0, -1),
                        Direction::Right => (0, 1),
                        _ => (0, 0),
                    };

                    history.insert(loc, cur.clone());
                    let np = (loc.0 + mv.0, loc.1 + mv.1);
                    // check bounds
                    if let Some(vp) = board.board.get(&np) {
                        if let Cell::Wall = vp {
                            break;
                        }
                        loc = np;
                    } else {
                        let (_, tp, nd) = wrap_cube(loc, &cur);
                        if let Some(b) = board.board.get(&tp) {
                            if let Cell::Wall = b {
                                break;
                            }
                        }

                        loc = tp;
                        cur = nd;
                    }
                }
            }
            Path::Dir(d) => {
                cur = cur.rotate(d);
            }
            _ => {}
        }
    }

    // println!("{board:?}");
    // print_board(&board, &history);
    // println!("History: {history:#?}");
    println!("Facing: {cur:?}");
    println!("Loc: {loc:?}");

    let facing_value = match cur {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => panic!("should not get here."),
    };

    let password = (1000 * (loc.0 + 1)) + (4 * (loc.1 + 1)) + facing_value;
    println!("Password: {password}");

    Ok(())
}

fn wrap_cube(from: Point, d: &Direction) -> (usize, Point, Direction) {
    let side_idx = (from.0 / SIZE) * 3 + from.1 / SIZE;
    // println!("side_idx({from:?}): {side_idx}");
    let sides = [0, 2, 3, 0, 1, 0, 4, 5, 0, 6];

    let side = sides[side_idx as usize];

    // let row_offset = from.0 % SIZE;
    // let col_offset = from.1 % SIZE;
    let row = from.0;
    let col = from.1;

    match (side, d) {
        (1, Direction::Right) => (3, (SIZE - 1, row + 50), Direction::Up),
        (1, Direction::Left) => (4, ((SIZE * 2), row - SIZE), Direction::Down),
        (2, Direction::Up) => (6, (col + 100, 0), Direction::Right),
        (2, Direction::Left) => (4, (149 - row, 0), Direction::Right),
        (3, Direction::Up) => (6, ((SIZE * 4) - 1, col - 100), Direction::Up),
        (3, Direction::Right) => (5, (149 - row, (SIZE * 2) - 1), Direction::Left),
        (3, Direction::Down) => (1, (col - SIZE, (SIZE * 2) - 1), Direction::Left),
        (4, Direction::Up) => (1, (col + SIZE, SIZE), Direction::Right),
        // (4, Direction::Right) => (5, (from.0, (SIZE + col_offset)), Direction::Right),
        (4, Direction::Left) => (2, (149 - row, SIZE), Direction::Right),
        (5, Direction::Right) => (3, (149 - row, (SIZE * 3) - 1), Direction::Left),
        (5, Direction::Down) => (6, (col + 100, SIZE - 1), Direction::Left),
        (6, Direction::Right) => (5, (SIZE * 3 - 1, row - 100), Direction::Up),
        (6, Direction::Down) => (3, (0, col + 100), Direction::Down),
        (6, Direction::Left) => (2, (0, row - 100), Direction::Down),

        // (1, Direction::Right) => (3, (SIZE - 1, SIZE * 2 + row_offset), Direction::Up),
        // (1, Direction::Left) => (4, ((SIZE * 2), row_offset), Direction::Down),
        // (2, Direction::Up) => (6, (SIZE * 3 + col_offset, 0), Direction::Right),
        // (2, Direction::Left) => (4, (SIZE * 2 + row_offset, 0), Direction::Right),
        // (3, Direction::Up) => (6, ((SIZE * 4) - 1, col_offset), Direction::Up),
        // (3, Direction::Right) => (
        //     5,
        //     ((SIZE * 2) + row_offset, (SIZE * 2) - 1),
        //     Direction::Left,
        // ),
        // (3, Direction::Down) => (1, (SIZE + col_offset, (SIZE * 2) - 1), Direction::Left),
        // (4, Direction::Up) => (1, (SIZE + col_offset, (SIZE)), Direction::Right),
        // // (4, Direction::Right) => (5, (from.0, (SIZE + col_offset)), Direction::Right),
        // (4, Direction::Left) => (2, (row_offset, SIZE), Direction::Right),
        // (5, Direction::Right) => (3, (row_offset, (SIZE * 3) - 1), Direction::Left),
        // (5, Direction::Down) => (6, ((SIZE * 3) + col_offset, SIZE - 1), Direction::Left),
        // (6, Direction::Right) => (5, (SIZE * 3 - 1, (SIZE + row_offset)), Direction::Up),
        // (6, Direction::Down) => (3, (0, SIZE * 2 + col_offset), Direction::Down),
        // (6, Direction::Left) => (2, (0, SIZE + col_offset), Direction::Down),
        _ => panic!("should not get here: {} -> {:?}", side, d),
    }
}

fn print_board(board: &Board, history: &HashMap<Point, Direction>) {
    let max_r = board.board.iter().map(|(k, _)| k.0).max().unwrap();
    let max_c = board.board.iter().map(|(k, _)| k.1).max().unwrap();

    for r in 0..max_r {
        for c in 0..max_c {
            if let Some(d) = history.get(&(r, c)) {
                print!("{d}");
            } else {
                if let Some(c) = board.board.get(&(r, c)) {
                    print!("{c}");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

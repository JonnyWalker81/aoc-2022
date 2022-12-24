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

type Point = (i64, i64);

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn adjacent(&self) -> Vec<Point> {
        match *self {
            Self::North => vec![(-1, 0), (-1, 1), (-1, -1)],
            Self::South => vec![(1, 0), (1, 1), (1, -1)],
            Self::West => vec![(0, -1), (-1, -1), (1, -1)],
            Self::East => vec![(0, 1), (-1, 1), (1, 1)],
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let (board, _) = play(input, 10);

    print_board(&board);
    let min_r = board.iter().map(|k| k.0).min().unwrap().clone();
    let min_c = board.iter().map(|k| k.1).min().unwrap().clone();
    // println!("Min Row: {min_r}");
    // println!("Min Col: {min_c}");

    let max_r = board.iter().map(|k| k.0).max().unwrap().clone();
    let max_c = board.iter().map(|k| k.1).max().unwrap().clone();
    // println!("Max Row: {max_r}");
    // println!("Max Col: {max_c}");

    let area = (max_r - min_r + 1) * (max_c - min_c + 1);
    let elves: i64 = board.iter().count() as i64;
    let total = area - elves;
    println!("{total}");
    let mut count = 0;
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if !board.contains(&(r, c)) {
                count += 1;
            }
        }
    }

    println!("{count}");

    Ok(())
}

fn play(input: &str, iterations: i32) -> (HashSet<Point>, i32) {
    let mut board: HashSet<Point> = HashSet::new();
    for (r, l) in input.lines().enumerate() {
        for (c, s) in l.chars().enumerate() {
            if s == '#' {
                board.insert((r as i64, c as i64));
            }
        }
    }

    // print_board(&board);

    // propose moves
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut round = 0;
    loop {
        let mut proposals: HashMap<Point, Point> = HashMap::new();
        let mut proposal_counts: HashMap<Point, i32> = HashMap::new();

        for &(r, c) in board.iter() {
            let mut can_move = false;
            for dir in &directions {
                let adjacent = dir.adjacent();
                for a in adjacent {
                    let p = (r + a.0, c + a.1);
                    let available = board.contains(&p);

                    if available {
                        can_move = true;
                        break;
                    }
                }

                if can_move {
                    break;
                }
            }

            if !can_move {
                continue;
            }

            for dir in &directions {
                let adjacent = dir.adjacent();
                let mut able_to_move = true;
                for a in adjacent {
                    let p = (r + a.0, c + a.1);
                    let available = board.contains(&p);
                    if available {
                        able_to_move = false;
                        break;
                    }
                }

                if able_to_move {
                    let p = match dir {
                        Direction::North => (r - 1, c),
                        Direction::South => (r + 1, c),
                        Direction::West => (r, c - 1),
                        Direction::East => (r, c + 1),
                    };
                    proposals.entry((r, c)).or_insert(p);
                    proposal_counts
                        .entry(p)
                        .and_modify(|f| *f += 1)
                        .or_insert(1);
                    break;
                }
            }
        }

        if iterations == -1 && proposals.is_empty() {
            break;
        } else if iterations == round {
            break;
        }

        let front = directions.remove(0);
        directions.insert(directions.len(), front);

        // move if possible
        for (k, p) in proposals.iter() {
            if board.contains(k) {
                if let Some(c) = proposal_counts.get(&p) {
                    if *c == 1 {
                        board.remove(k);
                        board.insert(*p);
                    }
                }
            }
        }

        round += 1;
    }

    (board, round + 1)
}

fn part2(input: &str) -> Result<()> {
    let (_, rounds) = play(input, -1);
    println!("Round: {rounds}");

    Ok(())
}

fn print_board(board: &HashSet<Point>) {
    let max_r = board.iter().map(|k| k.0).max().unwrap().clone();
    let max_c = board.iter().map(|k| k.1).max().unwrap().clone();

    let min_r = board.iter().map(|k| k.0).min().unwrap().clone();
    let min_c = board.iter().map(|k| k.1).min().unwrap().clone();

    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if board.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

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

#[derive(Debug, Clone)]
enum Command {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Command {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        let n = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "U" => Self::Up(n),
            "D" => Self::Down(n),
            "L" => Self::Left(n),
            "R" => Self::Right(n),
            _ => panic!("unrecognized command: {:?}", parts),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let commands: Vec<Command> = input.lines().map(|l| l.into()).collect();

    let mut tail_positions = HashSet::new();
    let mut tail_position: (i32, i32) = (0, 0);
    let mut position: (i32, i32) = (0, 0);
    let mut prev_poistion: (i32, i32) = position;
    tail_positions.insert(tail_position);
    for c in commands {
        update_position(
            &c,
            &mut position,
            &mut tail_position,
            &mut tail_positions,
            &mut prev_poistion,
        )?;
    }

    println!("Tail Positions: {:?}", tail_positions.len());
    // println!("Tail Positions: {:?}", tail_positions);

    Ok(())
}

fn update_position(
    c: &Command,
    position: &mut (i32, i32),
    tail_position: &mut (i32, i32),
    tail_positions: &mut HashSet<(i32, i32)>,
    prev_position: &mut (i32, i32),
) -> Result<()> {
    let n = match c {
        Command::Up(n) => *n,
        Command::Down(n) => *n,
        Command::Left(n) => *n,
        Command::Right(n) => *n,
    };
    for _ in 0..n {
        *prev_position = *position;
        let d = match c {
            Command::Up(_) => {
                *position = (position.0 - 1, position.1);
                -1
            }
            Command::Down(_) => {
                *position = (position.0 + 1, position.1);
                1
            }
            Command::Left(_) => {
                *position = (position.0, position.1 - 1);
                -1
            }
            Command::Right(_) => {
                *position = (position.0, position.1 + 1);
                1
            }
        };

        // check row and column for tail
        if tail_position.0 == position.0 {
            if (position.1 - tail_position.1).abs() > 1 {
                tail_position.1 += d;
            }
        } else if tail_position.1 == position.1 {
            if (position.0 - tail_position.0).abs() > 1 {
                tail_position.0 += d;
            }
        } else {
            // diagonal
            let mut is_touching = false;
            let mut test_pos = (tail_position.0 - 1, tail_position.1 + 1);
            if test_pos.0 == position.0 && test_pos.1 == position.1
            // && (tail_position.0 != position.0 && tail_position.1 != position.1)
            {
                is_touching = true;
                // *tail_position = *position;
            }

            test_pos = (tail_position.0 + 1, tail_position.1 + 1);
            if test_pos.0 == position.0 && test_pos.1 == position.1
            // && (tail_position.0 != position.0 && tail_position.1 != position.1)
            {
                is_touching = true;
                // *tail_position = *position;
            }

            test_pos = (tail_position.0 + 1, tail_position.1 - 1);
            if test_pos.0 == position.0 && test_pos.1 == position.1
            // && (tail_position.0 != position.0 && tail_position.1 != position.1)
            {
                is_touching = true;
                // *tail_position = *position;
            }

            test_pos = (tail_position.0 - 1, tail_position.1 - 1);
            if test_pos.0 == position.0 && test_pos.1 == position.1
            // && (tail_position.0 != position.0 && tail_position.1 != position.1)
            {
                is_touching = true;
                // *tail_position = *position;
            }

            if !is_touching {
                *tail_position = *prev_position;
            }
        }

        tail_positions.insert(*tail_position);
    }

    Ok(())
}

#[derive(Debug)]
struct KnotPosition {
    current: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

fn part2(input: &str) -> Result<()> {
    let commands: Vec<Command> = input.lines().map(|l| l.into()).collect();

    let mut tail_positions: HashMap<i32, KnotPosition> = HashMap::new();
    // let mut tail_position: (i32, i32) = (0, 0);
    let position: (i32, i32) = (0, 0);
    let mut prev_poistion: HashMap<i32, (i32, i32)> = HashMap::new();
    prev_poistion.insert(0, position);
    for i in 0..10 {
        let mut init = HashSet::new();
        init.insert((0, 0));
        tail_positions.insert(
            i,
            KnotPosition {
                current: (0, 0),
                visited: init,
            },
        );
    }

    for c in commands {
        update_all_knots(&c, &mut tail_positions)?;
    }

    println!("Tail Positions (9): {:?}", tail_positions[&9].visited.len());

    Ok(())
}

fn update_all_knots(c: &Command, tail_positions: &mut HashMap<i32, KnotPosition>) -> Result<()> {
    let n = match c {
        Command::Up(n) => *n,
        Command::Left(n) => *n,
        Command::Down(n) => *n,
        Command::Right(n) => *n,
    };
    for _ in 0..n {
        match c {
            Command::Up(_) => {
                tail_positions.entry(0).and_modify(|p| {
                    (*p).current.0 -= 1;
                });
            }
            Command::Down(_) => {
                tail_positions.entry(0).and_modify(|p| {
                    (*p).current.0 += 1;
                });
            }
            Command::Left(_) => {
                tail_positions.entry(0).and_modify(|p| {
                    (*p).current.1 -= 1;
                });
            }
            Command::Right(_) => {
                tail_positions.entry(0).and_modify(|p| {
                    (*p).current.1 += 1;
                });
            }
        }

        for k in 1..10 {
            let (dx, dy) = (
                tail_positions[&(k - 1)].current.0 - tail_positions[&k].current.0,
                tail_positions[&(k - 1)].current.1 - tail_positions[&k].current.1,
            );
            if dx.abs() > 1 || dy.abs() > 1 {
                tail_positions.entry(k).and_modify(|s| {
                    s.current.0 += dx.signum();
                    s.current.1 += dy.signum();
                    s.visited.insert(s.current);
                });
            }
        }
    }

    Ok(())
}

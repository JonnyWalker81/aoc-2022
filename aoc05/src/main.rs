//     [H]         [D]     [P]
// [W] [B]         [C] [Z] [D]
// [T] [J]     [T] [J] [D] [J]
// [H] [Z]     [H] [H] [W] [S]     [M]
// [P] [F] [R] [P] [Z] [F] [W]     [F]
// [J] [V] [T] [N] [F] [G] [Z] [S] [S]
// [C] [R] [P] [S] [V] [M] [V] [D] [Z]
// [F] [G] [H] [Z] [N] [P] [M] [N] [D]
//  1   2   3   4   5   6   7   8   9
//
//
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

use regex::Regex;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

/*
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
 */

impl From<&str> for Move {
    fn from(item: &str) -> Self {
        let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures_iter(item).next().unwrap();

        // println!("{:?}", caps);
        Self {
            from: caps[2].parse().unwrap(),
            to: caps[3].parse().unwrap(),
            amount: caps[1].parse().unwrap(),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let mut stacks = vec![
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M'],
    ];

    // Sample input
    // let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let moves: Vec<Move> = input.lines().map(|l| l.into()).collect();

    for m in moves {
        // println!("{:?}", m);
        let mut s = vec![];
        for _ in 0..m.amount {
            if !stacks[m.from - 1].is_empty() {
                let c = stacks[m.from - 1].pop().unwrap();
                s.push(c);
            }
        }

        for c in s {
            stacks[m.to - 1].push(c);
        }
    }

    // for c in &stacks {
    //     println!("{:?}", c);
    // }

    let msg = stacks.iter().fold(String::new(), |mut acc, s| {
        let top = s.last().unwrap();
        acc.push(*top);
        acc
    });

    println!("Part 1: {}", msg);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut stacks = vec![
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M'],
    ];

    // Sample input
    // let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let moves: Vec<Move> = input.lines().map(|l| l.into()).collect();

    for m in moves {
        let mut s = vec![];
        if !stacks[m.from - 1].is_empty() {
            let len = stacks[m.from - 1].len();
            let c: Vec<char> = stacks[m.from - 1].drain(len - m.amount..).collect();
            for i in c {
                s.push(i);
            }
        }

        for c in s {
            stacks[m.to - 1].push(c);
        }
    }

    // for c in &stacks {
    //     println!("{:?}", c);
    // }

    let msg = stacks.iter().fold(String::new(), |mut acc, s| {
        let top = s.last().unwrap();
        acc.push(*top);
        acc
    });

    println!("Part 2: {}", msg);

    Ok(())
}

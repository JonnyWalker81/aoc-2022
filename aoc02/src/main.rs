use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn lose(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn win(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl From<&str> for Shape {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("unrecognized value: {}", item),
        }
    }
}

struct Round {
    opponent: Shape,
    ours: Shape,
}

fn part1(input: &str) -> Result<()> {
    let rounds: Vec<Round> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let opponent: Shape = parts[0].into();
            let ours: Shape = parts[1].into();
            Round { opponent, ours }
        })
        .collect();

    let total = rounds.iter().fold(0, |mut acc, r| {
        match (&r.opponent, &r.ours) {
            (opp, ours) if opp == ours => acc += r.ours.value() + 3,
            (Shape::Rock, Shape::Scissors) => acc += r.ours.value(),
            (Shape::Scissors, Shape::Paper) => acc += r.ours.value(),
            (Shape::Paper, Shape::Rock) => acc += r.ours.value(),
            (Shape::Scissors, Shape::Rock) => acc += r.ours.value() + 6,
            (Shape::Paper, Shape::Scissors) => acc += r.ours.value() + 6,
            (Shape::Rock, Shape::Paper) => acc += r.ours.value() + 6,
            _ => panic!("{:?} -> {:?}", r.opponent, r.ours),
        }
        acc
    });

    println!("Part 1 Total: {}", total);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(item: &str) -> Self {
        match item {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unexpected value: {}", item),
        }
    }
}

#[derive(Debug)]
struct RoundTwo {
    opponent: Shape,
    outcome: Outcome,
}

fn part2(input: &str) -> Result<()> {
    let rounds: Vec<RoundTwo> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let opponent: Shape = parts[0].into();
            let outcome: Outcome = parts[1].into();
            RoundTwo { opponent, outcome }
        })
        .collect();

    let total = rounds.iter().fold(0, |acc, r| {
        let score = match (&r.opponent, &r.outcome) {
            (opp, Outcome::Lose) => opp.lose().value(),
            (opp, Outcome::Win) => opp.win().value() + 6,
            (opp, Outcome::Draw) => opp.value() + 3,
        };

        acc + score
    });

    println!("Part 2 Total: {}", total);
    Ok(())
}

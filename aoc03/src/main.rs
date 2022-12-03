use std::{
    collections::HashSet,
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

#[derive(Debug)]
struct Sack {
    comp1: String,
    comp2: String,
}

impl From<&str> for Sack {
    fn from(item: &str) -> Self {
        let parts = item.split_at(item.len() / 2);
        Self {
            comp1: parts.0.to_string(),
            comp2: parts.1.to_string(),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let sacks: Vec<Sack> = input.lines().map(|l| l.into()).collect();

    let commons: Vec<char> = sacks
        .iter()
        .map(|s| {
            let comp_chars: HashSet<char> = s.comp1.chars().fold(HashSet::new(), |mut acc, c| {
                acc.insert(c);
                acc
            });

            for c in s.comp2.chars() {
                if comp_chars.contains(&c) {
                    return c;
                }
            }
            '\0'
        })
        .collect();

    let total = commons.iter().fold(0, |acc, c| {
        let v = if c.is_lowercase() {
            *c as i32 - 'a' as i32 + 1
        } else {
            *c as i32 - 'A' as i32 + 27
        };
        acc + v
    });

    println!("Total: {}", total);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let sacks: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let chunks: Vec<&[String]> = sacks.as_slice().chunks(3).collect();
    let vals: Vec<i32> = chunks
        .iter()
        .map(|g| {
            let chars1: HashSet<char> = g[0].chars().fold(HashSet::new(), |mut acc, c| {
                acc.insert(c);
                acc
            });

            let chars2: HashSet<char> = g[1].chars().fold(HashSet::new(), |mut acc, c| {
                acc.insert(c);
                acc
            });

            let chars3: HashSet<char> = g[2].chars().fold(HashSet::new(), |mut acc, c| {
                acc.insert(c);
                acc
            });

            let mut common = '\0';
            for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
                let found1 = chars1.iter().any(|cc| *cc == c);
                let found2 = chars2.iter().any(|cc| *cc == c);
                let found3 = chars3.iter().any(|cc| *cc == c);
                if found1 && found2 && found3 {
                    common = c;
                    break;
                }
            }

            if common.is_lowercase() {
                common as i32 - 'a' as i32 + 1
            } else {
                common as i32 - 'A' as i32 + 27
            }
        })
        .collect();

    let total: i32 = vals.iter().sum();
    println!("Total: {}", total);

    Ok(())
}

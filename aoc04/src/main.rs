use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl From<&str> for Range {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split('-').collect();
        Self {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let sections: Vec<(Range, Range)> = input
        .lines()
        .map(|l| {
            let sections: Vec<Range> = l.split(',').map(|s| s.into()).collect();
            (sections[0].clone(), sections[1].clone())
        })
        .collect();
    let fully_overlaps = sections.iter().fold(0, |mut acc, s| {
        if s.0.start <= s.1.start && s.0.end >= s.1.end
            || s.1.start <= s.0.start && s.1.end >= s.0.end
        {
            acc += 1;
        }

        acc
    });

    println!("{}", fully_overlaps);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let sections: Vec<(Range, Range)> = input
        .lines()
        .map(|l| {
            let sections: Vec<Range> = l.split(',').map(|s| s.into()).collect();
            (sections[0].clone(), sections[1].clone())
        })
        .collect();
    let overlaps: usize = sections.iter().fold(0, |mut acc, s| {
        if !(s.0.end < s.1.start || s.1.end < s.0.start) {
            acc += 1;
        }

        acc
    });

    println!("{}", overlaps);
    Ok(())
}

use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut elves = vec![];
    let mut current = vec![];
    for l in lines {
        if l.is_empty() {
            let total = current.iter().fold(0, |mut acc: i32, c: &&str| {
                if !c.is_empty() {
                    let v: i32 = c.parse::<i32>().unwrap();
                    acc += v;
                }
                acc
            });
            elves.push(total);
            current.clear();
        }

        current.push(l);
    }

    elves.sort_by(|a, b| b.cmp(a));
    let m = elves.first().unwrap();
    println!("{}", m);

    let top_three: i32 = elves.iter().take(3).sum();
    println!("{}", top_three);
    Ok(())
}

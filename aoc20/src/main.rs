use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let numbers: Vec<(usize, i32)> = input
        .lines()
        .enumerate()
        .map(|(k, v)| (k, v.parse().unwrap()))
        .collect();
    // println!("{numbers:?}");

    let mut mixer = numbers.clone();
    for i in 0..numbers.len() {
        let n = mixer.iter().position(|s| s.0 == numbers[i].0).unwrap();
        let cur = mixer.remove(n);
        let shift = n as i32 + cur.1;
        let idx = shift.rem_euclid(mixer.len() as i32);
        mixer.insert(idx as usize, cur);
    }

    println!("{mixer:?}");
    let zeroth = mixer.iter().position(|(_, v)| *v == 0);
    if let Some(n) = zeroth {
        println!("Oth: {}", n);
        let first = (n + 1000) % mixer.len();
        let next = mixer[first].1;
        println!("{next}");

        let second = (n + 2000) % mixer.len();
        let next2 = mixer[second].1;
        println!("{next2}");

        let third = (n + 3000) % mixer.len();
        let next3 = mixer[third].1;
        println!("{next3}");

        let sum = next + next2 + next3;
        println!("{sum}");
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let numbers: Vec<(usize, i64)> = input
        .lines()
        .enumerate()
        .map(|(k, v)| (k, v.parse::<i64>().unwrap() * 811589153))
        .collect();

    let mut mixer = numbers.clone();
    for _r in 0..10 {
        for i in 0..numbers.len() {
            let n = mixer.iter().position(|s| s.0 == numbers[i].0).unwrap();
            let cur = mixer.remove(n);
            let shift = n as i64 + cur.1;
            let idx = shift.rem_euclid(mixer.len() as i64);
            mixer.insert(idx as usize, cur);
        }
    }

    println!("{mixer:?}");
    let zeroth = mixer.iter().position(|(_, v)| *v == 0);
    if let Some(n) = zeroth {
        println!("Oth: {}", n);
        let first = (n + 1000) % mixer.len();
        let next = mixer[first].1;
        println!("{next}");

        let second = (n + 2000) % mixer.len();
        let next2 = mixer[second].1;
        println!("{next2}");

        let third = (n + 3000) % mixer.len();
        let next3 = mixer[third].1;
        println!("{next3}");

        let sum = next + next2 + next3;
        println!("{sum}");
    }

    Ok(())
}

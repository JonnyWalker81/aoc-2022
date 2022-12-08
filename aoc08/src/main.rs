use std::{
    collections::HashMap,
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

fn part1(input: &str) -> Result<()> {
    let mut grid = vec![];
    for l in input.lines().collect::<Vec<&str>>() {
        let mut v: Vec<u32> = vec![];
        for c in l.chars() {
            v.push(c.to_digit(10).unwrap());
        }
        grid.push(v);
    }

    let edge_count = (2 * grid.len()) + (2 * grid[0].len()) - 4;
    let mut visible_count = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            // check up
            let mut is_all_shorter = true;
            for u in 0..r {
                if grid[u][c] >= grid[r][c] {
                    is_all_shorter = false;
                    break;
                }
            }

            if is_all_shorter {
                visible_count += 1;
                continue;
            }

            is_all_shorter = true;

            // check down
            for d in r + 1..grid.len() {
                if grid[d][c] >= grid[r][c] {
                    is_all_shorter = false;
                    break;
                }
            }

            if is_all_shorter {
                visible_count += 1;
                continue;
            }

            is_all_shorter = true;

            // check left
            for l in 0..c {
                if grid[r][l] >= grid[r][c] {
                    is_all_shorter = false;
                    break;
                }
            }

            if is_all_shorter {
                visible_count += 1;
                continue;
            }

            is_all_shorter = true;

            // check right
            for right in c + 1..grid[r].len() {
                if grid[r][right] >= grid[r][c] {
                    is_all_shorter = false;
                    break;
                }
            }

            if is_all_shorter {
                visible_count += 1;
                continue;
            }
        }
    }

    println!("{}", visible_count + edge_count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut grid = vec![];
    for l in input.lines().collect::<Vec<&str>>() {
        let mut v: Vec<u32> = vec![];
        for c in l.chars() {
            v.push(c.to_digit(10).unwrap());
        }
        grid.push(v);
    }

    let mut scores: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            // check up
            let mut up_count = 0;
            for u in (0..r).rev() {
                up_count += 1;
                if grid[u][c] >= grid[r][c] {
                    break;
                }
            }

            // check down
            let mut down_count = 0;
            for d in r + 1..grid.len() {
                down_count += 1;
                if grid[d][c] >= grid[r][c] {
                    break;
                }
            }

            // check left
            let mut left_count = 0;
            for l in (0..c).rev() {
                left_count += 1;
                if grid[r][l] >= grid[r][c] {
                    break;
                }
            }

            // check right
            let mut right_count = 0;
            for right in c + 1..grid[r].len() {
                right_count += 1;
                if grid[r][right] >= grid[r][c] {
                    break;
                }
            }

            let score = up_count * down_count * left_count * right_count;
            scores.insert((r, c), score);
        }
    }

    let max = scores.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();

    println!("{:#?}", max.1);

    Ok(())
}

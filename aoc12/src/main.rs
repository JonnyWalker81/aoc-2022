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

fn find(grid: &Vec<Vec<char>>, key: char) -> Result<(usize, usize)> {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == key {
                return Ok((r, c));
            }
        }
    }

    Ok((0, 0))
}

fn height(grid: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    if grid[r][c].is_lowercase() {
        grid[r][c] as i32
    } else if grid[r][c] == 'S' {
        'a' as i32
    } else if grid[r][c] == 'E' {
        'z' as i32
    } else {
        -1
    }
}

fn neighbors(grid: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut n = vec![];
    for d in dirs {
        let rr = r as i32 + d.0;
        let cc = c as i32 + d.1;

        if rr >= 0 && rr < grid.len() as i32 && cc >= 0 && cc < grid[r].len() as i32 {
            n.push((rr as usize, cc as usize));
        }
    }

    n
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Node {
    point: (usize, usize),
    dist: i32,
}

fn part1(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = find(&grid, 'S')?;
    let end = find(&grid, 'E')?;

    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    let sp = find_shortest_path(&grid, start, end);
    println!("Steps: {}", sp);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = find(&grid, 'S')?;
    let end = find(&grid, 'E')?;

    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    let mut steps = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'a' || grid[r][c] == 'S' {
                let sp = find_shortest_path(&grid, (r, c), end);
                steps.push(sp);
            }
        }
    }

    println!("{:#?}", steps);
    let min_steps = steps.iter().filter(|s| **s > 0).min();
    println!("{:#?}", min_steps);

    Ok(())
}

fn find_shortest_path(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push(Node {
        point: start,
        dist: 0,
    });

    while !queue.is_empty() {
        let item = queue.remove(0);
        if visited.contains(&item.point) {
            continue;
        }

        visited.insert(item.point);
        if item.point == end {
            return item.dist;
        }
        let nbrs = neighbors(&grid, item.point.0, item.point.1);

        for n in nbrs {
            let d = height(&grid, n.0, n.1) - height(&grid, item.point.0, item.point.1);
            if d <= 1 && !visited.contains(&n) {
                queue.push(Node {
                    point: n,
                    dist: item.dist + 1,
                });
            }
        }
    }

    0
}

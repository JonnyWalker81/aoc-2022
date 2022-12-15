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

type Point = (i32, i32);

#[derive(Debug)]
struct Path {
    start: Point,
    end: Point,
}

fn part1(input: &str) -> Result<()> {
    let mut paths = vec![];
    for l in input.lines() {
        let p: Vec<&str> = l.split(" -> ").collect();
        let pnts: Vec<Point> = p
            .iter()
            .map(|p| {
                let p: Vec<&str> = p.split(",").collect();
                (p[1].parse().unwrap(), p[0].parse().unwrap())
            })
            .collect();
        let mut points = vec![];
        for p in pnts {
            points.push(p);
        }

        let mut start = points[0];
        for p in points.iter().skip(1) {
            paths.push(Path { start, end: *p });
            start = *p;
        }
    }

    let mut grid = HashMap::new();

    for p in &paths {
        let d = if p.start.0 == p.end.0 {
            (0, (p.end.1 - p.start.1).signum())
        } else {
            ((p.end.0 - p.start.0).signum(), 0)
        };
        let mut s = p.start;
        loop {
            // same row
            grid.insert(s, "#");
            if s == p.end {
                break;
            }

            s.0 += d.0;
            s.1 += d.1;
        }
    }

    // find bottom
    let max_row = {
        grid.iter()
            .max_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .clone()
            .unwrap()
    };

    let mut resting: HashSet<Point> = HashSet::new();
    let possibilities = vec![(1, 0), (1, -1), (1, 1)];

    let m = max_row.0.clone();
    'outer: loop {
        // each sand unit
        let mut sand = (0, 500);
        loop {
            // move until it comes to rest
            let prev = sand;
            for p in &possibilities {
                let test_next = (sand.0 + p.0, sand.1 + p.1);
                if !grid.contains_key(&test_next)
                    || (grid[&test_next] != "#" && grid[&test_next] != "o")
                {
                    sand = test_next;
                    break;
                }
            }

            if sand.0 > m.0 {
                break 'outer;
            }

            if prev == sand {
                grid.insert(sand, "o");
                resting.insert(sand);
                break;
            }
        }
    }

    let sand_count = grid.iter().filter(|(_, v)| **v == "o").count();
    println!("{}", sand_count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut paths = vec![];
    for l in input.lines() {
        let p: Vec<&str> = l.split(" -> ").collect();
        let pnts: Vec<Point> = p
            .iter()
            .map(|p| {
                let p: Vec<&str> = p.split(",").collect();
                (p[1].parse().unwrap(), p[0].parse().unwrap())
            })
            .collect();
        let mut points = vec![];
        for p in pnts {
            points.push(p);
        }

        let mut start = points[0];
        for p in points.iter().skip(1) {
            paths.push(Path { start, end: *p });
            start = *p;
        }
    }

    let mut grid = HashMap::new();

    for p in &paths {
        let d = if p.start.0 == p.end.0 {
            (0, (p.end.1 - p.start.1).signum())
        } else {
            ((p.end.0 - p.start.0).signum(), 0)
        };
        let mut s = p.start;
        loop {
            // same row
            grid.insert(s, "#");
            if s == p.end {
                break;
            }

            s.0 += d.0;
            s.1 += d.1;
        }
    }

    // find bottom
    let max_row = {
        grid.iter()
            .max_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .clone()
            .unwrap()
    };

    let mut resting: HashSet<Point> = HashSet::new();
    let possibilities = vec![(1, 0), (1, -1), (1, 1)];

    let floor = (max_row.0 .0 + 2, 0);
    'outer: loop {
        // each sand unit
        let mut sand = (0, 500);
        // let mut stack = vec![sand];
        loop {
            // move until it comes to rest
            let prev = sand;
            for p in &possibilities {
                let test_next = (sand.0 + p.0, sand.1 + p.1);
                if (!grid.contains_key(&test_next)
                    || (grid[&test_next] != "#" && grid[&test_next] != "o"))
                    && test_next.0 < floor.0
                {
                    sand = test_next;
                    break;
                }
            }

            if prev == sand {
                grid.insert(sand, "o");
                resting.insert(sand);
                if sand.0 == 0 {
                    break 'outer;
                }
                break;
            }
        }
    }

    // print_grid(&grid);
    let sand_count = grid.iter().filter(|(_, v)| **v == "o").count();
    println!("{}", sand_count);

    Ok(())
}

fn print_grid(grid: &HashMap<Point, &str>) {
    let mut cave = vec![vec!["."; 700]; 700];

    for (k, v) in grid {
        cave[k.0 as usize][k.1 as usize] = v;
    }

    for r in 0..cave.len() {
        for c in 0..cave[r].len() {
            print!("{}", cave[r][c]);
        }

        println!();
    }
}

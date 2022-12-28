use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Copy)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Point3 {
    fn from(item: &str) -> Self {
        let parts = item.split(",").collect::<Vec<&str>>();
        Self {
            x: parts[0].parse().expect("expected number"),
            y: parts[1].parse().expect("expected number"),
            z: parts[2].parse().expect("expected number"),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let cubes: Vec<Point3> = input.lines().map(|l| l.into()).collect();
    println!("Cubes: {:?}", cubes);
    let mut sides: HashSet<Point3> = HashSet::new();
    let mut total_sides = cubes.len() * 6;
    println!("{total_sides}");

    for c in &cubes {
        sides.insert(c.clone());
    }

    let dirs: Vec<(i32, i32, i32)> = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for c in &cubes {
        for d in &dirs {
            let p = Point3 {
                x: c.x + d.0,
                y: c.y + d.1,
                z: c.z + d.2,
            };
            if sides.contains(&p) {
                total_sides -= 1;
            }
        }
    }
    println!("{total_sides}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let cubes: Vec<Point3> = input.lines().map(|l| l.into()).collect();
    // println!("Cubes: {:?}", cubes);
    let mut sides: HashSet<Point3> = HashSet::new();
    // let mut total_sides = cubes.len() * 6;
    // println!("{total_sides}");

    for c in &cubes {
        sides.insert(c.clone());
    }

    let dirs: Vec<(i32, i32, i32)> = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    // for c in &cubes {
    //     for d in &dirs {
    //         let p = Point3 {
    //             x: c.x + d.0,
    //             y: c.y + d.1,
    //             z: c.z + d.2,
    //         };
    //         if sides.contains(&p) {
    //             total_sides -= 1;
    //         }
    //     }
    // }

    let mut outer = HashMap::new();
    let mut x_range = (i32::MAX, i32::MIN);
    let mut y_range = (i32::MAX, i32::MIN);
    let mut z_range = (i32::MAX, i32::MIN);

    for p in &sides {
        // for d in &dirs {
        //     let p = Point3 {
        //         x: c.x + d.0,
        //         y: c.y + d.1,
        //         z: c.z + d.2,
        //     };
        x_range.0 = x_range.0.min(p.x);
        x_range.1 = x_range.1.max(p.x);
        y_range.0 = x_range.0.min(p.y);
        y_range.1 = x_range.1.max(p.y);
        z_range.0 = x_range.0.min(p.z);
        z_range.1 = x_range.1.max(p.z);
        outer.insert(p, 6);
        // }
    }

    for (p, c) in outer.iter_mut() {
        for d in &dirs {
            let n = Point3 {
                x: p.x + d.0,
                y: p.y + d.1,
                z: p.z + d.2,
            };
            if sides.contains(&n) {
                *c -= 1;
            }
        }
    }

    x_range = (x_range.0 - 1, x_range.1 + 1);
    y_range = (y_range.0 - 1, y_range.1 + 1);
    z_range = (z_range.0 - 1, z_range.1 + 1);

    let mut visited = HashSet::new();
    let mut queue: VecDeque<Point3> = vec![Point3 {
        x: x_range.0,
        y: y_range.0,
        z: z_range.0,
    }]
    .iter()
    .cloned()
    .collect();
    let mut found = HashMap::new();
    let mut count = 0;

    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }

        for d in &dirs {
            let n = Point3 {
                x: p.x + d.0,
                y: p.y + d.1,
                z: p.z + d.2,
            };

            if n.x < x_range.0
                || n.x > x_range.1
                || n.y < y_range.0
                || n.y > y_range.1
                || n.z < z_range.0
                || n.z > z_range.1
            {
                continue;
            }

            if let Some(surface) = outer.get(&n) {
                found.insert(n, *surface);
                count += 1;
            } else {
                queue.push_back(n);
            }
        }
    }

    // for c in &cubes {
    //     let mut surrounded = true;
    //     for d in &dirs {
    //         let p = Point3 {
    //             x: c.x + d.0,
    //             y: c.y + d.1,
    //             z: c.z + d.2,
    //         };
    //         if !sides.contains(&p) {
    //             surrounded = false;
    //             break;
    //         }
    //     }
    //
    //     if surrounded {
    //         total_sides -= 6;
    //     }
    // }
    println!("{}", found.values().sum::<i32>());
    println!("{}", count);
    Ok(())
}

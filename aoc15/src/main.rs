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

type Point = (i64, i64);

#[derive(Debug)]
struct Sensor {
    sensor: Point,
    beacon: Point,
}

fn part1(input: &str) -> Result<()> {
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let s_x_parts: Vec<&str> = parts[2].split("=").collect();
            let s_y_parts: Vec<&str> = parts[3].split("=").collect();
            let b_x_parts: Vec<&str> = parts[8].split("=").collect();
            let b_y_parts: Vec<&str> = parts[9].split("=").collect();

            let s_x = s_x_parts[1].trim_end_matches(",");
            let s_y = s_y_parts[1].trim_end_matches(":");
            let b_x = b_x_parts[1].trim_end_matches(",");
            let b_y = b_y_parts[1].trim_end_matches(":");

            Sensor {
                sensor: (s_x.parse().unwrap(), s_y.parse().unwrap()),
                beacon: (b_x.parse().unwrap(), b_y.parse().unwrap()),
            }
        })
        .collect();

    let mut map = HashSet::new();
    // let row = 10;
    let row = 2_000_000;
    for s in &sensors {
        let radius = (s.sensor.0 - s.beacon.0).abs() + (s.sensor.1 - s.beacon.1).abs();

        let d = (s.sensor.1 - row).abs();
        if d > radius {
            continue;
        }

        let remainder = radius - d;
        let lx = s.sensor.0 - remainder;
        let rx = s.sensor.0 + remainder;

        for p in lx..=rx {
            map.insert(p);
        }
    }

    let beacons: HashSet<i64> = HashSet::from_iter(
        sensors
            .iter()
            .filter(|s| s.beacon.1 == row)
            .map(|s| s.beacon.0),
    );
    println!("{}", map.len() - beacons.len());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let s_x_parts: Vec<&str> = parts[2].split("=").collect();
            let s_y_parts: Vec<&str> = parts[3].split("=").collect();
            let b_x_parts: Vec<&str> = parts[8].split("=").collect();
            let b_y_parts: Vec<&str> = parts[9].split("=").collect();

            let s_x = s_x_parts[1].trim_end_matches(",");
            let s_y = s_y_parts[1].trim_end_matches(":");
            let b_x = b_x_parts[1].trim_end_matches(",");
            let b_y = b_y_parts[1].trim_end_matches(":");

            Sensor {
                sensor: (s_x.parse().unwrap(), s_y.parse().unwrap()),
                beacon: (b_x.parse().unwrap(), b_y.parse().unwrap()),
            }
        })
        .collect();

    // let row = 20;
    let row = 4_000_000;
    let mut rowdata = vec![vec![0..=row]; row as usize + 1];
    // let row = 2_000_000;
    for s in &sensors {
        let radius = (s.sensor.0 - s.beacon.0).abs() + (s.sensor.1 - s.beacon.1).abs();
        let top = 0.max(s.sensor.1 - radius);
        let bottom = row.min(s.sensor.1 + radius);

        for r in top..=bottom {
            let dist = (s.sensor.1 - r).abs();
            let min_x = 0.max(s.sensor.0 - (radius - dist));
            let max_x = row.min(s.sensor.0 + (radius - dist));
            let mut new_range = vec![];
            for rng in &rowdata[r as usize] {
                let start = *rng.start();
                if start > max_x {
                    new_range.push(rng.clone());
                    continue;
                }

                let end = *rng.end();
                if end < min_x {
                    new_range.push(rng.clone());
                    continue;
                }

                if start < min_x {
                    new_range.push(start..=min_x - 1);
                }

                if end > max_x {
                    new_range.push(max_x + 1..=end);
                }
            }

            rowdata[r as usize] = new_range;
        }
    }

    for (y, r) in rowdata.iter().enumerate() {
        if !r.is_empty() {
            let x = r[0].start();
            println!("{}", x * 4_000_000 + y as i64);
            break;
        }
    }

    Ok(())
}

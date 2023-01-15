use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
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
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("unrecognized char: {item}"),
        }
    }
}

type Point = (i64, i64);

#[derive(Debug, Clone, Default, Hash)]
enum ShapeKind {
    Star,
    HorizontalLine,
    #[default]
    VerticalLine,
    L,
    Square,
}

#[derive(Debug, Clone, Default, Hash)]
struct Shape {
    kind: ShapeKind,
    points: Vec<Point>,
    bottom: i64,
    height: i64,
    width: i64,
}

fn make_shapes() -> Vec<Shape> {
    vec![
        Shape {
            kind: ShapeKind::HorizontalLine,
            points: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            bottom: 0,
            height: 1,
            width: 4,
        },
        Shape {
            kind: ShapeKind::Star,
            points: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            bottom: 0,
            height: 3,
            width: 3,
        },
        Shape {
            kind: ShapeKind::L,
            points: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            bottom: 0,
            height: 3,
            width: 3,
        },
        Shape {
            kind: ShapeKind::VerticalLine,
            points: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            bottom: 0,
            height: 4,
            width: 4,
        },
        Shape {
            kind: ShapeKind::Square,
            points: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            bottom: 0,
            height: 2,
            width: 2,
        },
    ]
}

const RIGHT_EDGE: i64 = 7;

fn part1(input: &str) -> Result<()> {
    let gusts: Vec<Direction> = input.trim_end().chars().map(|c| c.into()).collect();

    let shapes = make_shapes();

    let mut chamber: Vec<Shape> = vec![];

    let mut current_shape_index = 0;
    let mut gust_idx = 0;

    while chamber.len() < 2022 {
        drop_rock(
            &mut chamber,
            &shapes,
            &gusts,
            &mut current_shape_index,
            &mut gust_idx,
        );
    }
    // print_chamber(&chamber, None);

    println!("Rock count: {}", chamber.len());
    let height = chamber_height(&chamber);
    println!("Height: {height}");

    Ok(())
}

fn drop_rock(
    chamber: &mut Vec<Shape>,
    shapes: &Vec<Shape>,
    gusts: &Vec<Direction>,
    current_shape_index: &mut usize,
    gust_idx: &mut usize,
) -> Shape {
    let mut shape = shapes[*current_shape_index % shapes.len()].clone();
    let height = chamber_height(chamber) as i64;
    translate(&chamber, &mut shape, (height + 3, 2), height as usize); // set initial position

    loop {
        let height = chamber_height(&chamber);
        let gust = &gusts[*gust_idx % gusts.len()];
        *gust_idx += 1;
        match gust {
            Direction::Left => translate(&chamber, &mut shape, (0, -1), height),
            Direction::Right => translate(&chamber, &mut shape, (0, 1), height),
        };

        if !translate(&chamber, &mut shape, (-1, 0), height) {
            chamber.push(shape.clone());
            *current_shape_index += 1;
            break;
        }
    }

    shape
}

fn part2(input: &str) -> Result<()> {
    let gusts: Vec<Direction> = input.trim_end().chars().map(|c| c.into()).collect();
    // println!("{gusts:?}");

    let shapes = make_shapes();

    let mut chamber: Vec<Shape> = vec![];
    let mut current_shape_index = 0;
    let mut gust_idx = 0;
    let rock_count = 1_000_000_000_000;
    let mut hashes: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut drops = 0;

    let ihs = hash_state(0, &shapes[0], &chamber);
    hashes.insert(ihs, (0usize, 0usize));

    while chamber.len() < rock_count {
        let shape = drop_rock(
            &mut chamber,
            &shapes,
            &gusts,
            &mut current_shape_index,
            &mut gust_idx,
        );
        drops += 1;

        if chamber.len() < 10 {
            continue;
        }

        let gi = gust_idx % gusts.len();
        let h = hash_state(gi, &shape, &chamber);

        let height = chamber_height(&chamber);

        if let Some(v) = hashes.get(&h) {
            println!("Found cycle...");
            let delta_height = (height as usize) - v.0;
            let delta_drops = drops - v.1;
            let remaining_drops = rock_count - v.1;
            let div = remaining_drops / delta_drops;
            let mmod = remaining_drops % delta_drops;

            let int_height = v.0 + delta_height * div;

            for _ in 0..mmod {
                drop_rock(
                    &mut chamber,
                    &shapes,
                    &gusts,
                    &mut current_shape_index,
                    &mut gust_idx,
                );
            }

            let height_after_drops = chamber_height(&chamber);
            let leftover_height = height_after_drops - height as usize;
            println!("Leftover height: {}", leftover_height);
            println!("Total: {}", int_height + leftover_height as usize);

            break;
        }

        hashes.insert(h, (height as usize, drops));
    }

    // print_chamber(&chamber, None);

    println!("Rock count: {}", chamber.len());
    let height = chamber_height(&chamber);
    println!("Height: {height}");

    Ok(())
}

fn chamber_height(chamber: &Vec<Shape>) -> usize {
    chamber
        .iter()
        .flat_map(|s| &s.points)
        .map(|p| p.0 + 1)
        .max()
        .unwrap_or(0) as usize
}

fn hash_state(gust_idx: usize, shape: &Shape, chamber: &Vec<Shape>) -> u64 {
    if chamber.is_empty() {
        return 0;
    }
    let mut hasher = DefaultHasher::new();
    gust_idx.hash(&mut hasher);
    shape.kind.hash(&mut hasher);
    let shape_idx = chamber.len() - 1;
    let start = shape_idx - 4;
    let height = chamber_height(chamber) as i64;
    let below_height = height - 20;
    let mut i = 0;
    let mut j = 0;
    for r in below_height..height {
        for c in 0..7 {
            for k in start..shape_idx {
                let s = &chamber[k];
                let shape_points: HashSet<&Point> = s.points.iter().collect();
                if shape_points.contains(&(r, c)) {
                    (i, j).hash(&mut hasher);
                }
            }
            j += 1;
        }
        i += 1;
    }

    hasher.finish()
}

fn translate(chamber: &Vec<Shape>, shape: &mut Shape, d: (i64, i64), height: usize) -> bool {
    let mut sandbox = shape.points.clone();
    let mut can_move = true;
    for p in &mut sandbox {
        p.0 += d.0;
        p.1 += d.1;
        if chamber.is_empty() && p.0 < height as i64 {
            can_move = false;
            break;
        }

        if p.1 > RIGHT_EDGE - 1 {
            can_move = false;
            break;
        }

        if p.1 < 0 {
            can_move = false;
            break;
        }
    }

    for shape in chamber.iter().rev() {
        let shape_points: HashSet<&Point> = shape.points.iter().collect();
        for p in &sandbox {
            if shape_points.contains(&p) {
                can_move = false;
                break;
            }
        }
        if !can_move {
            break;
        }
    }

    if can_move {
        shape.points = sandbox;
    }

    can_move
}

fn _print_chamber(chamber: &Vec<Shape>, current: Option<&Shape>) {
    let height = chamber_height(chamber) as i64;
    let mut shapes = chamber.clone();
    if let Some(cur) = current {
        shapes.push(cur.clone());
    }
    let points: HashSet<Point> = shapes.iter().rev().flat_map(|s| s.points.clone()).collect();
    for r in (0..(height + 10)).rev() {
        for c in 0..7 {
            if points.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

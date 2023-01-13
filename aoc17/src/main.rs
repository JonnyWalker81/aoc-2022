use std::{
    collections::HashSet,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

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

type Point = (i32, i32);

#[derive(Debug, Clone, Default)]
enum ShapeKind {
    Star,
    HorizontalLine,
    #[default]
    VerticalLine,
    L,
    Square,
}

#[derive(Debug, Clone, Default)]
struct Shape {
    kind: ShapeKind,
    points: Vec<Point>,
    bottom: i32,
    height: i32,
    width: i32,
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
// const LEFT_EDGE: i32 = 0;
const RIGHT_EDGE: i32 = 7;

fn part1(input: &str) -> Result<()> {
    let gusts: Vec<Direction> = input.trim_end().chars().map(|c| c.into()).collect();
    println!("{gusts:?}");

    let shapes = make_shapes();

    let mut is_movement = true;
    let mut current_shape_index = 0;
    let mut chamber: Vec<Shape> = vec![];
    let mut new_falling_rock = true;
    let mut shape = Shape::default();
    let mut i = 0;
    let mut gust_idx = 0;
    loop {
        i += 1;
        if i % 50 == 0 {
            break;
        }
        if new_falling_rock {
            shape = shapes[current_shape_index % shapes.len()].clone();
            let height: i32 = chamber
                .iter()
                .flat_map(|s| &s.points)
                .map(|p| p.1)
                .max()
                .unwrap_or(0);
            // chamber.push(shape.clone());
            // current_shape_index += 1;
            let top = chamber.last();
            translate(top, &mut shape, (height + 3, 2), height); // set initial position
            new_falling_rock = false;
        }

        print_chamber(&chamber, Some(&shape));
        // let height: i32 = chamber.iter().map(|s| s.height).sum();
        let height: i32 = chamber
            .iter()
            .flat_map(|s| &s.points)
            .map(|p| p.1)
            .max()
            .unwrap_or(0);
        let top = chamber.last();
        if is_movement {
            // move left or right
            let gust = &gusts[gust_idx % gusts.len()];
            gust_idx += 1;
            match gust {
                Direction::Left => translate(top, &mut shape, (0, -1), height),
                Direction::Right => translate(top, &mut shape, (0, 1), height),
            };
        } else {
            // move down
            // let top = chamber.last_mut().unwrap();
            // let height: i32 = chamber.iter().map(|s| s.height).sum();
            if !translate(top, &mut shape, (-1, 0), height) {
                chamber.push(shape.clone());
                current_shape_index += 1;
                new_falling_rock = true;
            }
        }

        // if will_collision(&chamber, &shape) {
        //     // println!("Collision...");
        //     new_falling_rock = true;
        // }

        // if i == 10 {
        //     break;
        // }
        // if chamber.len() >= 2022 {
        if chamber.len() >= 10 {
            break;
        }

        is_movement = !is_movement;
    }

    print_chamber(&chamber, None);

    println!("Rock count: {}", chamber.len());
    let height: i32 = chamber.iter().map(|s| s.height).sum();
    println!("Height: {height}");

    Ok(())
}

fn translate(top: Option<&Shape>, shape: &mut Shape, d: (i32, i32), height: i32) -> bool {
    // shape.bottom += d.0;
    // if shape.bottom < 0 {
    //     shape.bottom = 0;
    // }

    let mut sandbox = shape.points.clone();
    let mut can_move = true;
    for p in &mut sandbox {
        p.0 += d.0;
        p.1 += d.1;
        if top.is_none() && p.0 < height {
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

    if let Some(top) = top {
        let top_points: HashSet<&Point> = top.points.iter().collect();
        for p in &sandbox {
            if top_points.contains(&p) {
                can_move = false;
                break;
            }
        }
    }

    if can_move {
        shape.points = sandbox;
    }

    can_move
}

fn will_collision(chamber: &Vec<Shape>, shape: &Shape) -> bool {
    if chamber.is_empty() {
        if shape.bottom - 1 <= 0 {
            return true;
        }
        false
    } else {
        let mut test_points = vec![];
        for p in &shape.points {
            test_points.push(p.0 - 1);
        }

        let top = chamber.last().unwrap();
        for p in &top.points {
            for tp in &test_points {
                if *tp == p.0 {
                    return true;
                }
            }
        }
        false
    }
}

fn print_chamber(chamber: &Vec<Shape>, current: Option<&Shape>) {
    // let height: i32 = chamber.iter().map(|s| s.height).sum();
    let height: i32 = chamber
        .iter()
        .flat_map(|s| &s.points)
        .map(|p| p.1)
        .max()
        .unwrap_or(0);
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

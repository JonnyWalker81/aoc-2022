use std::io::{self, Read};

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
}

fn make_shapes() -> Vec<Shape> {
    vec![
        Shape {
            kind: ShapeKind::HorizontalLine,
            points: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            bottom: 0,
            height: 1,
        },
        Shape {
            kind: ShapeKind::Star,
            points: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            bottom: 0,
            height: 3,
        },
        Shape {
            kind: ShapeKind::L,
            points: vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)],
            bottom: 0,
            height: 3,
        },
        Shape {
            kind: ShapeKind::VerticalLine,
            points: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            bottom: 0,
            height: 4,
        },
        Shape {
            kind: ShapeKind::Square,
            points: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            bottom: 0,
            height: 2,
        },
    ]
}
const LEFT_EDGE: i32 = 0;
const RIGHT_EDGE: i32 = 7;

fn part1(input: &str) -> Result<()> {
    let gusts: Vec<Direction> = input.trim_end().chars().map(|c| c.into()).collect();
    println!("{gusts:?}");

    let mut shapes = make_shapes();

    let mut is_movement = true;
    let mut current_shape_index = 0;
    let mut chamber: Vec<Shape> = vec![];
    let mut new_falling_rock = true;
    let mut shape = Shape::default();
    let mut i = 0;
    loop {
        if new_falling_rock {
            shape = shapes[current_shape_index % shapes.len()].clone();
            let height: i32 = chamber.iter().map(|s| s.height).sum();
            translate(&mut shape, (height + 4, 2)); // set initial position
            new_falling_rock = false;
        }

        // let shape = chamber.last_mut().unwrap();
        if is_movement {
            // move left or right
            let gust = &gusts[i % gusts.len()];
            match gust {
                Direction::Left => translate(&mut shape, (0, -1)),
                Direction::Right => translate(&mut shape, (0, 1)),
            }
        } else {
            // move down
            translate(&mut shape, (-1, 0))
        }

        if will_collision(&chamber, &shape) {
            // println!("Collision...");
            current_shape_index += 1;
            new_falling_rock = true;
            chamber.push(shape.clone());
        }

        // if i == 10 {
        //     break;
        // }
        // if chamber.len() >= 2022 {
        if chamber.len() >= 2 {
            break;
        }

        is_movement = !is_movement;
        i += 1;
    }

    print_chamber(&chamber);

    println!("Rock count: {}", chamber.len());
    let height: i32 = chamber.iter().map(|s| s.height).sum();
    println!("Height: {height}");

    Ok(())
}

fn translate(shape: &mut Shape, d: (i32, i32)) {
    shape.bottom += d.0;
    if shape.bottom < 0 {
        shape.bottom = 0;
    }

    let mut sandbox = shape.points.clone();
    let mut can_move = true;
    for p in &mut sandbox {
        if p.0 > 0 {
            p.0 += d.0;
        } else {
            can_move = false;
            break;
        }

        if p.1 < RIGHT_EDGE - 1 {
            p.1 += d.1;
            can_move = false;
            break;
        }

        if p.1 < 0 {
            p.1 = 0;
            can_move = false;
            break;
        }

        if p.0 < 0 {
            p.0 = 0;
            can_move = false;
            break;
        }
    }

    if can_move {
        shape.points = sandbox;
    }
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

fn print_chamber(chamber: &Vec<Shape>) {
    let mut row = 0;
    for s in chamber.iter().rev() {
        for r in 0..4 {
            for c in 0..7 {
                let rr = s.points.iter().find(|p| p.0 == (r + row));
                let col = s.points.iter().find(|p| p.1 == c);
                if rr.is_some() && col.is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        row += 4;
    }
}

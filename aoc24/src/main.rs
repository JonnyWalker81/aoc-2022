use std::cmp::{max, min, Ordering};
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{self, Display},
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

type ID = (usize, usize);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Cell {
    #[default]
    Noop,
    Wall,
    Space,
    Blizzard(HashMap<ID, Direction>),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Noop => write!(f, " "),
            Cell::Space => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Blizzard(ref d) => {
                if d.len() > 1 {
                    write!(f, "{}", d.len())
                } else {
                    if d.len() == 1 {
                        write!(f, "{}", d.iter().next().unwrap().1)
                    } else {
                        write!(f, ".")
                    }
                }
            }
        }
    }
}

type Point = (usize, usize);

#[derive(Debug, Default, PartialEq, Eq)]
struct Maze(Vec<Vec<Cell>>);

impl Maze {
    fn get_mut(&mut self, p: Point) -> &mut Cell {
        &mut self.0[p.0][p.1]
    }

    fn get(&self, p: Point) -> &Cell {
        &self.0[p.0][p.1]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p: (usize, usize),
    dist: usize,
    step: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // manhattan(other.p, end_point).cmp(&manhattan(self.p, end_point).then_with(|| other.dist.cmp(&self.dist)
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.step.cmp(&self.step))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part1(input: &str) -> Result<()> {
    let board: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Space,
                    '>' => {
                        Cell::Blizzard(vec![((r, i), Direction::Right)].iter().cloned().collect())
                    }
                    '<' => {
                        Cell::Blizzard(vec![((r, i), Direction::Left)].iter().cloned().collect())
                    }
                    '^' => Cell::Blizzard(vec![((r, i), Direction::Up)].iter().cloned().collect()),
                    'v' => {
                        Cell::Blizzard(vec![((r, i), Direction::Down)].iter().cloned().collect())
                    }
                    _ => panic!("unrecognized: {}", c),
                })
                .collect()
        })
        .collect();

    let mut maze = Maze(board);

    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let start = maze.0[0].iter().position(|c| *c == Cell::Space).unwrap();
    let end = maze.0[maze.0.len() - 1]
        .iter()
        .position(|c| *c == Cell::Space)
        .unwrap();
    let end_point = (maze.0.len() - 1, end);
    // let mut visited: HashSet<(Point, usize)> = HashSet::new();
    let mut visited: HashSet<State> = HashSet::new();
    // visited.insert(((0, start), 0));
    // queue.push_back(((0, start), 0));
    // queue.push(((0, start), 0));
    queue.push(State {
        p: (0, start),
        dist: manhattan((0, start), end_point) as usize,
        step: 1,
    });

    let lcm = lcm(maze.0.len() - 2, maze.0[0].len() - 2);
    println!("lcm: {lcm}");

    let mut steps = 0;
    while !queue.is_empty() {
        // let p = queue.pop_front().unwrap();
        let p = queue.pop().unwrap();

        if visited.contains(&p) {
            continue;
        }
        visited.insert(p.clone());

        // if p.0 == end_point || (p.0.0 == maze.0.len() - 2 && p.0.1 == maze.0[0].len() - 2){
        // if p.0 .0 == maze.0.len() - 2 && p.0 .1 == maze.0[0].len() - 2 {
        //     print_maze(&maze, p.0);
        // }

        // println!("Step: {steps}");
        // print_maze(&maze, p.0);
        if p.p == end_point {
            println!("{}", p.step);
            print_maze(&maze, p.p);
            break;
        }

        // println!("Step: {}", steps);
        // println!("Current: {:?}", p);
        // println!("Queue: {:?}\n", queue);
        // let cell = &maze.get_mut(p);

        // if p.1 < steps {
        //     continue;
        // }
        // can move

        // let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        // for d in dirs {
        //     let move_to = ()
        // }

        // print_maze(&maze, p.0);
        let mut blizzard_moves: HashMap<Point, Vec<(ID, Direction, Point)>> = HashMap::new();
        for r in 0..maze.0.len() {
            for c in 0..maze.0[r].len() {
                if let Cell::Blizzard(v) = maze.get((r, c)) {
                    for (id, d) in v {
                        let mv: (i32, i32) = match d {
                            Direction::Up => (-1, 0),
                            Direction::Down => (1, 0),
                            Direction::Left => (0, -1),
                            Direction::Right => (0, 1),
                        };
                        let mut move_to = (r as i32 + mv.0, c as i32 + mv.1);
                        if move_to.0 < 1 {
                            move_to.0 = maze.0.len() as i32 - 2 as i32;
                        }

                        if move_to.0 >= maze.0.len() as i32 - 1 as i32 {
                            move_to.0 = 1;
                        }

                        if move_to.1 < 1 {
                            move_to.1 = maze.0[0].len() as i32 - 2;
                        }

                        if move_to.1 >= maze.0[0].len() as i32 - 1 {
                            move_to.1 = 1;
                        }

                        let to = (move_to.0 as usize, move_to.1 as usize);
                        blizzard_moves
                            .entry((r, c))
                            .and_modify(|f| f.push((*id, d.clone(), to)))
                            .or_insert(vec![(*id, d.clone(), to)]);
                    }
                }
            }
        }

        if !blizzard_moves.is_empty() {
            for (k, v) in &blizzard_moves {
                for (id, d, to) in v {
                    let from_cell = maze.get_mut(*k);
                    if let Cell::Blizzard(ref mut dd) = from_cell {
                        dd.remove(id);
                        if dd.len() == 0 {
                            *from_cell = Cell::Space;
                        }
                    }
                    // let mut d = match cell {
                    //     Cell::Blizzard(ref d) => d,
                    //     _ => panic!("should not get here"),
                    // };

                    // *cell = Cell::Space;

                    // let dv = v.iter().map(|(dd,_)| )
                    let to_cell = maze.get_mut(*to);
                    if let Cell::Blizzard(ref mut dd) = to_cell {
                        dd.insert(*id, d.clone());
                    }

                    if let Cell::Space = to_cell {
                        *to_cell = Cell::Blizzard(vec![(*id, d.clone())].iter().cloned().collect());
                    }
                }
            }
        }

        // print_maze(&maze, p.0);
        let neighbors = neighbors(&maze, p.p, steps);
        // println!("neighbors: {:?}", neighbors);
        // if neighbors.is_empty() {
        //     queue.clear();
        //     queue.push_back((p.0, p.1 + 1));
        //     // visited.remove(&p);
        // } else {
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];
        for d in dirs {
            let n = ((p.p.0 as i32 + d.0), (p.p.1 as i32 + d.1));

            if n.0 < 1
                || n.0 as usize >= maze.0.len() - 2
                || n.1 < 1
                || n.1 as usize <= maze.0[0].len() - 2
            {
                continue;
            }

            let n = (n.0 as usize, n.1 as usize);
            // if let Cell::Blizzard(..) = maze.get(n) {
            //     continue;
            // }

            match maze.get(n) {
                Cell::Blizzard(_) | Cell::Wall => continue,
                _ => {}
            }

            // if n == p.0 {
            //     queue.clear();
            // }
            // queue.push_back((n, p.1 + 1));
            // queue.push((n, p.1 + 1));
            queue.push(State {
                p: n,
                dist: manhattan(n, end_point) as usize,
                step: p.step + 1,
            });
            // queue.push_back((n, steps));
            // visited.insert(n);
        }
        // println!("Queue (after neighbors): {:?}\n", queue);
        // println!("Visited: {:?}\n", visited);
        // }

        // queue
        //     .make_contiguous()
        //     .sort_by(|a, b| manhattan(a.0, end_point).cmp(&manhattan(b.0, end_point)));
        // queue.make_contiguous().sort_by(|a, b| a.1.cmp(&b.1));

        steps += 1;
        // if steps % 10_0        == 0{
        // if steps % 12 == 0 {
        // println!("Step: {steps}");
        // print_maze(&maze, p.0);
        // }
        // if steps == 50 {
        //     break;
        // }
    }

    // print_maze(&maze, end_point);
    println!("{steps}");
    Ok(())
}

fn manhattan(a: Point, b: Point) -> i32 {
    (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
}

fn neighbors(maze: &Maze, p: Point, step: usize) -> Vec<Point> {
    let mut points = vec![p];
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

    for d in dirs {
        let t = (p.0 as i32 + d.0, p.1 as i32 + d.1);
        if t.0 <= 0 || t.1 < 0 || t.0 as usize >= maze.0.len() || t.1 as usize >= maze.0[0].len() {
            continue;
        }

        let n = (t.0 as usize, t.1 as usize);
        // if visited.contains(&(n, step)) {
        //     continue;
        // }

        if let Cell::Space = maze.get(n) {
            points.push(n);
        }
    }

    points
}

fn print_maze(maze: &Maze, p: Point) {
    for r in 0..maze.0.len() {
        for c in 0..maze.0[r].len() {
            if p == (r, c) {
                print!("E");
            } else {
                print!("{}", maze.0[r][c]);
            }
        }
        println!();
    }
    println!();
}

use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    fmt,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum Cell {
    #[default]
    Noop,
    Wall,
    Space,
    Blizzard(Vec<Direction>),
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
                    write!(f, "{}", d[0])
                }
            }
        }
    }
}

type Point = (usize, usize);

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Maze(Vec<Vec<Cell>>);

impl Maze {
    fn get(&self, p: Point) -> &Cell {
        &self.0[p.0][p.1]
    }

    fn set(&mut self, p: Point, cell: Cell) {
        self.0[p.0][p.1] = cell;
    }

    fn add_blizzard(&mut self, p: Point, d: Direction) {
        match &mut self.0[p.0][p.1] {
            Cell::Blizzard(ref mut v) => {
                v.push(d);
            }
            c @ Cell::Space => {
                *c = Cell::Blizzard(vec![d]);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p: (usize, usize),
    maze: Maze,
    step: usize,
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
        .map(|(_, l)| {
            l.chars()
                .enumerate()
                .map(|(_, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Space,
                    '>' => Cell::Blizzard(vec![Direction::Right]),
                    '<' => Cell::Blizzard(vec![Direction::Left]),
                    '^' => Cell::Blizzard(vec![Direction::Up]),
                    'v' => Cell::Blizzard(vec![Direction::Down]),
                    _ => panic!("unrecognized: {}", c),
                })
                .collect()
        })
        .collect();

    let maze = Maze(board);

    let mut queue: VecDeque<State> = VecDeque::new();
    let start = maze.0[0].iter().position(|c| *c == Cell::Space).unwrap();
    let end = maze.0[maze.0.len() - 1]
        .iter()
        .position(|c| *c == Cell::Space)
        .unwrap();
    let end_point = (maze.0.len() - 1, end);
    let mut visited: HashSet<State> = HashSet::new();
    queue.push_back(State {
        p: (0, start),
        maze: maze.clone(),
        step: 0,
    });

    let lcm = lcm(maze.0.len() - 2, maze.0[0].len() - 2);
    println!("lcm: {lcm}");

    let mut steps = 0;
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        if visited.contains(&p) {
            continue;
        }

        visited.insert(p.clone());
        if p.p == end_point {
            println!("{}", p.step);
            break;
        }

        let moved = move_blizzards(&p.maze);
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];
        for d in dirs {
            let n = ((p.p.0 as i32 + d.0), (p.p.1 as i32 + d.1));

            if n.0 < 0
                || n.0 as usize > moved.0.len() - 1
                || n.1 < 0
                || n.1 as usize > moved.0[0].len() - 1
            {
                continue;
            }

            let np = (n.0 as usize, n.1 as usize);
            match moved.get(np) {
                Cell::Blizzard(_) | Cell::Wall => continue,
                _ => {}
            }

            let state = State {
                p: np,
                maze: moved.clone(),
                step: p.step + 1,
            };
            queue.push_back(state);
        }
        steps += 1;
    }

    println!("{steps}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let board: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(_, l)| {
            l.chars()
                .enumerate()
                .map(|(_, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Space,
                    '>' => Cell::Blizzard(vec![Direction::Right]),
                    '<' => Cell::Blizzard(vec![Direction::Left]),
                    '^' => Cell::Blizzard(vec![Direction::Up]),
                    'v' => Cell::Blizzard(vec![Direction::Down]),
                    _ => panic!("unrecognized: {}", c),
                })
                .collect()
        })
        .collect();

    let maze = Maze(board);

    let mut queue: VecDeque<State> = VecDeque::new();
    let start = maze.0[0].iter().position(|c| *c == Cell::Space).unwrap();
    let end = maze.0[maze.0.len() - 1]
        .iter()
        .position(|c| *c == Cell::Space)
        .unwrap();
    let end_point = (maze.0.len() - 1, end);
    let mut visited: HashSet<State> = HashSet::new();
    queue.push_back(State {
        p: (0, start),
        maze: maze.clone(),
        step: 0,
    });

    let lcm = lcm(maze.0.len() - 2, maze.0[0].len() - 2);
    println!("lcm: {lcm}");

    let mut steps = 0;
    let goals = vec![end_point, (0, start), end_point];
    let mut goal_idx = 0;

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        if visited.contains(&p) {
            continue;
        }

        visited.insert(p.clone());
        if p.p == goals[goal_idx] {
            println!("Part 2: {}", p.step);
            goal_idx += 1;
            if goal_idx >= goals.len() {
                break;
            }
            queue.clear();
            queue.push_back(p.clone());
        }

        let moved = move_blizzards(&p.maze);
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];
        for d in dirs {
            let n = ((p.p.0 as i32 + d.0), (p.p.1 as i32 + d.1));

            if n.0 < 0
                || n.0 as usize > moved.0.len() - 1
                || n.1 < 0
                || n.1 as usize > moved.0[0].len() - 1
            {
                continue;
            }

            let np = (n.0 as usize, n.1 as usize);
            match moved.get(np) {
                Cell::Blizzard(_) | Cell::Wall => continue,
                _ => {}
            }

            let state = State {
                p: np,
                maze: moved.clone(),
                step: p.step + 1,
            };
            queue.push_back(state);
        }
        steps += 1;
    }

    println!("{steps}");
    Ok(())
}

fn move_blizzards(maze: &Maze) -> Maze {
    let mut new_maze = Maze(vec![vec![Cell::Space; maze.0[0].len()]; maze.0.len()]);
    for r in 0..maze.0.len() {
        for c in 0..maze.0[r].len() {
            match maze.get((r, c)) {
                Cell::Wall => new_maze.set((r, c), Cell::Wall),
                Cell::Blizzard(v) => {
                    for d in v {
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
                        new_maze.add_blizzard(to, d.clone());
                    }
                }
                _ => {}
            }
        }
    }

    new_maze
}

fn _print_maze(maze: &Maze, p: Point) {
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

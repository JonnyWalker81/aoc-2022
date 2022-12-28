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

#[derive(Debug, Default)]
enum Operation {
    #[default]
    Noop,
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, Default)]
struct Monkey {
    name: String,
    number: i64,
    operation: Operation,
}

impl From<&str> for Monkey {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts.len() > 2 {
            Self {
                name: parts[0].trim_end_matches(':').to_string(),
                operation: match parts[2] {
                    "+" => Operation::Add(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
                    "-" => Operation::Sub(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
                    "*" => Operation::Mul(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
                    "/" => Operation::Div(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
                    _ => panic!("unrecognized operation: {}", parts[2]),
                },
                ..Default::default()
            }
        } else {
            Self {
                name: parts[0].trim_end_matches(':').to_string(),
                number: parts[1].parse().unwrap(),
                ..Default::default()
            }
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let monkeys: HashMap<String, Monkey> = input
        .lines()
        .map(|l| l.into())
        .map(|m: Monkey| (m.name.clone(), m))
        .collect();
    println!("{monkeys:?}");

    let root = evaluate(&monkeys, "root");
    println!("Root: {}", root);
    Ok(())
}

fn evaluate(monkeys: &HashMap<String, Monkey>, monkey: &str) -> i64 {
    if let Some(m) = monkeys.get(monkey) {
        match &m.operation {
            Operation::Noop => m.number,
            Operation::Add(left_name, right_name) => {
                let left = evaluate(monkeys, &left_name);
                let right = evaluate(monkeys, &right_name);
                left + right
            }
            Operation::Sub(left_name, right_name) => {
                let left = evaluate(monkeys, &left_name);
                let right = evaluate(monkeys, &right_name);
                left - right
            }
            Operation::Mul(left_name, right_name) => {
                let left = evaluate(monkeys, &left_name);
                let right = evaluate(monkeys, &right_name);
                left * right
            }
            Operation::Div(left_name, right_name) => {
                let left = evaluate(monkeys, &left_name);
                let right = evaluate(monkeys, &right_name);
                left / right
            }
        }
    } else {
        0
    }
}

fn part2(input: &str) -> Result<()> {
    let mut monkeys: HashMap<String, Monkey> = input
        .lines()
        .map(|l| l.into())
        .map(|m: Monkey| (m.name.clone(), m))
        .collect();
    println!("{monkeys:?}");

    // let mut human_attempt = 1_000_000_000_000_00i64;
    let mut human_attempt = 1;

    let (left, right) = {
        let root_monkey = monkeys.get("root").unwrap().clone();
        let (left, right) = match &root_monkey.operation {
            Operation::Add(l, r) => (l, r),
            _ => panic!("expected add operation"),
        };
        (left.clone(), right.clone())
    };

    let mut i = 0;
    loop {
        i += 1;
        monkeys.get_mut("humn").unwrap().number = human_attempt;

        let humn_value = evaluate(&monkeys, "humn");

        let l_value = evaluate(&monkeys, &left);
        let r_value = evaluate(&monkeys, &right);

        if l_value == r_value {
            println!("{l_value}");
            println!("{r_value}");
            println!("{humn_value}");
            println!("Iterations: {i}");
            break;
        } else {
            if l_value > r_value {
                let diff = l_value - r_value;
                human_attempt += diff / 500;
            } else {
                let diff = r_value - l_value;
                human_attempt += diff / 500;
            }
        }
        human_attempt += 1;
    }

    Ok(())
}

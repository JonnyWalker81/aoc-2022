use std::sync::{Arc, Mutex};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

#[derive(Debug)]
enum Operation {
    Add(u128),
    Mul(u128),
    Old,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: u128,
    throw_to: (usize, usize),
    inspection_count: u128,
}

fn _sample() -> Result<Vec<Arc<Mutex<Monkey>>>> {
    Ok(vec![
        Arc::new(Mutex::new(Monkey {
            items: vec![79, 98],
            operation: Operation::Mul(19),
            test: 23,
            throw_to: (2, 3),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![54, 65, 75, 74],
            operation: Operation::Add(6),
            test: 19,
            throw_to: (2, 0),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![79, 60, 97],
            operation: Operation::Old,
            test: 13,
            throw_to: (1, 3),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![74],
            operation: Operation::Add(3),
            test: 17,
            throw_to: (0, 1),
            inspection_count: 0,
        })),
    ])
}

fn input() -> Result<Vec<Arc<Mutex<Monkey>>>> {
    Ok(vec![
        Arc::new(Mutex::new(Monkey {
            items: vec![66, 71, 94],
            operation: Operation::Mul(5),
            test: 3,
            throw_to: (7, 4),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![70],
            operation: Operation::Add(6),
            test: 17,
            throw_to: (3, 0),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![62, 68, 56, 65, 94, 78],
            operation: Operation::Add(5),
            test: 2,
            throw_to: (3, 1),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![89, 94, 94, 67],
            operation: Operation::Add(2),
            test: 19,
            throw_to: (7, 0),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![71, 61, 73, 65, 98, 98, 63],
            operation: Operation::Mul(7),
            test: 11,
            throw_to: (5, 6),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![55, 62, 68, 61, 60],
            operation: Operation::Add(7),
            test: 5,
            throw_to: (2, 1),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![93, 91, 69, 64, 72, 89, 50, 71],
            operation: Operation::Add(1),
            test: 13,
            throw_to: (5, 2),
            inspection_count: 0,
        })),
        Arc::new(Mutex::new(Monkey {
            items: vec![76, 50],
            operation: Operation::Old,
            test: 7,
            throw_to: (4, 6),
            inspection_count: 0,
        })),
    ])
}

fn part1() -> Result<()> {
    // let mut monkeys = _sample()?;
    let mut monkeys = input()?;

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let monkey = monkeys[m].clone();
            while !monkey.lock().unwrap().items.is_empty() {
                let item = monkey.lock().unwrap().items.remove(0);
                let new_item = match monkey.lock().unwrap().operation {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::Old => item * item,
                } / 3;
                monkey.lock().unwrap().inspection_count += 1;
                if new_item % monkey.lock().unwrap().test == 0u128 {
                    monkeys[monkey.lock().unwrap().throw_to.0]
                        .lock()
                        .unwrap()
                        .items
                        .push(new_item);
                } else {
                    monkeys[monkey.lock().unwrap().throw_to.1]
                        .lock()
                        .unwrap()
                        .items
                        .push(new_item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| {
        b.lock()
            .unwrap()
            .inspection_count
            .cmp(&a.lock().unwrap().inspection_count)
    });

    let prod: u128 = monkeys
        .iter()
        .take(2)
        .map(|m| m.lock().unwrap().inspection_count)
        .product();
    println!("{:#?}", prod);

    Ok(())
}

fn part2() -> Result<()> {
    // let mut monkeys = _sample()?;
    let mut monkeys = input()?;

    let mod_val: u128 = monkeys.iter().map(|m| m.lock().unwrap().test).product();

    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            let monkey = monkeys[m].clone();
            while !monkey.lock().unwrap().items.is_empty() {
                let item = monkey.lock().unwrap().items.remove(0);
                let new_item = match monkey.lock().unwrap().operation {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::Old => item * item,
                };
                monkey.lock().unwrap().inspection_count += 1;
                if new_item % monkey.lock().unwrap().test == 0u128 {
                    monkeys[monkey.lock().unwrap().throw_to.0]
                        .lock()
                        .unwrap()
                        .items
                        .push(new_item % mod_val);
                } else {
                    monkeys[monkey.lock().unwrap().throw_to.1]
                        .lock()
                        .unwrap()
                        .items
                        .push(new_item % mod_val);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| {
        b.lock()
            .unwrap()
            .inspection_count
            .cmp(&a.lock().unwrap().inspection_count)
    });

    let prod: u128 = monkeys
        .iter()
        .take(2)
        .map(|m| m.lock().unwrap().inspection_count)
        .product();
    println!("{:#?}", prod);

    Ok(())
}

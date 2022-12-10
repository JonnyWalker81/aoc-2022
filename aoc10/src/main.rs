use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts.len() == 1 {
            Self::Noop
        } else {
            let arg: i32 = parts[1].parse().unwrap();
            Self::Addx(arg)
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();

    let mut clock = 0;
    let mut x = 1;
    let mut did_twenty = false;
    let mut signal = 0;
    for i in instructions {
        match i {
            Instruction::Noop => {
                clock += 1;
            }
            Instruction::Addx(v) => {
                for _ in 0..2 {
                    clock += 1;
                    if !did_twenty && clock % 20 == 0 {
                        did_twenty = true;
                        signal += clock * x;
                    } else if did_twenty && (clock + 20) % 40 == 0 {
                        signal += clock * x;
                    }
                }
                x += v;
            }
        }

        if clock >= 220 {
            break;
        }
    }

    println!("Part 1 Signal: {}", signal);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();

    let mut clock: usize = 0;
    let mut x: i32 = 1;
    let mut sprite: i32 = 1;
    let mut pixels = vec![vec!['.'; 40]; 6];
    for i in instructions {
        match i {
            Instruction::Noop => {
                clock += 1;
                let row = (clock / 40) as usize;
                let col = clock % 40;
                if col >= (sprite) as usize && col < (sprite + 3) as usize {
                    pixels[row][col] = '#';
                }
                if clock % 40 == 0 {
                    println!("{:?}", pixels[row - 1]);
                }
            }
            Instruction::Addx(v) => {
                for _ in 0..2 {
                    clock += 1;

                    let row = (clock / 40) as usize;
                    let col = clock % 40;
                    if col >= sprite as usize && col < (sprite + 3) as usize {
                        pixels[row][col] = '#';
                    }
                    if clock % 40 == 0 {
                        println!("{:?}", pixels[row - 1]);
                    }
                }
                x += v;
                sprite = x;
            }
        }

        if clock == 240 {
            break;
        }
    }

    Ok(())
}

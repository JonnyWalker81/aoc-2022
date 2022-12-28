use std::{
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Base5 {
    #[default]
    Two,
    One,
    Zero,
    Minus,
    Equal,
}

impl Base5 {
    fn value(&self) -> i64 {
        match *self {
            Self::Two => 2,
            Self::One => 1,
            Self::Zero => 0,
            Self::Minus => -1,
            Self::Equal => -2,
        }
    }
}

impl From<char> for Base5 {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '1' => Self::One,
            '0' => Self::Zero,
            '-' => Self::Minus,
            '=' => Self::Equal,
            _ => panic!("unrecognized: {c}"),
        }
    }
}

impl From<i64> for Base5 {
    fn from(d: i64) -> Self {
        match d {
            2 => Self::Two,
            1 => Self::One,
            0 => Self::Zero,
            4 => Self::Minus,
            3 => Self::Equal,
            _ => panic!("unrecognized: {d}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Fuel(Vec<Base5>);

impl Fuel {
    fn to_decimal(&self) -> i64 {
        let mut dec = 0;
        let mut place: i64 = 5i64.pow(self.0.len() as u32 - 1);
        println!("{place}");
        for b in &self.0 {
            dec += b.value() * place;
            place /= 5;
        }

        dec
    }

    fn from_decimal(mut d: i64) -> Fuel {
        // let mut base: Vec<Base5> = vec![];
        let mut base: Vec<i64> = vec![];
        let mut place = 0;
        let mut v = d;
        while v > 0 {
            let rem = v % 5;
            base.push(rem);
            v /= 5;
            place += 1;
        }

        // println!("{}", 5i32.pow(place - 1));

        // let mut q = d;
        // for i in (0..place).rev() {
        //     let e = 5i32.pow(i);
        //     // q = q / e;
        //     println!("{}", q / e);
        //     // println!("{}", q % e);
        //     q %= e;
        // }

        // base.reverse();
        println!("{base:?}");

        let mut carry = false;
        let mut snafu = String::new();
        for mut c in base {
            if carry {
                c += 1;
            }
            carry = false;
            match c {
                3 => {
                    carry = true;
                    snafu.push('=');
                }
                4 => {
                    carry = true;
                    snafu.push('-');
                }
                5 => {
                    carry = true;
                    snafu.push('0');
                }
                0 => snafu.push('0'),
                1 => snafu.push('1'),
                2 => snafu.push('2'),
                _ => panic!("should not get here: {}", c),
            }

            if carry {
                // let a = c + 1;
                // snafu.push(char::from_u32(a as u32).unwrap());
                // snafu.push('1');
            }
        }
        let rev_snafu: String = snafu.chars().rev().collect();
        println!("{}", rev_snafu);
        Fuel(vec![])
    }
}

impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            match b {
                Base5::Two => write!(f, "2"),
                Base5::One => write!(f, "1"),
                Base5::Zero => write!(f, "0"),
                Base5::Minus => write!(f, "-"),
                Base5::Equal => write!(f, "="),
            };
        }
        Ok(())
    }
}

impl From<&str> for Fuel {
    fn from(item: &str) -> Self {
        Self(item.chars().map(|c| c.into()).collect())
    }
}

fn part1(input: &str) -> Result<()> {
    let requirements: Vec<Fuel> = input.lines().map(|l| l.into()).collect();

    let mut sum = 0;
    for r in requirements {
        sum += r.to_decimal();
    }

    println!("{sum}");
    let snafu = Fuel::from_decimal(sum);
    println!("{snafu}");
    Ok(())
}

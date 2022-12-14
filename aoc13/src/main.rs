use std::cmp::Ordering;
use std::{
    io::{self, Read},
    iter::Peekable,
    str::Chars,
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l), Self::List(r)) => l == r,
            (Self::Number(l), Self::Number(r)) => l == r,
            (Self::List(l), Self::Number(r)) => l == &vec![Packet::Number(*r)],
            (Self::Number(l), Self::List(r)) => &vec![Packet::Number(*l)] == r,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::List(l), Packet::Number(r)) => l.cmp(&vec![Packet::Number(*r)]),
            (Packet::Number(l), Packet::List(r)) => vec![Packet::Number(*l)].cmp(&r),
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
        }
    }
}

fn parse_packet(item: &str) -> Packet {
    let mut chars: Vec<char> = item.chars().collect();
    let mut pos = 0;
    loop {
        match chars[pos] {
            '[' => {
                pos += 1;
                return parse_list(&mut pos, &mut chars);
            }
            cc @ _ if cc.is_digit(10) => {
                return parse_digits(&mut pos, &mut chars);
            }
            c => {
                panic!("unrecognized char: {:?}", c)
            }
        }
    }
}

fn parse_list(pos: &mut usize, chars: &Vec<char>) -> Packet {
    let mut items = vec![];
    loop {
        if *pos >= chars.len() {
            break;
        }

        match chars[*pos] {
            '[' => {
                *pos += 1;
                items.push(parse_list(pos, chars))
            }
            ',' => {
                *pos += 1;
                continue;
            }
            ']' => {
                *pos += 1;
                break;
            }
            c @ _ if c.is_digit(10) => {
                items.push(parse_digits(pos, chars));
            }
            c => {
                panic!("unrecognized char: {:?}", c)
            }
        };
    }

    Packet::List(items)
}

fn parse_digits(pos: &mut usize, chars: &Vec<char>) -> Packet {
    let mut result = String::new();
    loop {
        match chars[*pos] {
            c @ _ if c.is_digit(10) => {
                *pos += 1;
                result.push(c)
            }
            _ => break,
        }
    }

    Packet::Number(result.parse().unwrap())
}

fn part1(input: &str) -> Result<()> {
    let mut pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|l| l.split("\n").collect::<Vec<&str>>())
        .map(|l| (parse_packet(l[0]), parse_packet(l[1])))
        .collect();

    let mut valid_pairs = vec![];
    for (i, p) in pairs.iter_mut().enumerate() {
        let is_valid = check_order(&mut p.0, &mut p.1);
        // let is_valid = p.0.cmp(&p.1);
        if is_valid == Ordering::Less {
            valid_pairs.push(i + 1);
        }
    }

    println!("{:#?}", pairs);
    println!("{:#?}", valid_pairs);

    let sum: usize = valid_pairs.iter().sum();
    println!("{:#?}", sum);
    Ok(())
}

fn check_order(left: &mut Packet, right: &mut Packet) -> Ordering {
    match (left, right) {
        (Packet::Number(l), Packet::Number(r)) => l.cmp(&r), //Ordering::Greater,
        (Packet::List(l), Packet::List(r)) => {
            let mut i = 0;
            let mut j = 0;
            while i < l.len() && j < r.len() {
                let v = check_order(&mut l[i], &mut r[j]);
                if v == Ordering::Greater || v == Ordering::Less {
                    return v;
                }

                i += 1;
                j += 1;
            }

            if i == l.len() {
                if j == r.len() {
                    return Ordering::Equal;
                }
                return Ordering::Less;
            }

            if j >= r.len() {
                return Ordering::Greater;
            }

            Ordering::Equal
        }
        (Packet::Number(l), Packet::List(r)) => check_order(
            &mut Packet::List(vec![Packet::Number(*l)]),
            &mut Packet::List(r.clone()),
        ),
        (Packet::List(l), Packet::Number(r)) => check_order(
            &mut Packet::List(l.clone()),
            &mut Packet::List(vec![Packet::Number(*r)]),
        ),
    }
}

fn part2(input: &str) -> Result<()> {
    let mut packets: Vec<Packet> = input
        .split("\n\n")
        .map(|l| l.split("\n").collect::<Vec<&str>>())
        .map(|l| vec![parse_packet(l[0]), parse_packet(l[1])])
        .flatten()
        .collect();

    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    packets.push(packet_2.clone());
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(packet_6.clone());

    println!("{}", packets.len());

    packets.sort();

    println!("{:#?}", packets);

    let index_2 = packets
        .iter()
        .enumerate()
        .find(|(_i, p)| p == &&packet_2)
        .unwrap();
    let index_6 = packets
        .iter()
        .enumerate()
        .find(|(_i, p)| p == &&packet_6)
        .unwrap();

    println!("{:?}, {:?}", index_2, index_6);
    let key = (index_2.0 + 1) * (index_6.0 + 1);

    println!("{}", key);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // let input = "[[1],4]";
        let input = "[]";

        let packet = parse_packet(input);
        println!("{:?}", packet);
    }
}

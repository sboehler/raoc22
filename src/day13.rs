use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::Path;
use std::str::{Chars, FromStr};
use Packet::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<usize> {
    Ok(load(p)?
        .chunks(2)
        .enumerate()
        .filter_map(|(i, ps)| match &ps {
            &[a, b] if a <= b => Some(i + 1),
            _ => None,
        })
        .sum())
}

pub fn compute2(p: &Path) -> Result<usize> {
    let mut packets = load(p)?;
    let div1: Packet = "[[2]]".parse()?;
    let div2: Packet = "[[6]]".parse()?;
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    match (packets.binary_search(&div1), packets.binary_search(&div2)) {
        (Ok(i), Ok(j)) => Ok((i + 1) * (j + 1)),
        _ => return Err("something bad happened".into()),
    }
}

fn load(p: &Path) -> Result<Vec<Packet>> {
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(Lines::collect::<io::Result<Vec<String>>>)?
        .iter()
        .filter(|s| !s.is_empty())
        .map(|l| l.parse().map_err(String::into))
        .collect()
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Packet {
    Int(isize),
    List(Vec<Packet>),
}

impl Packet {
    fn from_chars(chars: &mut Peekable<Chars>) -> Result<Packet> {
        let mut vs = Vec::new();
        while let Some(c) = chars.next() {
            let pkt = match c {
                '[' => Packet::from_chars(chars)?,
                '0'..='9' => {
                    let mut s = String::from(c);
                    while let Some(d) = chars.next_if(|ch| ch.is_numeric()) {
                        s.push(d)
                    }
                    Packet::Int(s.parse::<isize>()?)
                }
                ']' => break,
                _ => return Err(format!("expected [ or number, got {}", c).into()),
            };
            vs.push(pkt);
            match chars.next() {
                Some(']') => break,
                Some(',') => continue,
                Some(c) => return Err(format!("expected ']' or ',', got '{}'", c).into()),
                None => return Err("unexpected end of input".into()),
            }
        }
        Ok(Packet::List(vs))
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        if let Some('[') = chars.next() {
            Packet::from_chars(&mut chars)
                .map_err(|e| format!("invalid packet: {} error: {}", s, e))
        } else {
            Err(format!("invalid packet: {}", s))
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Int(a), List(b)) => vec![Int(*a)].cmp(b),
            (List(a), Int(b)) => a.cmp(&vec![Int(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day13_part1_example() {
        assert_eq!(
            compute1(Path::new("src/inputs/day13_example.txt")).unwrap(),
            13
        );
    }
    #[test]
    fn day13_part1_input() {
        assert_eq!(
            compute1(Path::new("src/inputs/day13_input.txt")).unwrap(),
            5852
        );
    }

    #[test]
    fn day13_part2_example() {
        assert_eq!(
            compute2(Path::new("src/inputs/day13_example.txt")).unwrap(),
            140
        );
    }

    #[test]
    fn day13_part2_input() {
        assert_eq!(
            compute2(Path::new("src/inputs/day13_input.txt")).unwrap(),
            24190
        );
    }
}

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Part {
    One,
    Two,
}

pub fn compute1(p: &Path) -> Result<usize> {
    let ss: Vec<String> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| line.map_err(io::Error::into))
                .collect::<Result<Vec<_>>>()
        })?;
    let mut monkeys = parse_monkeys(&ss)?;
    let mut inspections: HashMap<usize, usize> = (0..monkeys.len())
        .map(|m| (m, 0))
        .collect::<HashMap<_, _>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let res = monkeys[i].inspect(|x| x / 3);
            inspections.get_mut(&i).map(|v| *v += res.len());
            res.iter().for_each(|t| {
                monkeys[t.to_monkey].items.push_back(t.item);
            });
        }
    }
    let mut res = inspections.values().collect::<Vec<_>>();
    res.sort();
    res.reverse();
    Ok(res.iter().take(2).map(|r| *r).product())
}

pub fn compute2(p: &Path) -> Result<usize> {
    let ss: Vec<String> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| line.map_err(io::Error::into))
                .collect::<Result<Vec<_>>>()
        })?;
    let mut monkeys = parse_monkeys(&ss)?;
    let mut inspections: HashMap<usize, usize> = (0..monkeys.len())
        .map(|m| (m, 0))
        .collect::<HashMap<_, _>>();

    let m: usize = monkeys
        .iter()
        .map(|m| match m.test {
            Test::DivisibleBy(x) => x,
        })
        .product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let res = monkeys[i].inspect(|x| x % m);
            inspections.get_mut(&i).map(|v| *v += res.len());
            res.iter().for_each(|t| {
                monkeys[t.to_monkey].items.push_back(t.item);
            });
        }
    }
    let mut res = inspections.values().collect::<Vec<_>>();
    res.sort();
    res.reverse();
    Ok(res.iter().take(2).map(|r| *r).product())
}

fn parse_monkeys(ss: &[String]) -> Result<Vec<Monkey>> {
    ss.split(|s| s.is_empty())
        .map(|l| Monkey::parse(l))
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Op,
    pub test: Test,
    pub if_true: usize,
    pub if_false: usize,
}

struct Throw {
    item: usize,
    to_monkey: usize,
}

#[derive(Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug)]
enum Test {
    DivisibleBy(usize),
}

impl Monkey {
    fn parse(ss: &[String]) -> Result<Monkey> {
        match *ss[0].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Monkey", _] => (),
            _ => return Err(format!("invalid first line: {}", ss[0]).into()),
        };
        let items = ss[1]
            .strip_prefix("  Starting items: ")
            .ok_or(format!("invalid line: {}", ss[1]))?
            .split(", ")
            .map(|i| i.parse::<usize>())
            .collect::<std::result::Result<VecDeque<_>, ParseIntError>>()?;
        let operation = match *ss[2].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Operation:", "new", "=", "old", "*", "old"] => Op::Square,
            ["Operation:", "new", "=", "old", op, operand] => {
                let n: usize = operand.parse().unwrap();
                match op {
                    "*" => Op::Mul(n),
                    "+" => Op::Add(n),
                    _ => return Err(format!("invalid operation: {}", op).into()),
                }
            }
            _ => return Err(format!("invalid third line: {}", ss[2]).into()),
        };
        let test: Test = match *ss[3].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Test:", "divisible", "by", divisor] => Test::DivisibleBy(divisor.parse()?),
            _ => return Err(format!("invalid fourth line: {}", ss[2]).into()),
        };
        let if_true: usize = match *ss[4].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["If", "true:", "throw", "to", "monkey", m] => m.parse()?,
            _ => return Err(format!("invalid fifth line: {}", ss[5]).into()),
        };
        let if_false: usize = match *ss[5].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["If", "false:", "throw", "to", "monkey", m] => m.parse()?,
            _ => return Err(format!("invalid sixth line: {}", ss[5]).into()),
        };
        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
        })
    }

    fn inspect<F>(&mut self, worry_fn: F) -> Vec<Throw>
    where
        F: Fn(usize) -> usize,
    {
        self.items
            .drain(..)
            .map(|level| match self.operation {
                Op::Add(x) => level + x,
                Op::Mul(x) => level * x,
                Op::Square => level * level,
            })
            .map(worry_fn)
            .map(|level| {
                let to_monkey = match self.test {
                    Test::DivisibleBy(x) => {
                        if level % x == 0 {
                            self.if_true
                        } else {
                            self.if_false
                        }
                    }
                };
                Throw {
                    item: level,
                    to_monkey: to_monkey,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day11_part1_example() {
        assert_eq!(
            compute1(Path::new("src/inputs/day11_example.txt")).unwrap(),
            10605
        );
    }
    #[test]
    fn day11_part1_input() {
        assert_eq!(
            compute1(Path::new("src/inputs/day11_input.txt")).unwrap(),
            67830
        );
    }

    #[test]
    fn day11_part2_example() {
        assert_eq!(
            compute2(Path::new("src/inputs/day11_example.txt")).unwrap(),
            2713310158
        );
    }

    #[test]
    fn day11_part2_input() {
        assert_eq!(
            compute2(Path::new("src/inputs/day11_input.txt")).unwrap(),
            15305381442
        );
    }
}

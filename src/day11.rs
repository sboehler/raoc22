use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<usize> {
    let monkeys = load_monkeys(p)?;
    Ok(chase(monkeys, 20, |x| x / 3))
}

pub fn compute2(p: &Path) -> Result<usize> {
    let monkeys = load_monkeys(p)?;
    let period: usize = monkeys.iter().map(|m| m.divisible_by).product();
    Ok(chase(monkeys, 10000, |x| x % period))
}

fn chase<F>(mut monkeys: Vec<Monkey>, rounds: usize, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut inspections = HashMap::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            *inspections.entry(i).or_insert(0) += monkeys[i]
                .inspect(&f)
                .iter()
                .inspect(|t| {
                    monkeys[t.to_monkey].items.push_back(t.item);
                })
                .count();
        }
    }
    let mut res = inspections.values().copied().collect::<Vec<_>>();
    res.sort();
    res.iter().rev().take(2).product()
}

fn load_monkeys(p: &Path) -> Result<Vec<Monkey>> {
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(Lines::collect::<io::Result<Vec<_>>>)?
        .split(String::is_empty)
        .map(Monkey::parse)
        .collect()
}

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Op,
    pub divisible_by: usize,
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
            .map(str::parse::<usize>)
            .collect::<std::result::Result<VecDeque<_>, _>>()?;
        let operation = match *ss[2].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Operation:", "new", "=", "old", "*", "old"] => Op::Square,
            ["Operation:", "new", "=", "old", op, operand] => {
                let n: usize = operand
                    .parse()
                    .map_err(|e| format!("invalid third line: {} {}", ss[2], e))?;
                match op {
                    "*" => Op::Mul(n),
                    "+" => Op::Add(n),
                    _ => return Err(format!("invalid operation: {}", op).into()),
                }
            }
            _ => return Err(format!("invalid third line: {}", ss[2]).into()),
        };
        let divisible_by: usize = match *ss[3].split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Test:", "divisible", "by", divisor] => divisor.parse()?,
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
            divisible_by,
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
            .map(|level| Throw {
                item: level,
                to_monkey: match level % self.divisible_by {
                    0 => self.if_true,
                    _ => self.if_false,
                },
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

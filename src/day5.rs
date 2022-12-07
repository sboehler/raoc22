use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<String> {
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|mut lines| {
            let mut stacks = parse_stacks(&mut lines)?;
            let moves = parse_moves(&mut lines)?;
            for mv in moves {
                stacks.apply(&mv)?
            }
            Ok(stacks.read_top())
        })
}

pub fn compute2(p: &Path) -> Result<String> {
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|mut lines| {
            let mut stacks = parse_stacks(&mut lines)?;
            let moves = parse_moves(&mut lines)?;
            for mv in moves {
                stacks.apply2(&mv)?;
            }
            Ok(stacks.read_top())
        })
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(n: usize) -> Self {
        let mut stacks = Vec::new();
        for _ in 0..n {
            stacks.push(Vec::new())
        }
        Stacks { stacks: stacks }
    }

    pub fn apply(&mut self, m: &Move) -> Result<()> {
        for _ in 0..m.nbr {
            let v = self.stacks[m.from]
                .pop()
                .ok_or_else(|| format!("invalid move: {:?}", m))?;
            self.stacks[m.to].push(v);
        }
        Ok(())
    }

    pub fn apply2(&mut self, m: &Move) -> Result<()> {
        let mut tmp = Vec::new();
        for _ in 0..m.nbr {
            let v = self.stacks[m.from]
                .pop()
                .ok_or_else(|| format!("invalid move: {:?}", m))?;
            tmp.push(v);
        }
        for _ in 0..m.nbr {
            let v = tmp.pop().ok_or_else(|| format!("invalid move: {:?}", m))?;
            self.stacks[m.to].push(v);
        }
        Ok(())
    }

    pub fn read_top(&self) -> String {
        self.stacks
            .iter()
            .flat_map(|s| s.last())
            .collect::<String>()
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    nbr: usize,
}

fn parse_stacks(ls: &mut std::io::Lines<BufReader<File>>) -> Result<Stacks> {
    let mut lines = Vec::new();
    for line in ls {
        let ln = line?;
        if ln.is_empty() {
            break;
        }
        lines.push(ln)
    }
    lines.pop();
    let line_length = lines
        .iter()
        .map(String::len)
        .max()
        .ok_or_else(|| "no stack lines found")?;
    let nbr_stacks = (line_length + 1) / 4;
    let mut res = Stacks::new(nbr_stacks);
    let positions = (0..nbr_stacks).map(|i| 4 * i + 1).collect::<Vec<_>>();
    for line in lines.iter().rev() {
        let bs = line.as_bytes();
        for (i, pos) in positions.iter().enumerate() {
            let ch = bs[*pos] as char;
            if ch != ' ' {
                res.stacks[i].push(ch)
            }
        }
    }
    Ok(res)
}

fn parse_moves(ls: &mut std::io::Lines<BufReader<File>>) -> Result<Vec<Move>> {
    ls.map(|line| {
        line.map_err(io::Error::into).and_then(|s| {
            let ss = s.split_whitespace().collect::<Vec<_>>();
            match ss.as_slice() {
                &["move", nbr, "from", from, "to", to] => Ok(Move {
                    nbr: nbr.parse::<usize>()?,
                    from: from.parse::<usize>()? - 1,
                    to: to.parse::<usize>()? - 1,
                }),
                _ => Err(format!("invalid line: {:?}", ss).into()),
            }
        })
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day5_part1_example() {
        assert_eq!(
            compute1(Path::new("src/day5/example.txt")).unwrap(),
            "CMZ".to_string()
        );
    }
    #[test]
    fn day5_part1_input() {
        assert_eq!(
            compute1(Path::new("src/day5/input.txt")).unwrap(),
            "PSNRGBTFT".to_string()
        );
    }
    #[test]
    fn day5_part2_example() {
        assert_eq!(
            compute2(Path::new("src/day5/example.txt")).unwrap(),
            "MCD".to_string()
        );
    }
    #[test]
    fn day5_part2_input() {
        assert_eq!(
            compute2(Path::new("src/day5/input.txt")).unwrap(),
            "BNTZFPMMW".to_string()
        );
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/**
Day #5, Part 1:
*/
pub fn compute(p: &Path) -> Result<String> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();
    let mut stacks = parse_stacks(&mut lines)?;
    let moves = parse_moves(&mut lines)?;
    for mv in moves {
        stacks.apply(&mv)?;
    }
    Ok(stacks.read_top())
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
    let mut res = Vec::new();
    for line in ls {
        let ln = line?;
        let tokens = ln.split_whitespace().collect::<Vec<_>>();
        if tokens.len() != 6 {
            return Err(format!("invalid line: {}", ln).into());
        }
        let m = Move {
            nbr: tokens[1].parse::<usize>()?,
            from: tokens[3].parse::<usize>()? - 1,
            to: tokens[5].parse::<usize>()? - 1,
        };
        res.push(m)
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day5() {
        use super::*;
        use std::path::Path;

        assert_eq!(
            compute(Path::new("src/day5/example.txt")).unwrap(),
            "CMZ".to_string()
        );
        assert_eq!(
            compute(Path::new("src/day5/input.txt")).unwrap(),
            "PSNRGBTFT".to_string()
        );
        // assert_eq!(compute(Path::new("src/day5/example.txt")).unwrap(), 0);
        // assert_eq!(compute(Path::new("src/day5/input.txt")).unwrap(), 0);
    }
}

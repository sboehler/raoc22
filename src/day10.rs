use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use Inst::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<isize> {
    let instrs: Vec<Inst> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| line.map_err(io::Error::into).and_then(|s| parse(&s)))
                .collect::<Result<Vec<_>>>()
        })?;
    let mut cpu = CPU::new(instrs);
    let times = vec![20, 60, 100, 140, 180, 220];
    Ok(times
        .iter()
        .map(|t| {
            while cpu.cycle < *t {
                cpu.tick();
            }
            cpu.x * (*t as isize)
        })
        .sum::<isize>())
}

#[derive(Clone, Copy, Debug)]
enum Inst {
    AddX(isize),
    NoOp,
}

#[derive(Debug)]
struct CPU {
    program: Vec<Inst>,
    cycle: usize,
    icycles: usize,
    pc: usize,
    x: isize,
    cur: Option<Inst>,
}

impl CPU {
    pub fn new(p: Vec<Inst>) -> Self {
        CPU {
            program: p,
            cycle: 0,
            icycles: 0,
            cur: None,
            pc: 0,
            x: 1,
        }
    }

    pub fn tick(&mut self) {
        match self.cur {
            None => self.load(),
            Some(inst) => match inst {
                AddX(_) if self.icycles == 0 => {
                    self.icycles += 1;
                }
                AddX(dx) => {
                    self.x += dx;
                    self.load();
                }
                NoOp => {
                    self.load();
                }
            },
        }
        self.cycle += 1;
    }

    fn load(&mut self) {
        self.cur = self.program.get(self.pc).copied();
        self.pc += 1;
        self.icycles = 0;
    }
}

fn parse(s: &str) -> Result<Inst> {
    match *s.split_whitespace().collect::<Vec<&str>>().as_slice() {
        ["addx", dx] => Ok(AddX(dx.parse::<isize>()?)),
        ["noop"] => Ok(NoOp),
        _ => return Err(format!("invalid instruction: {}", s).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day10_part1_example() {
        assert_eq!(
            compute1(Path::new("src/inputs/day10_example.txt")).unwrap(),
            13140
        );
    }
    #[test]
    fn day10_part1_input() {
        assert_eq!(
            compute1(Path::new("src/inputs/day10_input.txt")).unwrap(),
            12880
        );
    }
    // #[test]
    // fn day10_part2_example() {
    //     assert_eq!(
    //         compute(Path::new("src/inputs/day10_example.txt"), 10).unwrap(),
    //         1
    //     );
    // }
    // #[test]
    // fn day10_part2_input() {
    //     assert_eq!(
    //         compute(Path::new("src/inputs/day10_input.txt"), 10).unwrap(),
    //         2449
    //     );
    // }
}

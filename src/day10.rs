use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use Instruction::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<isize> {
    let mut cpu = CPU::new(load_instructions(p)?);
    Ok((20..=220)
        .step_by(40)
        .map(|t| {
            cpu.tick_until(t);
            cpu.x * (t as isize)
        })
        .sum::<isize>())
}

pub fn compute2(p: &Path) -> Result<String> {
    let mut cpu = CPU::new(load_instructions(p)?);
    cpu.tick_until(240);
    Ok(cpu.display)
}

fn load_instructions(p: &Path) -> Result<Vec<Instruction>> {
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| line.map_err(io::Error::into).and_then(|s| parse(&s)))
                .collect::<Result<Vec<_>>>()
        })
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddX(isize),
    NoOp,
}

#[derive(Debug)]
struct CPU {
    program: Vec<Instruction>,
    cycle: usize,
    icycles: usize,
    pc: usize,
    x: isize,
    cur: Option<Instruction>,

    display: String,
}

impl CPU {
    fn new(p: Vec<Instruction>) -> Self {
        CPU {
            program: p,
            cycle: 0,
            icycles: 0,
            cur: None,
            pc: 0,
            x: 1,
            display: String::new(),
        }
    }

    fn tick_until(&mut self, cycle: usize) {
        while self.cycle < cycle {
            match self.cur {
                Some(AddX(_)) if self.icycles == 0 => {
                    self.icycles += 1;
                }
                Some(AddX(dx)) => {
                    self.x += dx;
                    self.load();
                }
                Some(NoOp) => {
                    self.load();
                }
                None => self.load(),
            }
            self.cycle += 1;

            let pos = (self.cycle as isize - 1) % 40;
            let is_match = (pos - self.x).abs() <= 1;
            self.display.push(if is_match { '#' } else { '.' });
            if pos == 39 {
                self.display.push('\n')
            }
        }
    }

    fn load(&mut self) {
        self.cur = self.program.get(self.pc).copied();
        self.pc += 1;
        self.icycles = 0;
    }
}

fn parse(s: &str) -> Result<Instruction> {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    match *tokens.as_slice() {
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
    #[test]
    fn day10_part2_example() {
        let want = "\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\n";
        assert_eq!(
            compute2(Path::new("src/inputs/day10_example.txt")).unwrap(),
            want
        );
    }
    #[test]
    fn day10_part2_input() {
        let want = "\
        ####..##....##..##..###....##.###..####.\n\
        #....#..#....#.#..#.#..#....#.#..#.#....\n\
        ###..#.......#.#..#.#..#....#.#..#.###..\n\
        #....#.......#.####.###.....#.###..#....\n\
        #....#..#.#..#.#..#.#....#..#.#.#..#....\n\
        #.....##...##..#..#.#.....##..#..#.####.\n";
        assert_eq!(
            compute2(Path::new("src/inputs/day10_input.txt")).unwrap(),
            want
        );
    }
}

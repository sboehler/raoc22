use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute(p: &Path, l: usize) -> Result<usize> {
    let mvs: Vec<Move> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| line.map_err(io::Error::into).and_then(|s| parse(&s)))
                .collect::<Result<Vec<_>>>()
        })?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut rope = vec![Point(0, 0); l];

    Ok(mvs
        .iter()
        .map(|mv| {
            rope[0].apply(*mv);
            *rope
                .iter_mut()
                .reduce(|prev, cur| {
                    cur.pull(*prev);
                    cur
                })
                .unwrap()
        })
        .collect::<HashSet<_>>()
        .len())
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point(isize, isize);

impl Point {
    pub fn apply(&mut self, mv: Move) {
        match mv {
            Move::Left => self.0 -= 1,
            Move::Right => self.0 += 1,
            Move::Up => self.1 += 1,
            Move::Down => self.1 -= 1,
        };
    }

    pub fn pull(&mut self, prev: Self) {
        let dx = prev.0 - self.0;
        let dy = prev.1 - self.1;
        if (dx * dx + dy * dy) > 2 {
            self.0 += dx.signum();
            self.1 += dy.signum();
        }
    }
}

fn parse(s: &str) -> Result<Vec<Move>> {
    match *s.split_whitespace().collect::<Vec<&str>>().as_slice() {
        [dir, times] => {
            let mv = match dir {
                "R" => Move::Right,
                "L" => Move::Left,
                "U" => Move::Up,
                "D" => Move::Down,
                _ => return Err(format!("invalid move: {}", s).into()),
            };
            Ok(vec![mv; times.parse::<usize>()?])
        }
        _ => return Err(format!("invalid move: {}", s).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day09_part1_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day09_example.txt"), 2).unwrap(),
            13
        );
    }
    #[test]
    fn day09_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day09_input.txt"), 2).unwrap(),
            6236
        );
    }
    #[test]
    fn day09_part2_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day09_example.txt"), 10).unwrap(),
            1
        );
    }
    #[test]
    fn day09_part2_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day09_input.txt"), 10).unwrap(),
            2449
        );
    }
}

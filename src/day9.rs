use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute(p: &Path) -> Result<usize> {
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

    let mut pos = Pos {
        head: Point(0, 0),
        tail: Point(0, 0),
    };
    let mut set = HashSet::new();
    set.insert(pos.tail);
    for mv in mvs {
        pos = pos.apply(mv);
        set.insert(pos.tail);
    }
    Ok(set.len())
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos {
    pub head: Point,
    pub tail: Point,
}

impl Pos {
    pub fn apply(self, mv: Move) -> Pos {
        let head = match mv {
            Move::Left => Point(self.head.0 - 1, self.head.1),
            Move::Right => Point(self.head.0 + 1, self.head.1),
            Move::Up => Point(self.head.0, self.head.1 + 1),
            Move::Down => Point(self.head.0, self.head.1 - 1),
        };
        let dx = head.0 - self.tail.0;
        let dy = head.1 - self.tail.1;
        if (dx * dx + dy * dy) <= 2 {
            Pos {
                head,
                tail: self.tail,
            }
        } else {
            let tail = Point(self.tail.0 + dx.signum(), self.tail.1 + dy.signum());
            Pos { head, tail }
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
    fn day9_part1_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day9_example.txt")).unwrap(),
            13
        );
    }
    #[test]
    fn day9_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day9_input.txt")).unwrap(),
            6236
        );
    }
}

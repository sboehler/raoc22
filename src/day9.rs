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

    let mut pos = Pos {
        rope: vec![Point(0, 0); l],
    };
    let mut set = HashSet::new();
    set.insert(Point(0, 0));
    for mv in mvs {
        pos.apply(mv);
        set.insert(pos.rope[pos.rope.len() - 1]);
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    pub rope: Vec<Point>,
}

impl Pos {
    pub fn apply(&mut self, mv: Move) {
        let head = &mut self.rope[0];

        *head = match mv {
            Move::Left => Point(head.0 - 1, head.1),
            Move::Right => Point(head.0 + 1, head.1),
            Move::Up => Point(head.0, head.1 + 1),
            Move::Down => Point(head.0, head.1 - 1),
        };
        let mut head = self.rope[0];
        for mut tail in self.rope.iter_mut().skip(1) {
            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;
            if (dx * dx + dy * dy) > 2 {
                tail.0 += dx.signum();
                tail.1 += dy.signum();
            }
            head = *tail
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
            compute(Path::new("src/inputs/day9_example.txt"), 2).unwrap(),
            13
        );
    }
    #[test]
    fn day9_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day9_input.txt"), 2).unwrap(),
            6236
        );
    }
    #[test]
    fn day9_part2_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day9_example.txt"), 10).unwrap(),
            1
        );
    }
    #[test]
    fn day9_part2_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day9_input.txt"), 10).unwrap(),
            2449
        );
    }
}

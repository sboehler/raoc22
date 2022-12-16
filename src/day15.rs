use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path, y: isize) -> Result<usize> {
    let mut v: Vec<(isize, isize)> = load(p)?
        .iter()
        .map(|s| s.x_range_without_beacon(y))
        .filter(|r| !r.is_empty())
        .flat_map(|r| vec![(*r.start(), 1), (*r.end() + 1, -1)])
        .collect();
    v.sort();
    let mut res: Vec<(isize, isize)> = Vec::new();
    let mut i = isize::MIN;
    for sp in v {
        if sp.0 != i {
            i = sp.0;
            res.push(sp);
        } else {
            let i = res.len() - 1;
            res[i].1 += sp.1
        }
    }
    let mut sum = 0;
    let mut lvl = 0;
    let mut last = isize::MIN;
    for sl in res {
        if last > isize::MIN && lvl > 0 {
            sum += (last..sl.0).len();
        }
        last = sl.0;
        lvl += sl.1;
    }
    Ok(sum)
}

pub fn compute2(p: &Path, rng: RangeInclusive<isize>) -> Result<isize> {
    let sensors = load(p)?;
    let (lines1, lines2): (Vec<Line>, Vec<Line>) = sensors
        .iter()
        .flat_map(|s| s.lines())
        .partition(|l| l.m > 0);
    let mut candidates = lines1
        .iter()
        .flat_map(|l| lines2.iter().flat_map(|l2| l.intersect(l2)))
        .filter(|pos| rng.contains(&pos.x) && rng.contains(&pos.y))
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.dedup();
    candidates
        .iter()
        .find(|pos| !sensors.iter().any(|s| s.contains(pos)))
        .map(|p| 4000000 * p.x + p.y)
        .ok_or("not found".into())
}

fn load(p: &Path) -> Result<Vec<Sensor>> {
    Ok(File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(Lines::collect::<io::Result<Vec<String>>>)?
        .iter()
        .map(|l| l.parse())
        .collect::<Result<_>>()?)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn distance(&self, other: &Self) -> isize {
        return (other.x - self.x).abs() + (other.y - self.y).abs();
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Line {
    m: isize,
    b: isize,
}

impl Line {
    fn intersect(&self, other: &Self) -> Vec<Pos> {
        let mut res = Vec::new();
        if self.m == other.m {
            return res;
        }
        let x = (other.b - self.b) / (self.m - other.m);
        let y = self.m * x + self.b;
        res.push(Pos { x, y });
        if other.b - self.b % 2 != 0 {
            res.push(Pos { x: x + 1, y });
            res.push(Pos { x, y: y + 1 });
            res.push(Pos { x: x + 1, y: y + 1 });
        }
        res
    }
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn x_range_without_beacon(&self, y: isize) -> RangeInclusive<isize> {
        let dx = self.pos.distance(&self.beacon) - (y - self.pos.y).abs();
        let x0 = self.pos.x - dx + (self.beacon.y == y && self.beacon.x <= self.pos.x) as isize;
        let x1 = self.pos.x + dx - (self.beacon.y == y && self.beacon.x >= self.pos.x) as isize;
        x0..=x1
    }

    fn contains(&self, pos: &Pos) -> bool {
        return self.pos.distance(pos) <= self.pos.distance(&self.beacon);
    }

    fn lines(&self) -> Vec<Line> {
        let d = self.pos.distance(&self.beacon) + 1;
        let mut res = Vec::new();
        for m in (-1..=1).step_by(2) {
            for n in (-1..=1).step_by(2) {
                let b = self.pos.y - m * self.pos.x + n * d;
                res.push(Line { m, b });
            }
        }
        res
    }
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            &["Sensor", "at", sx, sy, "closest", "beacon", "is", "at", bx, by] => Ok(Sensor {
                pos: Pos {
                    x: parse_coordinate(sx)?,
                    y: parse_coordinate(sy)?,
                },
                beacon: Pos {
                    x: parse_coordinate(bx)?,
                    y: parse_coordinate(by)?,
                },
            }),
            _ => return Err(format!("invalid line: {}", s).into()),
        }
    }
}

fn parse_coordinate(s: &str) -> Result<isize> {
    Ok(s.chars()
        .filter(|c| c.is_numeric() || *c == '-')
        .collect::<String>()
        .parse::<isize>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const EXAMPLE: &str = "src/inputs/day15_example.txt";
    const INPUT: &str = "src/inputs/day15_input.txt";

    #[test]
    fn day15_part1_example() {
        assert_eq!(compute1(Path::new(EXAMPLE), 10).unwrap(), 26);
    }
    #[test]
    fn day15_part1_input() {
        assert_eq!(compute1(Path::new(INPUT), 2000000).unwrap(), 4873353);
    }
    #[test]
    fn day15_part2_example() {
        assert_eq!(compute2(Path::new(EXAMPLE), 0..=20).unwrap(), 56000011);
    }

    #[test]
    fn day15_part2_input() {
        assert_eq!(
            compute2(Path::new(INPUT), 0..=4000000).unwrap(),
            11600823139120
        );
    }
}

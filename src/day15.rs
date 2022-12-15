use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path, y: isize) -> Result<usize> {
    let sensors = load(p)?;
    Ok(sensors
        .range_x(y)
        .filter(|x| sensors.contains(Pos { x: *x, y }))
        // .inspect(|x| println!("{}, ", x))
        .count())
}

fn load(p: &Path) -> Result<Sensors> {
    Ok(Sensors(
        File::open(p)
            .map_err(io::Error::into)
            .map(BufReader::new)
            .map(BufRead::lines)
            .and_then(Lines::collect::<io::Result<Vec<String>>>)?
            .iter()
            .map(|l| l.parse())
            .collect::<Result<_>>()?,
    ))
}

enum Range {
    Start(isize),
    End(isize),
    Beacon(isize),
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn distance(&self, other: &Self) -> isize {
        return (other.x - self.x).abs() + (other.y - self.y).abs();
    }
}
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn range_x(&self, y: isize) -> RangeInclusive<isize> {
        let dx = self.pos.distance(&self.beacon) - (y - self.pos.y).abs();
        (self.pos.x - dx)..=(self.pos.x + dx)
    }

    fn contains(&self, pos: &Pos) -> bool {
        return self.pos.distance(pos) <= self.pos.distance(&self.beacon);
    }

    fn is_beacon(&self, pos: &Pos) -> bool {
        return self.beacon == *pos || self.pos == *pos;
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

struct Sensors(Vec<Sensor>);

impl Sensors {
    fn range_x(&self, y: isize) -> RangeInclusive<isize> {
        self.0.iter().fold(isize::MAX..=isize::MIN, |r, s| {
            let d = s.range_x(y);
            if !d.is_empty() {
                min(*r.start(), *d.start())..=max(*r.end(), *d.end())
            } else {
                r
            }
        })
    }

    fn contains(&self, pos: Pos) -> bool {
        self.0.iter().any(|s| s.contains(&pos)) && !self.0.iter().any(|s| s.is_beacon(&pos))
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

    // #[test]
    // fn day15_part2_example() {
    //     assert_eq!(compute2(Path::new(EXAMPLE)).unwrap(), 140);
    // }

    // #[test]
    // fn day15_part2_input() {
    //     assert_eq!(compute2(Path::new(INPUT)).unwrap(), 24190);
    // }
}

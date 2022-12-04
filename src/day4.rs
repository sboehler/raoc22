use std::cmp::{max, min};
use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::ops::RangeInclusive;
use std::path::Path;

/**
Day #4, Part 1:
```
use raoc22::day4::{compute, overlap, fully_contained};
use std::path::Path;
assert_eq!(compute(Path::new("src/day4/example.txt"), fully_contained).unwrap(), 2);
assert_eq!(compute(Path::new("src/day4/input.txt"), fully_contained).unwrap(), 547);
assert_eq!(compute(Path::new("src/day4/example.txt"), overlap).unwrap(), 4);
assert_eq!(compute(Path::new("src/day4/input.txt"), overlap).unwrap(), 843);
```
*/
pub fn compute<F>(p: &Path, pred: F) -> Result<i64>
where
    F: Fn(&RangeInclusive<i64>, &RangeInclusive<i64>) -> bool,
{
    let f = File::open(p)?;
    BufReader::new(f)
        .lines()
        .map(|res| {
            res.and_then(|l| decode(&l))
                .map(|(r1, r2)| pred(&r1, &r2) as i64)
        })
        .sum()
}

fn decode(s: &str) -> Result<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    s.split_once(',')
        .ok_or_else(|| err(format!("invalid line: {}", s)))
        .and_then(|(a, b)| {
            let x = decode_range(a)?;
            let y = decode_range(b)?;
            Ok((x, y))
        })
}

fn decode_range(s: &str) -> Result<RangeInclusive<i64>> {
    s.split_once('-')
        .ok_or_else(|| err(format!("invalid range: {}", s)))
        .and_then(|(a, b)| {
            let x = parse_int(a)?;
            let y = parse_int(b)?;
            Ok(x..=y)
        })
}

fn parse_int(s: &str) -> Result<i64> {
    s.parse::<i64>().map_err(|e| err(e))
}

fn err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

pub fn fully_contained(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    return r1.start() <= r2.start() && r1.end() >= r2.end()
        || r2.start() <= r1.start() && r2.end() >= r1.end();
}

pub fn overlap(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    return max(r1.start(), r2.start()) <= min(r1.end(), r2.end());
}

use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    F: Fn(&(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool,
{
    let f = File::open(p)?;
    let mut res: i64 = 0;
    for line in BufReader::new(f).lines() {
        let ln = line?;
        let ranges = decode(&ln)?;
        if pred(&ranges) {
            res += 1
        }
    }
    Ok(res)
}

fn decode(s: &str) -> Result<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    s.split_once(',')
        .ok_or_else(|| format!("invalid line: {}", s).into())
        .and_then(|(a, b)| {
            let x = decode_range(a)?;
            let y = decode_range(b)?;
            Ok((x, y))
        })
}

fn decode_range(s: &str) -> Result<RangeInclusive<i64>> {
    s.split_once('-')
        .ok_or_else(|| format!("invalid range: {}", s).into())
        .and_then(|(a, b)| {
            let x = a.parse::<i64>()?;
            let y = b.parse::<i64>()?;
            Ok(x..=y)
        })
}

pub fn fully_contained(r: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool {
    return r.0.start() <= r.1.start() && r.0.end() >= r.1.end()
        || r.1.start() <= r.0.start() && r.1.end() >= r.0.end();
}

pub fn overlap(r: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool {
    return max(r.0.start(), r.1.start()) <= min(r.0.end(), r.1.end());
}

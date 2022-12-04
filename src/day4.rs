use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
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
    let lines = BufReader::new(f).lines();

    let mut res = 0;
    for line in lines {
        let (r1, r2) = decode(&line?);
        if pred(&r2, &r1) {
            res += 1;
        }
    }
    Ok(res)
}

fn decode(s: &str) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let ts: Vec<i64> = s
        .split(',')
        .flat_map(|t| t.split('-'))
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    (ts[0]..=ts[1], ts[2]..=ts[3])
}

pub fn fully_contained(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    return r1.start() <= r2.start() && r1.end() >= r2.end()
        || r2.start() <= r1.start() && r2.end() >= r1.end();
}

pub fn overlap(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    return max(r1.start(), r2.start()) <= min(r1.end(), r2.end());
}

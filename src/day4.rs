use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::{Range, RangeInclusive};
use std::path::Path;

/**
Day #4, Part 1:
```
use raoc22::day4::compute1;
use std::path::Path;
assert_eq!(compute1(Path::new("src/day4/example.txt")).unwrap(), 2);
assert_eq!(compute1(Path::new("src/day4/input.txt")).unwrap(), 547);
```
*/
pub fn compute1(p: &Path) -> Result<i64> {
    let f = File::open(p)?;
    let lines = BufReader::new(f).lines();

    let mut res = 0;
    for line in lines {
        let (r1, r2) = decode(&line?);
        if fully_contains(&r1, &r2) || fully_contains(&r2, &r1) {
            res = res + 1;
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

fn fully_contains(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    return r1.start() <= r2.start() && r1.end() >= r2.end();
}

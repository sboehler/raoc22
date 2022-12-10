use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute<F>(p: &Path, pred: F) -> Result<usize>
where
    F: Fn(&(RangeInclusive<i64>, RangeInclusive<i64>)) -> bool,
{
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|reader| {
            reader
                .map(|line| {
                    line.map_err(io::Error::into)
                        .and_then(|s| decode(&s))
                        .map(|rs| pred(&rs) as usize)
                })
                .sum()
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day04_part1_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day04_example.txt"), fully_contained).unwrap(),
            2
        );
    }
    #[test]
    fn day04_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day04_input.txt"), fully_contained).unwrap(),
            547
        );
    }
    #[test]
    fn day04_part2_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day04_example.txt"), overlap).unwrap(),
            4
        );
    }
    #[test]
    fn day04_part2_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day04_input.txt"), overlap).unwrap(),
            843
        );
    }
}

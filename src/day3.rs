use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<i64> {
    let f = File::open(p)?;
    let lines = BufReader::new(f).lines();
    let mut res = 0;
    for line in lines {
        let s = line?;
        let (s1, s2) = s.split_at(s.len() / 2);
        let a: HashSet<_> = s1.chars().collect();
        let b: HashSet<_> = s2.chars().collect();
        res += a.intersection(&b).map(score).sum::<i64>()
    }
    Ok(res)
}

pub fn compute2(p: &Path) -> Result<i64> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();
    let mut res = 0;
    while let Some(line) = lines.next() {
        if let (Some(s1), Some(s2)) = (lines.next(), lines.next()) {
            let dupes: HashSet<_> = line?.chars().collect();
            let h1: HashSet<_> = s1?.chars().collect();
            let h2: HashSet<_> = s2?.chars().collect();

            res += dupes
                .iter()
                .filter(|c| h1.contains(c) && h2.contains(c))
                .map(score)
                .sum::<i64>()
        } else {
            return Err("invalid input, want more lines".into());
        }
    }
    Ok(res)
}

fn score(c: &char) -> i64 {
    let v = *c as i64;
    match v {
        65..=90 => v - 38,
        97..=122 => v - 96,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day3_part1_example() {
        assert_eq!(compute1(Path::new("src/day3/example.txt")).unwrap(), 157);
    }
    #[test]
    fn day3_part1_input() {
        assert_eq!(compute1(Path::new("src/day3/input.txt")).unwrap(), 7831);
    }
    #[test]
    fn day3_part2_example() {
        assert_eq!(compute2(Path::new("src/day3/example.txt")).unwrap(), 70);
    }
    #[test]
    fn day3_part2_input() {
        assert_eq!(compute2(Path::new("src/day3/input.txt")).unwrap(), 2683);
    }
}

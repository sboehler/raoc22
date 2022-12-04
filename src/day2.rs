use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/**
Day #2
```
use raoc22::day2::{compute, decode1, decode2};
use std::path::Path;
assert_eq!(compute(Path::new("src/day2/example.txt"), decode1).unwrap(), 15);
assert_eq!(compute(Path::new("src/day2/input.txt"), decode1).unwrap(), 8392);
assert_eq!(compute(Path::new("src/day2/example.txt"), decode2).unwrap(), 12);
assert_eq!(compute(Path::new("src/day2/input.txt"), decode2).unwrap(), 10116);
```
*/
pub fn compute<F>(p: &Path, decode: F) -> Result<i64>
where
    F: Fn(&str, &str) -> i64,
{
    let f = File::open(p)?;
    let mut res: i64 = 0;
    for line in BufReader::new(f).lines() {
        let s = line?;
        res += match s.split_once(' ') {
            Some((a, b)) => decode(a, b),
            None => return Err(format!("invalid string: {}", s).into()),
        }
    }
    Ok(res)
}

pub fn decode1(s1: &str, s2: &str) -> Result<i64> {
    let m = Move::from_str(s2)?;
    let t = Move::from_str(s1)?;
    Ok(Move::compute_score(m, t))
}

pub fn decode2(s1: &str, s2: &str) -> Result<i64> {
    let theirs = Move::from_str(s1)?;
    let res = Outcome::from_str(s2)?;
    let mine = Move::from_i64(((theirs as i64) + (res as i64) + 2) % 3);
    Ok(Move::compute_score(mine, theirs))
}

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn compute_score(mine: Move, theirs: Move) -> i64 {
        let m = mine as i64;
        let t = theirs as i64;
        3 * ((m - t + 4) % 3) + m + 1
    }
    fn from_i64(i: i64) -> Move {
        match i {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("invalid move: {}", i),
        }
    }

    fn from_str(s: &str) -> Result<Move> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("invalid move: {}", s).into()),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}
impl Outcome {
    fn from_str(s: &str) -> Result<Outcome> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("invalid outcome: {}", s).into()),
        }
    }
}

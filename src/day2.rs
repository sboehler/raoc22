use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{error, io};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn compute<F>(p: &Path, decode: F) -> Result<i64>
where
    F: Fn(&str, &str) -> Result<i64>,
{
    File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|lines| {
            lines
                .map(|line| {
                    line.map_err(io::Error::into).and_then(|s| {
                        s.split_once(' ')
                            .ok_or_else(|| format!("invalid string: {}", s).into())
                            .and_then(|(a, b)| decode(a, b))
                    })
                })
                .sum()
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day2_part1_example() {
        assert_eq!(
            compute(Path::new("src/day2/example.txt"), decode1).unwrap(),
            15
        );
    }
    #[test]
    fn day2_part1_input() {
        assert_eq!(
            compute(Path::new("src/day2/input.txt"), decode1).unwrap(),
            8392
        );
    }
    #[test]
    fn day2_part2_example() {
        assert_eq!(
            compute(Path::new("src/day2/example.txt"), decode2).unwrap(),
            12
        );
    }
    #[test]
    fn day2_part2_input() {
        assert_eq!(
            compute(Path::new("src/day2/input.txt"), decode2).unwrap(),
            10116
        );
    }
}

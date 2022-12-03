use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn compute(p: &Path) -> io::Result<i64> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();

    let mut res = 0;
    while let Some(line) = lines.next().transpose()? {
        let mut s = HashSet::<_>::from_iter(line.chars());
        for _ in 0..2 {
            let line = lines
                .next()
                .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, ""))??;
            let chars = line.chars().collect::<HashSet<_>>();
            s.retain(|c| chars.contains(c))
        }
        s.iter().for_each(|c| res += score(*c));
    }
    Ok(res)
}

fn score(c: char) -> i64 {
    let v = c as i64;
    match v {
        65..=90 => v - 38,
        97..=122 => v - 96,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute(Path::new("src/day3/example.txt")).unwrap(), 70);
        assert_eq!(compute(Path::new("src/day3/input.txt")).unwrap(), 2683);
    }
}

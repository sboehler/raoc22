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
    while let Some(line) = lines.next() {
        let mut s = line?.chars().collect::<HashSet<_>>();
        for _ in 0..2 {
            let t = lines.next().ok_or_else(unexpected_eof)??;
            let l = t.chars().collect::<HashSet<_>>();
            s.retain(|c| l.contains(c))
        }
        s.iter().for_each(|c| res += score(*c));
    }
    Ok(res)
}

fn unexpected_eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "")
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

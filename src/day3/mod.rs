use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/**
Day #3, Part 1:
```
use raoc22::day3::compute1;
use std::path::Path;
assert_eq!(compute1(Path::new("src/day3/example.txt")).unwrap(), 157);
assert_eq!(compute1(Path::new("src/day3/input.txt")).unwrap(), 7831);
```
*/
pub fn compute1(p: &Path) -> io::Result<i64> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();

    let mut res = 0;
    while let Some(line) = lines.next() {
        let ln = line?;
        let bs = ln.as_bytes();
        let l = bs.len() / 2;
        let a: HashSet<_> = bs[0..l].iter().collect();
        let b: HashSet<_> = bs[l..].iter().collect();
        for c in a.intersection(&b) {
            res += score(**c as char)
        }
    }
    Ok(res)
}

/**
Day #3, Part 2:
```
use raoc22::day3::compute2;
use std::path::Path;
assert_eq!(compute2(Path::new("src/day3/example.txt")).unwrap(), 70);
assert_eq!(compute2(Path::new("src/day3/input.txt")).unwrap(), 2683);
```
 */
pub fn compute2(p: &Path) -> io::Result<i64> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();

    let mut res = 0;
    while let Some(line) = lines.next() {
        let mut s: HashSet<char> = line?.chars().collect();
        for _ in 0..2 {
            let t = lines.next().ok_or_else(unexpected_eof)??;
            let l: HashSet<char> = t.chars().collect();
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

use std::fs::File;
use std::io::{BufRead, Result};
use std::{io::BufReader, path::Path};

/**
Day #3, Parts 1 & 2:
```
use raoc22::day1::compute;
use std::path::Path;

assert_eq!(compute(Path::new("src/day1/example.txt"), 1).unwrap(), 24000);
assert_eq!(compute(Path::new("src/day1/input.txt"), 1).unwrap(), 70116);
assert_eq!(compute(Path::new("src/day1/example.txt"), 3).unwrap(), 45000);
assert_eq!(compute(Path::new("src/day1/input.txt"), 3).unwrap(), 206582);
```
*/
pub fn compute(p: &Path, n: usize) -> Result<i64> {
    let f = File::open(p)?;
    let mut lines = BufReader::new(f).lines();
    let mut max = Max::new(n);
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let ln = line?;
        if ln.len() == 0 {
            max.update(sum);
            sum = 0;
        } else {
            sum += ln.parse::<i64>().unwrap();
        }
    }
    max.update(sum);
    Ok(max.sum())
}

struct Max {
    max: Vec<i64>,
}

impl Max {
    pub fn new(len: usize) -> Self {
        return Max { max: vec![0; len] };
    }
    pub fn update(&mut self, v: i64) {
        if let Some(min) = self.max.iter_mut().min() {
            if v > *min {
                *min = v
            }
        }
    }
    pub fn sum(&self) -> i64 {
        self.max.iter().sum()
    }
}

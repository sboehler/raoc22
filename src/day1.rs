use std::error;
use std::fs::File;
use std::io::BufRead;
use std::{io::BufReader, path::Path};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
    let lines = BufReader::new(f).lines();
    let mut max = Max::new(n);
    let mut sum = 0;
    for line in lines {
        let ln = line?;
        if ln.is_empty() {
            max.update(sum);
            sum = 0;
        } else {
            sum += ln.parse::<i64>()?;
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
        Max { max: vec![0; len] }
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

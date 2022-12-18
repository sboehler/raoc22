use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<usize> {
    let mut v = load(p)?.iter().flat_map(Point::faces).collect::<Vec<_>>();
    let l = v.len();
    v.sort();
    v.dedup();
    Ok(2 * v.len() - l)
}

fn load(p: &Path) -> Result<Vec<Point>> {
    Ok(File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(Lines::collect::<io::Result<Vec<String>>>)?
        .iter()
        .map(|l| l.parse())
        .collect::<Result<_>>()?)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Surface {
    origin: Point,
    orientation: Orientation,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Orientation {
    XY,
    YZ,
    XZ,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn faces(&self) -> Vec<Surface> {
        vec![
            Surface {
                origin: *self,
                orientation: Orientation::XY,
            },
            Surface {
                origin: *self,
                orientation: Orientation::YZ,
            },
            Surface {
                origin: *self,
                orientation: Orientation::XZ,
            },
            Surface {
                origin: Point {
                    x: self.x,
                    y: self.y,
                    z: self.z + 1,
                },
                orientation: Orientation::XY,
            },
            Surface {
                origin: Point {
                    x: self.x + 1,
                    y: self.y,
                    z: self.z,
                },
                orientation: Orientation::YZ,
            },
            Surface {
                origin: Point {
                    x: self.x,
                    y: self.y + 1,
                    z: self.z,
                },
                orientation: Orientation::XZ,
            },
        ]
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.splitn(3, ",").collect::<Vec<&str>>().as_slice() {
            &[x, y, z] => Ok(Point {
                x: x.parse::<isize>()?,
                y: y.parse::<isize>()?,
                z: z.parse::<isize>()?,
            }),
            _ => return Err(format!("invalid line: {}", s).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const EXAMPLE: &str = "src/inputs/day18_example.txt";
    const INPUT: &str = "src/inputs/day18_input.txt";

    #[test]
    fn day18_part1_example() {
        assert_eq!(compute1(Path::new(EXAMPLE)).unwrap(), 64);
    }
    #[test]
    fn day18_part1_input() {
        assert_eq!(compute1(Path::new(INPUT)).unwrap(), 3454);
    }
    // #[test]
    // fn day18_part2_example() {
    //     assert_eq!(compute2(Path::new(EXAMPLE), 0..=20).unwrap(), 56000011);
    // }

    // #[test]
    // fn day18_part2_input() {
    //     assert_eq!(
    //         compute2(Path::new(INPUT), 0..=4000000).unwrap(),
    //         11600823139120
    //     );
    // }
}

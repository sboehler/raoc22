use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<usize> {
    let mut width = 0;
    let trees1: Vec<Vec<Tree>> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|lines| {
            lines
                .map(|line| {
                    line.map_err(io::Error::into).map(|s| {
                        width = s.len();
                        s.chars()
                            .map(|ch| Tree {
                                height: ch as isize - 48,
                                visible: false,
                            })
                            .collect()
                    })
                })
                .collect::<Result<Vec<_>>>()
        })?;

    let height = trees1.len();
    // todo: be smarter about creating this vector directly from the file.
    let mut trees = trees1.into_iter().flatten().collect::<Vec<Tree>>();
    let l = trees.len();

    for line_offset in (0..l).step_by(height) {
        fill_visibility(&mut trees, line_offset..line_offset + width);
        fill_visibility(&mut trees, (line_offset..line_offset + width).rev())
    }
    for col_offset in 0..width {
        fill_visibility(&mut trees, (col_offset..l).step_by(height));
        fill_visibility(&mut trees, (col_offset..l).step_by(height).rev());
    }
    Ok(trees.iter().filter(|t| t.visible).count())
}

fn fill_visibility<I>(trees: &mut Vec<Tree>, iter: I)
where
    I: Iterator<Item = usize>,
{
    let mut h = -1;
    for i in iter {
        let mut tree = &mut trees[i];
        if tree.height > h {
            tree.visible = true;
            h = tree.height;
        }
    }
}

pub fn compute2(p: &Path) -> Result<usize> {
    let mut width = 0;
    let trees1: Vec<Vec<Tree>> = File::open(p)
        .map_err(io::Error::into)
        .map(BufReader::new)
        .map(BufRead::lines)
        .and_then(|lines| {
            lines
                .map(|line| {
                    line.map_err(io::Error::into).map(|s| {
                        width = s.len();
                        s.chars()
                            .map(|ch| Tree {
                                height: ch as isize - 48,
                                visible: false,
                            })
                            .collect()
                    })
                })
                .collect::<Result<Vec<_>>>()
        })?;

    let height = trees1.len();
    // todo: be smarter about creating this vector directly from the file.
    let mut trees = trees1.into_iter().flatten().collect::<Vec<Tree>>();
    Ok(determine_score(&mut trees, width, height))
}

fn determine_score(trees: &mut Vec<Tree>, width: usize, height: usize) -> usize {
    let mut max = 0;
    for line in 1..height - 1 {
        let line_offset = line * height;
        for col in 1..width - 1 {
            let idx = line * height + col;
            let h = trees[idx].height;
            let right = (line_offset + col + 1..line_offset + width)
                .position(|i| trees[i].height >= h)
                .map(|i| i + 1)
                .unwrap_or(width - col - 1);

            let left = (line_offset..line_offset + col)
                .rev()
                .position(|i| trees[i].height >= h)
                .map(|i| i + 1)
                .unwrap_or(col);

            let bottom = (idx + height..width * height)
                .step_by(height)
                .position(|i| trees[i].height >= h)
                .map(|i| i + 1)
                .unwrap_or(height - line - 1);

            let top = (col..idx)
                .step_by(height)
                .rev()
                .position(|i| trees[i].height >= h)
                .map(|i| i + 1)
                .unwrap_or(line);
            let score = right * left * top * bottom;
            if score > max {
                max = score
            }
        }
    }
    max
}

#[derive(Clone, Copy, Debug)]
struct Tree {
    pub height: isize,
    pub visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day8_part1_example() {
        assert_eq!(
            compute1(Path::new("src/inputs/day8_example.txt")).unwrap(),
            21
        );
    }
    #[test]
    fn day8_part1_input() {
        assert_eq!(
            compute1(Path::new("src/inputs/day8_input.txt")).unwrap(),
            1789
        );
    }
    #[test]
    fn day8_part2_example() {
        assert_eq!(
            compute2(Path::new("src/inputs/day8_example.txt")).unwrap(),
            8
        );
    }
    #[test]
    fn day8_part2_input() {
        assert_eq!(
            compute2(Path::new("src/inputs/day8_input.txt")).unwrap(),
            314820
        );
    }
}

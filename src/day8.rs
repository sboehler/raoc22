use std::fs::{self};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute1(p: &Path) -> Result<usize> {
    let s = fs::read_to_string(p)?;
    let width = s.bytes().position(|b| b == b'\n').unwrap_or(s.len());
    let trees = s
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| ch as isize - 48)
        .collect::<Vec<_>>();

    Ok(part1(&trees, width))
}

fn part1(trees: &[isize], width: usize) -> usize {
    let len = trees.len();
    let height = trees.len() / width;
    let mut visible = vec![false; trees.len()];

    for line_idx in (0..len).step_by(height) {
        fill_visibility(trees, &mut visible, line_idx..line_idx + width);
        fill_visibility(trees, &mut visible, (line_idx..line_idx + width).rev())
    }
    for col_idx in 0..width {
        fill_visibility(trees, &mut visible, (col_idx..len).step_by(height));
        fill_visibility(trees, &mut visible, (col_idx..len).step_by(height).rev());
    }
    visible.iter().filter(|t| **t).count()
}

fn fill_visibility<I>(heights: &[isize], visible: &mut [bool], iter: I)
where
    I: Iterator<Item = usize>,
{
    iter.fold(-1, |max, i| {
        if heights[i] > max {
            visible[i] = true;
            heights[i]
        } else {
            max
        }
    });
}

pub fn compute2(p: &Path) -> Result<usize> {
    let s = fs::read_to_string(p)?;
    let width = s.bytes().position(|b| b == b'\n').unwrap_or(s.len());
    let trees = s
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| ch as isize - 48)
        .collect::<Vec<_>>();

    Ok(part2(&trees, width))
}

fn part2(trees: &[isize], width: usize) -> usize {
    let height = trees.len() / width;
    let mut max = 0;
    for line in 1..height - 1 {
        let line_offset = line * height;
        for col in 1..width - 1 {
            let idx = line * height + col;
            let h = trees[idx];
            let right = (line_offset + col + 1..line_offset + width)
                .position(|i| trees[i] >= h)
                .map(|i| i + 1)
                .unwrap_or(width - col - 1);

            let left = (line_offset..line_offset + col)
                .rev()
                .position(|i| trees[i] >= h)
                .map(|i| i + 1)
                .unwrap_or(col);

            let bottom = (idx + height..width * height)
                .step_by(height)
                .position(|i| trees[i] >= h)
                .map(|i| i + 1)
                .unwrap_or(height - line - 1);

            let top = (col..idx)
                .step_by(height)
                .rev()
                .position(|i| trees[i] >= h)
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

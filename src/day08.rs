use std::fs::{self};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Part {
    One,
    Two,
}

pub fn compute(p: &Path, part: Part) -> Result<usize> {
    let s = fs::read_to_string(p)?;
    let matrix = Matrix {
        width: s.bytes().position(|b| b == b'\n').unwrap_or(s.len()),
        values: s
            .chars()
            .filter(|ch| *ch != '\n')
            .map(|ch| ch as isize - 48)
            .collect::<Vec<_>>(),
    };
    match part {
        Part::One => Ok(part1(&matrix)),
        Part::Two => Ok(part2(&matrix)),
    }
}

fn part1(matrix: &Matrix<isize>) -> usize {
    let width = matrix.width;
    let height = matrix.height();
    let mut visible = vec![false; matrix.values.len()];

    for row in 0..height {
        fill_visibility(matrix, &mut visible, matrix.row(row, 0, width));
        fill_visibility(matrix, &mut visible, matrix.row(row, 0, width).rev());
    }
    for col in 0..width {
        fill_visibility(matrix, &mut visible, matrix.col(col, 0, height));
        fill_visibility(matrix, &mut visible, matrix.col(col, 0, height).rev());
    }
    visible.iter().filter(|t| **t).count()
}

fn fill_visibility<I>(matrix: &Matrix<isize>, visible: &mut [bool], iter: I)
where
    I: Iterator<Item = usize>,
{
    iter.fold(isize::MIN, |max, i| {
        if matrix.values[i] > max {
            visible[i] = true;
            matrix.values[i]
        } else {
            max
        }
    });
}

fn part2(matrix: &Matrix<isize>) -> usize {
    let width = matrix.width;
    let height = matrix.height();

    (1..height - 1)
        .flat_map(|row| {
            (1..width - 1).map(move |col| {
                let h = matrix.values[matrix.idx(row, col)];
                let right = matrix
                    .row(row, col + 1, width)
                    .position(|i| matrix.values[i] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(width - col - 1);

                let left = matrix
                    .row(row, 0, col)
                    .rev()
                    .position(|i| matrix.values[i] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(col);

                let bottom = matrix
                    .col(col, row + 1, height)
                    .position(|i| matrix.values[i] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(height - row - 1);

                let top = matrix
                    .col(col, 0, row)
                    .rev()
                    .position(|i| matrix.values[i] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(row);
                right * left * top * bottom
            })
        })
        .max()
        .unwrap_or(usize::MIN)
}

struct Matrix<T> {
    pub values: Vec<T>,
    pub width: usize,
}

impl<T> Matrix<T> {
    fn height(&self) -> usize {
        self.values.len() / self.width
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        self.height() * row + col
    }

    fn col(
        &self,
        col: usize,
        start: usize,
        end: usize,
    ) -> impl DoubleEndedIterator<Item = usize> + ExactSizeIterator {
        let height = self.values.len() / self.width;
        (col..end * height).step_by(height).skip(start)
    }

    fn row(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> impl DoubleEndedIterator<Item = usize> + ExactSizeIterator {
        let height = self.values.len() / self.width;
        row * height + start..row * height + end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day08_part1_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day08_example.txt"), Part::One).unwrap(),
            21
        );
    }
    #[test]
    fn day08_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day08_input.txt"), Part::One).unwrap(),
            1789
        );
    }
    #[test]
    fn day08_part2_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day08_example.txt"), Part::Two).unwrap(),
            8
        );
    }
    #[test]
    fn day08_part2_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day08_input.txt"), Part::Two).unwrap(),
            314820
        );
    }
}

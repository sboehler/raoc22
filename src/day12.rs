use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::{self};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compute(p: &Path, part2: bool) -> Result<usize> {
    let s = fs::read_to_string(p)?;
    let mut start = 0;
    let mut end = 0;
    let width = s.bytes().position(|b| b == b'\n').unwrap_or(s.len());
    let area = s
        .chars()
        .filter(|ch| *ch != '\n')
        .enumerate()
        .map(|(i, ch)| match ch {
            'S' => {
                start = i;
                0
            }
            'E' => {
                end = i;
                'z' as isize - 97
            }
            ch @ 'a'..='z' => ch as isize - 97,
            _ => {
                panic!("invalid character: {}", ch)
            }
        })
        .collect::<Vec<_>>();
    let height = area.len() / width;
    let mut min_steps = vec![usize::MAX; area.len()];
    min_steps[start] = 0;
    let mut todo = BinaryHeap::new();
    todo.push(Item { min: 0, idx: start });
    while let Some(cur) = todo.pop() {
        if cur.idx == end {
            break;
        }
        let row = cur.idx / width;
        let col = cur.idx % width;
        if row > 0 {
            let neighbor = (row - 1) * width + col;
            update(&area, &mut min_steps, &mut todo, cur, neighbor, part2)
        }
        if row < height - 1 {
            let neighbor = (row + 1) * width + col;
            update(&area, &mut min_steps, &mut todo, cur, neighbor, part2)
        }
        if col > 0 {
            let neighbor = row * width + col - 1;
            update(&area, &mut min_steps, &mut todo, cur, neighbor, part2)
        }
        if col < width - 1 {
            let neighbor = row * width + col + 1;
            update(&area, &mut min_steps, &mut todo, cur, neighbor, part2)
        }
    }
    Ok(min_steps[end])
}

fn update(
    heights: &[isize],
    min_steps: &mut [usize],
    q: &mut BinaryHeap<Item>,
    cur: Item,
    next: usize,
    part2: bool,
) {
    if part2 && heights[next] == 0 {
        if min_steps[next] > 0 {
            q.push(Item { idx: next, min: 0 });
            min_steps[next] = 0;
            return;
        };
    }
    if heights[next] - heights[cur.idx] > 1 {
        // not an edge
        return;
    }
    let min = min_steps[cur.idx] + 1;
    if min_steps[next] <= min {
        // already found a faster way
        return;
    }
    min_steps[next] = min;
    q.push(Item { idx: next, min });
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Item {
    min: usize,
    idx: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .min
            .cmp(&self.min)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day12_part1_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day12_example.txt"), false).unwrap(),
            31
        );
    }
    #[test]
    fn day12_part1_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day12_input.txt"), false).unwrap(),
            456
        );
    }

    #[test]
    fn day12_part2_example() {
        assert_eq!(
            compute(Path::new("src/inputs/day12_example.txt"), true).unwrap(),
            29
        );
    }
    #[test]
    fn day12_part2_input() {
        assert_eq!(
            compute(Path::new("src/inputs/day12_input.txt"), true).unwrap(),
            454
        );
    }
}

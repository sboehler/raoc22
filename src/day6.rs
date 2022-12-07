use std::{cmp::min, collections::VecDeque};

pub fn find_marker(s: &str, n: usize) -> Option<usize> {
    let mut window = VecDeque::with_capacity(n);
    for (i, ch) in s.chars().enumerate() {
        let mut has_conflict = false;
        window.push_front(Item {
            char: ch,
            conflict: n,
        });
        for offset in (0..min(n, window.len())).rev() {
            if offset + window[offset].conflict < n {
                has_conflict = true;
            }
            if offset > 0 && window[offset].char == ch {
                window[0].conflict = offset
            }
        }
        if i >= n {
            if !has_conflict {
                return Some(i + 1);
            }
            window.pop_back();
        }
    }
    None
}

struct Item {
    pub char: char,
    pub conflict: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day6_part1_example() {
        let tests = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(6)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11)),
        ];
        for (input, want) in tests {
            assert_eq!(find_marker(&input, 4), want)
        }
    }

    #[test]
    fn day6_part1_input() {
        let s = std::fs::read_to_string(Path::new("src/day6/input.txt")).unwrap();
        assert_eq!(find_marker(&s, 4), Some(1651),);
    }

    #[test]
    fn day6_part2_example() {
        let tests = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(23)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26)),
        ];
        for (input, want) in tests {
            assert_eq!(find_marker(&input, 14), want)
        }
    }

    #[test]
    fn day6_part2_input() {
        let s = std::fs::read_to_string(Path::new("src/day6/input.txt")).unwrap();
        assert_eq!(find_marker(&s, 14), Some(3837),);
    }
}

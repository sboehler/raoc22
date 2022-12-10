use std::collections::VecDeque;

pub fn find_marker(s: &str, n: usize) -> Option<usize> {
    let mut window: VecDeque<Pos> = VecDeque::with_capacity(n);
    for (i, char) in s.chars().enumerate() {
        let conflict = window.iter().position(|pos| pos.char == char).unwrap_or(n) + 1;
        window.push_front(Pos { char, conflict });
        if i < n {
            continue;
        }
        if window
            .iter()
            .enumerate()
            .all(|(offset, pos)| pos.conflict + offset >= n)
        {
            return Some(i + 1);
        }
        window.pop_back();
    }
    None
}

struct Pos {
    pub char: char,
    pub conflict: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day06_part1_example() {
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
    fn day06_part1_input() {
        let s = std::fs::read_to_string(Path::new("src/inputs/day06_input.txt")).unwrap();
        assert_eq!(find_marker(&s, 4), Some(1651),);
    }

    #[test]
    fn day06_part2_example() {
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
    fn day06_part2_input() {
        let s = std::fs::read_to_string(Path::new("src/inputs/day06_input.txt")).unwrap();
        assert_eq!(find_marker(&s, 14), Some(3837),);
    }
}

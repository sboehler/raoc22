pub fn find_marker(s: &str, n: usize) -> Option<usize> {
    let mut window = vec!['\0'; n];
    let mut conflict_offset = vec![0; n];
    for (i, ch) in s.chars().enumerate() {
        let mut has_conflict = false;
        let idx = (i + (n - 1)) % n;
        window[idx] = ch;
        conflict_offset[idx] = n;
        for offset in (0..=(n - 1)).rev() {
            let off_idx = (i + (n - 1) - offset) % n;
            if offset + conflict_offset[off_idx] < n {
                has_conflict = true;
            }
            if offset > 0 && window[off_idx] == ch {
                conflict_offset[idx] = offset;
            }
        }
        if !has_conflict {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_day5_part1_examples() {
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
    fn test_day5_part1() {
        let s = std::fs::read_to_string(Path::new("src/day6/input.txt")).unwrap();
        assert_eq!(find_marker(&s, 4), Some(1651),);
    }

    #[test]
    fn test_day5_part2_examples() {
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
    fn test_day5_part2() {
        let s = std::fs::read_to_string(Path::new("src/day6/input.txt")).unwrap();
        assert_eq!(find_marker(&s, 14), Some(3837),);
    }
}

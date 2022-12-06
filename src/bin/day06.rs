use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

fn run1(input: &str) -> usize {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (a, b, c, d))| {
            if a != b && a != c && a != d && b != c && b != d && c != d {
                println!("{}{}{}{}", a, b, c, d);
                Some(i + 4)
            } else {
                None
            }
        })
        .unwrap() + 4
}

fn run2(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .map(|w| w.iter().collect::<HashSet<_>>())
        .enumerate()
        .find_map(|(i, h)| {
            if h.len() == 14 {
                Some(i + 14)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    println!("{}", run1(INPUT.trim()));
    println!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(run1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(run1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(run1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(run1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(""), 0);
    }
}

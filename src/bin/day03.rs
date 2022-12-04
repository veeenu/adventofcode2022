use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day03.txt");

fn run1(input: &str) -> usize {
    let priority_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    input.trim().lines().fold(0usize, |priority, line| {
        let (left, right) = (
            &line[..line.len() / 2].chars().collect::<HashSet<_>>(),
            &line[line.len() / 2..].chars().collect::<HashSet<_>>(),
        );

        let duplicated = left.intersection(right).into_iter().next().unwrap();

        priority + priority_map.get(duplicated).unwrap()
    })
}

fn run2(input: &str) -> usize {
    let priority_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0usize, |priority, chunk| {
            let common = chunk
                .into_iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .reduce(|o, i| o.intersection(&i).copied().collect());

            let badge_key = common
                .into_iter()
                .next()
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            priority + priority_map.get(&badge_key).unwrap()
        })
}

fn main() {
    println!("{}", run1(INPUT));
    println!("{}", run2(INPUT));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

#[cfg(test)]
const SAMPLE02: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 157);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE02), 70);
    }
}

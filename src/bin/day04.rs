const INPUT: &str = include_str!("../../inputs/day04.txt");

fn run1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let [(a1, a2), (b1, b2)] = &line
                .split(',')
                .map(|range| {
                    let mut ends = range.split('-').map(|s| s.parse::<usize>().unwrap());
                    (ends.next().unwrap(), ends.next().unwrap())
                })
                .collect::<Vec<_>>()[0..2]
            {
                usize::from((a1 <= b1 && a2 >= b2) || (a1 >= b1 && a2 <= b2))
            } else {
                panic!()
            }
        })
        .sum()
}

fn run2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let [(a1, a2), (b1, b2)] = &line
                .split(',')
                .map(|range| {
                    let mut ends = range.split('-').map(|s| s.parse::<usize>().unwrap());
                    (ends.next().unwrap(), ends.next().unwrap())
                })
                .collect::<Vec<_>>()[0..2]
            {
                usize::from((a1 <= b1 && a2 >= b1) || (a1 <= b2 && a2 >= b2) ||
(b1 <= a1 && b2 >= a1) || (b1 <= a2 && b2 >= a2))
            } else {
                panic!()
            }
        })
        .sum()
}

fn main() {
    println!("{}", run1(INPUT.trim()));
    println!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

#[cfg(test)]
const SAMPLE02: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01.trim()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE02.trim()), 4);
    }
}

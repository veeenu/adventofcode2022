const INPUT: &str = include_str!("../../inputs/day04.txt");

fn run1(input: &str) -> usize {
}

fn run2(input: &str) -> usize {
}

fn main() {
    println!("{}", run1(INPUT));
    println!("{}", run2(INPUT));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
"#;

#[cfg(test)]
const SAMPLE02: &str = r#"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), ());
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE02), ());
    }
}

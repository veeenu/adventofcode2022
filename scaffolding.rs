const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

fn run1(input: &'static str) -> usize {
}

fn run2(input: &'static str) -> usize {
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 24);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 93);
    }
}

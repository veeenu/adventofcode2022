use std::{collections::VecDeque, ops::Deref, str::{Chars, Split}};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

fn it_take<const K: usize, I: Iterator<Item = T>, T: Default + Copy>(mut it: I) -> [T; K] {
    let mut r = [Default::default(); K];
    for rr in r.iter_mut().take(K) {
        *rr = it.next().unwrap();
    }
    r
}

struct CrateIter<'a>(Chars<'a>);

impl<'a> CrateIter<'a> {
    fn new(line: &'a str) -> Self {
        Self(line.chars())
    }
}

impl<'a> Iterator for CrateIter<'a> {
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = it_take(self.0.by_ref());
        let cr = match chunk {
            [' ', ' ', ' '] => Some(None),
            ['[', c, ']'] => Some(Some(c)),
            a => unreachable!("{:?}", a),
        };
        self.0.next()?;
        cr
    }
}

#[derive(Debug)]
struct Move {
    qty: usize,
    src: usize,
    dest: usize,
}

impl Move {
    fn new(line: &str) -> Self {
        let [mov, qty, from, src, to, dest] = it_take(line.split(' '));
        assert_eq!(mov, "move");
        assert_eq!(from, "from");
        assert_eq!(to, "to");

        let qty = qty.parse::<usize>().unwrap();
        let src = src.parse::<usize>().unwrap();
        let dest = dest.parse::<usize>().unwrap();

        Self { qty, src, dest }
    }
}

#[derive(Debug)]
struct Problem(Vec<VecDeque<char>>, Vec<Move>);

impl Problem {
    fn parse(lines: &str) -> Self {
        let mut lines = lines.lines();

        let crates = lines
            .by_ref()
            .take_while(|l| !l.starts_with(" 1"))
            .map(CrateIter::new)
            .fold(Vec::new(), |mut o, crates_row| {
                for (col, crate_item) in crates_row.into_iter().enumerate() {
                    if o.len() <= col {
                        o.push(VecDeque::new());
                    }
                    if let Some(c) = crate_item {
                        o[col].push_front(c);
                    }
                }

                o
            });

        lines.next();

        let moves = lines.map(Move::new);

        Self(crates, moves)
    }

    fn tops(&self) -> String {
        self.0.iter().map(|i| i.back().unwrap()).collect()
    }

    fn apply9000(&mut self) {
        let stacks = &mut self.0;
        for Move { qty, src, dest } in &self.1 {
            for _ in 0..*qty {
                let c = stacks[src - 1].pop_back().unwrap();
                stacks[dest - 1].push_back(c);
            }
        }
    }

    fn apply9001(&mut self) {
        let stacks = &mut self.0;
        for Move { qty, src, dest } in &self.1 {
            for c in (0..*qty)
                .map(|_| stacks[src - 1].pop_back().unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
            {
                stacks[dest - 1].push_back(c);
            }
        }
    }
}

impl Deref for Crate {
    type Target = Option<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn run1(input: &str) -> String {
    let mut p = Problem::parse(input);
    p.apply9000();
    p.tops()
}

fn run2(input: &str) -> String {
    let mut p = Problem::parse(input);
    p.apply9001();
    p.tops()
}

fn main() {
    println!("{}", run1(INPUT.trim()));
    println!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(&run1(SAMPLE01), "CMZ")
    }

    #[test]
    fn test2() {
        assert_eq!(&run2(SAMPLE01), "MCD");
    }
}

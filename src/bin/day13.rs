use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character,
    combinator::map,
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug)]
enum AoCList {
    Value(i64),
    List(Vec<AoCList>),
}

impl AoCList {
    fn print(&self) {
        match self {
            AoCList::Value(i) => print!("{i}"),
            AoCList::List(l) => {
                print!("[");
                for i in l {
                    i.print();
                    print!(",");
                }
                print!("]");
            }
        }
    }

    fn singleton(&self) -> Option<&Self> {
        match self {
            AoCList::Value(_) => None,
            AoCList::List(l) if l.len() == 1 => Some(&l[0]),
            _ => None,
        }
    }

    fn is_divider(&self) -> bool {
        self.singleton()
            .and_then(|i| i.singleton())
            .map(|i| {
                if let AoCList::Value(i) = i {
                    *i == 2 || *i == 6
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

fn cmp_list(pkt1: &AoCList, pkt2: &AoCList, indent: usize) -> Ordering {
    let prind = move || {
        for i in 0..indent {
            print!("\x1b[{};1m| ", if i % 2 == 0 { 37 } else { 90 });
        }
        print!("\x1b[0m");
    };

    if false {
        prind();
        pkt1.print();
        println!();
        prind();
        println!("vs");
        prind();
        pkt2.print();
        println!();
    }

    use AoCList::*;

    let outcome = match (pkt1, pkt2) {
        (Value(a), Value(b)) if a < b => Ordering::Less,
        (Value(a), Value(b)) if a > b => Ordering::Greater,
        (Value(_), Value(_)) => Ordering::Equal,
        (List(pkt1), List(pkt2)) if pkt1.is_empty() && !pkt2.is_empty() => Ordering::Less,
        (List(pkt1), List(pkt2)) if !pkt1.is_empty() && pkt2.is_empty() => Ordering::Greater,
        (List(pkt1), List(pkt2)) => pkt1
            .iter()
            .zip_longest(pkt2.iter())
            .map(|a| match a {
                EitherOrBoth::Both(a, b) => cmp_list(a, b, indent + 1),
                EitherOrBoth::Right(_) => Ordering::Less,
                EitherOrBoth::Left(_) => Ordering::Greater,
            })
            .find(|&i| i != Ordering::Equal)
            .unwrap_or(Ordering::Equal),
        (List(_), Value(i2)) => cmp_list(pkt1, &List(vec![Value(*i2)]), indent + 1),
        (Value(i1), List(_)) => cmp_list(&List(vec![Value(*i1)]), pkt2, indent + 1),
    };

    if false {
        prind();
        println!("Outcome: {:?}", outcome);
    }

    outcome
}

fn parse_list(i: &str) -> IResult<&str, AoCList> {
    alt((
        map(character::complete::i64, AoCList::Value),
        map(
            tuple((tag("["), separated_list0(tag(","), parse_list), tag("]"))),
            |(_, l, _)| AoCList::List(l),
        ),
    ))(i)
}

fn parse_two_lists(i: &str) -> IResult<&str, (AoCList, AoCList)> {
    let (i, (a, _, b)) = tuple((parse_list, tag("\n"), parse_list))(i)?;
    Ok((i, (a, b)))
}

fn parse(i: &str) -> IResult<&str, Vec<(AoCList, AoCList)>> {
    separated_list0(tag("\n\n"), parse_two_lists)(i)
}

fn parse2(i: &str) -> IResult<&str, Vec<AoCList>> {
    separated_list0(many1(tag("\n")), parse_list)(i)
}

fn run1(input: &'static str) -> usize {
    let lists = parse(input.trim()).unwrap().1;
    let mut ordered = 0;

    lists
        .into_iter()
        .enumerate()
        .for_each(|(idx, (pkt1, pkt2))| {
            println!("=== Pair {} ===", idx + 1);
            let l = cmp_list(&pkt1, &pkt2, 0);
            println!(
                "\nOutcome: {}\n",
                if l == Ordering::Less {
                    "\x1b[32;1mordered\x1b[0m"
                } else {
                    "\x1b[31;1munordered\x1b[0m"
                }
            );
            if l == Ordering::Less {
                ordered += idx + 1;
            }
        });

    println!("Final outcome {ordered}");

    ordered
}

fn run2(input: &'static str) -> usize {
    let mut lists = parse2(input.trim()).unwrap().1;
    for i in parse2("[[2]]\n[[6]]").unwrap().1 {
        lists.push(i);
    }

    let lists = lists
        .into_iter()
        .sorted_by(|pkt1, pkt2| cmp_list(pkt1, pkt2, 0));

    let mut acc = 1usize;

    for (idx, l) in lists.enumerate() {
        print!("{idx:>4}: ");
        l.print();
        if l.is_divider() {
            print!(" \x1b[32;1mdivider!\x1b[0m");
            acc *= idx + 1;
        }
        println!();
    }

    acc
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}
#[cfg(test)]
const SAMPLE01: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

"#;

#[cfg(test)]
const SAMPLE02: &str = r#"
[[[],8,8,[]],[10,[10,[8,7],1,[5,9,9,1,7],4],5],[1,8,[5,1,[9,7,10,5],7],[[6,7],[8],[9],0,6],4]]
[[[]],[[8,4],7,3,[]],[[],10,[5,[1],[8,3,2,1]]],[[[2,3,10,2]],4,1,4],[9,2]]
"#;

#[cfg(test)]
const SAMPLE03: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

[[2]]
[[6]]
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        run1(SAMPLE02);
        assert_eq!(run1(SAMPLE01), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE03), 140);
    }
}

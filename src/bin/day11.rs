use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug)]
enum Op {
    Add(i64),
    Sub(i64),
    Mul(i64),
    Div(i64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: i64,
    if_true: usize,
    if_false: usize,
}

struct Toss(usize, i64);

impl Monkey {
    fn parse(input: &str) -> Vec<Self> {
        input
            .split("\n\n")
            .map(|m| {
                let mut m = m.lines();
                m.next();

                let items = m
                    .next()
                    .unwrap()
                    .strip_prefix("  Starting items: ")
                    .map(|items| {
                        items
                            .split(", ")
                            .map(|i| i.parse::<i64>().unwrap())
                            .collect()
                    })
                    .unwrap();

                let op = m
                    .next()
                    .unwrap()
                    .strip_prefix("  Operation: new = old ")
                    .map(|op| {
                        let (op, val) = op.split(' ').next_tuple().unwrap();
                        if val == "old" {
                            Op::Square
                        } else {
                            let val = val.parse::<i64>().unwrap();
                            match op {
                                "+" => Op::Add(val),
                                "-" => Op::Sub(val),
                                "*" => Op::Mul(val),
                                "/" => Op::Div(val),
                                i => panic!("op {i}"),
                            }
                        }
                    })
                    .unwrap();

                let test = m
                    .next()
                    .unwrap()
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();

                let if_true = m
                    .next()
                    .unwrap()
                    .strip_prefix("    If true: throw to monkey ")
                    .map(|i| i.parse::<usize>().unwrap())
                    .unwrap();

                let if_false = m
                    .next()
                    .unwrap()
                    .strip_prefix("    If false: throw to monkey ")
                    .map(|i| i.parse::<usize>().unwrap())
                    .unwrap();

                Monkey {
                    items,
                    op,
                    test,
                    if_true,
                    if_false,
                }
            })
            .collect()
    }

    fn iter_mut1(&'_ mut self) -> MonkeyRound<'_> {
        MonkeyRound(self, 3, i64::MAX)
    }

    fn iter_mut2(&'_ mut self, modulo: i64) -> MonkeyRound<'_> {
        MonkeyRound(self, 1, modulo)
    }

    fn receive(&mut self, i: i64) {
        self.items.push_back(i);
    }
}

struct MonkeyRound<'a>(&'a mut Monkey, i64, i64);

impl<'a> Iterator for MonkeyRound<'a> {
    type Item = Toss;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.items.pop_front()?;
        let item = match self.0.op {
            Op::Add(i) => item + i,
            Op::Sub(i) => item - i,
            Op::Mul(i) => item * i,
            Op::Div(i) => item / i,
            Op::Square => item * item,
        } / self.1;
        let item = item % self.2;

        if item % self.0.test == 0 {
            Some(Toss(self.0.if_true, item))
        } else {
            Some(Toss(self.0.if_false, item))
        }
    }
}

fn run1(input: &str) -> usize {
    let mut monkeys = Monkey::parse(input.trim());
    let mut monkey_actions = vec![0usize; monkeys.len()];

    for i in 1..=20 {
        for m in 0..monkeys.len() {
            let actions = monkeys[m].iter_mut1().collect::<Vec<_>>();
            monkey_actions[m] += actions.len();
            for Toss(tgt_monkey, item) in actions {
                monkeys[tgt_monkey].receive(item);
            }
        }
        println!("Round {i}");
        for (i, m) in monkeys.iter().enumerate() {
            print!("  Monkey {i}: ");
            for item in &m.items {
                print!("{item}, ");
            }
            println!();
        }
    }

    let (fst, snd) = monkey_actions
        .into_iter()
        .sorted()
        .rev()
        .next_tuple()
        .unwrap();

    fst * snd
}

fn run2(input: &str) -> usize {
    let mut monkeys = Monkey::parse(input.trim());
    let mut monkey_actions = vec![0usize; monkeys.len()];

    let modulo = monkeys.iter().map(|i| i.test).product();

    for i in 1..=10000 {
        for m in 0..monkeys.len() {
            let actions = monkeys[m].iter_mut2(modulo).collect::<Vec<_>>();
            monkey_actions[m] += actions.len();
            for Toss(tgt_monkey, item) in actions {
                monkeys[tgt_monkey].receive(item);
            }
        }
        println!("Round {i}");
        for (i, m) in monkeys.iter().enumerate() {
            print!("  Monkey {i}: ");
            for item in &m.items {
                print!("{item}, ");
            }
            println!();
        }
    }

    let (fst, snd) = monkey_actions
        .into_iter()
        .sorted()
        .rev()
        .next_tuple()
        .unwrap();

    fst * snd
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

#[test]
fn test1() {
    assert_eq!(run1(SAMPLE01), 10605);
}

#[test]
fn test2() {
    assert_eq!(run2(SAMPLE01), 2713310158);
}

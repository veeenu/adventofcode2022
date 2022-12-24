use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cube(i32, i32, i32);

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(",")
            .map(str::parse)
            .map(Result::unwrap)
            .next_tuple()
            .unwrap();
        Self(x, y, z)
    }
}

impl Cube {
    fn is_adj(&self, &Self(rx, ry, rz): &Self) -> bool {
        let Self(x, y, z) = self;
        let dx = rx - x;
        let dy = ry - y;
        let dz = rz - z;

        #[inline]
        fn abs_one(a: i32) -> bool {
            a == -1 || a == 1
        }

        (dx == 0 && dy == 0 && abs_one(dz))
            || (dx == 0 && abs_one(dy) && dz == 0)
            || (abs_one(dx) && dy == 0 && dz == 0)
    }
}

struct CubeGraph(HashMap<Cube, HashSet<Cube>>);

impl CubeGraph {
    fn new(input: &str) -> Self {
        let mut g = Self(
            input
                .trim()
                .lines()
                .map(Cube::from)
                .map(|cube| (cube, Default::default()))
                .collect::<HashMap<_, _>>(),
        );

        for c in input.trim().lines().map(Cube::from) {
            for (k, set) in &mut g.0 {
                if k.is_adj(&c) {
                    set.insert(c.clone());
                }
            }
        }

        g
    }

    fn count_blocked_faces(&self) -> usize {
        self.0.iter()
            .map(|(cube, adj)| 6 - adj.len())
            .sum()
    }
}

fn run1(input: &'static str) -> usize {
    CubeGraph::new(input).count_blocked_faces()
}

fn run2(input: &'static str) -> usize {
    0
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 64);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 58);
    }
}

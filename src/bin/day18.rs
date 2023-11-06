use std::{
    collections::{HashMap, HashSet},
    ops,
};

use itertools::{iproduct, Itertools};

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

struct CubeGraph(Vec<Cube>);

impl CubeGraph {
    fn new(input: &str) -> Self {
        Self(input.trim().lines().map(Cube::from).collect())
    }

    fn count_blocked_faces(&self) -> usize {
        self.0
            .iter()
            .map(|a| {
                6usize
                    - self
                        .0
                        .iter()
                        .map(|b| if a.is_adj(b) { 1 } else { 0 })
                        .sum::<usize>()
            })
            .sum()
    }

    fn count_air_pockets(&self) -> usize {
        let (range_x, range_y, range_z) = self.0.iter().fold(
            (Range::new(), Range::new(), Range::new()),
            |(rx, ry, rz), cube| (rx.expand(cube.0), ry.expand(cube.1), rz.expand(cube.2)),
        );

        println!("{range_x:?} {range_y:?} {range_z:?}");

        let (range_x, range_y, range_z) = (
            range_x.into_range(),
            range_y.into_range(),
            range_z.into_range(),
        );

        let cube_set = self.0.iter().cloned().collect::<HashSet<_>>();

        let cubes = iproduct!(range_x, range_y, range_z);

        cubes
            .map(|(x, y, z)| {
                let adjacents = [
                    Cube(x + 1, y, z),
                    Cube(x - 1, y, z),
                    Cube(x, y + 1, z),
                    Cube(x, y - 1, z),
                    Cube(x, y, z + 1),
                    Cube(x, y, z - 1),
                ];

                // print!("{x},{y},{z}:  ");
                // for Cube(x, y, z) in &adjacents {
                //     print!("{x},{y},{z} {} ", cube_set.contains(&Cube(*x, *y, *z)));
                // }
                // println!();

                if adjacents.iter().all(|c| cube_set.contains(c)) {
                    println!("{x},{y},{z}");
                    6
                } else {
                    0
                }
            })
            .sum()
    }
}

#[derive(Clone, Copy, Debug)]
struct Range(i32, i32);

impl Range {
    fn new() -> Self {
        Self(i32::MAX, 0)
    }

    fn expand(self, v: i32) -> Self {
        Self(i32::min(self.0, v), i32::max(self.0, v))
    }

    fn into_range(self) -> ops::Range<i32> {
        ops::Range {
            start: self.0,
            end: self.1 + 1,
        }
    }
}

fn run1(input: &'static str) -> usize {
    CubeGraph::new(input).count_blocked_faces()
}

fn run2(input: &'static str) -> usize {
    let g = CubeGraph::new(input);
    g.count_blocked_faces() - g.count_air_pockets()
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

use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

#[derive(Debug)]
struct Grid {
    rocks: HashSet<Point>,
    bounds: (Point, Point),
    sand: HashSet<Point>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let rocks = input
            .trim()
            .lines()
            .flat_map(|l| {
                l.split(" -> ")
                    .map(|p| {
                        let (x, y) = p.split(',').next_tuple().unwrap();
                        Point(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .tuple_windows()
                    .flat_map(|(start, stop)| {
                        let xr = if start.0 < stop.0 {
                            start.0..=stop.0
                        } else {
                            stop.0..=start.0
                        };
                        let yr = move || {
                            if start.1 < stop.1 {
                                start.1..=stop.1
                            } else {
                                stop.1..=start.1
                            }
                        };
                        xr.flat_map(move |x| yr().map(move |y| Point(x, y)))
                    })
            })
            .collect::<HashSet<_>>();

        let bounds = rocks.iter().fold(
            (Point(usize::MAX, 0), Point(0, 0)),
            |(mut min, mut max), Point(x, y)| {
                min.0 = usize::min(min.0, *x);
                min.1 = usize::min(min.1, *y);
                max.0 = usize::max(max.0, *x);
                max.1 = usize::max(max.1, *y);
                (min, max)
            },
        );

        Self {
            rocks,
            bounds,
            sand: HashSet::with_capacity(10000),
        }
    }

    fn simulate1(&mut self) -> bool {
        let mut sand_pos = Point(500, 0);

        loop {
            if !self.is_in_bounds(sand_pos) {
                break true;
            }

            let down = Point(sand_pos.0, sand_pos.1 + 1);
            let left = Point(sand_pos.0 - 1, sand_pos.1 + 1);
            let right = Point(sand_pos.0 + 1, sand_pos.1 + 1);

            if !self.occupied(down) {
                sand_pos = down;
            } else if !self.occupied(left) {
                sand_pos = left;
            } else if !self.occupied(right) {
                sand_pos = right;
            } else {
                self.sand.insert(sand_pos);
                break false;
            }
        }
    }

    fn simulate2(&mut self) -> bool {
        let mut sand_pos = Point(500, 0);

        loop {
            if self.sand.contains(&Point(500, 0)) {
                break true;
            }

            let down = Point(sand_pos.0, sand_pos.1 + 1);
            let left = Point(sand_pos.0 - 1, sand_pos.1 + 1);
            let right = Point(sand_pos.0 + 1, sand_pos.1 + 1);

            if !self.occupied2(down) {
                sand_pos = down;
            } else if !self.occupied2(left) {
                sand_pos = left;
            } else if !self.occupied2(right) {
                sand_pos = right;
            } else {
                self.sand.insert(sand_pos);
                self.stretch_bounds(sand_pos);
                break false;
            }
        }
    }

    fn is_in_bounds(&self, p: Point) -> bool {
        p.0 >= self.bounds.0 .0
            && p.0 <= self.bounds.1 .0
            && p.1 >= self.bounds.0 .1
            && p.1 <= self.bounds.1 .1
    }

    fn occupied(&self, p: Point) -> bool {
        self.rocks.contains(&p) || self.sand.contains(&p)
    }

    fn occupied2(&self, p: Point) -> bool {
        self.occupied(p) || p.1 > (self.bounds.1 .1 + 1)
    }

    fn stretch_bounds(&mut self, Point(x, y): Point) {
        let (Point(min_x, min_y), Point(max_x, max_y)) = self.bounds;
        self.bounds = (
            Point(usize::min(min_x, x), usize::min(min_y, y)),
            Point(usize::max(max_x, x), max_y),
        )
    }

    fn render(&self) {
        for y in self.bounds.0 .1..=(self.bounds.1 .1 + 2) {
            for x in self.bounds.0 .0..=self.bounds.1 .0 {
                if self.rocks.contains(&Point(x, y)) {
                    print!("\x1b[41m#");
                } else if self.sand.contains(&Point(x, y)) {
                    print!("\x1b[43mo");
                } else {
                    print!("\x1b[40m.");
                }
            }
            println!("\x1b[0m");
        }
    }
}

fn run1(input: &'static str) -> usize {
    let mut grid = Grid::parse(input);

    let mut count = 0usize;
    println!("\x1bc\x1b[s");
    while !grid.simulate1() {
        count += 1;
        if count % 20 == 0 {
            print!("\x1b[u");
            grid.render();
        }
    }
    print!("\x1b[u");
    grid.render();
    std::thread::sleep(std::time::Duration::from_millis(1000));

    grid.sand.len()
}

fn run2(input: &'static str) -> usize {
    let mut grid = Grid::parse(input);

    let mut count = 0usize;
    println!("\x1bc\x1b[s");
    while !grid.simulate2() {
        count += 1;
        if count % 20 == 0 {
            print!("\x1b[u");
            grid.render();
        }
    }
    print!("\x1b[u");
    grid.render();
    std::thread::sleep(std::time::Duration::from_millis(1000));

    grid.sand.len()
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

const SAMPLE01: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
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

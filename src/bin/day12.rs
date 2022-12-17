use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Spot(u8),
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct Grid {
    data: Vec<Cell>,
    width: usize,
    height: usize,
    s: Point,
    e: Point,
}

impl Grid {
    fn val(&self, Point(x, y): Point) -> Cell {
        self.data[x + y * self.width]
    }

    fn neighborhood(&self, src: Point) -> impl Iterator<Item = Point> + '_ {
        let Point(x, y) = src;
        [
            Point(x.saturating_sub(1), y),                // left
            Point(x, y.saturating_sub(1)),                // above
            Point(usize::min(x + 1, self.width - 1), y),  // right
            Point(x, usize::min(y + 1, self.height - 1)), // below
        ]
        .into_iter()
        .filter(move |&dst| {
            if src == dst {
                return false;
            }

            let srcv = self.val(src);
            let dstv = self.val(dst);

            match (dstv, srcv) {
                (Cell::Start, _) => true,
                (Cell::End, _) => false,
                (Cell::Spot(s), Cell::End) => s >= 24,
                (Cell::Spot(a), Cell::Spot(b)) => b <= a + 1,
                _ => false,
            }
        })
    }

    fn nodes(&self) -> impl Iterator<Item = Point> + '_ {
        cartesian(0, 0, self.width, self.height)
    }

    fn dijkstra_start(&self) -> usize {
        *self.dijkstra().get(&self.s).unwrap()
    }

    fn dijkstra_min(&self) -> usize {
        let distances = self.dijkstra();
        self.nodes()
            .filter(|&n| matches!(self.val(n), Cell::Start | Cell::Spot(0)))
            .filter_map(|a| distances.get(&a))
            .fold(usize::MAX, |o, i| usize::min(o, *i))
    }

    fn dijkstra(&self) -> HashMap<Point, usize> {
        let mut distances = self
            .nodes()
            .map(|node| (node, usize::MAX - 1))
            .collect::<HashMap<_, _>>();
        let mut unvisited_nodes = self.nodes().collect::<HashSet<_>>();
        *distances.entry(self.e).or_default() = 0;

        let mut count = 0usize;
        loop {
            if count % 100 == 0 {
                print!("{count}\r");
                std::io::stdout().flush().ok();
            }
            count += 1;
            if unvisited_nodes.is_empty() {
                break;
            }

            let (v, v_dist) = unvisited_nodes
                .iter()
                .fold((Point(0, 0), usize::MAX), |(v, v_dist), i| {
                    let i_dist = *distances.get(i).unwrap();
                    if i_dist < v_dist {
                        (*i, i_dist)
                    } else {
                        (v, v_dist)
                    }
                });

            unvisited_nodes.remove(&v);

            self.neighborhood(v).for_each(|dst| {
                let entry = distances.entry(dst).or_default();
                *entry = usize::min(*entry, v_dist.saturating_add(1));
            });
        }

        distances
    }
}

fn cartesian(ax: usize, ay: usize, bx: usize, by: usize) -> impl Iterator<Item = Point> {
    (ax..bx)
        .into_iter()
        .flat_map(move |x| (ay..by).into_iter().map(move |y| Point(x, y)))
}

impl From<&'static str> for Grid {
    fn from(input: &'static str) -> Self {
        let data: Vec<_> = input
            .trim()
            .lines()
            .flat_map(|line| {
                line.as_bytes().iter().map(|&b| {
                    if b == b'S' {
                        Cell::Start
                    } else if b == b'E' {
                        Cell::End
                    } else {
                        Cell::Spot(b - b'a')
                    }
                })
            })
            .collect();
        let width = input.trim().lines().next().unwrap().len();
        let height = input.trim().lines().count();
        let s_idx = data
            .iter()
            .enumerate()
            .find_map(|(idx, &i)| if let Cell::Start = i { Some(idx) } else { None })
            .unwrap();
        let e_idx = data
            .iter()
            .enumerate()
            .find_map(|(idx, &i)| if let Cell::End = i { Some(idx) } else { None })
            .unwrap();

        let s = Point(s_idx % width, s_idx / width);
        let e = Point(e_idx % width, e_idx / width);
        Grid {
            data,
            width,
            height,
            s,
            e,
        }
    }
}

fn run1(input: &'static str) -> usize {
    Grid::from(input).dijkstra_start()
}

fn run2(input: &'static str) -> usize {
    Grid::from(input).dijkstra_min()
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 31);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 29);
    }
}

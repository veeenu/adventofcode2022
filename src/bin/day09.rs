use std::{collections::HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Clone, Copy)]
enum Move {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

impl Move {
    fn moves(input: &str) -> impl Iterator<Item = Move> + '_ {
        input.trim().lines().map(|line| {
            let (dir, q) = line.split(' ').next_tuple().unwrap();
            let q = q.parse::<usize>().unwrap();
            match dir {
                "R" => Move::R(q),
                "L" => Move::L(q),
                "U" => Move::U(q),
                "D" => Move::D(q),
                e => unreachable!("{e}"),
            }
        })
    }

    fn as_iter(self) -> impl Iterator<Item = (isize, isize)> {
        let (q, x, y) = match self {
            Move::R(q) => (q, 1, 0),
            Move::L(q) => (q, -1, 0),
            Move::U(q) => (q, 0, 1),
            Move::D(q) => (q, 0, -1),
        };

        (0..q).map(move |_| (x, y))
    }
}

fn follow(tx: &mut isize, ty: &mut isize, cx: isize, cy: isize) {
    let xdist = cx - *tx;
    let ydist = cy - *ty;
    let axdist = xdist.abs();
    let aydist = ydist.abs();

    if (axdist > 1 && aydist > 0) || (axdist > 0 && aydist > 1) {
        *tx += xdist / xdist.abs();
        *ty += ydist / ydist.abs();
    } else if axdist > 1 {
        *tx += xdist / xdist.abs();
    } else if aydist > 1 {
        *ty += ydist / ydist.abs();
    }
}

fn run1(input: &str) -> usize {
    #[derive(Default)]
    struct State {
        cx: isize,
        cy: isize,
        tx: isize,
        ty: isize,
        width: usize,
        height: usize,
        visited: HashSet<(isize, isize)>,
    }

    let state =
        Move::moves(input)
            .flat_map(Move::as_iter)
            .fold(State::default(), |mut state, (x, y)| {
                state.cx += x;
                state.cy += y;

                follow(&mut state.tx, &mut state.ty, state.cx, state.cy);

                state.width = usize::max(state.width, state.cx as usize);
                state.height = usize::max(state.height, state.cy as usize);

                state.visited.insert((state.tx, state.ty));

                state
            });

    state.visited.len()
}

fn run2(input: &str) -> usize {
    #[derive(Default)]
    struct State {
        cx: isize,
        cy: isize,
        tx: [isize; 10],
        ty: [isize; 10],
        width: usize,
        height: usize,
        visited: HashSet<(isize, isize)>,
    }

    let state =
        Move::moves(input)
            .flat_map(Move::as_iter)
            .fold(State::default(), |mut state, (x, y)| {
                state.cx += x;
                state.cy += y;

                follow(&mut state.tx[0], &mut state.ty[0], state.cx, state.cy);
                for i in 0..8 {
                    let (cx, cy) = (state.tx[i], state.ty[i]);
                    follow(&mut state.tx[i + 1], &mut state.ty[i + 1], cx, cy);
                }

                state.width = usize::max(state.width, state.cx as usize);
                state.height = usize::max(state.height, state.cy as usize);

                state.visited.insert((state.tx[8], state.ty[8]));

                state
            });

    state.visited.len()
}

fn main() {
    dbg!("{}", run1(INPUT.trim()));
    dbg!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

#[cfg(test)]
const SAMPLE02: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 1);
        assert_eq!(run2(SAMPLE02), 36);
    }
}

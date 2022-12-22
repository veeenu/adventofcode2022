use std::{io::Write, time::Instant};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Action {
    Left,
    Right,
    Down,
}

fn parse(input: &str) -> Vec<Action> {
    input
        .chars()
        .map(|c| match c {
            '<' => Action::Left,
            '>' => Action::Right,
            _ => unreachable!(),
        })
        .collect()
}

struct LoopingIter<I: IntoIterator<Item = T> + Copy, T: Clone + Copy>(I, I::IntoIter);

impl<I: IntoIterator<Item = T> + Copy, T: Clone + Copy> LoopingIter<I, T> {
    fn new(i: I) -> Self {
        Self(i, i.into_iter())
    }
}

impl<I: IntoIterator<Item = T> + Copy, T: Clone + Copy> Iterator for LoopingIter<I, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.1.next() {
            Some(t)
        } else {
            self.1 = self.0.into_iter();
            self.1.next()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    Horiz,
    Plus,
    Corner,
    Vert,
    Square,
}

impl Shape {
    fn size(&self) -> (usize, usize) {
        match self {
            Shape::Horiz => (4, 1),
            Shape::Plus => (3, 3),
            Shape::Corner => (3, 3),
            Shape::Vert => (1, 4),
            Shape::Square => (2, 2),
        }
    }

    fn is_lit_bro(&self, x: usize, y: usize) -> bool {
        match self {
            Shape::Horiz => y == 0 && x <= 3,
            Shape::Plus => matches!((x, y), (1, 0) | (0, 1) | (1, 1) | (2, 1) | (1, 2)),
            Shape::Corner => matches!((x, y), (0, 0) | (1, 0) | (2, 0) | (2, 1) | (2, 2)),
            Shape::Vert => x == 0 && y <= 3,
            Shape::Square => x <= 1 && y <= 1,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rock {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Rock {
    fn action(&mut self, action: Action, others: &[Rock], force: bool) -> bool {
        let mut this_next_pos = self.clone();
        match action {
            Action::Left => {
                this_next_pos.x = self.x.saturating_sub(1);
            }
            Action::Right => {
                this_next_pos.x = usize::min(7 - self.shape.size().0, self.x + 1);
            }
            Action::Down => {
                if self.y == 0 {
                    return false;
                }
                this_next_pos.y -= 1;
            }
        }

        if force
            || others.is_empty()
            || !others
                .iter()
                .rev()
                .take(8)
                .any(|other_rock| other_rock.collides(&this_next_pos))
        {
            self.x = this_next_pos.x;
            self.y = this_next_pos.y;
            true
        } else {
            false
        }
    }

    fn is_lit_bro(&self, (x, y): (usize, usize)) -> bool {
        let (w, h) = self.shape.size();
        if x < self.x || y < self.y || x > (self.x + w) || y > (self.y + h) {
            false
        } else {
            let sx = x - self.x;
            let sy = y - self.y;
            self.shape.is_lit_bro(sx, sy)
        }
    }

    fn collides(&self, other: &Rock) -> bool {
        let (ax1, ay1, (aw, ah)) = (self.x, self.y, self.shape.size());
        let (bx1, by1, (bw, bh)) = (other.x, other.y, other.shape.size());
        let (ax2, ay2) = (ax1 + aw, ay1 + ah);
        let (bx2, by2) = (bx1 + bw, by1 + bh);

        if ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 {
            let minx = usize::max(ax1, bx1);
            let miny = usize::max(ay1, by1);
            let maxx = usize::min(ax2, bx2);
            let maxy = usize::min(ay2, by2);
            for y in miny..maxy {
                for x in minx..maxx {
                    if self.is_lit_bro((x, y)) && other.is_lit_bro((x, y)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn simulate(input: &'static str, count: usize) -> usize {
    let start = Instant::now();

    let actions = parse(input.trim());
    let mut actions = LoopingIter::new(&actions);
    let mut rock_spawn = LoopingIter::new(&[
        Shape::Horiz,
        Shape::Plus,
        Shape::Corner,
        Shape::Vert,
        Shape::Square,
    ]);
    let mut rocks: Vec<Rock> = Vec::new();
    let mut max_y_ever = 0usize;

    let mut print = |rock: &Rock, rocks: &[Rock]| {
        let maxy = rocks
            .iter()
            .map(|rock| rock.y + rock.shape.size().1)
            .max()
            .unwrap_or(0);
        let maxy = usize::max(maxy, rock.y + rock.shape.size().1);
        max_y_ever = usize::max(max_y_ever, maxy);

        for y in (0..=max_y_ever).rev() {
            print!("|");
            for x in 0..7 {
                if rocks.iter().rev().any(|rock| rock.is_lit_bro((x, y))) {
                    print!("#");
                } else if rock.is_lit_bro((x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!();
    };

    let mut max_y_in_queue = 0usize;
    let mut height_increases = Vec::new();

    for i in 0..count {
        let maxy = max_y_in_queue;
        let mut current_rock = rock_spawn
            .next()
            .map(|&shape| Rock {
                shape,
                x: 2,
                y: maxy + 3,
            })
            .unwrap();
        let mut cur_loop = 0usize;
        loop {
            current_rock.action(*actions.next().unwrap(), &rocks, cur_loop < 2);
            if !current_rock.action(Action::Down, &rocks, cur_loop < 2) {
                break;
            }
            cur_loop += 1;

            if true {
                if i % 100 == 0 {
                    let elapsed = start.elapsed();
                    print!(
                        "{:.2} ({:10.2}/s) {}/{} ({:.2}%)\r",
                        elapsed.as_secs_f32(),
                        (i as f32) / elapsed.as_secs_f32(),
                        i,
                        count,
                        100. * (i as f32) / (count as f32)
                    );
                    std::io::stdout().flush().ok();
                }
            } else {
                println!("\n*** {i}\n{:?}\n_________", current_rock);
                print(&current_rock, &rocks);
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
        let rock_y = current_rock.y + current_rock.shape.size().1;
        rocks.push(current_rock);

        let height_increase = rock_y.saturating_sub(max_y_in_queue);
        height_increases.push(height_increase);

        if let Some((len, start)) = find_cycle(&height_increases) {
            let cycle_height_increase: usize = height_increases[start..start + len].iter().sum();
            let before_cycle_height: usize = height_increases[0..start].iter().sum();
            let how_many_cycles: usize = (count - start) / len;
            let after_cycles_count = (count - start) % len;
            let after_cycles_height: usize = height_increases[start..start + after_cycles_count]
                .iter()
                .sum();
            let sol =
                before_cycle_height + after_cycles_height + cycle_height_increase * how_many_cycles;
            println!("\nSolution {sol}\n");
            return sol;
        }

        max_y_in_queue = usize::max(max_y_in_queue, rock_y);
    }
    println!();

    rocks
        .iter()
        .map(|rock| rock.y + rock.shape.size().1)
        .max()
        .unwrap_or(0)
}

fn find_cycle<T: Eq>(v: &[T]) -> Option<(usize, usize)> {
    for cycle_len in 20..(v.len() / 2) {
        let fst_point = v.len() - cycle_len;
        let snd_point = v.len() - cycle_len * 2;
        let fst = &v[fst_point..];
        let snd = &v[snd_point..fst_point];
        if fst.iter().zip(snd.iter()).all(|(a, b)| a == b) {
            println!("\n\nFound cycle of length {cycle_len} starting at {snd_point} {fst_point}\n\n");
            return Some((cycle_len, snd_point));
        }
    }
    None
}

fn run1(input: &'static str) -> usize {
    simulate(input, 2022)
}

fn run2(input: &'static str) -> usize {
    simulate(input, 1000000000000)
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 3068);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 1514285714288);
    }
}

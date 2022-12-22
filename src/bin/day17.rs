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
            Shape::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].contains(&(x, y)),
            Shape::Corner => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].contains(&(x, y)),
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
    fn action(&mut self, action: Action, others: &[Rock]) -> bool {
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

        if others.is_empty()
            || !others
                .iter()
                .rev()
                .take(64)
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
            let minx = usize::min(ax1, bx1);
            let miny = usize::min(ay1, by1);
            let maxx = usize::max(ax2, bx2);
            let maxy = usize::max(ay2, by2);
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
    let mut rocks: Vec<Rock> = Vec::with_capacity(count);
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
        println!("");
    };

    let mut height_diffs = Vec::new();

    for i in 0..count {
        let maxy = rocks
            .iter()
            .rev()
            .take(64)
            .map(|rock| rock.y + rock.shape.size().1)
            .max()
            .unwrap_or(0);
        let mut current_rock = rock_spawn
            .next()
            .map(|&shape| Rock {
                shape,
                x: 2,
                y: maxy + 3,
            })
            .unwrap();
        loop {
            current_rock.action(*actions.next().unwrap(), &rocks);
            if !current_rock.action(Action::Down, &rocks) {
                break;
            }

            if true {
                let elapsed = start.elapsed();
                print!(
                    "{:.2} {} ({}/s) \r",
                    elapsed.as_secs_f32(),
                    i,
                    (i as f32) / elapsed.as_secs_f32()
                );
                std::io::stdout().flush().ok();
            } else {
                println!("\n*** {i}\n{:?}\n_________", current_rock);
                print(&current_rock, &rocks);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }

        if let Some((start, len)) = find_cycle(&height_diffs) {
            println!("Found cycle starting at {start} of length {len}");
            let cycle_height: usize = height_diffs[start..start + len].iter().sum();
            let cycle_count = (count - start).div_euclid(len);
            let after_count = (count - start).rem_euclid(len);
            let before_height: usize = height_diffs[0..start].iter().sum();
            let after_height: usize = height_diffs[start..start + after_count].iter().sum();
            println!("Before {start}, count {cycle_count}, after {after_count}");
            return cycle_height * cycle_count + before_height + after_height;
        }
        let new_max_y = current_rock.y + current_rock.shape.size().1;
        height_diffs.push(new_max_y.saturating_sub(maxy));
        rocks.push(current_rock);
    }
    println!();

    rocks
        .iter()
        .map(|rock| rock.y + rock.shape.size().1)
        .max()
        .unwrap_or(0)
}

fn find_cycle(v: &Vec<usize>) -> Option<(usize, usize)> {
    for cycle_start in (0..v.len() / 2).rev() {
        for cycle_len in 35..(v.len() - cycle_start) / 2 {
            let fst = &v[cycle_start..cycle_start + cycle_len];
            let snd = &v[cycle_start + cycle_len..cycle_start + cycle_len * 2];
            if fst == snd {
                return Some((cycle_start, cycle_len));
            }
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
        // assert_eq!(run2(SAMPLE01), 1514285714288);
    }
}

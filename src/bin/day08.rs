const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug)]
struct Tree {
    rows: usize,
    cols: usize,
    heights: Vec<usize>,
}

impl Tree {
    fn new(input: &str) -> Self {
        let (rows, cols, heights) = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, l)| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .enumerate()
                    .map(move |(col, height)| (row, col, height))
            })
            .fold(
                (0usize, 0usize, Vec::new()),
                |(mut rows, mut cols, mut heights), (row, col, height)| {
                    heights.push(height);
                    rows = usize::max(rows, row);
                    cols = usize::max(cols, col);

                    (rows, cols, heights)
                },
            );

        Self {
            rows: rows + 1,
            cols: cols + 1,
            heights,
        }
    }

    fn get(&self, row: usize, col: usize) -> usize {
        self.heights[self.cols * row + col]
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.heights.iter().enumerate().map(|(i, h)| {
            let row = i / self.cols;
            let col = i % self.cols;
            (row, col, *h)
        })
    }
}

fn run1(input: &str) -> usize {
    let tree = Tree::new(input);

    tree.iter()
        .map(|(row, col, height)| {
            let from_left = (0..col).map(|c| height > tree.get(row, c)).all(|i| i);
            let from_right = (col + 1..tree.cols)
                .map(|c| height > tree.get(row, c))
                .all(|i| i);
            let from_top = (0..row).map(|r| height > tree.get(r, col)).all(|i| i);
            let from_btm = (row + 1..tree.rows)
                .map(|r| height > tree.get(r, col))
                .all(|i| i);
            let is_edge = col == 0 || row == 0 || col == tree.cols - 1 || row == tree.rows - 1;

            usize::from(from_left || from_right || from_top || from_btm || is_edge)
        })
        .sum()
}

fn run2(input: &str) -> usize {
    let tree = Tree::new(input);

    struct TakeUntil<I: Iterator<Item = bool>>(I, bool);

    impl<I: Iterator<Item = bool>> Iterator for TakeUntil<I> {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            if self.1 {
                None
            } else {
                self.0.next().map(|i| {
                    if i {
                        self.1 = true;
                    }
                })
            }
        }
    }

    fn take_until<I: Iterator<Item = bool>>(i: I) -> TakeUntil<I> {
        TakeUntil(i, false)
    }

    tree.iter()
        .map(|(row, col, height)| {
            let from_left: usize =
                take_until((0..col).rev().map(|c| height <= tree.get(row, c))).count();
            let from_right: usize =
                take_until((col + 1..tree.cols).map(|c| height <= tree.get(row, c))).count();
            let from_top: usize =
                take_until((0..row).rev().map(|r| height <= tree.get(r, col))).count();
            let from_btm: usize =
                take_until((row + 1..tree.rows).map(|r| height <= tree.get(r, col))).count();

            let scenic_score = from_left * from_right * from_top * from_btm;
            println!(
                "{} {} {}: {} {} {} {}",
                row, col, scenic_score, from_left, from_right, from_top, from_btm
            );

            scenic_score
        })
        .max()
        .unwrap()
}

fn main() {
    dbg!("{}", run1(INPUT.trim()));
    dbg!("{}", run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
30373
25512
65332
33549
35390
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 8);
    }
}

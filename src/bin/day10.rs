const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

struct Signal(isize);

impl Signal {
    fn parse(input: &str) -> impl Iterator<Item = Signal> + '_ {
        input
            .trim()
            .lines()
            .flat_map(|l| {
                if let Some(Ok(s)) = l.strip_prefix("addx ").map(|c| c.parse::<isize>()) {
                    [Some(Signal(0)), Some(Signal(s))].into_iter()
                } else if l.starts_with("noop") {
                    [Some(Signal(0)), None].into_iter()
                } else {
                    unreachable!()
                }
            })
            .flatten()
            .chain([Signal(0), Signal(0)].into_iter())
    }
}

fn run1(input: &str) -> usize {
    Signal::parse(input)
        .map(|i| i.0)
        .scan(1isize, |signal, add| {
            let cur_signal = *signal;
            *signal += add;
            Some(cur_signal)
        })
        .enumerate()
        .map(|(idx, signal)| (idx as isize + 1, signal))
        .filter(|&(idx, _)| idx == 20 || ((idx + 20) % 40 == 0))
        .fold(0isize, |o, (idx, signal)| {
            println!("{idx} {signal} {}", idx * signal);
            o + idx * signal
        }) as usize
}

fn run2(input: &str) {
    Signal::parse(input)
        .map(|i| i.0)
        .scan(1isize, |signal, add| {
            let cur_signal = *signal;
            *signal += add;
            Some(cur_signal)
        })
        .enumerate()
        .fold([false; 40 * 6], |mut crt, (idx, signal)| {
            let col = idx % 40;
            if (col as isize - signal).abs() <= 1 {
                crt[idx] = true;
            }

            crt
        })
        .into_iter()
        .enumerate()
        .for_each(|(idx, pixel)| {
            print!("{}", if pixel { '#' } else { '.' });
            if (idx + 1) % 40 == 0 {
                println!();
            }
        })
}

fn main() {
    run1(INPUT.trim());
    run2(INPUT.trim());
}

const SAMPLE01: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 13140);
    }

    #[test]
    fn test2() {
    }
}

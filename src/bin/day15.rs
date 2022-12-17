use std::collections::HashSet;

use itertools::Itertools;

use nom::{bytes::complete::tag, character, error::ErrorKind, sequence::tuple};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug, Clone, Copy)]
struct Sensor(i64, i64);
#[derive(Debug, Clone, Copy)]
struct Beacon(i64, i64);

impl Sensor {
    fn manhattan_distance(&self, Beacon(x, y): Beacon) -> i64 {
        (self.0 - x).abs() + (self.1 - y).abs()
    }
}

fn parse(input: &str) -> impl Iterator<Item = (Sensor, Beacon)> + '_ {
    input.trim().lines().map(|l| {
        let (_, x, _, y, _, bx, _, by) = tuple::<_, _, (_, ErrorKind), _>((
            tag("Sensor at x="),
            character::complete::i64,
            tag(", y="),
            character::complete::i64,
            tag(": closest beacon is at x="),
            character::complete::i64,
            tag(", y="),
            character::complete::i64,
        ))(l)
        .unwrap()
        .1;
        (Sensor(x, y), Beacon(bx, by))
    })
}

fn run1(input: &'static str, target: i64) -> usize {
    let values = parse(input)
        .flat_map(|(sensor, beacon)| {
            let d = sensor.manhattan_distance(beacon).abs();
            let radius_x = d - (sensor.1 - target).abs();
            (sensor.0 - radius_x)..(sensor.0 + radius_x)
        })
        .collect::<HashSet<_>>();

    values.len()
}

fn run2(input: &'static str, max: i64) -> i64 {
    let sensor_md = parse(input)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan_distance(beacon).abs()))
        .collect::<Vec<_>>();

    let r = (0..=max).find_map(|y| {
        let mut this_line_ranges = sensor_md
            .iter()
            .filter_map(|(sensor, d)| {
                let radius_x = d - (sensor.1 - y).abs();
                if radius_x <= 0 {
                    None
                } else {
                    Some((sensor.0 - radius_x, sensor.0 + radius_x))
                }
            })
            .sorted_by_key(|r| r.0);

        let mut last_end = this_line_ranges.next().unwrap().1;
        for (start, end) in this_line_ranges {
            if last_end >= start - 1 {
                last_end = i64::max(last_end, end);
            } else {
                let p = 4000000 * (last_end + 1) + y;
                return Some(p);
            }
        }

        None
    });

    r.unwrap()
}

fn main() {
    dbg!(run1(INPUT.trim(), 2000000));
    dbg!(run2(INPUT.trim(), 4000000));
    println!("Done");
}

#[cfg(test)]
const SAMPLE01: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01, 10), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01, 20), 56000011);
    }
}

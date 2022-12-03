fn main() {
    let mut elves =
        include_str!("../../inputs/day01.txt")
            .lines()
            .fold(vec![0u64], |mut elves, line| {
                if line.is_empty() {
                    elves.push(0);
                } else {
                    let calories = line.parse::<u64>().unwrap();
                    *elves.last_mut().unwrap() += calories;
                }

                elves
            });

    elves.sort();
    let max_count = elves.iter().rev().next().unwrap();
    let sum: u64 = elves.iter().rev().take(3).sum();

    println!("Max elf has {max_count} calories");
    println!("The first three elves have {sum} calories");
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day02.txt");

    enum Move {
        Rock,
        Paper,
        Scissor,
    }

    impl From<char> for Move {
        fn from(c: char) -> Self {
            match c {
                'A' | 'X' => Move::Rock,
                'B' | 'Y' => Move::Paper,
                'C' | 'Z' => Move::Scissor,
                _ => panic!(),
            }
        }
    }

    impl Move {
        fn wins_against(&self) -> Self {
            match self {
                Move::Rock => Move::Scissor,
                Move::Paper => Move::Rock,
                Move::Scissor => Move::Paper,
            }
        }

        fn loses_against(&self) -> Self {
            match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissor,
                Move::Scissor => Move::Rock,
            }
        }

        fn value(&self) -> usize {
            match self {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissor => 3,
            }
        }
    }

    let (score1, score2) = INPUT
        .lines()
        .map(|line| {
            (
                line.as_bytes()[0] as char,
                line.as_bytes()[2] as char,
            )
        })
        .fold((0, 0), |(mut score1, mut score2), (opponent, player)| {
            // Intrinsic value
            score1 += Move::from(player).value();

            // Win/lose/draw
            score1 += match (Move::from(player), Move::from(opponent)) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 0,
                (Move::Rock, Move::Scissor) => 6,
                (Move::Paper, Move::Rock) => 6,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissor) => 0,
                (Move::Scissor, Move::Rock) => 0,
                (Move::Scissor, Move::Paper) => 6,
                (Move::Scissor, Move::Scissor) => 3,
            };

            score2 += match player {
                'X' => Move::from(opponent).wins_against().value(),
                'Y' => 3 + Move::from(opponent).value(),
                'Z' => 6 + Move::from(opponent).loses_against().value(),
                _ => unreachable!(),
            };

            (score1, score2)
        });

    println!("Score 1: {score1}");
    println!("Score 2: {score2}");
}


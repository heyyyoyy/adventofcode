use std::str::SplitWhitespace;


enum RoundScore {
    LOST = 0,
    DRAW = 3,
    WIN = 6
}


impl<'a> From<&'a str> for RoundScore {
    fn from(state: &str) -> Self {
        match state {
            "X" => RoundScore::LOST,
            "Y" => RoundScore::DRAW,
            "Z" => RoundScore::WIN,
            _ => unreachable!()
        }
    }
}


enum Enemy {
    A = 1, // rock
    B = 2, // paper
    C = 3  // scissors
}

impl<'a> From<&'a str> for Enemy {
    fn from(state: &str) -> Self {
        match state {
            "A" => Enemy::A,
            "B" => Enemy::B,
            "C" => Enemy::C,
            _ => unreachable!()
        }
    }
}

enum Player {
    X = 1, // rock
    Y = 2, // paper
    Z = 3  // scissors
}


struct Round(Enemy, RoundScore);

impl Round {
    fn build(enemy: Enemy, round_res: RoundScore) -> Self {
        Round(enemy, round_res)
    }

    fn round_score(&self) -> u64 {
        match self {
            Self(Enemy::A, x) => {
                match x {
                    RoundScore::LOST => Player::Z as u64 + RoundScore::LOST as u64,
                    RoundScore::DRAW => Player::X as u64 + RoundScore::DRAW as u64,
                    RoundScore::WIN => Player::Y as u64 + RoundScore::WIN as u64
                }
            },
            Self(Enemy::B, x) => {
                match x {
                    RoundScore::LOST => Player::X as u64 + RoundScore::LOST as u64,
                    RoundScore::DRAW => Player::Y as u64 + RoundScore::DRAW as u64,
                    RoundScore::WIN => Player::Z as u64 + RoundScore::WIN as u64
                }
            },
            Self(Enemy::C, x) => {
                match x {
                    RoundScore::LOST => Player::Y as u64 + RoundScore::LOST as u64,
                    RoundScore::DRAW => Player::Z as u64 + RoundScore::DRAW as u64,
                    RoundScore::WIN => Player::X as u64 + RoundScore::WIN as u64
                }
            }
        }
        
    }

}

impl<'a> From<SplitWhitespace<'a>> for Round {
    fn from(mut text: SplitWhitespace<'a>) -> Self {
        Round::build(text.next().unwrap().into(), text.next().unwrap().into())
    }
}


fn rock_paper_scissors(input_str: &str) -> u64 {
    input_str
    .lines()
    .fold(0, |mut acc, line| {
        acc += Round::from(line.split_whitespace()).round_score();
        acc
    })
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_rock_paper_scissors() {
        let input_str = "A Y
B X
C Z";
        assert_eq!(12, rock_paper_scissors(input_str));
    }

    #[test]
    fn test_rock_paper_scissors_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_2\\input.txt").unwrap();
        assert_eq!(13889, rock_paper_scissors(&input_str));
    }
}

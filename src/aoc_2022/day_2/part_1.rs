use std::str::SplitWhitespace;


enum RoundResult {
    Lost = 0,
    Draw = 3,
    Win = 6
}


enum Enemy {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl<'a> From<&'a str> for Enemy {
    fn from(state: &str) -> Self {
        match state {
            "A" => Enemy::Rock,
            "B" => Enemy::Paper,
            "C" => Enemy::Scissors,
            _ => unreachable!()
        }
    }
}

enum Player {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl<'a> From<&'a str> for Player {
    fn from(state: &str) -> Self {
        match state {
            "X" => Player::Rock,
            "Y" => Player::Paper,
            "Z" => Player::Scissors,
            _ => unreachable!()
        }
    }
}


struct Round(Enemy, Player);

impl Round {
    fn build(enemy: Enemy, player: Player) -> Self {
        Round(enemy, player)
    }

    fn round_score(&self) -> u64 {
        match self {
            Self(Enemy::Rock, x) => {
                match x {
                    Player::Rock => Player::Rock as u64 + RoundResult::Draw as u64,
                    Player::Paper => Player::Paper as u64 + RoundResult::Win as u64,
                    Player::Scissors => Player::Scissors as u64 + RoundResult::Lost as u64
                }
            },
            Self(Enemy::Paper, x) => {
                match x {
                    Player::Rock => Player::Rock as u64 + RoundResult::Lost as u64,
                    Player::Paper => Player::Paper as u64 + RoundResult::Draw as u64,
                    Player::Scissors => Player::Scissors as u64 + RoundResult::Win as u64
                }
            },
            Self(Enemy::Scissors, x) => {
                match x {
                    Player::Rock => Player::Rock as u64 + RoundResult::Win as u64,
                    Player::Paper => Player::Paper as u64 + RoundResult::Lost as u64,
                    Player::Scissors => Player::Scissors as u64 + RoundResult::Draw as u64
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
    .map(|line| {
        Round::from(line.split_whitespace()).round_score()
    })
    .sum()
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
        assert_eq!(15, rock_paper_scissors(input_str));
    }

    #[test]
    fn test_rock_paper_scissors_from_file() {
        let input_str = fs::read_to_string(".\\src\\aoc_2022\\day_2\\input.txt").unwrap();
        assert_eq!(14827, rock_paper_scissors(&input_str));
    }
}

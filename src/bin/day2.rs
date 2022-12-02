static DAY: u8 = 2;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", game_score(&input, false));
    println!("{DAY}b: {}", game_score(&input, true));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn draws_against(&self) -> Shape {
        *self
    }

    fn loses_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

struct Round {
    opponent: Shape,
    you: Shape,
}

impl Round {
    fn new(input: &str, result_indicator: bool) -> Round {
        assert_eq!(input.len(), 3);
        let chars : Vec<char> = input.chars().collect();
        let opponent = match chars[0] {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };
        let you = if !result_indicator {
            match chars[2] {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissors,
                _ => unreachable!(),
            }
        } else {
            match chars[2] {
                'X' => opponent.wins_against(),
                'Y' => opponent.draws_against(),
                'Z' => opponent.loses_against(),
                _ => unreachable!(),
            }
        };

        Round { opponent, you }
    }

    fn outcome_score(&self) -> u32 {
        if self.you.wins_against() == self.opponent {
            6
        } else if self.you.draws_against() == self.opponent {
            3
        } else if self.you.loses_against() == self.opponent {
            0
        } else {
            unreachable!()
        }
    }

    fn score(&self) -> u32 {
        self.outcome_score() + self.you.score()
    }
}

fn game_score(input: &[String], result_indicator: bool) -> u32 {
    let rounds = input.iter()
                      .map(|x| Round::new(x, result_indicator))
                      .collect::<Vec<_>>();
    rounds.iter()
          .map(|x| x.score())
          .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        assert_eq!(game_score(&input, false), 15);
        assert_eq!(game_score(&input, true), 12);
    }
}

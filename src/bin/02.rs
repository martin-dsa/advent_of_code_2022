enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn from_str(letter: &str) -> Self {
        match letter {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("unexpected input"),
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Lose,
}
impl GameResult {
    fn points(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
    fn from_str(letter: &str) -> Self {
        match letter {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("unexpected input"),
        }
    }
}

fn calc1((m1, m2): (Move, Move)) -> u32 {
    let r = match (m1, &m2) {
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => GameResult::Draw,

        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => GameResult::Lose,

        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => GameResult::Win,
    };
    r.points() + m2.points()
}

fn calc2((m1, r): (Move, GameResult)) -> u32 {
    let m2: Move = match (m1, &r) {
        (Move::Rock, GameResult::Draw)
        | (Move::Scissors, GameResult::Win)
        | (Move::Paper, GameResult::Lose) => Move::Rock,

        (Move::Paper, GameResult::Draw)
        | (Move::Rock, GameResult::Win)
        | (Move::Scissors, GameResult::Lose) => Move::Paper,

        (Move::Scissors, GameResult::Draw)
        | (Move::Rock, GameResult::Lose)
        | (Move::Paper, GameResult::Win) => Move::Scissors,
    };

    r.points() + m2.points()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|x| (Move::from_str(x.0), Move::from_str(x.1)))
                .map(calc1)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|x| (Move::from_str(x.0), GameResult::from_str(x.1)))
                .map(calc2)
        })
        .sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter)]
enum Plays {
    Rock,
    Paper,
    Scissors,
}

impl Plays {
    fn value(&self) -> usize {
        match self {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
        }
    }
}

impl PartialOrd for Plays {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Not using a match here as I don't want to duplicate code
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        } else if self == &Plays::Rock && other == &Plays::Scissors
            || self == &Plays::Paper && other == &Plays::Rock
            || self == &Plays::Scissors && other == &Plays::Paper
        {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

#[derive(PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn value(&self) -> usize {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        }
    }

    fn build(opponent_play: &Plays, my_play: &Plays) -> RoundResult {
        if my_play > opponent_play {
            RoundResult::Win
        } else if my_play < opponent_play {
            RoundResult::Lose
        } else {
            RoundResult::Draw
        }
    }
}

fn parse_row_first_hypothesis(row: &str) -> usize {
    let mut values = row.split(" ");

    let opponent_play = match values.next() {
        Some("A") => Plays::Rock,
        Some("B") => Plays::Paper,
        Some("C") => Plays::Scissors,
        _ => panic!("Opponent play not understood"),
    };

    let my_play = match values.next() {
        Some("X") => Plays::Rock,
        Some("Y") => Plays::Paper,
        Some("Z") => Plays::Scissors,
        _ => panic!("Selected play not understood"),
    };

    let result = RoundResult::build(&opponent_play, &my_play);

    let result = my_play.value() + result.value();

    result
}

fn parse_row_second_hypothesis(row: &str) -> usize {
    let mut values = row.split(" ");

    let opponent_play = match values.next() {
        Some("A") => Plays::Rock,
        Some("B") => Plays::Paper,
        Some("C") => Plays::Scissors,
        _ => panic!("Opponent play not understood"),
    };

    let result = match values.next() {
        Some("X") => RoundResult::Lose,
        Some("Y") => RoundResult::Draw,
        Some("Z") => RoundResult::Win,
        _ => panic!("Result not understood"),
    };

    // We re-use the build function we had earlier !
    for my_play in Plays::iter() {
        if RoundResult::build(&opponent_play, &my_play) == result {
            return my_play.value() + result.value();
        }
    }

    // We should never reach this point so we panic
    panic!("Cannot select proper play to make for the desired result");
}

pub fn calculate_score_first_method(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, x| acc + parse_row_first_hypothesis(x))
}

pub fn calculate_score_second_method(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, x| acc + parse_row_second_hypothesis(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let paper = Plays::Paper;
        let rock = Plays::Rock;

        assert!(paper > rock);
    }

    #[test]
    fn test_other_ordering() {
        let paper = Plays::Paper;
        let other_paper = Plays::Paper;

        assert!(paper == other_paper);
    }
}

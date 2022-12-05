use std::str::Split;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A play in a game of Rocks Paper Scissors
#[derive(PartialEq, EnumIter)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    /// Returns what this [`Play`] beats
    fn beats(&self) -> &Play {
        match self {
            Play::Rock => &Play::Scissors,
            Play::Paper => &Play::Rock,
            Play::Scissors => &Play::Paper,
        }
    }

    /// Returns the point value of this [`Play`]
    fn value(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
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

    fn build(opponent_play: &Play, my_play: &Play) -> RoundResult {
        if my_play == opponent_play {
            RoundResult::Draw
        } else if my_play.beats() == opponent_play {
            RoundResult::Win
        } else if opponent_play.beats() == my_play {
            RoundResult::Lose
        } else {
            panic!("Could not compare plays")
        }
    }
}

fn get_opponent_play(values: &mut Split<&str>) -> Play {
    let opponent_play = match values.next() {
        Some("A") => Play::Rock,
        Some("B") => Play::Paper,
        Some("C") => Play::Scissors,
        Some(x) => panic!("Opponent play not understood. Received {x}"),
        None => panic!("Empty row found"),
    };

    opponent_play
}

fn parse_row_first_hypothesis(row: &str) -> usize {
    let mut values = row.split(" ");

    let opponent_play = get_opponent_play(&mut values);

    let my_play = match values.next() {
        Some("X") => Play::Rock,
        Some("Y") => Play::Paper,
        Some("Z") => Play::Scissors,
        Some(x) => panic!("My play not understood. Received {x}"),
        None => panic!("No play instruction found"),
    };

    let result = RoundResult::build(&opponent_play, &my_play);

    my_play.value() + result.value()
}

fn parse_row_second_hypothesis(row: &str) -> usize {
    let mut values = row.split(" ");

    let opponent_play = get_opponent_play(&mut values);

    let result = match values.next() {
        Some("X") => RoundResult::Lose,
        Some("Y") => RoundResult::Draw,
        Some("Z") => RoundResult::Win,
        Some(x) => panic!("My play not understood. Received {x}"),
        None => panic!("No play instruction found"),
    };

    // We re-use the build function we had earlier !
    for my_play in Play::iter() {
        if RoundResult::build(&opponent_play, &my_play) == result {
            return my_play.value() + result.value();
        }
    }

    // We should never reach this point so we panic
    panic!("Cannot select proper play for the desired result");
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

    const DEMO_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_calculate_score_first_method() {
        assert_eq!(calculate_score_first_method(DEMO_INPUT), 15);
    }

    #[test]
    fn test_first_row() {
        assert_eq!(calculate_score_first_method("A Y"), 8);
    }

    #[test]
    fn test_calculate_score_second_method() {
        assert_eq!(calculate_score_second_method(DEMO_INPUT), 12);
    }
}

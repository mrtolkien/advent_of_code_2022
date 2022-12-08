use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A play in a game of Rocks Paper Scissors
#[derive(PartialEq, EnumIter, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    /// Returns what this [`Play`] beats
    const fn beats(&self) -> &'static Self {
        match self {
            Self::Rock => &Self::Scissors,
            Self::Paper => &Self::Rock,
            Self::Scissors => &Self::Paper,
        }
    }

    /// Returns the point value of this [`Play`]
    const fn value(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
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
    const fn value(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn build(opponent_play: &Play, my_play: &Play) -> Self {
        if my_play.beats() == opponent_play {
            Self::Win
        } else if opponent_play.beats() == my_play {
            Self::Lose
        } else {
            assert_eq!(opponent_play, my_play);
            Self::Draw
        }
    }
}

fn get_opponent_play(letter: &str) -> Play {
    let opponent_play = match letter {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        x => panic!("Opponent play not understood. Received {x}"),
    };

    opponent_play
}

fn parse_row_first_hypothesis(row: &str) -> usize {
    let (opponent_play, my_play) = row
        .split_once(' ')
        .expect("Could not understand play: {row:?");

    let opponent_play = get_opponent_play(opponent_play);

    let my_play = match my_play {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        x => panic!("My play not understood. Received {x}"),
    };

    let result = RoundResult::build(&opponent_play, &my_play);

    my_play.value() + result.value()
}

fn parse_row_second_hypothesis(row: &str) -> usize {
    let (opponent_play, my_play) = row
        .split_once(' ')
        .expect("Could not understand play: {row:?");

    let opponent_play = get_opponent_play(opponent_play);

    let result = match my_play {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        x => panic!("My play not understood. Received {x}"),
    };

    Play::iter()
        .find(|p| RoundResult::build(&opponent_play, p) == result)
        .expect("Did not find a play that matches the result")
        .value()
        + result.value()
}

#[must_use] pub fn calculate_score_first_method(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, x| acc + parse_row_first_hypothesis(x))
}

#[must_use] pub fn calculate_score_second_method(input: &str) -> usize {
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

    #[test]
    #[should_panic]
    fn test_calculate_score_second_method_panic() {
        calculate_score_second_method("A A");
    }

    // Testing that we make the first method panic
    #[test]
    #[should_panic]
    fn test_calculate_score_first_method_panic() {
        calculate_score_first_method("A A");
    }

    // Testing that we make the opponent play parser panic
    #[test]
    #[should_panic]
    fn test_get_opponent_play_panic() {
        get_opponent_play("D");
    }

    // Testing all beats results
    #[test]
    fn test_beats() {
        assert_eq!(Play::Rock.beats(), &Play::Scissors);
        assert_eq!(Play::Paper.beats(), &Play::Rock);
        assert_eq!(Play::Scissors.beats(), &Play::Paper);
    }
}

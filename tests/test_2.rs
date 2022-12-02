const DEMO_INPUT: &str = "A Y
B X
C Z";

#[test]
fn test_calculate_score_first_method() {
    assert_eq!(
        advent_of_code_2022::day_2::calculate_score_first_method(DEMO_INPUT),
        15
    );
}

#[test]
fn test_first_row() {
    assert_eq!(
        advent_of_code_2022::day_2::calculate_score_first_method("A Y"),
        8
    );
}

#[test]
fn test_calculate_score_second_method() {
    assert_eq!(
        advent_of_code_2022::day_2::calculate_score_second_method(DEMO_INPUT),
        12
    );
}

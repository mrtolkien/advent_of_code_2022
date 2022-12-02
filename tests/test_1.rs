const DEMO_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn test_get_max_calories() {
    assert_eq!(
        advent_of_code_2022::day_1::get_max_calories(DEMO_INPUT),
        24000
    );
}

#[test]
fn test_empty_input() {
    assert_eq!(advent_of_code_2022::day_1::get_max_calories(""), 0);
}

#[test]
#[should_panic(expected = "Could not parse integer: TEST")]
fn test_faulty_input() {
    assert_eq!(advent_of_code_2022::day_1::get_max_calories("TEST"), 0);
}

#[test]
fn test_get_top_three_calories() {
    assert_eq!(
        advent_of_code_2022::day_1::get_sum_top_three_calories(DEMO_INPUT),
        45000
    );
}

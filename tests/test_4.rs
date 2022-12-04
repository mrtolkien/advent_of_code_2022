// TODO Those should all be unit tests actually

const DEMO_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_first_part() {
    assert_eq!(
        advent_of_code_2022::day_4::fully_overlapping_sections(DEMO_INPUT),
        2
    );
}

#[test]
fn test_second_part() {
    assert_eq!(
        advent_of_code_2022::day_4::overlapping_sections(DEMO_INPUT),
        4
    );
}

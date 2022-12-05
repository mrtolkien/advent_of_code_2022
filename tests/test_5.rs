const DEMO_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test_first_part() {
    assert_eq!(
        advent_of_code_2022::day_5::find_top_crates(
            DEMO_INPUT,
            advent_of_code_2022::day_5::CrateMoverVersion::V1
        ),
        "CMZ"
    );
}

#[test]
fn test_second_part() {
    assert_eq!(
        advent_of_code_2022::day_5::find_top_crates(
            DEMO_INPUT,
            advent_of_code_2022::day_5::CrateMoverVersion::V2
        ),
        "MCD"
    );
}

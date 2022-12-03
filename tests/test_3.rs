const DEMO_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn test_sum_priorities() {
    assert_eq!(advent_of_code_2022::day_3::sum_priorities(DEMO_INPUT), 157);
}

#[test]
fn test_sum_triple_priorities() {
    assert_eq!(
        advent_of_code_2022::day_3::sum_triple_priorities(DEMO_INPUT),
        70
    );
}

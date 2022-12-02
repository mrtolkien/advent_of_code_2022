use std::fs;

fn main() {
    // TODO Add command line arguments to run a specific day

    let day_1_input =
        fs::read_to_string("data/day_1.txt").expect("Should have been able to read the file");

    println!(
        "Day 1.1 result: {}",
        advent_of_code_2022::day_1::get_max_calories(&day_1_input)
    );

    println!(
        "Day 1.2 result: {}",
        advent_of_code_2022::day_1::get_sum_top_three_calories(&day_1_input)
    );
}

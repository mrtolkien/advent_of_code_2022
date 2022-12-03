// TODO Learn how to do test coverage in Rust!
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Which day to run
    day: Option<u8>,
}

fn get_day_input(day: u8) -> String {
    let file_name = format!("data/day_{}.txt", day);

    fs::read_to_string(&file_name)
        .unwrap_or_else(|err| panic!("Could not read file {} - Error: {err}", file_name))
}

fn main() {
    let args = Args::parse();

    // TODO Find a better syntax for this, maybe `if let` ?
    // TODO -> Make days into a struct made of 2 functions
    if args.day.unwrap_or(1) == 1 {
        let input = get_day_input(1);

        println!(
            "Day 1.1 result: {}",
            advent_of_code_2022::day_1::get_max_calories(&input)
        );

        println!(
            "Day 1.2 result: {}",
            advent_of_code_2022::day_1::get_sum_top_three_calories(&input)
        );
    }
    if args.day.unwrap_or(2) == 2 {
        let input = get_day_input(2);

        println!(
            "Day 2.1 result: {}",
            advent_of_code_2022::day_2::calculate_score_first_method(&input)
        );
        println!(
            "Day 2.1 result: {}",
            advent_of_code_2022::day_2::calculate_score_second_method(&input)
        );
    }
}

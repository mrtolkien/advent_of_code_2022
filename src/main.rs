use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Which day to run
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    // TODO Make a function to get the day's input

    if args.day.unwrap_or(1) == 1 {
        let input =
            fs::read_to_string("data/day_1.txt").expect("Should have been able to read the file");

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
        let input =
            fs::read_to_string("data/day_2.txt").expect("Should have been able to read the file");

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

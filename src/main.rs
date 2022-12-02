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

    if args.day.is_none() || args.day.unwrap() == 1 {
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
}

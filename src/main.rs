// TODO Learn how to do test coverage in Rust!
// -> First add coverage with Tarpaulin and disable CLI test, then re-add CLI tests
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
            "Day 2.2 result: {}",
            advent_of_code_2022::day_2::calculate_score_second_method(&input)
        );
    }

    if args.day.unwrap_or(3) == 3 {
        let input = get_day_input(3);

        println!(
            "Day 3.1 result: {}",
            advent_of_code_2022::day_3::sum_priorities(&input)
        );

        println!(
            "Day 3.2 result: {}",
            advent_of_code_2022::day_3::sum_triple_priorities(&input)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_run() {
        main();
    }
}

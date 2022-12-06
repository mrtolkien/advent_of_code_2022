use advent_of_code_2022::{day_1, day_2, day_3, day_4, day_5, day_6};
use std::{fmt::Display, fs};

fn get_day_input(day: u8) -> String {
    let file_name = format!("data/day_{day}.txt");

    fs::read_to_string(&file_name)
        .unwrap_or_else(|err| panic!("Could not read file {file_name} - Error: {err}"))
}

fn main() {
    for i in 1..=6 {
        let input = get_day_input(i);
        let input = input.as_str();

        // We need to define results as dynamically typed first
        let results: (Box<dyn Display>, Box<dyn Display>);

        results = match i {
            1 => (
                Box::new(day_1::get_max_calories(input)),
                Box::new(day_1::get_sum_top_three_calories(input)),
            ),
            2 => (
                Box::new(day_2::calculate_score_first_method(input)),
                Box::new(day_2::calculate_score_second_method(input)),
            ),
            3 => (
                Box::new(day_3::sum_priorities(input)),
                Box::new(day_3::sum_triple_priorities(input)),
            ),
            4 => (
                Box::new(day_4::fully_overlapping_sections(input)),
                Box::new(day_4::overlapping_sections(input)),
            ),
            5 => (
                Box::new(day_5::find_top_crates(input, day_5::CrateMoverVersion::V1)),
                Box::new(day_5::find_top_crates(input, day_5::CrateMoverVersion::V2)),
            ),
            6 => (
                Box::new(day_6::get_packet_start(input, 4)),
                Box::new(day_6::get_packet_start(input, 14)),
            ),
            _ => panic!("No solution for day {i}"),
        };

        println!("Day {i}.1 result: {}", results.0);
        println!("Day {i}.2 result: {}", results.1);
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

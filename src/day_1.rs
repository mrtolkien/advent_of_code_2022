use rayon::prelude::*;

fn get_calories(input: &str) -> usize {
    input
        .lines()
        .map(|line| match line.parse::<usize>() {
            Ok(line_value) => line_value,
            Err(_) => panic!("Could not parse integer: {line}"),
        })
        .sum()
}

pub fn get_max_calories(input: &str) -> usize {
    let mut maximum_calories = 0;

    for elf_ration in input.split("\n\n") {
        let calories = get_calories(elf_ration);

        if calories > maximum_calories {
            maximum_calories = calories;
        }
    }

    maximum_calories
}

pub fn get_sum_top_three_calories(input: &str) -> usize {
    // We'll keep this sorted
    let mut top_3_calories: Vec<usize> = vec![0, 0, 0];

    for elf_ration in input.split("\n\n") {
        let calories = get_calories(elf_ration);

        if calories > top_3_calories[0] {
            // We put the new value instead of the first one
            top_3_calories[0] = calories;

            // We sort so the first one is the smallest
            // I'm not sure about the rust sort implementation but it should be fast since the
            //  list was already sorted
            top_3_calories.sort();
        }
    }

    // P A R A L L E L / / /
    top_3_calories.par_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(get_max_calories(DEMO_INPUT), 24000);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(get_max_calories(""), 0);
    }

    #[test]
    #[should_panic(expected = "Could not parse integer: TEST")]
    fn test_faulty_input() {
        assert_eq!(get_max_calories("TEST"), 0);
    }

    #[test]
    fn test_get_top_three_calories() {
        assert_eq!(get_sum_top_three_calories(DEMO_INPUT), 45000);
    }
}

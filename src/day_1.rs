fn get_calories(input: &str) -> usize {
    // TODO Write that in a functional way with fold
    let mut calories = 0;

    for row in input.lines() {
        match row.parse::<usize>() {
            Ok(row_value) => calories += row_value,
            Err(_) => panic!("Could not parse integer: {}", row),
        }
    }

    calories
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

    top_3_calories.iter().sum()
}

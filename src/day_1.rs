// TODO Make code common + handle errors instead of unwrap()

pub fn get_max_calories(input: &str) -> usize {
    let mut maximum_calories = 0;

    for elf_ration in input.split("\n\n") {
        let mut calories = 0;

        for row in elf_ration.lines() {
            calories += row.trim().parse::<usize>().unwrap();
        }

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
        let mut calories = 0;

        for row in elf_ration.lines() {
            calories += row.trim().parse::<usize>().unwrap();
        }

        if calories > top_3_calories[0] {
            // We put the new value instead of the first one
            top_3_calories[0] = calories;

            // We sort so the first one is the smallest
            top_3_calories.sort();
        }
    }

    top_3_calories.iter().sum()
}

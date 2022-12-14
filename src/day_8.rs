#[must_use]
pub fn get_visible_trees_count(input: &str) -> usize {
    let tree_map = create_tree_map(input);

    let len = tree_map.len();

    // Iterate on all rows
    (0..len)
        // We map each row to the # of visible trees in the row
        .map(|row_idx| {
            // Iterate on column
            (0..len)
                // Filter on visible trees and count them
                .filter(|col_idx| {
                    let tree_height = tree_map[row_idx][*col_idx];

                    // Visible from the left
                    tree_map[row_idx][0..*col_idx]
                        .iter()
                        .all(|h| h < &tree_height) ||
                    // Visible from the right
                    tree_map[row_idx][col_idx + 1..len]
                    .iter()
                    .all(|h| h < &tree_height) ||
                    // Visible from above
                    tree_map[0..row_idx]
                            .iter()
                            .all(|h| h[*col_idx] < tree_height) ||
                    // Visible from below
                    tree_map[row_idx + 1..len]
                            .iter()
                            .all(|h| h[*col_idx] < tree_height)
                })
                .count()
        })
        // Sum each row
        .sum()
}

#[must_use]
pub fn get_max_scenic_score(input: &str) -> usize {
    let mut tree_map = create_tree_map(input);

    let len = tree_map.len();

    // We put all the borders at 10 to always count them as a limit
    // A bit dirty but it makes it much simpler as there's no take_until function in std or itertools
    for row in &mut tree_map {
        row[0] = 10;
        row[len - 1] = 10;
    }

    for col_idx in 0..len {
        tree_map[0][col_idx] = 10;
        tree_map[len - 1][col_idx] = 10;
    }

    // We check everything except borders (always a score of 0 anyways)
    (1..len - 1)
        // We map each row to its maximum score
        .map(|row_idx| {
            (1..len - 1)
                // We map each column to its score then use max() on it
                .map(|col_idx| {
                    let tree_height = &tree_map[row_idx][col_idx];

                    // This stops at most at the border, and we add 1 as we always see at least 1 tree
                    let left_score = tree_map[row_idx][0..col_idx]
                        .iter()
                        .rev()
                        .take_while(|h| h < &tree_height)
                        .count()
                        + 1;

                    let right_score = tree_map[row_idx][col_idx + 1..len]
                        .iter()
                        .take_while(|h| h < &tree_height)
                        .count()
                        + 1;

                    let top_score = tree_map[0..row_idx]
                        .iter()
                        .rev()
                        .take_while(|h| h[col_idx] < *tree_height)
                        .count()
                        + 1;

                    let bottom_score = tree_map[row_idx + 1..len]
                        .iter()
                        .take_while(|h| h[col_idx] < *tree_height)
                        .count()
                        + 1;

                    left_score * right_score * top_score * bottom_score
                })
                // Maxing on the column
                .max()
                .expect("No max found in column {col_idx}")
        })
        // Maxing on the row
        .max()
        .expect("No max found for input {tree_map}")
}

fn create_tree_map(input: &str) -> Vec<Vec<u8>> {
    let mut result = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for number in line.chars() {
            row.push(
                number
                    .to_digit(10)
                    .expect("Cannot cast {number} as an integer") as u8,
            );
        }

        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(get_visible_trees_count(DEMO_INPUT), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_max_scenic_score(DEMO_INPUT), 8);
    }

    #[test]
    fn test_create_tree_map() {
        let tree_map = create_tree_map(DEMO_INPUT);
        assert_eq!(tree_map[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(tree_map[1][0], 2);
    }
}

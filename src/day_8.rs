pub fn get_visible_trees_count(input: &str) -> usize {
    0
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
}

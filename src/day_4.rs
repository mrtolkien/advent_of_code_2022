pub fn fully_overlapping_sections(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let mut parts = line.split(',');

        let first_interval = string_to_interval(parts.next().unwrap());
        let second_interval = string_to_interval(parts.next().unwrap());

        // TODO REWRITE
        if (first_interval.0 >= second_interval.0 && first_interval.1 <= second_interval.1)
            || (second_interval.0 >= first_interval.0 && second_interval.1 <= first_interval.1)
        {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn overlapping_sections(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let mut parts = line.split(',');

        let x = string_to_interval(parts.next().unwrap());
        let y = string_to_interval(parts.next().unwrap());

        // Smart solution:
        // https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
        if x.0 <= y.1 && y.0 <= x.1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn string_to_interval(input: &str) -> (usize, usize) {
    let mut parts = input.split('-');
    let first = parts.next().unwrap().parse::<usize>().unwrap();
    let second = parts.next().unwrap().parse::<usize>().unwrap();

    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_first_part() {
        assert_eq!(fully_overlapping_sections(DEMO_INPUT), 2);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(overlapping_sections(DEMO_INPUT), 4);
    }

    #[test]
    fn simple_overlapping() {
        assert_eq!(overlapping_sections("2-4,6-8"), 0);
        assert_eq!(overlapping_sections("5-7,7-9"), 1);
        assert_eq!(overlapping_sections("6-6,4-6"), 1);
        assert_eq!(overlapping_sections("2-6,4-8"), 1);
        assert_eq!(overlapping_sections("2-8,3-7"), 1);
    }
}

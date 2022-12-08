struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    const fn contains(&self, other_interval: &Self) -> bool {
        self.start <= other_interval.start && other_interval.end <= self.end
    }

    const fn overlap(&self, other_interval: &Self) -> bool {
        // Smart solution:
        // https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
        self.start <= other_interval.end && other_interval.start <= self.end
    }
}

fn get_intervals(line: &str) -> (Interval, Interval) {
    let (left, right) = line.split_once(',').expect("Misformed input: {line:?");

    (string_to_interval(left), string_to_interval(right))
}

/// This function takes an input string `input` and returns the number of
/// fully overlapping sections of two intervals.
///
/// # Arguments
///
/// * `input` - A string slice containing the input intervals in the "a-b,x-y" format.
///
/// # Examples
///
/// ```
/// use advent_of_code_2022::day_4::fully_overlapping_sections;
/// let input = "1-4,1-2
/// 1-2,3-4";
/// let result = fully_overlapping_sections(input);
/// assert_eq!(result, 1);
/// ```
pub fn fully_overlapping_sections(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (x, y) = get_intervals(line);

        if x.contains(&y) || y.contains(&x) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn overlapping_sections(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (x, y) = get_intervals(line);

        if x.overlap(&y) {
            acc + 1
        } else {
            acc
        }
    })
}

fn string_to_interval(input: &str) -> Interval {
    let (left, right) = input.split_once('-').expect("Misformed input: {input:?}");

    Interval {
        start: left
            .parse::<usize>()
            .expect("Could not parse {left:?} as integer"),
        end: right
            .parse::<usize>()
            .expect("Could not parse {right:?} as integer"),
    }
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

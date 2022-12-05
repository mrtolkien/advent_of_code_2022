struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn contains(&self, other_interval: &Interval) -> bool {
        self.start <= other_interval.start && other_interval.end <= self.end
    }

    fn overlap(&self, other_interval: &Interval) -> bool {
        // Smart solution:
        // https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
        self.start <= other_interval.end && other_interval.start <= self.end
    }
}

fn get_intervals(line: &str) -> (Interval, Interval) {
    let (left, right) = line.split_once(',').unwrap();

    (string_to_interval(left), string_to_interval(right))
}

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
    let (left, right) = input.split_once('-').unwrap();

    Interval {
        start: left.parse::<usize>().unwrap(),
        end: right.parse::<usize>().unwrap(),
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

pub fn sum_priorities(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (left, right) = line.split_at(line.len() / 2);

        for c in left.chars() {
            if right.contains(c) {
                return acc + get_priority(c);
            }
        }

        panic!("No match found for line: {}", line);
    })
}

pub fn sum_triple_priorities(input: &str) -> usize {
    let mut result = 0;

    let lines_count = input.lines().count();

    // TODO Find a way to do that without unwrap
    for i in 0..lines_count / 3 {
        let line_1 = input.lines().nth(i * 3).unwrap();
        let line_2 = input.lines().nth(i * 3 + 1).unwrap();
        let line_3 = input.lines().nth(i * 3 + 2).unwrap();

        for c in line_1.chars() {
            if line_2.contains(c) && line_3.contains(c) {
                result += get_priority(c);
                break;
            }
        }
    }

    result
}

// Hardcoded constant with space first is simple and fast
const ALPHABET: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(c: char) -> usize {
    match ALPHABET.chars().into_iter().position(|x| x == c) {
        Some(x) => x,
        None => panic!("Unknown char: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('b'), 2);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('B'), 28);
        assert_eq!(get_priority('Z'), 52);
    }
}

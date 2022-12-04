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
    input.lines().array_chunks::<3>().fold(0, |acc, lines| {
        for c in lines[0].chars() {
            if lines[1].contains(c) && lines[2].contains(c) {
                return acc + get_priority(c);
            }
        }

        panic!("No match found for lines: {:?}", lines);
    })
}

// Hardcoded constant with space first is simple and *very* fast
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

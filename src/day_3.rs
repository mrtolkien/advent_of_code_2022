pub fn sum_priorities(input: &str) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let lenght = line.len();
        let left_side = &line[0..lenght / 2];
        let right_side = &line[lenght / 2..lenght];

        for c in left_side.chars() {
            if right_side.contains(c) {
                result += get_priority(c);
                break;
            }
        }
    }

    result
}

pub fn sum_triple_priorities(input: &str) -> usize {
    let mut result = 0;

    let lines_count = input.lines().count();

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

fn get_priority(c: char) -> usize {
    let mut result = 1;

    if c.is_uppercase() {
        result += 26;
    }

    result += ('a'..='z')
        .position(|x| x == c.to_lowercase().next().unwrap())
        .unwrap();

    result
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

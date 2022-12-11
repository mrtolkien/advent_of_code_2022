use nom::{bytes::complete::tag, character::complete::digit1, sequence::delimited, IResult};

struct Monkey {
    starting_items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

pub fn get_monkey_business_level(input: &str) -> usize {
    todo!()
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (_, monkey_id) = delimited(tag("Monkey "), digit1, tag(":"))(s)?;
    let (_, starting_items) = delimited(tag("Starting items: "), digit1, tag("\n"))(s)?;
    let (_, operation_str) = delimited(tag("Operation: "), digit1, tag("\n"))(s)?;
    let (_, divisible_by) = delimited(tag("Test: divisible by "), digit1, tag("\n"))(s)?;

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let monkey = parse_monkey(
            "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
If true: throw to monkey 2
If false: throw to monkey 3",
        )
        .unwrap()
        .1;

        assert_eq!(monkey.starting_items, vec![79, 98]);
        assert_eq!(monkey.divisible_by, 23);
        assert_eq!(monkey.if_true, 2);
        assert_eq!(monkey.if_false, 3);
    }

    const DEMO_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_first_part() {
        assert_eq!(get_monkey_business_level(DEMO_INPUT), 10605)
    }
}

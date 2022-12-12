use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, preceded, tuple},
    IResult,
};

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn apply(&self, value: usize) -> usize {
        match *self {
            Operation::Multiply(m) => m * value,
            Operation::Add(a) => a + value,
            Operation::Square => value * value,
        }
    }
}

pub enum MonkeyBusinessLevel {
    V1,
    V2,
}

pub fn get_monkey_business_level(input: &str, level: MonkeyBusinessLevel) -> usize {
    let mut monkeys = parse_monkeys(input);

    let length = match level {
        MonkeyBusinessLevel::V1 => 20,
        MonkeyBusinessLevel::V2 => 10_000,
    };

    for _ in 0..length {
        monkeys = play_round(monkeys, &level);
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted_unstable()
        .rev()
        .take(2)
        .product()
}

fn play_round(monkeys: Vec<Monkey>, level: &MonkeyBusinessLevel) -> Vec<Monkey> {
    let mut monkeys = monkeys;
    // We will check if worry_level is divisible by *any* of the monkeys' divisible_by
    //  So we can use the common divisor to simplify the value every time it gets bigger
    let common_divisor: usize = monkeys.iter().map(|m| m.divisible_by).product();

    for monkey_idx in 0..monkeys.len() {
        // Iterate while popping
        while let Some(item) = monkeys[monkey_idx].items.pop() {
            // We get the worry level
            let worry_level = match level {
                MonkeyBusinessLevel::V1 => monkeys[monkey_idx].operation.apply(item) / 3,
                MonkeyBusinessLevel::V2 => monkeys[monkey_idx].operation.apply(item),
            };

            monkeys[monkey_idx].inspected += 1;

            // We decide which monkey to send the item to
            let target;
            if worry_level % monkeys[monkey_idx].divisible_by == 0 {
                target = monkeys[monkey_idx].if_true;
            } else {
                target = monkeys[monkey_idx].if_false;
            };

            // We send the item to the target monkey
            monkeys[target].items.push(worry_level % common_divisor);
        }
    }

    monkeys
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    // TODO Separated list once again...
    input
        .split("\n\n")
        .map(|m| parse_monkey(m).unwrap().1)
        .collect_vec()
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // TODO I think this should use map_res instead of ?
    let (_, (_, items, operation, divisible_by, if_true, if_false)) = tuple((
        nom_monkey,
        nom_items,
        nom_operation,
        nom_divisible_by,
        nom_throw_to_true,
        nom_throw_to_false,
    ))(input)?;

    Ok((
        "",
        Monkey {
            items,
            operation: parse_operation(operation),
            divisible_by,
            if_true,
            if_false,
            inspected: 0,
        },
    ))
}

fn parse_operation(input: &str) -> Operation {
    // TODO try not to use unwrap/use nom here too
    let (operator, value) = input.split_once(' ').unwrap();

    match operator {
        "*" => match value {
            "old" => Operation::Square,
            _ => Operation::Multiply(value.parse().unwrap()),
        },
        "+" => Operation::Add(value.parse().unwrap()),
        _ => panic!("Operation not handled yet: {input}"),
    }
}

fn nom_monkey(s: &str) -> IResult<&str, usize> {
    map_res(
        delimited(tag("Monkey "), digit1, tag(":\n")),
        |out: &str| usize::from_str_radix(out, 10),
    )(s)
}

fn nom_items(s: &str) -> IResult<&str, Vec<usize>> {
    let (remainder, items) = delimited(tag("  Starting items: "), is_not("\n"), tag("\n"))(s)?;
    // TODO Understand that one and make it work
    // let (_, items) = separated_list0(tag(", "), is_not("/"))(items)?;

    // In the meanwhile, this works
    Ok((
        remainder,
        items.split(", ").map(|s| s.parse().unwrap()).collect(),
    ))
}

fn nom_operation(s: &str) -> IResult<&str, &str> {
    delimited(tag("  Operation: new = old "), is_not("\n"), tag("\n"))(s)
}

fn nom_divisible_by(s: &str) -> IResult<&str, usize> {
    map_res(
        delimited(tag("  Test: divisible by "), digit1, tag("\n")),
        |out: &str| usize::from_str_radix(out, 10),
    )(s)
}

fn nom_throw_to_true(s: &str) -> IResult<&str, usize> {
    map_res(
        delimited(tag("    If true: throw to monkey "), digit1, tag("\n")),
        |out: &str| usize::from_str_radix(out, 10),
    )(s)
}

fn nom_throw_to_false(s: &str) -> IResult<&str, usize> {
    map_res(
        preceded(tag("    If false: throw to monkey "), digit1),
        |out: &str| usize::from_str_radix(out, 10),
    )(s)
}

#[cfg(test)]
mod tests {

    use super::*;

    const DEMO_MONKEY: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

    #[test]
    fn test_parse_command() {
        let mul = parse_operation("* 19");

        assert_eq!(mul, Operation::Multiply(19));
        assert_eq!(mul.apply(20), 19 * 20);

        let add = parse_operation("+ 19");
        assert_eq!(add, Operation::Add(19));
        assert_eq!(add.apply(20), 19 + 20);
    }

    #[test]
    fn test_nom() {
        // Just testing nom for my sanity
        let (_, res) = nom_monkey(DEMO_MONKEY).unwrap();
        assert_eq!(res, 0);

        let (_, (_, res)) = tuple((nom_monkey, nom_items))(DEMO_MONKEY).unwrap();
        assert_eq!(res, vec![79, 98]);

        let (_, (_, _, res)) = tuple((nom_monkey, nom_items, nom_operation))(DEMO_MONKEY).unwrap();
        assert_eq!(res, "* 19");

        let (_, (_, _, _, res)) =
            tuple((nom_monkey, nom_items, nom_operation, nom_divisible_by))(DEMO_MONKEY).unwrap();
        assert_eq!(res, 23);

        let (_, (_, _, _, _, true_target, false_target)) = tuple((
            nom_monkey,
            nom_items,
            nom_operation,
            nom_divisible_by,
            nom_throw_to_true,
            nom_throw_to_false,
        ))(DEMO_MONKEY)
        .unwrap();

        assert_eq!(true_target, 2);
        assert_eq!(false_target, 3);
    }

    #[test]
    fn test_parse_monkey() {
        let monkey = parse_monkey(DEMO_MONKEY).unwrap().1;

        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.divisible_by, 23);
        assert_eq!(monkey.if_true, 2);
        assert_eq!(monkey.if_false, 3);
    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(DEMO_INPUT);

        assert_eq!(monkeys.len(), 4);
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
        assert_eq!(
            get_monkey_business_level(DEMO_INPUT, MonkeyBusinessLevel::V1),
            10605
        )
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            get_monkey_business_level(DEMO_INPUT, MonkeyBusinessLevel::V2),
            2713310158
        )
    }
}

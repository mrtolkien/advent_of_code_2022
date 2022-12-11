use nom::{
    bytes::complete::{is_not, tag},
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, preceded, tuple},
    IResult,
};

struct Monkey {
    id: usize,
    starting_items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

pub fn get_monkey_business_level(input: &str) -> usize {
    todo!()
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (_, (monkey_id, items, operation, divisible_by, true_target, false_target)) = tuple((
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
            id: monkey_id,
            starting_items: items,
            operation: Box::new(|x| x),
            divisible_by,
            if_true: true_target,
            if_false: false_target,
        },
    ))
}

fn nom_monkey(s: &str) -> IResult<&str, usize> {
    map_res(
        delimited(tag("Monkey "), digit1, tag(":\n")),
        |out: &str| usize::from_str_radix(out, 10),
    )(s)
}

fn nom_items(s: &str) -> IResult<&str, Vec<usize>> {
    let (remainder, items) = delimited(tag("Starting items: "), is_not("\n"), tag("\n"))(s)?;
    // TODO Understand that one
    // let (_, items) = separated_list0(tag(", "), is_not("/"))(items)?;

    // In the meanwhile, this works
    Ok((
        remainder,
        items.split(", ").map(|s| s.parse().unwrap()).collect(),
    ))
}

fn nom_operation(s: &str) -> IResult<&str, &str> {
    delimited(tag("Operation: new = "), is_not("\n"), tag("\n"))(s)
}

fn nom_divisible_by(s: &str) -> IResult<&str, usize> {
    map_res(
        delimited(tag("Test: divisible by "), digit1, tag("\n")),
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
    fn test_nom() {
        // Just testing nom for my sanity
        let (_, res) = nom_monkey(DEMO_MONKEY).unwrap();
        assert_eq!(res, 0);

        let (_, (_, res)) = tuple((nom_monkey, nom_items))(DEMO_MONKEY).unwrap();
        assert_eq!(res, vec![79, 98]);

        let (_, (_, _, res)) = tuple((nom_monkey, nom_items, nom_operation))(DEMO_MONKEY).unwrap();
        assert_eq!(res, "old * 19");

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

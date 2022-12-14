use std::iter::zip;

use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::delimited, IResult,
};

pub fn get_right_order_pairs_index_sum(input: &str) -> usize {
    let input = parse_input(input);
    let mut result = 0;

    for (idx, (left, right)) in input.iter().enumerate() {
        // TODO Check behaviour when one side is longer than the other
        for (left_value, right_value) in zip(left, right) {
            match (left_value, right_value) {
                (Value::Number(_), Value::Number(_)) => todo!(),
                (Value::Array(_), Value::Array(_)) => todo!(),
                (Value::Number(_), Value::Array(_)) => todo!(),
                (Value::Array(_), Value::Number(_)) => todo!(),
            }
        }
        // TODO Get the right test here!
        if left == right {
            result += idx + 1;
        }
    }

    result
}

#[derive(Debug, PartialEq)]
enum Value {
    Number(usize),
    Array(Vec<Value>),
}

fn parse_input(input: &str) -> Vec<(Vec<Value>, Vec<Value>)> {
    match separated_list0(tag("\n\n"), parse_packet)(input) {
        Ok((_, values)) => values,
        Err(_) => panic!("Failed to parse input: {input}"),
    }
}

fn parse_packet(s: &str) -> IResult<&str, (Vec<Value>, Vec<Value>)> {
    separated_pair(parse_row, tag("\n"), parse_row)(s)
}

fn parse_value(s: &str) -> IResult<&str, Value> {
    alt((
        // If we have a single digit, we wrap it in a Value::Number and return
        map(digit1, |s: &str| Value::Number(s.parse().unwrap())),
        // If we have an array, we parse it as a list of values, wrap it in a Value::Array and return
        map(
            delimited(tag("["), separated_list0(tag(","), parse_value), tag("]")),
            |v| Value::Array(v),
        ),
    ))(s)
}

fn parse_row(s: &str) -> IResult<&str, Vec<Value>> {
    delimited(tag("["), separated_list0(tag(","), parse_value), tag("]"))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("1").unwrap().1, Value::Number(1));
        assert_eq!(
            parse_value("[1]").unwrap().1,
            Value::Array(vec![Value::Number(1)])
        );
        assert_eq!(
            parse_value("[1,2]").unwrap().1,
            Value::Array(vec![Value::Number(1), Value::Number(2)])
        );
        assert_eq!(
            parse_value("[[1],[2,3,4]]").unwrap().1,
            Value::Array(vec![
                Value::Array(vec![Value::Number(1)]),
                Value::Array(vec![Value::Number(2), Value::Number(3), Value::Number(4)])
            ])
        );
        assert_eq!(parse_value("[]").unwrap().1, Value::Array(vec![]));
    }

    #[test]
    fn test_parse_row() {
        assert_eq!(
            parse_row("[1,1,3,1,1]").unwrap().1,
            vec![
                Value::Number(1),
                Value::Number(1),
                Value::Number(3),
                Value::Number(1),
                Value::Number(1)
            ]
        );

        assert_eq!(
            parse_row("[[1],[2,3,4]]").unwrap().1,
            vec![
                Value::Array(vec![Value::Number(1)]),
                Value::Array(vec![Value::Number(2), Value::Number(3), Value::Number(4)])
            ]
        );
    }

    #[test]
    fn test_parse_packet() {
        assert_eq!(
            parse_packet(
                "[1]
[2]"
            )
            .unwrap()
            .1,
            (vec![Value::Number(1)], vec![Value::Number(2)])
        )
    }

    const DEMO_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_first_part() {
        assert_eq!(get_right_order_pairs_index_sum(DEMO_INPUT), 13)
    }
}

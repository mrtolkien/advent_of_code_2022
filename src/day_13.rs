use std::iter::zip;

use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::delimited, IResult,
};
use strum_macros::Display;

pub fn get_right_order_pairs_index_sum(input: &str) -> usize {
    let input = parse_input(input);
    let mut result = 0;

    for (idx, (left, right)) in input.iter().enumerate() {
        if left < right {
            result += idx + 1;
        }
    }

    result
}

pub fn get_divider_packets_location(input: &str) -> usize {
    // We flatten all values into a single vec
    let input = parse_input(input);
    let mut values: Vec<&Value> = input
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect();

    // We add the divider packets
    let first_divider = Value::Array(vec![Value::Array(vec![Value::Number(2)])]);
    let second_divider = Value::Array(vec![Value::Array(vec![Value::Number(6)])]);
    values.push(&first_divider);
    values.push(&second_divider);

    // We sort the values
    values.sort();

    // We get indexes
    let first_divider_idx = values.iter().position(|v| v == &&first_divider).unwrap() + 1;
    let second_divider_idx = values.iter().position(|v| v == &&second_divider).unwrap() + 1;

    first_divider_idx * second_divider_idx
}

#[derive(Debug, PartialEq, Clone, Eq, Display)]
enum Value {
    Number(usize),
    Array(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Simple comparison
            (Value::Number(s), Value::Number(o)) => s.cmp(o),
            (Value::Array(s), Value::Array(o)) => {
                for (left, right) in zip(s, o) {
                    if left < right {
                        return std::cmp::Ordering::Less;
                    } else if left > right {
                        return std::cmp::Ordering::Greater;
                    }
                }

                // If we get there, the comparison was not conclusive
                //    -> We compare lengths
                s.len().cmp(&o.len())
            }
            (Value::Number(_), Value::Array(_)) => Value::Array(vec![self.clone()]).cmp(other),
            (Value::Array(_), Value::Number(_)) => self.cmp(&Value::Array(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // We simply re-use our order
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    match separated_list0(tag("\n\n"), parse_packet)(input) {
        Ok((_, values)) => values,
        Err(_) => panic!("Failed to parse input: {input}"),
    }
}

fn parse_packet(s: &str) -> IResult<&str, (Value, Value)> {
    separated_pair(parse_value, tag("\n"), parse_value)(s)
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
            parse_value("[1,1,3,1,1]").unwrap().1,
            Value::Array(vec![
                Value::Number(1),
                Value::Number(1),
                Value::Number(3),
                Value::Number(1),
                Value::Number(1)
            ])
        );

        assert_eq!(
            parse_value("[[1],[2,3,4]]").unwrap().1,
            Value::Array(vec![
                Value::Array(vec![Value::Number(1)]),
                Value::Array(vec![Value::Number(2), Value::Number(3), Value::Number(4)])
            ])
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
            (
                Value::Array(vec![Value::Number(1)]),
                Value::Array(vec![Value::Number(2)])
            )
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

    #[test]
    fn test_second_part() {
        assert_eq!(get_divider_packets_location(DEMO_INPUT), 140)
    }
}

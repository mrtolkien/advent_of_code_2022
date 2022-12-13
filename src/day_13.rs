use nom::multi::separated_list0;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map, multi::many0,
    sequence::delimited, IResult,
};
use nom::{error::ErrorKind, Err, Parser};

pub fn get_right_order_pairs_index_sum(input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq)]
enum Value {
    Number(usize),
    Array(Vec<Value>),
}

fn parse_packet(s: &str) -> (Vec<Value>, Vec<Value>) {
    todo!()
}

fn parse_single_digit(s: &str) -> IResult<&str, Value> {
    map(digit1, |s: &str| Value::Number(s.parse().unwrap()))(s)
}

fn parse_digit_list(s: &str) -> IResult<&str, Value> {
    map(separated_list0(tag(","), parse_single_digit), |x| {
        Value::Array(x)
    })(s)
}

fn parse_value(s: &str) -> IResult<&str, Value> {
    // TODO This should call itself with a separated_list somewhere
    alt((
        parse_single_digit,
        delimited(tag("["), parse_value, tag("]")),
        parse_digit_list,
    ))(s)
}

fn parse_row(s: &str) -> IResult<&str, Vec<Value>> {
    delimited(tag("["), many0(parse_value), tag("]"))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_digit_list() {
        assert_eq!(
            parse_digit_list("1,2").unwrap().1,
            Value::Array(vec![Value::Number(1), Value::Number(2)])
        );
        assert_eq!(
            parse_digit_list("1,2,3").unwrap().1,
            Value::Array(vec![Value::Number(1), Value::Number(2), Value::Number(3)])
        );
    }

    #[test]
    fn test_parse_single_digit() {
        assert_eq!(parse_single_digit("1").unwrap().1, Value::Number(1));
        assert_eq!(parse_single_digit("123").unwrap().1, Value::Number(123));
    }

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

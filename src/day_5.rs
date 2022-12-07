use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{map, value},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub enum CrateMoverVersion {
    V1,
    V2,
}

/// Finds the top crates after performing the given actions on them.
///
/// # Arguments
///
/// * `input`: a string representing the starting positions and actions to be performed on the crates.
/// * `version`: the version of the crate mover to use.
///
/// # Returns
///
/// A string containing a comma-separated list of the top crates after all the actions have been performed.
///
/// # Examples
/// ```
/// use advent_of_code_2022::day_5::find_top_crates;
/// use advent_of_code_2022::day_5::CrateMoverVersion;
///
/// let input = "[N] [C]
///[Z] [M]
///1   2  
///
///move 1 from 2 to 1";
///
/// assert_eq!(find_top_crates(input, CrateMoverVersion::V1), "CM");
/// ```
pub fn find_top_crates(input: &str, version: CrateMoverVersion) -> String {
    let mut data = input.split("\n\n");

    // This gets the starting block
    let mut positions = read_starting_position(data.next().expect("Empty input string"));

    // We then iterate on the rows
    for row in data.next().expect("No actions founds").lines() {
        match version {
            CrateMoverVersion::V1 => positions = move_crates(positions, row),
            CrateMoverVersion::V2 => positions = move_crates_v2(positions, row),
        }
    }

    get_top_crates(&positions)
}

fn read_starting_position(input: &str) -> Vec<Vec<char>> {
    let (_, rows) = crate_rows(input).expect("Cannot parse rows: {input}");

    // TODO Better init -> Use a hashmap and entry instead of a Vec
    let mut result: Vec<Vec<char>> = vec![vec![]; rows[0].len()];

    for row in rows.iter().rev() {
        for (idx, char) in row.iter().enumerate() {
            if let Some(c) = char {
                result[idx].push(*c);
            }
        }
    }

    result
}

/// Nom parser to parse "[A]" -> 'A'
fn crate_label(s: &str) -> IResult<&str, char> {
    let crate_char = satisfy(|c| c.is_ascii_uppercase());
    delimited(tag("["), crate_char, tag("]"))(s)
}

/// Nom parser to parse "[A]     [B] [C]" -> [Some('A'), None, Some('B'), Some('C')]
fn crate_row(s: &str) -> IResult<&str, Vec<Option<char>>> {
    let maybe_crate = map(crate_label, Some);
    let empty_space = value(None, tag("   "));
    let crate_or_empty = alt((maybe_crate, empty_space));
    separated_list1(tag(" "), crate_or_empty)(s)
}

/// Nom parser to parse multiple newline-separated rows of crates into a list
/// of rows, specified by `crate_row`.
fn crate_rows(s: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    separated_list1(tag("\n"), crate_row)(s)
}

struct MovementInfo {
    crates_count: usize,
    from: usize,
    to: usize,
}

fn get_movement_info(row: &str) -> MovementInfo {
    let mut lines = row.split(' ');

    lines.next();

    // TODO Fix all the unwraps with a proper Result return type!
    let crates_count: usize = lines.next().unwrap().parse().unwrap();

    lines.next();

    let from: usize = lines.next().unwrap().parse().unwrap();

    lines.next();

    let to: usize = lines.next().unwrap().parse().unwrap();

    MovementInfo {
        crates_count,
        from,
        to,
    }
}

fn move_crates(positions: Vec<Vec<char>>, row: &str) -> Vec<Vec<char>> {
    // TODO This does a copy? -> Understand the line
    let mut positions = positions;

    let movement_info = get_movement_info(row);

    let mut moving_crates = vec![];

    for _ in 0..movement_info.crates_count {
        moving_crates.push(positions[movement_info.from - 1].pop().unwrap());
    }

    for crate_ in moving_crates {
        positions[movement_info.to - 1].push(crate_);
    }

    positions
}

fn move_crates_v2(positions: Vec<Vec<char>>, row: &str) -> Vec<Vec<char>> {
    let mut positions = positions;

    let movement_info = get_movement_info(row);

    let mut moving_crates = vec![];

    for _ in 0..movement_info.crates_count {
        moving_crates.push(positions[movement_info.from - 1].pop().unwrap());
    }

    for crate_ in moving_crates.iter().rev() {
        positions[movement_info.to - 1].push(*crate_);
    }

    positions
}

fn get_top_crates(positions: &Vec<Vec<char>>) -> String {
    positions.iter().fold("".to_string(), |result, column| {
        result
            + column
                .last()
                .expect("No crate on column {column}")
                .to_string()
                .as_str()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_first_part() {
        assert_eq!(find_top_crates(DEMO_INPUT, CrateMoverVersion::V1), "CMZ");
    }

    #[test]
    fn test_second_part() {
        assert_eq!(find_top_crates(DEMO_INPUT, CrateMoverVersion::V2), "MCD");
    }
    #[test]
    fn test_starting_position() {
        assert_eq!(
            read_starting_position(
                "[D]
 1"
            ),
            vec![vec!['D']]
        );

        assert_eq!(
            read_starting_position(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "
            ),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_move() {
        assert_eq!(
            move_crates(
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                "move 1 from 2 to 1",
            ),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );

        assert_eq!(
            move_crates(
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                "move 2 from 2 to 3",
            ),
            vec![vec!['Z', 'N'], vec!['M'], vec!['P', 'D', 'C']]
        );

        assert_eq!(
            move_crates(
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                "move 2 from 1 to 3",
            ),
            vec![vec![], vec!['M', 'C', 'D'], vec!['P', 'N', 'Z']]
        );
    }

    #[test]
    fn test_get_top_crates() {
        assert_eq!(
            get_top_crates(&mut vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]),
            "DCP"
        );

        assert_eq!(
            get_top_crates(&mut vec![vec!['Z'], vec!['M', 'C'], vec!['P']]),
            "ZCP"
        );
    }

    #[test]
    fn test_crate_rows() {
        assert_eq!(
            crate_rows("[A] [B] [C]"),
            Ok(("", vec![vec![Some('A'), Some('B'), Some('C')]]))
        );
        assert_eq!(
            crate_rows(
                "[D]     [F]
[A] [B] [C]"
            ),
            Ok((
                "",
                vec![
                    vec![Some('D'), None, Some('F')],
                    vec![Some('A'), Some('B'), Some('C')]
                ]
            ))
        );
    }

    #[test]
    fn test_doctest() {
        let input = "[N] [C]
[Z] [M]
1   2  

move 1 from 2 to 1";

        assert_eq!(find_top_crates(input, CrateMoverVersion::V1), "CM");
    }
}

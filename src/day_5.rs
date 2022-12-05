pub enum CrateMoverVersion {
    V1,
    V2,
}

pub fn find_top_crates(input: &str, version: CrateMoverVersion) -> String {
    let mut data = input.split("\n\n");

    // This gets the starting block
    let mut positions = read_starting_position(data.next().unwrap());

    // We then iterate on the rows
    for row in data.next().unwrap().lines() {
        match version {
            CrateMoverVersion::V1 => positions = move_crates(positions, row),
            CrateMoverVersion::V2 => positions = move_crates_v2(positions, row),
        }
    }

    get_top_crates(&mut positions)
}

struct MovementInfo {
    crates_count: usize,
    from: usize,
    to: usize,
}

fn read_starting_position(input: &str) -> Vec<Vec<char>> {
    // TODO This is disgusting but it works, clean it up later
    // -> Try nom: https://docs.rs/nom/latest/nom/
    let mut reversed_iterator = input.lines().rev();

    // We simply check the width here and init with empty vectors
    let columns = reversed_iterator.next().unwrap().len() / 4 + 1 as usize;

    let mut result = vec![];

    for _ in 0..columns {
        result.push(vec![]);
    }

    for line in reversed_iterator {
        let line = line.to_owned() + " ";

        let chunked_iterator = line.chars().array_chunks::<4>();

        for (idx, chars) in chunked_iterator.enumerate() {
            if chars[1] != ' ' {
                result[idx].push(chars[1]);
            }
        }
    }

    // Get the first line from the bottom for initialization
    result
}

fn get_movement_info(row: &str) -> MovementInfo {
    let mut lines = row.split(' ');

    lines.next();

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
    let mut new_positions = positions.clone();

    let movement_info = get_movement_info(row);

    let mut moving_crates = vec![];

    for _ in 0..movement_info.crates_count {
        moving_crates.push(new_positions[movement_info.from - 1].pop().unwrap());
    }

    for crate_ in moving_crates {
        new_positions[movement_info.to - 1].push(crate_);
    }

    new_positions
}

fn move_crates_v2(positions: Vec<Vec<char>>, row: &str) -> Vec<Vec<char>> {
    let mut new_positions = positions.clone();

    let movement_info = get_movement_info(row);

    let mut moving_crates = vec![];

    for _ in 0..movement_info.crates_count {
        moving_crates.push(new_positions[movement_info.from - 1].pop().unwrap());
    }

    for crate_ in moving_crates.iter().rev() {
        new_positions[movement_info.to - 1].push(*crate_);
    }

    new_positions
}

fn get_top_crates(positions: &mut Vec<Vec<char>>) -> String {
    let mut result = "".to_string();

    for column in positions {
        result = result + column.pop().unwrap().to_string().as_str();
    }

    result
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
}

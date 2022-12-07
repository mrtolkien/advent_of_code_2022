use std::collections::HashMap;

use nom::character::complete::{alpha1, space0};
use nom::sequence::tuple;
use nom::IResult;

struct Dir {
    name: String,
    file_sizes: usize,
}

/// Gets the sum of the sizes of all directories that are less than max_size
pub fn get_sum_of_dir_sizes(input: &str, max_size: usize) -> usize {
    // Current dir will point to the current directory
    let mut cwd = Vec::new();
    // Directories will hold directory information through a unique path
    let mut directories = HashMap::new();

    // First, we build our filesystem tree
    for command_input in input.split("$ ") {
        // TODO This is once again disgusting but I want to get it done... I think nom can do that better
        // -> The first split matches to "" since it starts with the pattern
        if command_input == "" {
            continue;
        }

        let mut lines = command_input.lines();
        let command = parse_command(lines.next().unwrap());

        // We apply the command
        match command {
            // Backards -> We remove one element from the current_dir
            Command::CD(CommandCD::Backwards) => {
                cwd.pop();
            }
            // Forward -> We add one element to the current_dir
            Command::CD(CommandCD::Path(x)) => cwd.push(x),
            // LS -> We parse the information
            Command::LS => {
                let current_dir_name = get_current_dir_name(&cwd);

                let dir_struct = directories.entry(current_dir_name.clone()).or_insert(Dir {
                    name: current_dir_name.clone(),
                    file_sizes: 0,
                });

                for line in lines {
                    match DirValue::build(line) {
                        DirValue::File(size) => {
                            dir_struct.file_sizes += size;
                        }
                        // We simply pass if we found a dir, we'll match on path names!
                        DirValue::Dir(_) => (),
                    }
                }
            }
        }
    }

    // Then, we check each folder's size by recursively checking its children
    let mut result = 0;

    for directory in directories.values() {
        let size = directories
            .iter()
            .filter(|x| x.0.starts_with(&directory.name))
            .fold(0, |size, x| size + x.1.file_sizes);

        if size <= max_size {
            result += size
        }
    }

    result
}

fn get_current_dir_name(current_dir: &Vec<String>) -> String {
    current_dir
        .iter()
        .skip(1) // Skipping the / at the beginning of all
        .fold(String::new(), |acc, x| acc + "/" + x)
}

#[derive(PartialEq, Debug)]
enum Command {
    CD(CommandCD),
    LS,
}

#[derive(PartialEq, Debug)]
enum CommandCD {
    Path(String),
    Backwards,
}

fn nom_command(cmd: &str) -> IResult<&str, Command> {
    let (folder, (cmd, _)) = tuple((alpha1, space0))(cmd)?;

    match cmd {
        "cd" => match folder {
            ".." => Ok(("", Command::CD(CommandCD::Backwards))),
            _ => Ok(("", Command::CD(CommandCD::Path(folder.to_string())))),
        },
        "ls" => Ok(("", Command::LS)),
        _ => panic!("Unknown command"),
    }
}

fn parse_command(cmd: &str) -> Command {
    let (_, command) = nom_command(cmd).expect("Cannot parse command");
    command
}

#[derive(PartialEq, Debug)]
enum DirValue {
    // We just want to know the size
    File(usize),
    // We just want to know the name
    Dir(String),
}

impl DirValue {
    fn build(input: &str) -> DirValue {
        let (left_part, right_part) = input.split_once(' ').unwrap();

        match left_part.parse::<usize>() {
            Ok(v) => DirValue::File(v),
            Err(_) => DirValue::Dir(right_part.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("ls"), Command::LS);
        assert_eq!(
            parse_command("cd /"),
            Command::CD(CommandCD::Path("/".to_string()))
        );
        assert_eq!(
            parse_command("cd 123456789"),
            Command::CD(CommandCD::Path("123456789".to_string()))
        );
        assert_eq!(parse_command("cd .."), Command::CD(CommandCD::Backwards));
    }

    #[test]
    fn test_build_dir_value() {
        assert_eq!(DirValue::build("14848514 b.txt"), DirValue::File(14848514));
        assert_eq!(DirValue::build("8504156 c.dat"), DirValue::File(8504156));
        assert_eq!(DirValue::build("dir d"), DirValue::Dir("d".to_string()));
    }

    #[test]
    fn test_print_current_dir() {
        assert_eq!(
            get_current_dir_name(&vec!["/".to_string(), "a".to_string()]),
            "/a"
        );
        assert_eq!(
            get_current_dir_name(&vec!["/".to_string(), "a".to_string(), "b".to_string()]),
            "/a/b"
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_command_panic() {
        parse_command("$ test");
    }

    #[test]
    fn test_first_part() {
        assert_eq!(get_sum_of_dir_sizes(DEMO_INPUT, 100_000), 95_437);
    }

    const DEMO_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
}

#[derive(Debug, PartialEq)]
enum Command {
    NoOp,
    AddX(isize),
}

struct State {
    register_value: isize,
    add_x_started: bool,
    commands: Vec<Command>,
    command_index: usize,
}

impl State {
    fn new(raw_commands: &str) -> Self {
        Self {
            register_value: 1,
            add_x_started: false,
            commands: raw_commands.lines().map(parse_command).collect(),
            command_index: 0,
        }
    }
}

pub fn get_signal_strength_sum(raw_commands: &str) -> isize {
    let mut state = State::new(raw_commands);

    let mut result = 0;

    for cycle in 1..=220 {
        // We get the 20th cycle's value and every 40th cycle after that
        if (cycle - 20) % 40 == 0 {
            result += state.register_value * cycle;
        }

        state = apply_command(state);
    }

    result
}

pub fn get_drawing(raw_commands: &str) -> String {
    let mut state = State::new(raw_commands);

    let mut result = String::new();

    for cycle in 1..=240 {
        let pixel_position = (cycle - 1) % 40;

        // We check if we should draw a pixel at position cycle - 1
        if pixel_position == state.register_value - 1
            || pixel_position == state.register_value
            || pixel_position == state.register_value + 1
        {
            result.push_str("#");
        // Else we draw a dot
        } else {
            result.push_str(".");
        }

        state = apply_command(state);

        // We add a line jump every 40 commands
        if pixel_position == 39 {
            result.push_str("\n")
        }
    }

    result
}

fn apply_command(state: State) -> State {
    let command = &state.commands[state.command_index];

    match *command {
        // No operation -> We get the next command
        Command::NoOp => State {
            command_index: state.command_index + 1,
            ..state
        },
        // AddX
        Command::AddX(v) => match state.add_x_started {
            // First cycle -> we change the bool to say we started
            false => State {
                add_x_started: true,
                ..state
            },
            // Second cycle -> we change register value and reset the bool
            true => State {
                register_value: state.register_value + v,
                add_x_started: false,
                command_index: state.command_index + 1,
                ..state
            },
        },
    }
}

fn parse_command(input: &str) -> Command {
    let mut split = input.split(' ');

    match split.next().unwrap() {
        "noop" => Command::NoOp,
        "addx" => Command::AddX(split.next().unwrap().parse::<isize>().unwrap()),
        _ => panic!("Unknown command: {}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("noop"), Command::NoOp);
        assert_eq!(parse_command("addx 1"), Command::AddX(1));
        assert_eq!(parse_command("addx -1"), Command::AddX(-1));
    }

    #[test]
    #[should_panic]
    fn test_parse_command_panic() {
        parse_command("hello");
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
noop";

    #[test]
    fn test_first_part() {
        assert_eq!(get_signal_strength_sum(INPUT), 13140);
    }

    #[test]
    fn test_second_part() {
        let drawing = get_drawing(INPUT);
        let result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(drawing, result)
    }
}

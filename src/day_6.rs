use itertools::Itertools;
use std::collections::VecDeque;

/// Returns the starting index (starting at 1) of the first packet in the given input string.
///
/// A packet is defined as a sequence of `distinct_letters` distinct letters in the input string.
///
/// # Arguments
///
/// * `input` - A string slice containing the input text to search for packets in.
/// * `distinct_letters` - The number of distinct letters that a packet should contain.
/// # Examples
///
/// ```
/// use advent_of_code_2022::day_6::get_packet_start;
///
/// assert_eq!(get_packet_start("ABC", 2), 2);
/// assert_eq!(get_packet_start("AAABC", 3), 5);
/// assert_eq!(get_packet_start("ABCCDEF", 4), 7);
/// ```
pub fn get_packet_start(input: &str, distinct_letters: usize) -> usize {
    let mut last_chars = VecDeque::new();

    for (idx, current_char) in input.chars().enumerate() {
        // We add the new character at the end
        last_chars.push_back(current_char);

        // If we don't have distinct_letters characters already, we continue
        // This uses Rust itertools
        if last_chars.len() < distinct_letters {
            continue;
        }

        // We check if we have distinct_letters different characters
        if last_chars.iter().unique().count() == distinct_letters {
            // Index starts at one 😱
            return idx + 1;
        }

        // Otherwise we pop the first value and continue
        last_chars.pop_front();
    }

    panic!("No packet marker found in input: {input:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_packet_marker() {
        assert_eq!(get_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_packet_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_start_message_marker() {
        assert_eq!(get_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(get_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(get_packet_start("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            get_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(get_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    #[should_panic]
    fn test_start_message_marker_panic() {
        get_packet_start("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm", 15);
    }
}

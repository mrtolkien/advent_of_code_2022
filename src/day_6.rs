use itertools::Itertools;
use std::collections::VecDeque;

pub fn get_packet_start(input: &str, distinct_letters: usize) -> usize {
    let mut last_chars = VecDeque::new();

    for (idx, current_char) in input.chars().enumerate() {
        // We add the new character at the end
        last_chars.push_back(current_char);

        // If we don't have distinct_letters characters already, we continue
        if last_chars.len() < distinct_letters {
            continue;
        } else {
            // We check if we have distinct_letters different characters
            if last_chars.iter().unique().count() == distinct_letters {
                // Index starts at one 😱
                return idx + 1;
            }

            // Otherwise we pop the first value and continue
            last_chars.pop_front();
        }
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
}

use std::collections::HashSet;

static DAY: u8 = 6;

fn main() {
    let input = advent::read_file(DAY);
    println!("{DAY}a: {}", start_of_packet(&input));
    println!("{DAY}b: {}", start_of_message(&input));
}

fn start_of_prefix(input: &str, prefix_len: usize) -> usize {
    input.trim()
         .chars()
         .collect::<Vec<_>>()
         .windows(prefix_len)
         .enumerate()
         .map(|(pos, window)| (pos + prefix_len, window.iter().collect::<HashSet<_>>()))
         .find(|(_, set)| set.len() == prefix_len)
         .unwrap_or_else(|| panic!("set should contain {} different characters", prefix_len))
         .0
}

fn start_of_packet(input: &str) -> usize {
    start_of_prefix(input, 4)
}

fn start_of_message(input: &str) -> usize {
    start_of_prefix(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

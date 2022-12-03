use std::collections::HashSet;

static DAY: u8 = 3;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_priorities(&input));
    println!("{DAY}b: {}", sum_badge_priorities(&input));
}

fn item_value(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item.to_digit(36).unwrap() - 9
    } else if item.is_ascii_uppercase() {
        item.to_digit(36).unwrap() + 17
    } else {
        panic!("unexpected common character: {}", item);
    }
}

fn sum_priorities(input: &[String]) -> u32 {
    let mut priorities = 0;

    for line in input {
        let (first, second) = line.split_at(line.len() / 2);
        let first = first.chars().collect::<HashSet<_>>();
        let second = second.chars().collect::<HashSet<_>>();
        let common = first.intersection(&second).copied().collect::<Vec<_>>();
        assert_eq!(common.len(), 1);
        priorities += item_value(*common.first().unwrap());
    }

    priorities
}

fn sum_badge_priorities(input: &[String]) -> u32 {
    let mut priorities = 0;

    for group in input.chunks(3) {
        let first = group[0].chars().collect::<HashSet<_>>();
        let second = group[1].chars().collect::<HashSet<_>>();
        let third = group[2].chars().collect::<HashSet<_>>();
        let common = first.intersection(&second).copied().collect::<HashSet<_>>();
        let common = common.intersection(&third).copied().collect::<Vec<_>>();
        assert_eq!(common.len(), 1);
        priorities += item_value(*common.first().unwrap());
    }

    priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(sum_priorities(&input), 157);
        assert_eq!(sum_badge_priorities(&input), 70);
    }
}

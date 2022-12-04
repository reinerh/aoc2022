static DAY: u8 = 4;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", overlaps(&input, true));
    println!("{DAY}b: {}", overlaps(&input, false));
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(input: &str) -> Range {
        let (start, end) = input.split_once('-').unwrap();
        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();
        Range { start, end }
    }

    fn overlaps(&self, other: &Range, fully_contained: bool) -> bool {
        if fully_contained {
            self.start <= other.start && self.end >= other.end
        } else {
            self.start <= other.start && self.end >= other.start
        }
    }
}

fn overlaps(input: &[String], fully_contained: bool) -> u32 {
    let mut count = 0;
    for line in input {
        let (range1, range2) = line.split_once(',').unwrap();
        let range1 = Range::new(range1);
        let range2 = Range::new(range2);
        if range1.overlaps(&range2, fully_contained) || range2.overlaps(&range1, fully_contained) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];

        assert_eq!(overlaps(&input, true), 2);
        assert_eq!(overlaps(&input, false), 4);
    }
}

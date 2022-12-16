use std::cmp::Ordering;

static DAY: u8 = 13;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", pair_indices(&input));
    println!("{DAY}b: {}", decoder_key(&input));
}

#[derive(PartialEq,Eq,Debug)]
enum PacketData {
    Int { value: u32 },
    List { value: Vec<PacketData> },
}

#[derive(PartialEq,Eq,Debug,PartialOrd,Ord)]
enum ComparisonResult {
    True,
    False,
    Next,
}

impl PacketData {
    fn new(input: &str) -> PacketData {
        let mut datalist = Vec::new();
        let mut head = PacketData::List { value: Vec::new() };
        let mut number = None;
        assert_eq!(input.chars().next().unwrap(), '[');
        for c in input.chars() {
            match c {
                '[' => {
                    datalist.push(head);
                    head = PacketData::List { value: Vec::new() };
                },
                ']' => {
                    let mut parent = datalist.pop().expect("list should not be empty");
                    match parent {
                        PacketData::Int { value: _ } => panic!("parent should be a list"),
                        PacketData::List { ref mut value } => {
                            if let Some(n) = number {
                                match head {
                                    PacketData::List { ref mut value } => value.push(PacketData::Int { value: n }),
                                    PacketData::Int { value: _ } => panic!("head should be a list"),
                                }
                                number = None;
                            }
                            value.push(head);
                            head = parent;
                        }
                    }
                },
                digit if c.is_ascii_digit() => {
                    let digit = digit.to_digit(10).unwrap();
                    match number {
                        None => number = Some(digit),
                        Some(n) => {
                            number = Some(n * 10 + digit);
                        }
                    }
                },
                ',' => {
                    if let Some(n) = number {
                            match head {
                                PacketData::List { ref mut value } => value.push(PacketData::Int { value: n }),
                                PacketData::Int { value: _ } => panic!("head should be a list"),
                            }
                            number = None;
                    }
                    /* if previous char waas ], it has already been handled */
                }
                _ => unimplemented!(),
            }
        }

        match head {
            PacketData::List { mut value } => value.pop().unwrap(),
            PacketData::Int { value: _ } => panic!("head should have a list"),
        }
    }

    fn right_order(left: &PacketData, right: &PacketData) -> ComparisonResult {
        match left {
            PacketData::Int { value: val_left } => {
                match right {
                    PacketData::Int { value: val_right } => match val_left.cmp(val_right) {
                        Ordering::Less => ComparisonResult::True,
                        Ordering::Greater => ComparisonResult::False,
                        Ordering::Equal => ComparisonResult::Next,
                    }
                    PacketData::List { value: _val_right } => {
                        let new_left = PacketData::new(&format!("[{}]", val_left));
                        PacketData::right_order(&new_left, right)
                    }
                }
            },
            PacketData::List { value: val_left } => {
                match right {
                    PacketData::Int { value: val_right } => {
                        let new_right = PacketData::new(&format!("[{}]", val_right));
                        PacketData::right_order(left, &new_right)
                    },
                    PacketData::List { value: val_right } => {
                        for (item_left, item_right) in val_left.iter().zip(val_right.iter()) {
                            let result = PacketData::right_order(item_left, item_right);
                            match result {
                                ComparisonResult::Next => continue,
                                res => return res,
                            }
                        }
                        match val_left.len().cmp(&val_right.len()) {
                            Ordering::Less => ComparisonResult::True,
                            Ordering::Greater => ComparisonResult::False,
                            Ordering::Equal => ComparisonResult::Next,
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match PacketData::right_order(self, other) {
            ComparisonResult::True => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}

fn pair_indices(input: &[String]) -> usize {
    input.chunks(3)
         .map(|c| (PacketData::new(&c[0]), PacketData::new(&c[1])))
         .enumerate()
         .filter(|(_,pair)| PacketData::right_order(&pair.0, &pair.1) == ComparisonResult::True)
         .map(|(idx,_)| idx + 1)
         .sum()
}

fn decoder_key(input: &[String]) -> usize {
    let mut input = Vec::from(input);
    input.push("[[2]]".to_string());
    input.push("[[6]]".to_string());
    let mut input = input.iter()
                         .filter(|line| !line.is_empty())
                         .map(|line| PacketData::new(line))
                         .collect::<Vec<_>>();
    input.sort();
    let packet2 = PacketData::new("[[2]]");
    let packet6 = PacketData::new("[[6]]");
    input.iter()
         .enumerate()
         .filter(|&(_,packet)| [&packet2, &packet6].contains(&packet))
         .map(|(pos,_)| pos + 1)
         .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(pair_indices(&input), 13);
        assert_eq!(decoder_key(&input), 140);
    }
}

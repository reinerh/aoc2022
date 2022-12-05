use std::collections::VecDeque;
use regex::Regex;

static DAY: u8 = 5;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", find_top_crates(&input));
    println!("{DAY}b: {}", find_top_crates_9001(&input));
}

struct CrateMover {
    instructions: Vec<Instruction>,
    stacks: Vec<VecDeque<char>>,
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl CrateMover {
    fn new(input: &[String]) -> CrateMover {
        let re_instruction = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

        let mut instructions = Vec::new();
        let mut stacks = Vec::new();
        for line in input {
            if let Some(cap) = re_instruction.captures(line) {
                instructions.push(Instruction {
                    amount: cap[1].parse::<usize>().unwrap(),
                    from: cap[2].parse::<usize>().unwrap() - 1,
                    to: cap[3].parse::<usize>().unwrap() - 1,
                });
            }

            if line.find('[').is_some() {
                let chars = line.chars().collect::<Vec<_>>();
                for (index, chunk) in chars.chunks(4).enumerate() {
                    // chunk: "[x] "
                    let name = chunk[1];
                    if name == ' ' {
                        // empty position
                        continue;
                    }
                    if index >= stacks.len() {
                        stacks.resize(index + 1, VecDeque::new());
                    }
                    stacks[index].push_front(name);
                }
            }
        }

        CrateMover { stacks, instructions }
    }

    fn move_crates(&mut self) {
        for instruction in &self.instructions {
            for _ in 0 .. instruction.amount {
                let name = self.stacks[instruction.from].pop_back().expect("Stack has no crate");
                self.stacks[instruction.to].push_back(name);
            }
        }
    }

    fn move_crates_9001(&mut self) {
        for instruction in &self.instructions {
            let drain_from = self.stacks[instruction.from].len() - instruction.amount;
            let moved_stack = self.stacks[instruction.from].drain(drain_from ..).collect::<Vec<_>>();
            self.stacks[instruction.to].extend(moved_stack);
        }
    }
}

fn find_top_crates(input: &[String]) -> String {
    let mut cratemover = CrateMover::new(input);
    cratemover.move_crates();
    cratemover.stacks.iter()
                     .map(|stack| stack.back().unwrap())
                     .collect()
}

fn find_top_crates_9001(input: &[String]) -> String {
    let mut cratemover = CrateMover::new(input);
    cratemover.move_crates_9001();
    cratemover.stacks.iter()
                     .map(|stack| stack.back().unwrap())
                     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        assert_eq!(find_top_crates(&input), "CMZ");
        assert_eq!(find_top_crates_9001(&input), "MCD");
    }
}

static DAY: u8 = 10;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", signal_strengths(&input));
    println!("{DAY}b: \n{}", display_output(&input));
}

enum Instruction {
    Noop,
    Addx { value: isize },
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        if let Some((instr, val)) = input.split_once(' ') {
            match instr {
                "addx" => Instruction::Addx { value: val.parse().unwrap() },
                _ => unimplemented!(),
            }
        } else {
            match input {
                "noop" => Instruction::Noop,
                _ => unimplemented!(),
            }
        }
    }

    fn cycletime(&self) -> usize {
        match *self {
            Instruction::Noop => 1,
            Instruction::Addx { value: _ } => 2,
        }
    }
}

struct Cpu {
    x: isize,
}

fn run_program(input: &[String]) -> (isize, String) {
    let mut instructions = input.iter()
                                .rev()
                                .map(|x| Instruction::new(x))
                                .collect::<Vec<_>>();
    let mut cpu = Cpu { x: 1 };
    let mut cycle = 0;
    let mut sigstr = 0;
    let mut display = String::with_capacity(246);

    while let Some(instr) = instructions.pop() {
        for _ in 0 .. instr.cycletime() {
            cycle += 1;
            if cycle % 40 == 20 {
                sigstr += cycle * cpu.x;
            }
            if [cpu.x - 1, cpu.x, cpu.x + 1].contains(&((cycle - 1) % 40)) {
                display.push('#');
            } else {
                display.push('.');
            }
            if cycle % 40 == 0 {
                display.push('\n');
            }
        }
        match instr {
            Instruction::Addx { value } => cpu.x += value,
            Instruction::Noop => {},
        }
    }

    (sigstr, display)
}

fn signal_strengths(input: &[String]) -> isize {
    run_program(input).0
}

fn display_output(input: &[String]) -> String {
    run_program(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signalstrength() {
        let input = [
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx -35",
            "addx 1",
            "addx 24",
            "addx -19",
            "addx 1",
            "addx 16",
            "addx -11",
            "noop",
            "noop",
            "addx 21",
            "addx -15",
            "noop",
            "noop",
            "addx -3",
            "addx 9",
            "addx 1",
            "addx -3",
            "addx 8",
            "addx 1",
            "addx 5",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx -36",
            "noop",
            "addx 1",
            "addx 7",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "addx 6",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx 7",
            "addx 1",
            "noop",
            "addx -13",
            "addx 13",
            "addx 7",
            "noop",
            "addx 1",
            "addx -33",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "noop",
            "noop",
            "noop",
            "addx 8",
            "noop",
            "addx -1",
            "addx 2",
            "addx 1",
            "noop",
            "addx 17",
            "addx -9",
            "addx 1",
            "addx 1",
            "addx -3",
            "addx 11",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx -13",
            "addx -19",
            "addx 1",
            "addx 3",
            "addx 26",
            "addx -30",
            "addx 12",
            "addx -1",
            "addx 3",
            "addx 1",
            "noop",
            "noop",
            "noop",
            "addx -9",
            "addx 18",
            "addx 1",
            "addx 2",
            "noop",
            "noop",
            "addx 9",
            "noop",
            "noop",
            "noop",
            "addx -1",
            "addx 2",
            "addx -37",
            "addx 1",
            "addx 3",
            "noop",
            "addx 15",
            "addx -21",
            "addx 22",
            "addx -6",
            "addx 1",
            "noop",
            "addx 2",
            "addx 1",
            "noop",
            "addx -10",
            "noop",
            "noop",
            "addx 20",
            "addx 1",
            "addx 2",
            "addx 2",
            "addx -6",
            "addx -11",
            "noop",
            "noop",
            "noop",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(signal_strengths(&input), 13140);

        let expected = "##..##..##..##..##..##..##..##..##..##..\n\
                        ###...###...###...###...###...###...###.\n\
                        ####....####....####....####....####....\n\
                        #####.....#####.....#####.....#####.....\n\
                        ######......######......######......####\n\
                        #######.......#######.......#######.....\n";
        assert_eq!(display_output(&input), expected);
    }
}

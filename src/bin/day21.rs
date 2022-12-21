use std::collections::HashMap;

static DAY: u8 = 21;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", yelled_number(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(PartialEq,Eq,Hash)]
enum Job {
    Yell { number: u64 },
    Add { op1: String, op2: String },
    Sub { op1: String, op2: String },
    Mul { op1: String, op2: String },
    Div { op1: String, op2: String },
}

impl Job {
    fn new(input: &str) -> Job {
        if let Some((op1, op2)) = input.split_once(" + ") {
            Job::Add { op1: op1.to_string(), op2: op2.to_string() }
        } else if let Some((op1, op2)) = input.split_once(" - ") {
            Job::Sub { op1: op1.to_string(), op2: op2.to_string() }
        } else if let Some((op1, op2)) = input.split_once(" * ") {
            Job::Mul { op1: op1.to_string(), op2: op2.to_string() }
        } else if let Some((op1, op2)) = input.split_once(" / ") {
            Job::Div { op1: op1.to_string(), op2: op2.to_string() }
        } else {
            Job::Yell { number: input.parse().unwrap() }
        }
    }
}

#[derive(PartialEq,Eq,Hash)]
struct Monkey {
    name: String,
    job: Job,
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let (name, job) = input.split_once(": ").unwrap();
        Monkey {
            name: name.to_string(),
            job: Job::new(job),
        }
    }
}

fn calculate_number(monkeys: &HashMap<String,Monkey>, name: &str) -> u64 {
    let monkey = monkeys.get(name).expect("monkey should be in list");
    match &monkey.job {
        Job::Yell { number } => *number,
        Job::Add { op1, op2 } => calculate_number(monkeys, &op1) + calculate_number(monkeys, &op2),
        Job::Sub { op1, op2 } => calculate_number(monkeys, &op1) - calculate_number(monkeys, &op2),
        Job::Mul { op1, op2 } => calculate_number(monkeys, &op1) * calculate_number(monkeys, &op2),
        Job::Div { op1, op2 } => calculate_number(monkeys, &op1) / calculate_number(monkeys, &op2),
    }
}

fn yelled_number(input: &[String]) -> u64 {
    let monkeys = input.iter()
                       .map(|x| Monkey::new(x))
                       .map(|x| (x.name.clone(), x))
                       .collect::<HashMap<_,_>>();
    calculate_number(&monkeys, "root")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(yelled_number(&input), 152);
    }
}

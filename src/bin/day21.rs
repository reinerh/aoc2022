use std::collections::HashMap;

static DAY: u8 = 21;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", yelled_number(&input));
    println!("{DAY}b: {}", your_number(&input));
}

#[derive(PartialEq,Eq,Hash)]
enum Job {
    Yell { number: i64 },
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

    fn get_operands(&self) -> (String, String) {
        match self {
            Job::Yell { number: _ } => panic!("Yell has no operands"),
            Job::Add { op1, op2 } => (op1.clone(), op2.clone()),
            Job::Sub { op1, op2 } => (op1.clone(), op2.clone()),
            Job::Mul { op1, op2 } => (op1.clone(), op2.clone()),
            Job::Div { op1, op2 } => (op1.clone(), op2.clone()),
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

fn calculate_number(monkeys: &HashMap<String,Monkey>, name: &str, ignore_human: bool) -> Option<i64> {
    if ignore_human && name == "humn" {
        return None;
    }
    let monkey = monkeys.get(name).expect("monkey should be in list");
    match &monkey.job {
        Job::Yell { number } => Some(*number),
        Job::Add { op1, op2 } => {
            if let (Some(op1), Some(op2)) = (calculate_number(monkeys, op1, ignore_human), calculate_number(monkeys, op2, ignore_human)) {
                Some(op1 + op2)
            } else {
                None
            }
        },
        Job::Sub { op1, op2 } => {
            if let (Some(op1), Some(op2)) = (calculate_number(monkeys, op1, ignore_human), calculate_number(monkeys, op2, ignore_human)) {
                Some(op1 - op2)
            } else {
                None
            }
        },
        Job::Mul { op1, op2 } => {
            if let (Some(op1), Some(op2)) = (calculate_number(monkeys, op1, ignore_human), calculate_number(monkeys, op2, ignore_human)) {
                Some(op1 * op2)
            } else {
                None
            }
        },
        Job::Div { op1, op2 } => {
            if let (Some(op1), Some(op2)) = (calculate_number(monkeys, op1, ignore_human), calculate_number(monkeys, op2, ignore_human)) {
                Some(op1 / op2)
            } else {
                None
            }
        },
    }
}

fn reverse_calculation(monkeys: &HashMap<String,Monkey>, name: &str, wanted: i64) -> i64 {
    if name == "humn" {
        return wanted;
    }
    let monkey = monkeys.get(name).expect("monkey should be in list");
    let (monkey1, monkey2) = monkey.job.get_operands();
    let (number1, number2) = (calculate_number(monkeys, &monkey1, true), calculate_number(monkeys, &monkey2, true));

    match &monkey.job {
        Job::Yell { number: _ } => panic!("unexpected job"),
        Job::Add { op1: _, op2: _ } => {
            match number1 {
                Some(n) => reverse_calculation(monkeys, &monkey2, wanted - n),
                None => reverse_calculation(monkeys, &monkey1, wanted - number2.unwrap()),
            }
        },
        Job::Sub { op1: _, op2: _ } => {
            match number1 {
                Some(n) => reverse_calculation(monkeys, &monkey2, n - wanted),
                None => reverse_calculation(monkeys, &monkey1, wanted + number2.unwrap()),
            }
        },
        Job::Mul { op1: _, op2: _ } => {
            match number1 {
                Some(n) => reverse_calculation(monkeys, &monkey2, wanted / n),
                None => reverse_calculation(monkeys, &monkey1, wanted / number2.unwrap()),
            }
        },
        Job::Div { op1: _, op2: _ } => {
            match number1 {
                Some(n) => reverse_calculation(monkeys, &monkey2, n / wanted),
                None => reverse_calculation(monkeys, &monkey1, wanted * number2.unwrap()),
            }
        },
    }
}

fn yelled_number(input: &[String]) -> i64 {
    let monkeys = input.iter()
                       .map(|x| Monkey::new(x))
                       .map(|x| (x.name.clone(), x))
                       .collect::<HashMap<_,_>>();
    calculate_number(&monkeys, "root", false).unwrap()
}

fn your_number(input: &[String]) -> i64 {
    let monkeys = input.iter()
                       .map(|x| Monkey::new(x))
                       .map(|x| (x.name.clone(), x))
                       .collect::<HashMap<_,_>>();
    let root = monkeys.get("root").expect("root monkey should exist");
    let (monkey1, monkey2) = root.job.get_operands();

    let result1 = calculate_number(&monkeys, &monkey1, true);
    let (number_to_match, needs_match) = match result1 {
        Some(n) => (n, monkey2),
        None => (calculate_number(&monkeys, &monkey2, true).unwrap(), monkey1),
    };
    reverse_calculation(&monkeys, &needs_match, number_to_match)
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
        assert_eq!(your_number(&input), 301);
    }
}

static DAY: u8 = 11;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", monkey_business(&input, 20, false));
    println!("{DAY}b: {}", monkey_business(&input, 10000, true));
}

#[derive(Debug, Clone)]
enum Operation {
    Add { operand: Option<usize> },
    Mul { operand: Option<usize> },
}

impl Operation {
    fn new(input: &str) -> Operation {
        let fixed = "  Operation: new = old ";
        let mut it = input.chars().skip(fixed.len());
        let operation = it.next().unwrap();
        it.next(); // space
        let operand = it.collect::<String>();
        let operand = operand.parse::<usize>().ok();
        match operation {
            '+' => Operation::Add { operand },
            '*' => Operation::Mul { operand },
            _ => unimplemented!(),
        }
    }

    fn compute(&self, old: usize) -> usize {
        match *self {
            Operation::Add { operand: None } => old + old,
            Operation::Add { operand: Some(operand) } => old + operand,
            Operation::Mul { operand: None } => old * old,
            Operation::Mul { operand: Some(operand) } => old * operand,
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisible: usize,
    iftrue: usize,
    iffalse: usize,
}

impl Test {
    fn new(input: &[String]) -> Test {
        let fixed1 = "  Test: divisible by ";
        let divisible = input[0].chars()
                                .skip(fixed1.len())
                                .collect::<String>()
                                .parse().unwrap();

        let fixed2 = "    If true: throw to monkey ";
        let iftrue = input[1].chars()
                             .nth(fixed2.len()).unwrap()
                             .to_digit(10).unwrap() as usize;
        let iffalse = input[2].chars()
                              .nth(fixed2.len() + 1).unwrap()
                              .to_digit(10).unwrap() as usize;

        Test { divisible, iftrue, iffalse }
    }

    fn eval(&self, operand: usize) -> usize {
        if operand % self.divisible == 0 {
            self.iftrue
        } else {
            self.iffalse
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn new(input: &[String]) -> Monkey {
        /*let id = input[0].chars()
                         .skip(7)
                         .next().unwrap()
                         .to_digit(10).unwrap();*/

        let items = input[1].split_once(':').unwrap().1
                            .split(',')
                            .map(|x| x.trim().parse().unwrap())
                            .collect();

        let operation = Operation::new(&input[2]);

        let test = Test::new(&input[3..]);

        Monkey { items, operation, test, inspections: 0 }
    }
}

fn monkey_business(input: &[String], rounds: usize, worried: bool) -> usize {
    let mut monkeys = Vec::new();
    for lines in input.chunks(7) {
        let monkey = Monkey::new(lines);
        monkeys.push(monkey);
    }

    let divisibility : usize = monkeys.iter()
                                      .map(|m| m.test.divisible)
                                      .product();
    for _ in 0 .. rounds {
        for i in 0 .. monkeys.len() {
            for item in monkeys[i].items.clone() {
                monkeys[i].inspections += 1;
                let mut worry = monkeys[i].operation.compute(item);
                if !worried {
                    worry /= 3;
                } else {
                    /* all divisibility tests are prime, so it's possible
                       to reduce the worry level by the product of all values */
                    worry %= divisibility;
                }
                let next = monkeys[i].test.eval(worry);
                monkeys[next].items.push(worry);
            }
            monkeys[i].items.clear();
        }
    }

    let mut inspections = monkeys.iter()
                                 .map(|m| m.inspections)
                                 .collect::<Vec<_>>();
    inspections.sort();
    inspections.iter()
               .rev()
               .take(2)
               .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(monkey_business(&input, 20, false), 10605);
        assert_eq!(monkey_business(&input, 10000, true), 2713310158);
    }
}

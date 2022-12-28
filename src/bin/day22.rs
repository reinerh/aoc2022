use std::collections::HashMap;

static DAY: u8 = 22;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", find_password(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone,Copy)]
enum Object {
    Floor,
    Wall,
}

enum Instruction {
    Left,
    Right,
    Move { amount: isize },
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

struct Map {
    map: HashMap<(isize,isize),Object>,
    instructions: Vec<Instruction>,
    pos: (isize,isize),
    direction: Direction,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut map = HashMap::new();
        let mut pos = None;
        for (y, line) in input.iter().enumerate() {
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as isize, y as isize);
                match c {
                    '.' => {
                        if pos.is_none() {
                            pos = Some((x, y));
                        }
                        map.insert((x,y), Object::Floor);
                    }
                    '#' => { map.insert((x,y), Object::Wall); }
                    _ => continue,
                }
            }
        }

        let mut instructions = Vec::new();
        let mut number = 0;
        for c in input.last().unwrap().chars() {
            if c.is_ascii_digit() {
                number *= 10;
                number += c.to_digit(10).unwrap() as isize;
            } else {
                if number > 0 {
                    instructions.push(Instruction::Move { amount: number });
                    number = 0;
                }
                match c {
                    'R' => instructions.push(Instruction::Right),
                    'L' => instructions.push(Instruction::Left),
                    _ => unimplemented!(),
                }
            }
        }
        if number > 0 {
            instructions.push(Instruction::Move { amount: number });
        }

        Map { map, instructions, pos: pos.unwrap(), direction: Direction::Right }
    }

    fn opposite_position(&self, direction: Direction) -> Option<(isize,isize)> {
        let mut old_pos = self.pos;
        let mut old_object = Object::Floor;
        loop {
            let (x, y) = old_pos;
            let check_pos = match direction {
                Direction::Left => (x-1, y),
                Direction::Right => (x+1, y),
                Direction::Up => (x, y-1),
                Direction::Down => (x, y+1),
            };
            match self.map.get(&check_pos) {
                None => return match old_object {
                    Object::Floor => Some(old_pos),
                    Object::Wall => None,
                },
                Some(obj) => old_object = *obj,
            }
            old_pos = check_pos;
        }
    }

    fn step_forward(&self) -> (isize,isize) {
        let (x, y) = self.pos;
        return match self.direction {
            Direction::Left => {
                match self.map.get(&(x-1,y)) {
                    None => self.opposite_position(Direction::Right).unwrap_or(self.pos),
                    Some(Object::Wall) => self.pos,
                    Some(Object::Floor) => (x-1,y),
                }
            },
            Direction::Right => {
                match self.map.get(&(x+1,y)) {
                    None => self.opposite_position(Direction::Left).unwrap_or(self.pos),
                    Some(Object::Wall) => self.pos,
                    Some(Object::Floor) => (x+1,y),
                }
            },
            Direction::Up => {
                match self.map.get(&(x,y-1)) {
                    None => self.opposite_position(Direction::Down).unwrap_or(self.pos),
                    Some(Object::Wall) => self.pos,
                    Some(Object::Floor) => (x,y-1),
                }
            },
            Direction::Down => {
                match self.map.get(&(x,y+1)) {
                    None => self.opposite_position(Direction::Up).unwrap_or(self.pos),
                    Some(Object::Wall) => self.pos,
                    Some(Object::Floor) => (x,y+1),
                }
            },
        }
    }

    fn run(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Left => {
                    self.direction = self.direction.turn_left();
                },
                Instruction::Right => {
                    self.direction = self.direction.turn_right();
                },
                Instruction::Move { amount } => {
                    for _ in 0 .. *amount {
                        self.pos = self.step_forward();
                    }
                },
            }
        }
    }

    fn password(&self) -> isize {
        let facing = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        1000 * (self.pos.1 + 1) + 4 * (self.pos.0 + 1) + facing
    }
}

fn find_password(input: &[String]) -> isize {
    let mut map = Map::new(input);
    map.run();
    map.password()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(find_password(&input), 6032);
    }
}

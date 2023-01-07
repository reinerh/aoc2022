use std::collections::HashMap;

static DAY: u8 = 22;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", find_password(&input, false));
    println!("{DAY}b: {}", find_password(&input, true));
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

#[derive(Clone,Copy)]
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

    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct Map {
    map: HashMap<(isize,isize),Object>,
    instructions: Vec<Instruction>,
    pos: (isize,isize),
    direction: Direction,
    cube_wrapping: bool,
}

impl Map {
    fn new(input: &[String], cube_wrapping: bool) -> Map {
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

        Map { map, instructions, pos: pos.unwrap(), direction: Direction::Right, cube_wrapping }
    }

    fn opposite_position(&self) -> Option<((isize,isize),Direction)> {
        let direction = self.direction.opposite();
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
                    Object::Floor => Some((old_pos,self.direction)),
                    Object::Wall => None,
                },
                Some(obj) => old_object = *obj,
            }
            old_pos = check_pos;
        }
    }

    fn next_cube_position(&self) -> Option<((isize,isize),Direction)> {
        let (new_pos, new_direction) = match self.direction {
            Direction::Left => {
                match self.pos.0 {
                    0 => {
                        if (100..150).contains(&self.pos.1) {
                            // 100 -> 49, 149 -> 0
                            ((50, 49 - (self.pos.1 - 100)), Direction::Right)
                        } else if (150..200).contains(&self.pos.1) {
                            // 150 -> 50, 199 -> 99
                            ((self.pos.1 - 100, 0), Direction::Down)
                        } else { panic!("no wrapping expected"); }
                    },
                    50 => {
                        if (0..50).contains(&self.pos.1) {
                            // 49 -> 100, 0 -> 149
                            ((0, 149 - self.pos.1), Direction::Right)
                        } else if (50..100).contains(&self.pos.1) {
                            // 50 -> 0, 99 -> 49
                            ((self.pos.1 - 50, 100), Direction::Down)
                        } else { panic!("no wrapping expected"); }
                    },
                    _ => panic!("no wrapping expected"),
                }
            },
            Direction::Right => {
                match self.pos.0 {
                    49 => {
                        if (150..200).contains(&self.pos.1) {
                            // 150 -> 50, 199 -> 99
                            ((self.pos.1 - 100, 149), Direction::Up)
                        } else { panic!("no wrapping expected"); }
                    },
                    99 => {
                        if (50..100).contains(&self.pos.1) {
                            // 50 -> 100, 99 -> 149
                            ((self.pos.1 + 50, 49), Direction::Up)
                        } else if (100..150).contains(&self.pos.1) {
                            // 149 -> 0, 100 -> 49
                            ((149, 49 - (self.pos.1 - 100)), Direction::Left)
                        } else { panic!("no wrapping expected"); }
                    },
                    149 => {
                        if (0..50).contains(&self.pos.1) {
                            // 0 -> 149, 49 -> 100
                            ((99, 149 - self.pos.1), Direction::Left)
                        } else { panic!("no wrapping expected"); }
                    },
                    _ => panic!("no wrapping expected"),
                }
            },
            Direction::Up => {
                match self.pos.1 {
                    0 => {
                        if (50..100).contains(&self.pos.0) {
                            // 50 -> 150, 99 -> 199
                            ((0, self.pos.0 + 100), Direction::Right)
                        } else if (100..150).contains(&self.pos.0) {
                            // 100 -> 0, 149 -> 49
                            ((self.pos.0 - 100, 199), Direction::Up)
                        } else { panic!("no wrapping expected"); }
                    },
                    100 => {
                        if (0..50).contains(&self.pos.0) {
                            // 0 -> 50, 49 -> 99
                            ((50, self.pos.0 + 50), Direction::Right)
                        } else { panic!("no wrapping expected"); }
                    },
                    _ => panic!("no wrapping expected"),
                }
            },
            Direction::Down => {
                match self.pos.1 {
                    49 => {
                        if (100..150).contains(&self.pos.0) {
                            // 100 -> 50, 149 -> 99
                            ((99, self.pos.0 - 50), Direction::Left)
                        } else { panic!("no wrapping expected"); }
                    },
                    149 => {
                        if (50..100).contains(&self.pos.0) {
                            // 50 -> 150, 99 -> 199
                            ((49, self.pos.0 + 100), Direction::Left)
                        } else { panic!("no wrapping expected"); }
                    },
                    199 => {
                        if (0..50).contains(&self.pos.0) {
                            // 0 -> 100, 49 -> 149
                            ((self.pos.0 + 100, 0), Direction::Down)
                        } else { panic!("no wrapping expected"); }
                    },
                    _ => panic!("no wrapping expected"),
                }
            }
        };
        if let Some(Object::Wall) = self.map.get(&new_pos) {
            None
        } else {
            Some((new_pos, new_direction))
        }
    }

    fn wrapped_position(&self) -> Option<((isize,isize),Direction)> {
        if self.cube_wrapping {
            self.next_cube_position()
        } else {
            self.opposite_position()
        }
    }

    fn step_forward(&self) -> ((isize,isize),Direction) {
        let (x, y) = self.pos;
        let default = (self.pos, self.direction);
        return match self.direction {
            Direction::Left => {
                match self.map.get(&(x-1,y)) {
                    None => self.wrapped_position().unwrap_or(default),
                    Some(Object::Wall) => (self.pos, Direction::Left),
                    Some(Object::Floor) => ((x-1,y), Direction::Left),
                }
            },
            Direction::Right => {
                match self.map.get(&(x+1,y)) {
                    None => self.wrapped_position().unwrap_or(default),
                    Some(Object::Wall) => (self.pos, Direction::Right),
                    Some(Object::Floor) => ((x+1,y), Direction::Right),
                }
            },
            Direction::Up => {
                match self.map.get(&(x,y-1)) {
                    None => self.wrapped_position().unwrap_or(default),
                    Some(Object::Wall) => (self.pos, Direction::Up),
                    Some(Object::Floor) => ((x,y-1), Direction::Up),
                }
            },
            Direction::Down => {
                match self.map.get(&(x,y+1)) {
                    None => self.wrapped_position().unwrap_or(default),
                    Some(Object::Wall) => (self.pos, Direction::Down),
                    Some(Object::Floor) => ((x,y+1), Direction::Down),
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
                        (self.pos, self.direction) = self.step_forward();
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

fn find_password(input: &[String], cube_wrapping: bool) -> isize {
    let mut map = Map::new(input, cube_wrapping);
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

        assert_eq!(find_password(&input, false), 6032);
        //assert_eq!(find_password(&input, true), 5031);
    }
}

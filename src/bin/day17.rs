use std::collections::HashSet;

static DAY: u8 = 17;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", simulate_rocks(&input[0], 2022));
    println!("{DAY}b: {}", 0);
}

#[derive(Debug)]
struct Shape {
    shape: u8,
    pos: (u64, u64),  // top left position of the bounding box
}

impl Shape {
    fn next(&self, map: &HashSet<(u64,u64)>) -> Shape {
        let next_shape = (self.shape + 1) % 5;
        Shape::new(map, next_shape)
    }

    fn new(map: &HashSet<(u64,u64)>, shape: u8) -> Shape {
        let max_y = map.iter()
                       .map(|pos| pos.1 as i64)
                       .max()
                       .unwrap_or(-1);

        let new_y = max_y + 3 + match shape {
            0 => 1,
            1 => 3,
            2 => 3,
            3 => 4,
            4 => 2,
            _ => unimplemented!(),
        };
        Shape {
            shape,
            pos: (2, new_y as u64),
        }
    }

    fn can_move_down(&self, map: &HashSet<(u64, u64)>) -> bool {
        let (x,y) = self.pos;
        match self.shape {
            0 => y > 0 &&
                 !map.contains(&(x, y-1)) &&
                 !map.contains(&(x+1, y-1)) &&
                 !map.contains(&(x+2, y-1)) &&
                 !map.contains(&(x+3, y-1)),
            1 => y > 2 &&
                 !map.contains(&(x, y-2)) &&
                 !map.contains(&(x+1, y-3)) &&
                 !map.contains(&(x+2, y-2)),
            2 => y > 2 &&
                 !map.contains(&(x, y-3)) &&
                 !map.contains(&(x+1, y-3)) &&
                 !map.contains(&(x+2, y-3)),
            3 => y > 3 &&
                 !map.contains(&(x, y-4)),
            4 => y > 1 &&
                 !map.contains(&(x, y-2)) &&
                 !map.contains(&(x+1, y-2)),
            _ => unimplemented!(),
        }
    }

    fn can_move_left(&self, map: &HashSet<(u64, u64)>) -> bool {
        let (x,y) = self.pos;
        if x == 0 {
            return false;
        }
        match self.shape {
            0 => !map.contains(&(x-1, y)),
            1 => !map.contains(&(x, y)) &&
                !map.contains(&(x-1, y-1)) &&
                !map.contains(&(x, y-2)),
            2 => !map.contains(&(x+1, y)) &&
                 !map.contains(&(x+1, y-1)) &&
                 !map.contains(&(x-1, y-2)),
            3 => !map.contains(&(x-1, y)) &&
                 !map.contains(&(x-1, y-1)) &&
                 !map.contains(&(x-1, y-2)) &&
                 !map.contains(&(x-1, y-3)),
            4 => !map.contains(&(x-1, y)) &&
                 !map.contains(&(x-1, y-1)),
            _ => unimplemented!(),
        }
    }

    fn can_move_right(&self, map: &HashSet<(u64, u64)>) -> bool {
        let (x,y) = self.pos;
        match self.shape {
            0 => x < (7-4) &&
                 !map.contains(&(x+4, y)),
            1 => x < (7-3) &&
                 !map.contains(&(x+2, y)) &&
                 !map.contains(&(x+3, y-1)) &&
                 !map.contains(&(x+2, y-2)),
            2 => x < (7-3) &&
                 !map.contains(&(x+3, y)) &&
                 !map.contains(&(x+3, y-1)) &&
                 !map.contains(&(x+3, y-2)),
            3 => x < (7-1) &&
                 !map.contains(&(x+1, y)) &&
                 !map.contains(&(x+1, y-1)) &&
                 !map.contains(&(x+1, y-2)) &&
                 !map.contains(&(x+1, y-3)),
            4 => x < (7-2) &&
                 !map.contains(&(x+2, y)) &&
                 !map.contains(&(x+2, y-1)),
            _ => unimplemented!(),
        }
    }

    fn move_left(&mut self, map: &HashSet<(u64, u64)>) {
        if self.can_move_left(map) {
            self.pos.0 -= 1;
        }
    }

    fn move_right(&mut self, map: &HashSet<(u64, u64)>) {
        if self.can_move_right(map) {
            self.pos.0 += 1;
        }
    }

    fn move_down(&mut self) {
        self.pos.1 -= 1;
    }

    fn movement(&mut self, map: &HashSet<(u64, u64)>, direction: char) {
        match direction {
            '>' => self.move_right(map),
            '<' => self.move_left(map),
            _ => unimplemented!(),
        }
    }

    fn add_to_map(&self, map: &mut HashSet<(u64, u64)>) {
        let (x, y) = self.pos;
        match self.shape {
            0 => {
                map.insert((x, y));
                map.insert((x+1, y));
                map.insert((x+2, y));
                map.insert((x+3, y));
            },
            1 => {
                map.insert((x+1, y));
                map.insert((x, y-1));
                map.insert((x+1, y-1));
                map.insert((x+2, y-1));
                map.insert((x+1, y-2));
            },
            2 => {
                map.insert((x+2, y));
                map.insert((x+2, y-1));
                map.insert((x+2, y-2));
                map.insert((x+1, y-2));
                map.insert((x, y-2));
            },
            3 => {
                map.insert((x, y));
                map.insert((x, y-1));
                map.insert((x, y-2));
                map.insert((x, y-3));
            },
            4 => {
                map.insert((x, y));
                map.insert((x+1, y));
                map.insert((x, y-1));
                map.insert((x+1, y-1));
            }
            _ => unimplemented!(),
        }
    }
}

struct Tetris {
    map: HashSet<(u64,u64)>,  // positions occupied by rocks
    rock: Shape,
    count: u64,
}

impl Tetris {
    fn new() -> Tetris {
        let map = HashSet::new();
        let rock = Shape::new(&map, 0);
        Tetris {
            map,
            rock,
            count: 0,
        }
    }

    fn step(&mut self, direction: char) {
        self.rock.movement(&self.map, direction);
        if self.rock.can_move_down(&self.map) {
            self.rock.move_down();
        } else {
            self.rock.add_to_map(&mut self.map);
            self.rock = self.rock.next(&self.map);
            self.count += 1;
        }
    }

    fn height(&self) -> u64 {
        1 + self.map.iter()
                    .map(|pos| pos.1)
                    .max()
                    .unwrap_or(0)
    }

    fn dump_line(&self, y: u64) -> String {
        let mut line = String::with_capacity(7);
        for x in 0 .. 7 {
            if self.map.contains(&(x, y as u64)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        line
    }

    fn dump_map(&self) {
        for y in (0 ..= self.height()).rev() {
            println!("|{}|", self.dump_line(y));
        }
        println!("+-------+\n\n");
    }
}

fn simulate_rocks(input: &str, max_rocks: u64) -> u64 {
    let mut tetris = Tetris::new();
    for direction in input.chars().cycle() {
        if tetris.count == max_rocks {
            return tetris.height();
        }
        tetris.step(direction);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(simulate_rocks(&input, 2022), 3068);
    }
}

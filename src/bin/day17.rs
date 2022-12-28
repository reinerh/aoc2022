static DAY: u8 = 17;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", simulate_rocks(&input[0], 2022));
    println!("{DAY}b: {}", simulate_rocks(&input[0], 1000000000000));
}

struct Map {
    map: Vec<u8>,
}

impl Map {
    fn new() -> Map {
        Map { map: Vec::new() }
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn occupied(&self, pos: &(usize, usize)) -> bool {
        assert!(pos.0 <= 6);
        if pos.1 >= self.height() {
            return false;
        }
        self.map[pos.1] & (1 << pos.0) != 0
    }

    fn place(&mut self, pos: (usize, usize)) {
        assert!(pos.0 <= 6);
        if self.height() <= pos.1 {
            self.map.resize(pos.1 + 1, 0);
        }
        self.map[pos.1] |= 1 << pos.0;
    }
}

#[derive(Debug)]
struct Shape {
    shape: u8,
    pos: (usize, usize),  // top left position of the bounding box
}

impl Shape {
    fn next(&self, map: &Map) -> Shape {
        let next_shape = (self.shape + 1) % 5;
        Shape::new(map, next_shape)
    }

    fn new(map: &Map, shape: u8) -> Shape {
        let new_y = map.height() + 2 + match shape {
            0 => 1,
            1 => 3,
            2 => 3,
            3 => 4,
            4 => 2,
            _ => unimplemented!(),
        };
        Shape {
            shape,
            pos: (2, new_y),
        }
    }

    fn can_move_down(&self, map: &Map) -> bool {
        let (x,y) = self.pos;
        match self.shape {
            0 => y > 0 &&
                 !map.occupied(&(x, y-1)) &&
                 !map.occupied(&(x+1, y-1)) &&
                 !map.occupied(&(x+2, y-1)) &&
                 !map.occupied(&(x+3, y-1)),
            1 => y > 2 &&
                 !map.occupied(&(x, y-2)) &&
                 !map.occupied(&(x+1, y-3)) &&
                 !map.occupied(&(x+2, y-2)),
            2 => y > 2 &&
                 !map.occupied(&(x, y-3)) &&
                 !map.occupied(&(x+1, y-3)) &&
                 !map.occupied(&(x+2, y-3)),
            3 => y > 3 &&
                 !map.occupied(&(x, y-4)),
            4 => y > 1 &&
                 !map.occupied(&(x, y-2)) &&
                 !map.occupied(&(x+1, y-2)),
            _ => unimplemented!(),
        }
    }

    fn can_move_left(&self, map: &Map) -> bool {
        let (x,y) = self.pos;
        if x == 0 {
            return false;
        }
        match self.shape {
            0 => !map.occupied(&(x-1, y)),
            1 => !map.occupied(&(x, y)) &&
                 !map.occupied(&(x-1, y-1)) &&
                 !map.occupied(&(x, y-2)),
            2 => !map.occupied(&(x+1, y)) &&
                 !map.occupied(&(x+1, y-1)) &&
                 !map.occupied(&(x-1, y-2)),
            3 => !map.occupied(&(x-1, y)) &&
                 !map.occupied(&(x-1, y-1)) &&
                 !map.occupied(&(x-1, y-2)) &&
                 !map.occupied(&(x-1, y-3)),
            4 => !map.occupied(&(x-1, y)) &&
                 !map.occupied(&(x-1, y-1)),
            _ => unimplemented!(),
        }
    }

    fn can_move_right(&self, map: &Map) -> bool {
        let (x,y) = self.pos;
        match self.shape {
            0 => x < (7-4) &&
                 !map.occupied(&(x+4, y)),
            1 => x < (7-3) &&
                 !map.occupied(&(x+2, y)) &&
                 !map.occupied(&(x+3, y-1)) &&
                 !map.occupied(&(x+2, y-2)),
            2 => x < (7-3) &&
                 !map.occupied(&(x+3, y)) &&
                 !map.occupied(&(x+3, y-1)) &&
                 !map.occupied(&(x+3, y-2)),
            3 => x < (7-1) &&
                 !map.occupied(&(x+1, y)) &&
                 !map.occupied(&(x+1, y-1)) &&
                 !map.occupied(&(x+1, y-2)) &&
                 !map.occupied(&(x+1, y-3)),
            4 => x < (7-2) &&
                 !map.occupied(&(x+2, y)) &&
                 !map.occupied(&(x+2, y-1)),
            _ => unimplemented!(),
        }
    }

    fn move_left(&mut self, map: &Map) {
        if self.can_move_left(map) {
            self.pos.0 -= 1;
        }
    }

    fn move_right(&mut self, map: &Map) {
        if self.can_move_right(map) {
            self.pos.0 += 1;
        }
    }

    fn move_down(&mut self) {
        self.pos.1 -= 1;
    }

    fn movement(&mut self, map: &Map, direction: char) {
        match direction {
            '>' => self.move_right(map),
            '<' => self.move_left(map),
            _ => unimplemented!(),
        }
    }

    fn add_to_map(&self, map: &mut Map) {
        let (x, y) = self.pos;
        match self.shape {
            0 => {
                map.place((x, y));
                map.place((x+1, y));
                map.place((x+2, y));
                map.place((x+3, y));
            },
            1 => {
                map.place((x+1, y));
                map.place((x, y-1));
                map.place((x+1, y-1));
                map.place((x+2, y-1));
                map.place((x+1, y-2));
            },
            2 => {
                map.place((x+2, y));
                map.place((x+2, y-1));
                map.place((x+2, y-2));
                map.place((x+1, y-2));
                map.place((x, y-2));
            },
            3 => {
                map.place((x, y));
                map.place((x, y-1));
                map.place((x, y-2));
                map.place((x, y-3));
            },
            4 => {
                map.place((x, y));
                map.place((x+1, y));
                map.place((x, y-1));
                map.place((x+1, y-1));
            }
            _ => unimplemented!(),
        }
    }
}

struct Tetris {
    map: Map,  // positions occupied by rocks
    rock: Shape,
    count: usize,
}

impl Tetris {
    fn new() -> Tetris {
        let map = Map::new();
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

    fn find_cycle(&self, properties: &[(usize, usize, usize, u8)]) -> Option<(usize,usize,usize)> {
        if properties.len() < 3 {
            return None;
        }
        let needle = properties.last().unwrap();

        let pos1 = match properties.iter().position(|&x| x == *needle) {
            Some(pos) => pos,
            None => return None,
        };
        let pos2 = match properties.iter().skip(pos1 + 1).position(|&x| x == *needle) {
            Some(pos) => pos1 + pos + 1,
            None => return None,
        };
        let pos3 = match properties.iter().skip(pos2 + 1).position(|&x| x == *needle) {
            Some(pos) => pos2 + pos + 1,
            None => return None,
        };

        if properties[pos1 ..= pos2] == properties[pos2 ..= pos3] {
            let cycle_len = pos1.abs_diff(pos2);
            let height_diff : usize = properties.iter()
                                                .skip(pos1)
                                                .take(cycle_len)
                                                .map(|x| x.0)
                                                .sum();
            return Some((pos1, cycle_len, height_diff));
        }
        None
    }
}

fn simulate_rocks(input: &str, max_rocks: usize) -> usize {
    let mut tetris = Tetris::new();
    let mut properties = Vec::new();
    let mut prev_height = 0;
    let mut prev_count = 0;
    for (idx, direction) in input.chars().cycle().enumerate() {
        let cur_height = tetris.map.height();
        tetris.step(direction);
        if tetris.count > prev_count {
            prev_count = tetris.count;
            /* properties that need to match:
               - height difference to previous shape
               - position in cycle
               - x position of shape
               - shape type
            */
            properties.push((cur_height - prev_height, idx % input.len(), tetris.rock.pos.0, tetris.rock.shape));
            prev_height = cur_height;
            if let Some(cycle) = tetris.find_cycle(&properties) {
                let cycle_begin = cycle.0;
                let cycle_len = cycle.1;
                let height_diff_per_cycle = cycle.2;
                let number_cycles = (max_rocks - cycle_begin + 1) / cycle_len;
                let remaining_rocks = (max_rocks - cycle_begin + 1) % cycle_len;

                let height_start : usize = properties.iter()
                                                     .take(cycle_begin)
                                                     .map(|x| x.0)
                                                     .sum();
                let height_end : usize = properties.iter()
                                                   .skip(cycle_begin)
                                                   .take(remaining_rocks)
                                                   .map(|x| x.0)
                                                   .sum();
                return height_start + number_cycles * height_diff_per_cycle + height_end;
            }
        }
    }

    panic!("no cycle found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(simulate_rocks(&input, 2022), 3068);
        assert_eq!(simulate_rocks(&input, 1000000000000), 1514285714288);
    }
}

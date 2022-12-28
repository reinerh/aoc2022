use std::collections::{HashMap, HashSet};

static DAY: u8 = 24;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", travel_time(&input));
    println!("{DAY}b: {}", travel_time_3(&input));
}

#[derive(Clone,Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone,Copy)]
enum Object {
    Wall,
    Blizzard { direction: Direction },
}

impl Object {
    fn new(input: char) -> Object {
        match input {
            '^' => Object::Blizzard { direction: Direction::Up },
            'v' => Object::Blizzard { direction: Direction::Down },
            '<' => Object::Blizzard { direction: Direction::Left },
            '>' => Object::Blizzard { direction: Direction::Right },
            '#' => Object::Wall,
            _ => unimplemented!(),
        }
    }
}

struct Map {
    map: HashMap<(isize,isize),Vec<Object>>,
    dimensions: (isize, isize),
    start: (isize, isize),
    end: (isize, isize),
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut map = HashMap::new();
        let mut dimensions = (0, 0);
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as isize, y as isize);
                if c == '.' {
                    if y == 0 {
                        start = (x, y);
                    } else {
                        end = (x, y);
                    }
                } else {
                    map.insert((x,y), vec![Object::new(c)]);
                }
                dimensions = (x, y);
            }
        }
        Map { map, dimensions, start, end }
    }

    fn next_map(&self) -> HashMap<(isize,isize),Vec<Object>> {
        let mut new_map = HashMap::<(isize,isize),Vec<Object>>::new();
        for (&(x,y), objects) in &self.map {
            for object in objects {
                let new_pos = match object {
                    Object::Wall => (x,y),
                    Object::Blizzard { direction: Direction::Up } => {
                        if y - 1 == 0 { (x, self.dimensions.1 - 1) } else { (x, y - 1) }
                    },
                    Object::Blizzard { direction: Direction::Down } => {
                        if y + 1 == self.dimensions.1 { (x, 1) } else { (x, y + 1) }
                    },
                    Object::Blizzard { direction: Direction::Left } => {
                        if x - 1 == 0 { (self.dimensions.0 - 1, y) } else { (x - 1, y) }
                    },
                    Object::Blizzard { direction: Direction::Right } => {
                        if x + 1 == self.dimensions.0 { (1, y) } else { (x + 1, y) }
                    },
                };
                new_map.entry(new_pos).or_default().push(*object);
            }
        }
        new_map
    }

    fn travel_time(&mut self) -> usize {
        let mut visited = HashSet::from([self.start]);
        let mut time = 0;
        loop {
            let next_map = self.next_map();
            let mut next_visited = HashSet::new();
            for &(x,y) in &visited {
                if !next_map.contains_key(&(x,y)) {
                    next_visited.insert((x,y));
                }
                if x < self.dimensions.0 && !next_map.contains_key(&(x+1,y)) {
                    next_visited.insert((x+1,y));
                }
                if x > 0 && !next_map.contains_key(&(x-1,y)) {
                    next_visited.insert((x-1,y));
                }
                if y < self.dimensions.1 && !next_map.contains_key(&(x,y+1)) {
                    next_visited.insert((x,y+1));
                }
                if y > 0 && !next_map.contains_key(&(x,y-1)) {
                    next_visited.insert((x,y-1));
                }
            }
            visited = next_visited;
            self.map = next_map;
            time += 1;
            if visited.contains(&self.end) {
                return time;
            }
        }
    }
}

fn travel_time(input: &[String]) -> usize {
    let mut map = Map::new(input);
    map.travel_time()
}

fn travel_time_3(input: &[String]) -> usize {
    let mut map = Map::new(input);
    let mut time = map.travel_time();
    (map.start, map.end) = (map.end, map.start);
    time += map.travel_time();
    (map.start, map.end) = (map.end, map.start);
    time += map.travel_time();

    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "#.######",
            "#>>.<^<#",
            "#.<..<<#",
            "#>v.><>#",
            "#<^v^^>#",
            "######.#",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(travel_time(&input), 18);
        assert_eq!(travel_time_3(&input), 54);
    }
}

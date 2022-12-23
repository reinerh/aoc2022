use std::collections::{HashSet, HashMap};

static DAY: u8 = 23;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", ground_tiles(&input));
    println!("{DAY}b: {}", no_movement(&input));
}

struct Map {
    elves: HashSet<(isize,isize)>,
    proposed: HashMap<(isize,isize), usize>,
    rounds: usize,
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut elves = HashSet::new();
        for (y, line) in input.iter().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                if symbol == '#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }

        Map { elves, proposed: HashMap::new(), rounds: 0 }
    }

    fn has_neighbors(&self, pos: (isize, isize), direction: Option<usize>) -> bool {
        let (x,y) = pos;
        match direction {
            Some(0) => self.elves.contains(&(x-1,y-1)) || self.elves.contains(&(x,y-1)) || self.elves.contains(&(x+1,y-1)),
            Some(1) => self.elves.contains(&(x-1,y+1)) || self.elves.contains(&(x,y+1)) || self.elves.contains(&(x+1,y+1)),
            Some(2) => self.elves.contains(&(x-1,y-1)) || self.elves.contains(&(x-1,y)) || self.elves.contains(&(x-1,y+1)),
            Some(3) => self.elves.contains(&(x+1,y-1)) || self.elves.contains(&(x+1,y)) || self.elves.contains(&(x+1,y+1)),
            Some(_) => unimplemented!(),
            None => self.has_neighbors(pos, Some(0)) || self.has_neighbors(pos, Some(1)) || self.has_neighbors(pos, Some(2)) || self.has_neighbors(pos, Some(3))
        }
    }

    fn next_round(&mut self) {
        let direction = self.rounds % 4;

        /* first half */
        for elf in &self.elves {
            let (x,y) = *elf;
            if !self.has_neighbors(*elf, None) {
                continue;
            }
            for d in 0 .. 4 {
                let d = (direction + d) % 4;
                if !self.has_neighbors(*elf, Some(d)) {
                    match d {
                        0 => *self.proposed.entry((x,y-1)).or_insert(0) += 1,
                        1 => *self.proposed.entry((x,y+1)).or_insert(0) += 1,
                        2 => *self.proposed.entry((x-1,y)).or_insert(0) += 1,
                        3 => *self.proposed.entry((x+1,y)).or_insert(0) += 1,
                        _ => unimplemented!(),
                    };
                    break;
                }
            }
        }

        /* second half */
        let mut new_positions = HashSet::new();
        for elf in &self.elves {
            let (x,y) = *elf;
            if !self.has_neighbors(*elf, None) {
                new_positions.insert(*elf);
                continue;
            }
            if self.has_neighbors(*elf, Some(0)) && self.has_neighbors(*elf, Some(1)) && self.has_neighbors(*elf, Some(2)) && self.has_neighbors(*elf, Some(3)) {
                new_positions.insert(*elf);
                continue;
            }
            for d in 0 .. 4 {
                let d = (direction + d) % 4;
                if !self.has_neighbors(*elf, Some(d)) {
                    match d {
                        0 => if *self.proposed.get(&(x,y-1)).unwrap() == 1 {
                            new_positions.insert((x,y-1))
                        } else {
                            new_positions.insert((x,y))
                        },
                        1 => if *self.proposed.get(&(x,y+1)).unwrap() == 1 {
                            new_positions.insert((x,y+1))
                        } else {
                            new_positions.insert((x,y))
                        },
                        2 => if *self.proposed.get(&(x-1,y)).unwrap() == 1 {
                            new_positions.insert((x-1,y))
                        } else {
                            new_positions.insert((x,y))
                        },
                        3 => if *self.proposed.get(&(x+1,y)).unwrap() == 1 {
                            new_positions.insert((x+1,y))
                        } else {
                            new_positions.insert((x,y))
                        },
                        _ => unimplemented!(),
                    };
                    break;
                }
            }
        }

        self.elves = new_positions;
        self.proposed.clear();
        self.rounds += 1;
    }

    fn count_ground_tiles(&self) -> usize {
        let min_x = self.elves.iter().map(|e| e.0).min().unwrap();
        let min_y = self.elves.iter().map(|e| e.1).min().unwrap();
        let max_x = self.elves.iter().map(|e| e.0).max().unwrap();
        let max_y = self.elves.iter().map(|e| e.1).max().unwrap();
        ((max_x.abs_diff(min_x) + 1) * (max_y.abs_diff(min_y) + 1)) - self.elves.len()
    }
}

fn ground_tiles(input: &[String]) -> usize {
    let mut map = Map::new(input);
    for _ in 0 .. 10 {
        map.next_round();
    }
    map.count_ground_tiles()
}

fn no_movement(input: &[String]) -> usize {
    let mut map = Map::new(input);
    let mut old_positions = HashSet::new();
    while map.elves.difference(&old_positions).count() > 0 {
        old_positions = map.elves.iter().cloned().collect();
        map.next_round();
    }
    map.rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "....#..",
            "..###.#",
            "#...#.#",
            ".#...##",
            "#.###..",
            "##.#.##",
            ".#..#..",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(ground_tiles(&input), 110);
        assert_eq!(no_movement(&input), 20);
    }
}

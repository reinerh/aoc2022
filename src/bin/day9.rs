use std::collections::HashSet;

static DAY: u8 = 9;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", tail_positions(&input, 2));
    println!("{DAY}b: {}", tail_positions(&input, 10));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: &str) -> Direction {
        match input {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

struct Movement {
    direction: Direction,
    distance: isize,
}

impl Movement {
    fn new(input: &str) -> Movement {
        let mut it = input.split_whitespace();
        let direction = Direction::new(it.next().unwrap());
        let distance = it.next().unwrap().parse().unwrap();
        Movement { direction, distance }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn is_adjacent(&self, other: &Position) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx <= 1 && dy <= 1
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow(&mut self, other: &Position) {
        if self.is_adjacent(other) {
            return;
        }

        if self.x == other.x {
            if self.y > other.y {
                self.y -= 1;
            } else {
                self.y += 1;
            }
            return;
        }

        if self.y == other.y {
            if self.x > other.x {
                self.x -= 1;
            } else {
                self.x += 1;
            }
            return;
        }

        if self.x > other.x {
            self.x -= 1;
        } else {
            self.x += 1;
        }
        if self.y > other.y {
            self.y -= 1;
        } else {
            self.y += 1;
        }
    }
}

fn tail_positions(input: &[String], knot_count: usize) -> usize {
    let movements = input.iter()
                         .map(|x| Movement::new(x))
                         .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut knots = vec![Position { x: 0, y: 0 }; knot_count];
    visited.insert(knots[0]);
    for movement in movements {
        for _ in 0 .. movement.distance {
            knots[0].step(&movement.direction);
            for idx in 1 .. knot_count {
                let new_pos = knots[idx-1];
                knots[idx].follow(&new_pos);
            }
            visited.insert(knots[knot_count-1]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "R 4",
            "U 4",
            "L 3",
            "D 1",
            "R 4",
            "D 1",
            "L 5",
            "R 2",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(tail_positions(&input, 2), 13);
        assert_eq!(tail_positions(&input, 10), 1);

        let input = [
            "R 5",
            "U 8",
            "L 8",
            "D 3",
            "R 17",
            "D 10",
            "L 25",
            "U 20",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(tail_positions(&input, 10), 36);
    }
}

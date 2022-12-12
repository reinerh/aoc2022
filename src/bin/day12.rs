use std::collections::HashMap;

static DAY: u8 = 12;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", steps_required_start(&input));
    println!("{DAY}b: {}", steps_required_any_a(&input));
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct Position {
    x: isize,
    y: isize,
}

fn parse_map(input: &[String]) -> (HashMap<Position,u32>, Position, Position) {
    let mut map = HashMap::new();
    let mut start = Position { x: 0, y: 0};
    let mut end = Position { x: 0, y: 0};

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position { x: x as isize, y: y as isize };
            let elevation = match c {
                'S' => { start = pos; 'a' }
                'E' => { end = pos; 'z' }
                e => e,
            };
            let elevation = elevation.to_digit(36).unwrap();
            map.insert(pos, elevation);
        }
    }

    (map, start, end)
}

fn steps_required(map: &HashMap<Position,u32>, end: &Position, found_position: impl Fn(&Position, u32) -> bool) -> u32 {
    let mut distances = HashMap::new();
    distances.insert(*end, 0);

    loop {
        let positions = distances.keys().cloned().collect::<Vec<_>>();
        for pos in positions {
            let next_dist = distances[&pos] + 1;
            let neighbors = [
                Position { x: pos.x + 1, y: pos.y },
                Position { x: pos.x, y: pos.y + 1 },
                Position { x: pos.x - 1, y: pos.y },
                Position { x: pos.x, y: pos.y - 1 },
            ];
            for neighbor in neighbors {
                if distances.contains_key(&neighbor) || !map.contains_key(&neighbor){
                    continue;
                }
                if map[&pos] <= map[&neighbor] || map[&pos] == map[&neighbor] + 1 {
                    distances.insert(neighbor, next_dist);
                    if found_position(&neighbor, map[&neighbor]) {
                        return next_dist;
                    }
                }
            }
        }
    }
}

fn steps_required_start(input: &[String]) -> u32 {
    let (map, start, end) = parse_map(input);
    let found_position = |pos: &Position, _ele: u32| -> bool {
        /* looking for the starting position */
        *pos == start
    };
    steps_required(&map, &end, found_position)
}

fn steps_required_any_a(input: &[String]) -> u32 {
    let (map, _, end) = parse_map(input);
    let found_position = |_pos: &Position, ele: u32| -> bool {
        /* looking for any location with elevation 'a' */
        ele == 'a'.to_digit(36).unwrap()
    };
    steps_required(&map, &end, found_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "Sabqponm",
            "abcryxxl",
            "accszExk",
            "acctuvwj",
            "abdefghi",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(steps_required_start(&input), 31);
        assert_eq!(steps_required_any_a(&input), 29);
    }
}

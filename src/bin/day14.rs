use std::collections::HashMap;

static DAY: u8 = 14;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sand_before_abyss(&input));
    println!("{DAY}b: {}", sand_to_rest(&input));
}

#[derive(Debug)]
enum Material {
    Rock,
    Sand,
}

fn parse_map(input: &[String]) -> HashMap<(isize,isize),Material> {
    let mut map = HashMap::new();

    for line in input {
        let mut path = Vec::new();
        for coord in line.split(" -> ") {
            let (x,y) = coord.split_once(',').expect("coordinate needs to contain ,");
            let x = x.parse::<isize>().expect("x coord should be number");
            let y = y.parse::<isize>().expect("y coord should be number");
            path.push((x,y));
        }
        for pos in path.windows(2) {
            let (x1, x2) = if pos[0].0 < pos[1].0 {
                (pos[0].0, pos[1].0)
            } else {
                (pos[1].0, pos[0].0)
            };

            let (y1, y2) = if pos[0].1 < pos[1].1 {
                (pos[0].1, pos[1].1)
            } else {
                (pos[1].1, pos[0].1)
            };

            for x in x1 ..= x2 {
                for y in y1 ..= y2 {
                    map.insert((x,y), Material::Rock);
                }
            }
        }
    }
    map
}

fn next_sand_pos(map: &mut HashMap<(isize,isize),Material>, pos: (isize,isize)) -> Option<(isize,isize)> {
    let next_positions = [
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
    ];
    next_positions.iter().find(|next_pos| !map.contains_key(next_pos)).copied()
}

fn sand_above_abyss(map: &mut HashMap<(isize,isize),Material>, above: isize) -> bool {
    let mut pos = (500,0);

    while pos.1 <= above {
        match next_sand_pos(map, pos) {
            Some(next_pos) => pos = next_pos,
            None => {
                map.insert(pos, Material::Sand);
                break;
            }
        }
    }

    pos.1 <= above
}

fn sand_at_source(map: &mut HashMap<(isize,isize),Material>, floor: isize) -> bool {
    let mut pos = (500,0);

    loop {
        match next_sand_pos(map, pos) {
            Some(next_pos) => {
                if next_pos.1 == floor {
                    map.insert(pos, Material::Sand);
                    break;
                }
                pos = next_pos;
            },
            None => {
                map.insert(pos, Material::Sand);
                break;
            }
        }
    }

    pos == (500,0)
}

fn sand_before_abyss(input: &[String]) -> usize {
    let mut map = parse_map(input);
    let lowest_rock = *map.keys().max_by_key(|(_,y)| y).expect("map should have a rock");
    let mut sand = 0;
    while sand_above_abyss(&mut map, lowest_rock.1) {
        sand += 1;
    }
    sand
}

fn sand_to_rest(input: &[String]) -> usize {
    let mut map = parse_map(input);
    let floor = map.keys().max_by_key(|(_,y)| y).expect("map should have a rock").1 + 2;
    let mut sand = 0;
    while !sand_at_source(&mut map, floor) {
        sand += 1;
    }
    sand + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(sand_before_abyss(&input), 24);
        assert_eq!(sand_to_rest(&input), 93);
    }
}

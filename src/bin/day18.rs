use std::collections::HashSet;

static DAY: u8 = 18;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", surface_area(&input));
    println!("{DAY}b: {}", exterior_surface_area(&input));
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(input: &str) -> Cube {
        let mut coords = input.split(',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();
        Cube { x, y, z }
    }

    fn unconnected_surfaces(&self, cubes: &[Cube]) -> usize {
        6 - cubes.iter()
                 .filter(|&c| self.has_common_surface(c))
                 .count()
    }

    fn has_common_surface(&self, other: &Cube) -> bool {
        let neighbors = [
            (self.x + 1, self.y, self.z),
            (self.x - 1, self.y, self.z),
            (self.x, self.y + 1, self.z),
            (self.x, self.y - 1, self.z),
            (self.x, self.y, self.z + 1),
            (self.x, self.y, self.z - 1),
        ];
        neighbors.contains(&(other.x, other.y, other.z))
    }
}

fn surface_area(input: &[String]) -> usize {
    let cubes = input.iter()
                     .map(|x| Cube::new(x))
                     .collect::<Vec<_>>();
    cubes.iter()
         .map(|c| c.unconnected_surfaces(&cubes))
         .sum()
}

fn find_outer_air(air: &mut HashSet<Cube>, cubes: &HashSet<Cube>, pos: &Cube, min: &Cube, max: &Cube) {
    if cubes.contains(pos) || air.contains(pos) {
        return;
    }
    air.insert(pos.clone());
    let neighbors = [
        Cube { x: pos.x - 1, y: pos.y, z: pos.z },
        Cube { x: pos.x + 1, y: pos.y, z: pos.z },
        Cube { x: pos.x, y: pos.y + 1, z: pos.z },
        Cube { x: pos.x, y: pos.y - 1, z: pos.z },
        Cube { x: pos.x, y: pos.y, z: pos.z + 1 },
        Cube { x: pos.x, y: pos.y, z: pos.z - 1 },
    ];
    for neigh in neighbors {
        if neigh.x < min.x || neigh.y < min.y || neigh.z < min.z || neigh.x > max.x || neigh.y > max.y || neigh.z > max.z {
            continue;
        }
        find_outer_air(air, cubes, &neigh, min, max);
    }
}

fn exterior_surface_area(input: &[String]) -> usize {
    let cubes = input.iter()
                     .map(|x| Cube::new(x))
                     .collect::<HashSet<_>>();
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let min_cube = Cube { x: min_x - 1, y: min_y - 1, z: min_z - 1 };
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();
    let max_cube = Cube { x: max_x + 1, y: max_y + 1, z: max_z + 1 };

    let mut air = HashSet::new();
    let start = Cube { x: 0, y: 0, z: 0 };
    assert!(!cubes.contains(&start));
    find_outer_air(&mut air, &cubes, &start, &min_cube, &max_cube);

    let mut surfaces = 0;
    for air_cube in air {
        surfaces += cubes.iter()
                         .filter(|c| c.has_common_surface(&air_cube))
                         .count();
    }

    surfaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "2,2,2",
            "1,2,2",
            "3,2,2",
            "2,1,2",
            "2,3,2",
            "2,2,1",
            "2,2,3",
            "2,2,4",
            "2,2,6",
            "1,2,5",
            "3,2,5",
            "2,1,5",
            "2,3,5",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(surface_area(&input), 64);
        assert_eq!(exterior_surface_area(&input), 58);
    }
}

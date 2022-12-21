static DAY: u8 = 18;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", surface_area(&input));
    println!("{DAY}b: {}", 0);
}

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
        let neighbors = [
            (self.x + 1, self.y, self.z),
            (self.x - 1, self.y, self.z),
            (self.x, self.y + 1, self.z),
            (self.x, self.y - 1, self.z),
            (self.x, self.y, self.z + 1),
            (self.x, self.y, self.z - 1),
        ];
        6 - cubes.iter()
             .filter(|c| neighbors.contains(&(c.x, c.y, c.z)))
             .count()
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
    }
}

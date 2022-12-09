use std::collections::HashMap;

static DAY: u8 = 8;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", visible_trees(&input));
    println!("{DAY}b: {}", highest_scenic_score(&input));
}

#[derive(Default, Debug)]
struct NeighborVisibility {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

fn visible_trees(input: &[String]) -> usize {
    let mut map = Vec::new();
    for line in input {
        let heights = line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        map.push(heights);
    }

    let mut visibilities = HashMap::<_,NeighborVisibility>::new();

    /* visibilities from top to bottom */
    for x in 1 .. map[0].len() - 1 {
        let mut current_vis = map[0][x];
        #[allow(clippy::needless_range_loop)]
        for y in 1 .. map.len() - 1 {
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.up = current_vis;
            current_vis = current_vis.max(map[y][x]);
        }
    }
    /* visibilities from bottom to top */
    for x in 1 .. map[0].len() - 1 {
        let mut current_vis = map[map.len()-1][x];
        for y in (1 .. map.len() - 1).rev() {
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.down = current_vis;
            current_vis = current_vis.max(map[y][x]);
        }
    }
    /* visibilities from left to right */
    for y in 1 .. map.len() - 1 {
        let mut current_vis = map[y][0];
        for x in 1 .. map[0].len() - 1 {
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.left = current_vis;
            current_vis = current_vis.max(map[y][x]);
        }
    }
    /* visibilities from right to left */
    for y in 1 .. map.len() - 1 {
        let mut current_vis = map[y][map[0].len()-1];
        for x in (1 .. map[0].len() - 1).rev() {
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.right = current_vis;
            current_vis = current_vis.max(map[y][x]);
        }
    }

    let mut total_visible = 2 * (map.len() - 1) + 2 * (map[0].len() - 1);
    for (pos, visibility) in visibilities {
        let height = map[pos.1][pos.0];
        if height > visibility.up || height > visibility.down || height > visibility.left || height > visibility.right {
            total_visible += 1;
        }
    }
    total_visible
}

fn highest_scenic_score(input: &[String]) -> usize {
    let mut map = Vec::new();
    for line in input {
        let heights = line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        map.push(heights);
    }

    let mut visibilities = HashMap::<_,NeighborVisibility>::new();

    let update_range_vector = |range: &mut [usize; 10], height: usize|{
        /* all heights less than the specified one will start with a
           new visibility of one; for greater heights the distance
           increases by 1. */
        for (i, val) in range.iter_mut().enumerate() {
            if i <= height as usize {
                *val = 1;
            } else {
                *val += 1;
            }
        }
    };

    /* visibilities from top to bottom */
    for x in 0 .. map[0].len() {
        let mut ranges = [0; 10];
        #[allow(clippy::needless_range_loop)]
        for y in 0 .. map.len() {
            let height = map[y][x];
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.up = ranges[height];
            update_range_vector(&mut ranges, height);
        }
    }
    /* visibilities from bottom to top */
    for x in 0 .. map[0].len() {
        let mut ranges = [0; 10];
        for y in (0 .. map.len()).rev() {
            let height = map[y][x];
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.down = ranges[height];
            update_range_vector(&mut ranges, height);
        }
    }
    /* visibilities from left to right */
    for y in 0 .. map.len() {
        let mut ranges = [0; 10];
        for x in 0 .. map[0].len() {
            let height = map[y][x];
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.left = ranges[height];
            update_range_vector(&mut ranges, height);
        }
    }
    /* visibilities from right to left */
    for y in 0 .. map.len() {
        let mut ranges = [0; 10];
        for x in (0 .. map[0].len()).rev() {
            let height = map[y][x];
            let vis_entry = visibilities.entry((x,y)).or_default();
            vis_entry.right = ranges[height];
            update_range_vector(&mut ranges, height);
        }
    }

    visibilities.values()
                .map(|v| v.up * v.down * v.left * v.right)
                .max()
                .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(visible_trees(&input), 21);
        assert_eq!(highest_scenic_score(&input), 8);
    }
}

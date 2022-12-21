static DAY: u8 = 20;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", grove_coordinates(&input));
    println!("{DAY}b: {}", 0);
}

fn grove_coordinates(input: &[String]) -> isize {
    let mut input = input.iter()
                         .map(|x| (x.parse::<isize>().unwrap(), false))
                         .collect::<Vec<_>>();

    loop {
        let pos = match input.iter().position(|x| !x.1) {
            None => break,
            Some(p) => p,
        };
        let val = input[pos];
        input.remove(pos);

        let input_len = input.len() as isize;

        let pos_diff = val.0 % input_len;
        let mut new_pos = (pos as isize + pos_diff) % input_len;
        if new_pos.is_negative() {
            new_pos += input_len;
        }

        if new_pos == 0 {
            input.push((val.0 as isize, true));
        } else {
            input.insert(new_pos as usize, (val.0 as isize, true));
        }
    }

    let pos = input.iter().position(|x| x.0 == 0).unwrap();
    let n = ((pos + 1000) % input.len(), (pos + 2000) % input.len(), (pos + 3000) % input.len());
    input[n.0].0 + input[n.1].0 + input[n.2].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "1",
            "2",
            "-3",
            "3",
            "-2",
            "0",
            "4",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(grove_coordinates(&input), 3);
    }
}

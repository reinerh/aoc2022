static DAY: u8 = 20;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", grove_coordinates(&input, false));
    println!("{DAY}b: {}", grove_coordinates(&input, true));
}

fn grove_coordinates(input: &[String], full_decryption: bool) -> isize {
    let (decryption_key, rounds) = if full_decryption { (811589153, 10) } else { (1, 1) };

    let mut input = input.iter()
                         .enumerate()
                         .map(|(idx,x)| (idx, x.parse::<isize>().unwrap() * decryption_key))
                         .collect::<Vec<_>>();

    for _ in 0 .. rounds {
        for idx in 0 .. input.len() {
            let pos = input.iter().position(|x| x.0 == idx).unwrap();
            let val = input[pos].1;
            input.remove(pos);

            let input_len = input.len() as isize;

            let pos_diff = val % input_len;
            let mut new_pos = (pos as isize + pos_diff) % input_len;
            if new_pos <= 0 {
                new_pos += input_len;
            }

            input.insert(new_pos as usize, (idx, val));
        }
    }

    let pos = input.iter().position(|x| x.1 == 0).unwrap();
    let n = ((pos + 1000) % input.len(), (pos + 2000) % input.len(), (pos + 3000) % input.len());
    input[n.0].1 + input[n.1].1 + input[n.2].1
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

        assert_eq!(grove_coordinates(&input, false), 3);
        assert_eq!(grove_coordinates(&input, true), 1623178306);
    }
}

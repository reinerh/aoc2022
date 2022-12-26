static DAY: u8 = 25;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", console_number(&input));
}

fn snafu_to_number(input: &str) -> i64 {
    let mut number = 0;
    for c in input.chars() {
        number *= 5;
        number += match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unimplemented!(),
        };
    }
    number
}

fn number_to_snafu(input: i64) -> String {
    let mut number = input;

    let mut digits = Vec::new();
    while number > 0 {
        digits.push(number % 5);
        number /= 5;
    }
    digits.push(0);

    for i in 0 .. digits.len() - 1 {
        if digits[i] == 3 {
            digits[i+1] += 1;
            digits[i] = -2;
        } else if digits[i] == 4 {
            digits[i+1] += 1;
            digits[i] = -1;
        } else if digits[i] == 5 {
            digits[i+1] += 1;
            digits[i] = 0;
        }
    }

    let mut snafu = String::new();
    for digit in digits.iter().rev().skip_while(|&x| *x == 0) {
        match digit {
            -1 => snafu.push('-'),
            -2 => snafu.push('='),
            0 => snafu.push('0'),
            1 => snafu.push('1'),
            2 => snafu.push('2'),
            _ => unimplemented!(),
        }
    }
    snafu
}

fn console_number(input: &[String]) -> String {
    let number = input.iter()
                      .map(|x| snafu_to_number(x))
                      .sum();
    number_to_snafu(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(console_number(&input), "2=-1=0");
    }

    #[test]
    fn test_number_to_snafu() {
        assert_eq!(number_to_snafu(1), "1");
        assert_eq!(number_to_snafu(2), "2");
        assert_eq!(number_to_snafu(3), "1=");
        assert_eq!(number_to_snafu(4), "1-");
        assert_eq!(number_to_snafu(5), "10");
        assert_eq!(number_to_snafu(6), "11");
        assert_eq!(number_to_snafu(7), "12");
        assert_eq!(number_to_snafu(8), "2=");
        assert_eq!(number_to_snafu(9), "2-");
        assert_eq!(number_to_snafu(10), "20");
        assert_eq!(number_to_snafu(15), "1=0");
        assert_eq!(number_to_snafu(20), "1-0");
        assert_eq!(number_to_snafu(2022), "1=11-2");
        assert_eq!(number_to_snafu(12345), "1-0---0");
        assert_eq!(number_to_snafu(314159265), "1121-1110-1=0");
    }
}

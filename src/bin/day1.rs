static DAY: u8 = 1;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", most_calories_elf(&input, 1));
    println!("{DAY}b: {}", most_calories_elf(&input, 3));
}

fn most_calories_elf(input: &[String], elf_count: usize) -> u32 {
    let mut input = input.to_vec();
    let mut calories = Vec::new();
    let mut current = 0;
    input.push("".to_string());
    for line in input {
        if line.as_str().is_empty() {
            calories.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<u32>().unwrap();
    }
    calories.sort();
    calories.iter().rev().take(elf_count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        assert_eq!(most_calories_elf(&input, 1), 24000);
        assert_eq!(most_calories_elf(&input, 3), 45000);
    }
}

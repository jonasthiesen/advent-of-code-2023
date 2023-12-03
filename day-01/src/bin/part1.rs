fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let first = line
            .chars()
            .find(|&c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last = line
            .chars()
            .rfind(|&c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();

        sum += first * 10 + last
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, process(input))
    }
}

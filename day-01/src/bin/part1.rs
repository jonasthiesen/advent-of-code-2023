fn main() {
    let input = include_str!("./input.txt");

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

    print!("{}", sum);
}

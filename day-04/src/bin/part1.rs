use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let lines = input.lines();

    let result = lines
        .map(|line| {
            let mut parse = line.split(|c| c == ':' || c == '|');
            let _card = parse.next().expect("should be there");
            let winning_numbers: HashSet<_> = parse
                .next()
                .expect("should be there")
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("should be there"))
                .collect();

            let numbers: HashSet<_> = parse
                .next()
                .expect("should be there")
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("should be there"))
                .collect();

            let intersect = winning_numbers.intersection(&numbers);

            return intersect.fold(0, |acc, _| if acc == 0 { return 1 } else { return acc * 2 });
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, process(input));
    }
}

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let sum = process(input);
    print!("{}", sum);
}

fn process(input: &str) -> u32 {
    let numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let number_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut sum = 0;

    for line in input.lines() {
        let first = find_first_of(line, &numbers).unwrap();
        let last = find_last_of(line, &numbers).unwrap();

        let first_digit = *number_map.get(first).unwrap_or(&first);
        let last_digit = *number_map.get(last).unwrap_or(&last);

        sum += first_digit.parse::<u32>().unwrap() * 10 + last_digit.parse::<u32>().unwrap()
    }

    return sum;
}

fn find_first_of<'a>(haystack: &str, needles: &Vec<&'a str>) -> Option<&'a str> {
    needles
        .iter()
        .filter_map(|&word| haystack.find(word).map(|index| (index, word)))
        .min_by_key(|&(index, _)| index)
        .map(|(_, number)| number)
}

fn find_last_of<'a>(haystack: &str, needles: &Vec<&'a str>) -> Option<&'a str> {
    needles
        .iter()
        .filter_map(|&word| haystack.rfind(word).map(|index| (index, word)))
        .max_by_key(|&(index, _)| index)
        .map(|(_, number)| number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, process(input))
    }
}

use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let lines = input.lines();

    let lookup: BTreeMap<u32, Card> = lines
        .map(|line| {
            let mut parse = line.split(|c| c == ':' || c == '|');

            let card_id = parse
                .next()
                .expect("should be there")
                .split_whitespace()
                .last()
                .expect("should be there")
                .parse::<u32>()
                .expect("should work");

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

            (card_id, Card::new(winning_numbers, numbers))
        })
        .collect();

    count_cards(lookup)
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn new(winning_numbers: HashSet<u32>, numbers: HashSet<u32>) -> Card {
        Card {
            winning_numbers,
            numbers,
        }
    }

    fn score(&self) -> u32 {
        self.winning_numbers.intersection(&self.numbers).count() as u32
    }
}

fn count_cards(lookup: BTreeMap<u32, Card>) -> u32 {
    let mut memo: HashMap<u32, u32> = HashMap::new();

    lookup.iter().rev().for_each(|(id, card)| {
        let length = card.score();

        let child_lengths: u32 = ((id + 1)..(id + 1 + length))
            .map(|id| memo.get(&id).unwrap_or(&0))
            .sum();

        memo.insert(*id, child_lengths + length);
    });

    let sum: u32 = memo.values().sum();

    // We need to add the initial scratch cards
    sum + memo.len() as u32
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

        assert_eq!(30, process(input));
    }
}

use std::cmp;

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let lines = input.lines();

    let max_cubes = Cubes::new(12, 13, 14);

    let result: u32 = lines
        .filter_map(|line| {
            let (game, cubes) = get_game_and_cubes_from_str(line);
            if let Some(combined_cubes) = cubes.into_iter().reduce(|acc, cur| acc.merge(cur)) {
                if combined_cubes.less_than_or_equal(max_cubes.clone()) {
                    return Some(game);
                }
            }

            None
        })
        .sum();

    result
}

#[derive(Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Cubes {
        Cubes { red, green, blue }
    }

    fn less_than_or_equal(&self, other_cubes: Cubes) -> bool {
        self.red <= other_cubes.red
            && self.green <= other_cubes.green
            && self.blue <= other_cubes.blue
    }

    fn new_of_color(color: &str, amount: u32) -> Cubes {
        if color == "red" {
            return Cubes::new(amount, 0, 0);
        } else if color == "green" {
            return Cubes::new(0, amount, 0);
        } else if color == "blue" {
            return Cubes::new(0, 0, amount);
        } else {
            panic!("oh shit");
        }
    }

    fn merge(&self, other_cubes: Cubes) -> Cubes {
        Cubes {
            red: cmp::max(self.red, other_cubes.red),
            green: cmp::max(self.green, other_cubes.green),
            blue: cmp::max(self.blue, other_cubes.blue),
        }
    }

    // "5 red, 2 green, 0 blue"
    fn from_str(str: &str) -> Cubes {
        let chunks = str.split(",").map(|c| c.trim());

        let cubes = chunks
            .map(|chunk| {
                let mut c = chunk.split_whitespace();

                let amount = c.next().unwrap();
                let color = c.next().unwrap();

                Cubes::new_of_color(color, amount.parse::<u32>().unwrap())
            })
            .reduce(|acc, cur| acc.merge(cur));

        cubes.unwrap()
    }
}

fn get_game_and_cubes_from_str(str: &str) -> (u32, Vec<Cubes>) {
    let game = str
        .split_whitespace()
        .nth(1)
        .unwrap()
        .replace(":", "")
        .parse::<u32>()
        .unwrap();

    let cubes = str
        .split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(|chunk| Cubes::from_str(chunk))
        .collect();

    (game, cubes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, process(input));
    }
}

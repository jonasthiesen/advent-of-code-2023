use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let gear = '*';

    let mut marked_points: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut number_positions: Vec<NumberPosition> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_value = String::new();
        let mut current_points: Vec<Point> = Vec::new();

        for (x, char) in line.char_indices() {
            let x = x as u32;
            let y = y as u32;

            // Build up the number and the locations it exists in
            if char.is_numeric() {
                current_value.push(char);
                current_points.push(Point::new(x, y));
            }
            if !char.is_numeric() || x == (line.len() - 1) as u32 {
                if !current_value.is_empty() {
                    number_positions.push(NumberPosition {
                        value: current_value
                            .parse::<u32>()
                            .expect("current_value should be a number"),
                        points: current_points.clone(),
                    });
                }
                current_value.clear();
                current_points.clear();
            }

            // We need to mark the locations surrounding the symbol
            // so we can figure out which numbers are "part numbers"
            if gear == char {
                marked_points.insert(
                    Point::new(x, y),
                    vec![
                        Point::new(x - 1, y - 1),
                        Point::new(x, y - 1),
                        Point::new(x + 1, y - 1),
                        Point::new(x - 1, y),
                        Point::new(x + 1, y),
                        Point::new(x - 1, y + 1),
                        Point::new(x, y + 1),
                        Point::new(x + 1, y + 1),
                    ],
                );
            }
        }
    }

    let gear_ratios = marked_points.iter().filter_map(|mp| {
        let matches: Vec<_> = number_positions
            .iter()
            .filter_map(|np| {
                let mp_match = mp.1.iter().any(|x| {
                    //
                    return np.points.contains(x);
                });

                if mp_match {
                    return Some(np.value);
                }

                None
            })
            .collect();

        if matches.len() == 2 {
            return Some(matches.iter().fold(1, |acc, cur| acc * cur));
        }

        None
    });

    gear_ratios.sum()
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct NumberPosition {
    value: u32,
    points: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(467835, process(input));
    }
}

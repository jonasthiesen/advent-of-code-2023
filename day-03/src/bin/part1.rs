use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    let symbols: HashSet<_> = input
        .chars()
        .filter(|&char| !char.is_numeric() && char != '.' && !char.is_whitespace())
        .collect();

    let mut marked_points: HashSet<Point> = HashSet::new();
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
            if symbols.contains(&char) {
                marked_points.insert(Point::new(x - 1, y - 1));
                marked_points.insert(Point::new(x, y - 1));
                marked_points.insert(Point::new(x + 1, y - 1));

                marked_points.insert(Point::new(x - 1, y));
                marked_points.insert(Point::new(x + 1, y));

                marked_points.insert(Point::new(x - 1, y + 1));
                marked_points.insert(Point::new(x, y + 1));
                marked_points.insert(Point::new(x + 1, y + 1));
            }
        }
    }

    let parts = number_positions.iter().filter_map(|np| {
        let is_marked = np.points.iter().any(|point| marked_points.contains(point));

        if is_marked {
            return Some(np.value);
        }

        None
    });

    parts.sum()
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

        assert_eq!(4361, process(input));
    }
}

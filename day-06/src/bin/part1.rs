use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, multispace0, space1},
    combinator::map_res,
    multi::{many_till, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u32 {
    if let Some((times, distances)) = parse_times_and_distances(input) {
        let times_and_distances = times.iter().zip(distances.iter());

        let mut sums: Vec<u32> = Vec::new();
        for (time, distance) in times_and_distances {
            let mut sum = 0;
            for t in 1..*time {
                let t_distance = t * (time - t);

                if t_distance > *distance {
                    sum += 1;
                }
            }
            sums.push(sum)
        }

        sums.iter().fold(1, |acc, cur| acc * cur)
    } else {
        panic!("Fuck")
    }
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        space1,
        map_res(digit1, |str_num: &str| str_num.parse::<u32>()),
    )(input)
}

fn parse_line_label<'a>(label: &str, input: &'a str) -> IResult<&'a str, Vec<u32>> {
    preceded(
        terminated(many_till(anychar, tag(label)), multispace0),
        parse_number_list,
    )(input)
}

fn parse_times_and_distances(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let time = parse_line_label("Time:", input);
    let distance = parse_line_label("Distance:", input);

    match (time, distance) {
        (Ok((_, t)), Ok((_, d))) => Some((t, d)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_list() {
        let input1 = "7  15   30";
        let input2 = "7 15 30";

        assert_eq!(vec![7, 15, 30], parse_number_list(input1).unwrap().1);
        assert_eq!(vec![7, 15, 30], parse_number_list(input2).unwrap().1);
    }

    #[test]
    fn test_parse_line_label() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(vec![7, 15, 30], parse_line_label("Time:", input).unwrap().1);
        assert_eq!(
            vec![9, 40, 200],
            parse_line_label("Distance:", input).unwrap().1
        );
    }

    #[test]
    fn test_parse_times_and_distances() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(
            Some((vec![7, 15, 30], vec![9, 40, 200])),
            parse_times_and_distances(input)
        );
    }

    #[test]
    fn test_exercise() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, process(input));
    }
}

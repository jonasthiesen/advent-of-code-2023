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

fn process(input: &str) -> u64 {
    if let Some((time, distance)) = parse_time_and_distance(input) {
        let mut sum = 0;
        for t in 1..time {
            let t_distance = t * (time - t);

            if t_distance > distance {
                sum += 1;
            }
        }
        sum
    } else {
        panic!("Fuck")
    }
}

fn parse_bad_kerning_number(input: &str) -> IResult<&str, u64> {
    map_res(separated_list1(space1, digit1), |vec: Vec<&str>| {
        vec.join("").parse::<u64>()
    })(input)
}

fn parse_line_label<'a>(label: &str, input: &'a str) -> IResult<&'a str, u64> {
    preceded(
        terminated(many_till(anychar, tag(label)), multispace0),
        parse_bad_kerning_number,
    )(input)
}

fn parse_time_and_distance(input: &str) -> Option<(u64, u64)> {
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
    fn test_parse_bad_kerning_number() {
        let input1 = "7  15   30";
        let input2 = "7 15 30";

        assert_eq!(71530, parse_bad_kerning_number(input1).unwrap().1);
        assert_eq!(71530, parse_bad_kerning_number(input2).unwrap().1);
    }

    #[test]
    fn test_parse_line_label() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71530, parse_line_label("Time:", input).unwrap().1);
        assert_eq!(940200, parse_line_label("Distance:", input).unwrap().1);
    }

    #[test]
    fn test_parse_time_and_distance() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(Some((71530, 940200)), parse_time_and_distance(input));
    }

    #[test]
    fn test_exercise() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, process(input));
    }
}

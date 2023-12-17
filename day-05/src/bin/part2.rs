use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");

    let result = process(input);

    println!("{result}");
}

fn process(input: &str) -> u64 {
    let mut groups = input.split("\n\n");

    let (_, seeds_raw) = groups.next().map(parse_seeds).unwrap().unwrap();
    let mappings = groups.map(parse_mapping).collect::<Vec<_>>();

    let mut seeds: Vec<u64> = Vec::new();

    seeds_raw
        .chunks(2)
        .filter_map(|chunk| match chunk {
            [a, b] => Some((*a, *b)),
            _ => None,
        })
        .for_each(|(seed_start, seed_range)| {
            for seed in seed_start..seed_start + seed_range {
                seeds.push(seed);
            }
        });

    let mut best_result: Option<u64> = None;

    seeds.iter().enumerate().for_each(|(index, seed)| {
        if index % 10_000_000 == 0 {
            println!("{}", (index as f64) / (seeds.len() as f64) * 100.0);
        }

        let mut value = *seed;
        for mapping in mappings.iter() {
            if let Ok((_, vec)) = mapping {
                for range in vec.iter() {
                    if value >= range.0 && value < range.0 + range.2 {
                        value = range.1 + value - range.0;
                        break;
                    }
                }
            } else {
                panic!("Fuck")
            }
        }

        if best_result.is_none() || value < best_result.unwrap() {
            best_result = Some(value);
        }
    });

    best_result.unwrap()
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(preceded(tag("seeds:"), space0), parse_numbers)(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    let mut parser = preceded(
        preceded(not_line_ending, line_ending),
        separated_list0(
            line_ending,
            map_res(parse_numbers, |line| match line.as_slice() {
                [dest, source, range] => Ok((*source, *dest, *range)),
                _ => Err("Failed"),
            }),
        ),
    );

    let (input, parsed) = parser(input)?;

    Ok((input, parsed))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(
        space1,
        map_res(digit1, |digit_str: &str| digit_str.parse::<u64>()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seeds_test() {
        let input = "seeds: 79 14 55 13";

        match parse_seeds(input) {
            Ok((_, seeds)) => assert_eq!(vec![79, 14, 55, 13], seeds),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_mapping_test() {
        let input = "soil-to-fertilizer map:
0 15 2
37 52 2
39 0 1";

        let mut vec: Vec<(u64, u64, u64)> = Vec::new();
        vec.push((15, 0, 2));
        vec.push((52, 37, 2));
        vec.push((0, 39, 1));

        assert_eq!(Ok(("", vec)), parse_mapping(input));
    }

    #[test]
    fn test_exercise() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(46, process(input));
    }
}

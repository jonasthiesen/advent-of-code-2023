use std::collections::HashMap;

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

    let (_, seeds) = groups.next().map(parse_seeds).unwrap().unwrap();
    let mappings = groups.map(parse_mapping).collect::<Vec<_>>();

    let results = seeds.iter().map(|seed| {
        let done = mappings.iter().fold(seed, |acc, cur| {
            cur.as_ref().unwrap().1.get(&acc).or(Some(&acc)).unwrap()
        });

        done
    });

    *results.min().unwrap()
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(preceded(tag("seeds:"), space0), parse_numbers)(input)
}

fn parse_mapping(input: &str) -> IResult<&str, HashMap<u64, u64>> {
    let mut parser = preceded(
        preceded(not_line_ending, line_ending),
        separated_list0(
            line_ending,
            map_res(parse_numbers, |line| match line.as_slice() {
                [dest, source, range] => {
                    Ok::<HashMap<u64, u64>, &str>(range_map(*source, *dest, *range))
                }
                _ => Err("Failed"),
            }),
        ),
    );

    let (input, parsed) = parser(input)?;

    let mut result: HashMap<u64, u64> = HashMap::new();
    for line in parsed.iter() {
        result.extend(line)
    }

    let result = parsed.iter().fold(HashMap::new(), |mut acc, cur| {
        acc.extend(cur);
        acc
    });

    Ok((input, result))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(
        space1,
        map_res(digit1, |digit_str: &str| digit_str.parse::<u64>()),
    )(input)
}

fn range_map(source: u64, destination: u64, range: u64) -> HashMap<u64, u64> {
    (source..(source + range))
        .zip(destination..(destination + range))
        .collect::<HashMap<u64, u64>>()
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

        let mut map: HashMap<u64, u64> = HashMap::new();
        map.insert(15, 0);
        map.insert(16, 1);
        map.insert(52, 37);
        map.insert(53, 38);
        map.insert(0, 39);

        assert_eq!(Ok(("", map)), parse_mapping(input));
    }

    #[test]
    fn range_map_test() {
        let mut result1 = HashMap::new();
        result1.insert(10, 20);
        result1.insert(11, 21);
        result1.insert(12, 22);

        let mut result2 = HashMap::new();
        result2.insert(5, 10);

        assert_eq!(result1, range_map(10, 20, 3));
        assert_eq!(result2, range_map(5, 10, 1));
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

        assert_eq!(35, process(input));
    }
}

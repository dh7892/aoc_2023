advent_of_code::solution!(6);

use chumsky::prelude::*;

fn integer_parser() -> impl Parser<char, u64, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<u64>().unwrap())
}

fn integer_list_parser() -> impl Parser<char, Vec<u64>, Error = Simple<char>> {
    integer_parser().separated_by(text::whitespace())
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times = just("Time:")
        .ignore_then(text::whitespace())
        .ignore_then(integer_list_parser())
        .parse(lines[0])
        .unwrap();

    let distances = just("Distance:")
        .ignore_then(text::whitespace())
        .ignore_then(integer_list_parser())
        .parse(lines[1])
        .unwrap();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>()
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn permutations_that_beat_distance(&self) -> u64 {
        // Use quadratic formula to find the range of values that
        // Our distance as a function of how long we hold the button is:
        // d = h(t - h)
        // Which gives a quadratic equation of the form:
        // 0 = h^2 - ht + d
        // The "root" part here is the sqrt(b^2 - 4ac)
        let root = ((self.time * self.time - 4 * self.distance) as f64).sqrt();

        // Round to the nearest integer that's strictly better.
        // hence using floor + 1 instead of ceil and this works properly
        // in the case where the root is an integer.
        let lower = ((self.time as f64 - root) / 2.0).floor() as u64 + 1;
        let upper = ((self.time as f64 + root) / 2.0).ceil() as u64 - 1;

        upper - lower + 1
    }
}

fn _distance_travelled(time: u64, held: u64) -> u64 {
    held * (time - held)
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_races(input);
    let score = races
        .into_iter()
        .map(|race| race.permutations_that_beat_distance())
        .product::<u64>();
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let time = just("Time:")
        .ignore_then(integer_parser())
        .parse(
            lines[0]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
        )
        .unwrap();

    let distance = just("Distance:")
        .ignore_then(integer_parser())
        .parse(
            lines[1]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
        )
        .unwrap();

    let race = Race { time, distance };
    Some(race.permutations_that_beat_distance() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[rstest::rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_permutations_that_beat_distance(
        #[case] time: u64,
        #[case] distance: u64,
        #[case] expected: u64,
    ) {
        let race = Race { time, distance };
        assert_eq!(race.permutations_that_beat_distance(), expected);
    }
}

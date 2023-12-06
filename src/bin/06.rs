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
        (1..self.time)
            .map(|held| distance_travelled(self.time, held))
            .filter(|distance| distance > &self.distance)
            .count() as u64
    }
}

fn distance_travelled(time: u64, held: u64) -> u64 {
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
    let stripped_input = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let lines = &stripped_input.lines().collect::<Vec<&str>>();
    let time = just("Time:")
        .ignore_then(text::whitespace())
        .ignore_then(integer_parser())
        .parse(lines[0])
        .unwrap();

    let distance = just("Distance:")
        .ignore_then(text::whitespace())
        .ignore_then(integer_parser())
        .parse(lines[1])
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
        let race = Race {
            time: 71530,
            distance: 940200,
        };
        let result = Some(race.permutations_that_beat_distance() as u32);
        assert_eq!(result, Some(71503));
    }
}

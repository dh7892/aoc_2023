advent_of_code::solution!(8);

extern crate num_integer;
use num_integer::Integer;

use std::collections::HashMap;

use chumsky::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}

// This struct holds the possible connections from a location to up to two other locations
struct Connection {
    left: Option<String>,
    right: Option<String>,
}

impl Connection {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

struct LocationMap {
    locations: HashMap<String, Connection>,
    current_locations: Vec<String>,
}

impl LocationMap {
    fn new(starts: Vec<String>) -> Self {
        Self {
            locations: HashMap::new(),
            current_locations: starts,
        }
    }

    fn add_location(&mut self, id: String) {
        self.locations.insert(id, Connection::new());
    }

    fn add_connection(&mut self, from: String, direction: Direction, to: String) {
        let connection = self.locations.get_mut(&from).unwrap();
        match direction {
            Direction::L => connection.left = Some(to),
            Direction::R => connection.right = Some(to),
        }
    }
    fn move_to(&mut self, direction: Direction) {
        let mut new_locations = Vec::new();
        for location in self.current_locations.iter() {
            let connection = self.locations.get(location).unwrap();
            match direction {
                Direction::L => {
                    new_locations.push(connection.left.clone().unwrap());
                }
                Direction::R => {
                    new_locations.push(connection.right.clone().unwrap());
                }
            }
        }
        self.current_locations = new_locations;
    }
    fn locations_ending_with_char(&self, c: char) -> Vec<String> {
        self.locations
            .keys()
            .filter(|id| id.ends_with(c))
            .cloned()
            .collect::<Vec<String>>()
    }
}

fn parse_map(input: &[&str]) -> LocationMap {
    let location_id_parser = filter(|c: &char| c.is_alphanumeric())
        .repeated()
        .exactly(3)
        .collect::<String>();

    let connection_parser = location_id_parser
        .then_ignore(just::<char, &str, Simple<char>>(" = ("))
        .then(location_id_parser)
        .then_ignore(just(", "))
        .then(location_id_parser)
        .then_ignore(just(")"))
        .map(|((from, left), right)| (from, left, right));

    let raw_locations = input
        .iter()
        .map(|line| connection_parser.parse(*line).unwrap())
        .collect::<Vec<(String, String, String)>>();

    // Loop over the raw locations and build up a map of locations to connections
    // Start by creating all the locations
    let mut location_map = LocationMap::new(vec!["AAA".to_string()]);
    for (from, _, _) in raw_locations.iter() {
        location_map.add_location(from.clone());
    }

    // Now loop again and put in the connections
    for (from, left, right) in raw_locations.into_iter() {
        location_map.add_connection(from.clone(), Direction::L, left);
        location_map.add_connection(from.clone(), Direction::R, right);
    }

    location_map
}

// Given a starting location, a map and a set of directions, find how many steps until we reach a finish
fn cycle_for_start(start: &str, location_map: &LocationMap, directions: &[Direction]) -> u32 {
    let mut step = 0;
    let mut location = start;
    for direction in directions.iter().cycle() {
        location = match direction {
            Direction::L => location_map
                .locations
                .get(location)
                .unwrap()
                .left
                .as_ref()
                .unwrap(),
            Direction::R => location_map
                .locations
                .get(location)
                .unwrap()
                .right
                .as_ref()
                .unwrap(),
        };
        step += 1;
        if location.ends_with('Z') {
            break;
        }
    }
    step
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let directions = lines[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => unreachable!(),
        })
        .collect::<Vec<Direction>>();
    let mut location_map = parse_map(&lines[2..]);
    let mut steps = 0;

    for direction in directions.into_iter().cycle() {
        location_map.move_to(direction);
        steps += 1;
        if location_map.current_locations[0] == "ZZZ" {
            break;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let directions = lines[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => unreachable!(),
        })
        .collect::<Vec<Direction>>();
    let mut location_map = parse_map(&lines[2..]);

    // Unlike in part 1, we need to set up our starting locations to be all locations
    // that end with 'A'
    location_map.current_locations = location_map.locations_ending_with_char('A');

    // Testing revealed that, despite the complexity of the map and directions,
    // each starting location yields a constant number of steps to reach an end
    // and continuing to repeat will not change the number of steps to get back to the end
    // again.
    // This makes our problem solvable as we can just find the period for each starting location
    // to get to an end and then we have to find the lowest common multiple of those to get our
    // solution.
    // We make use of the num_integer crate to get the lcm function.
    let steps: u64 = location_map
        .current_locations
        .iter()
        .map(|location| cycle_for_start(location, &location_map, &directions) as u64)
        .fold(1, |acc, x| acc.lcm(&x));

    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_one_other() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let result = part_two(input);
        assert_eq!(result, Some(6));
    }
}

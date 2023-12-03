advent_of_code::solution!(2);

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};

#[derive(Debug, Default, PartialEq)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn add_cubes(&mut self, cubes: Cubes) {
        match cubes {
            Cubes::Red(num) => self.red += num,
            Cubes::Green(num) => self.green += num,
            Cubes::Blue(num) => self.blue += num,
        }
    }
}

enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn red(input: &str) -> nom::IResult<&str, Cubes> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(" red")(input)?;
    Ok((input, Cubes::Red(num.parse().unwrap())))
}
fn green(input: &str) -> nom::IResult<&str, Cubes> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(" green")(input)?;
    Ok((input, Cubes::Green(num.parse().unwrap())))
}
fn blue(input: &str) -> nom::IResult<&str, Cubes> {
    let (input, num) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(" blue")(input)?;
    Ok((input, Cubes::Blue(num.parse().unwrap())))
}

fn read_hand(input: &str) -> nom::IResult<&str, Hand> {
    let mut hand = Hand::default();
    let (input, cubes) = separated_list1(tag(", "), alt((red, green, blue)))(input)?;
    let (input, _) = alt((tag("; "), line_ending))(input)?;

    for cube in cubes {
        hand.add_cubes(cube);
    }

    Ok((input, hand))
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn game_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for hand in &self.hands {
            if hand.red > red {
                return false;
            }
            if hand.green > green {
                return false;
            }
            if hand.blue > blue {
                return false;
            }
        }

        true
    }
    fn min_possible_power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for hand in &self.hands {
            if hand.red > red {
                red = hand.red;
            }
            if hand.green > green {
                green = hand.green;
            }
            if hand.blue > blue {
                blue = hand.blue;
            }
        }

        red * green * blue
    }
}

fn read_game(input: &str) -> nom::IResult<&str, Game> {
    let (input, _) = nom::bytes::complete::tag("Game ")(input)?;
    let (input, id) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;
    let (input, hands) = nom::multi::many1(read_hand)(input)?;

    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            hands,
        },
    ))
}
pub fn part_one(input: &str) -> Option<u32> {
    let games = nom::multi::many1(read_game)(input).unwrap().1;
    let valid_id_sum = games
        .iter()
        .filter(|game| game.game_possible(12, 13, 14))
        .map(|game| game.id)
        .sum::<u32>();
    Some(valid_id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = nom::multi::many1(read_game)(input).unwrap().1;
    let minimum_powers = games
        .iter()
        .map(|game| game.min_possible_power())
        .sum::<u32>();
    Some(minimum_powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_read_hand() {
        let result = read_hand("1 red, 2 green, 3 blue\n");
        assert_eq!(
            result,
            Ok((
                "",
                Hand {
                    red: 1,
                    green: 2,
                    blue: 3
                }
            ))
        );
    }

    #[test]
    fn test_minimum_power() {
        let game = Game {
            id: 1,
            hands: vec![
                Hand {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        };
        let result = game.min_possible_power();
        assert_eq!(result, 36);
    }
}

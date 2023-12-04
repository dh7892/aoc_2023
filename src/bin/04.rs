advent_of_code::solution!(4);

use chumsky::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn integer_parser() -> impl Parser<char, u32, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<u32>().unwrap())
}

fn integer_list_parser() -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
    integer_parser().separated_by(text::whitespace())
}

fn line_to_card_parser() -> impl Parser<char, Card, Error = Simple<char>> {
    just("Card")
        .ignore_then(text::whitespace())
        .ignore_then(integer_parser())
        .then_ignore(just(':'))
        .then_ignore(text::whitespace())
        .then(integer_list_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just('|'))
        .then_ignore(text::whitespace())
        .then(integer_list_parser())
        .map(|((card_number, first_list), second_list)| Card {
            id: card_number,
            winning_numbers: HashSet::from_iter(first_list),
            numbers: HashSet::from_iter(second_list),
        })
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

fn cards_from_input(input: &str) -> nom::IResult<&str, Vec<Card>> {
    let parser = line_to_card_parser();
    let cards = input
        .lines()
        .map(|line| parser.parse(line).unwrap())
        .collect();
    Ok(("", cards))
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = cards_from_input(input).unwrap().1;
    let result = cards
        .iter()
        .map(|card| {
            let numbers_i_have = card.winning_numbers.intersection(&card.numbers);
            let base: u32 = 2;
            let c = numbers_i_have.count() as u32;
            if c > 0 {
                base.pow(c - 1)
            } else {
                0
            }
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = cards_from_input(input).unwrap().1;
    let num_cards = cards.len();
    let mut card_counts: HashMap<u32, u32> =
        HashMap::from_iter(cards.iter().map(|card| (card.id, 1)));
    for id in 1..=num_cards as u32 {
        let card = cards.get(id as usize - 1).unwrap();
        let count = *card_counts.get(&id).unwrap_or(&0);
        let matching = card.winning_numbers.intersection(&card.numbers).count();
        for i in 0..matching {
            let index = id + i as u32 + 1;
            let old_value = card_counts.get(&index).unwrap_or(&0);
            card_counts.insert(index, old_value + count);
        }
    }
    let result = card_counts.values().sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_parse_line_to_card() {
        let result =
            line_to_card_parser().parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        if let Ok(card) = result {
            assert_eq!(
                card,
                Card {
                    id: 1,
                    winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
                    numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53])
                }
            );
        } else {
            panic!("Failed to parse card");
        }
    }

    #[test]
    fn test_card_parse() {
        let parser = just("Card ")
            .ignore_then(integer_parser())
            .then_ignore(just(":"))
            .then_ignore(text::whitespace())
            .then(integer_list_parser())
            .then_ignore(text::whitespace())
            .then_ignore(just("|"))
            .then_ignore(text::whitespace())
            .then(integer_list_parser())
            .map(|((card_number, first_list), second_list)| (card_number, first_list, second_list));
        let result = parser.parse("Card 1:  1  2 3 | 5 6 7");
        assert_eq!(result, Ok((1, vec![1, 2, 3], vec![5, 6, 7])));
    }

    #[rstest::rstest]
    #[case("Card 1:  1  2 3 | 5 6 7")]
    #[case( "Card   4:  2  4 14 88 19 99  3 84  1 77 | 61 59 53 88 31 72 62 11  9 50 46 42 66 79 55 87 30 63 37 69 95 83 15 21 35")]
    fn test_can_parse_line(#[case] line: &str) {
        let result = line_to_card_parser().parse(line);
        assert!(result.is_ok());
    }
}

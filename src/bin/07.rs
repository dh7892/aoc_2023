advent_of_code::solution!(7);

// Since the code structures are similar but the logic is different between part 1 and part 2
// I've split the code into two modules so I can define some of the types differently
// There is a lot of duplication though, so there's probably a cleaner way to handle this
mod part_one {
    use std::collections::HashMap;
    #[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy, Ord)]
    pub enum Card {
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        CT,
        CJ,
        CQ,
        CK,
        CA,
    }

    impl Card {
        pub fn from_char(c: char) -> Self {
            match c {
                '2' => Card::C2,
                '3' => Card::C3,
                '4' => Card::C4,
                '5' => Card::C5,
                '6' => Card::C6,
                '7' => Card::C7,
                '8' => Card::C8,
                '9' => Card::C9,
                'T' => Card::CT,
                'J' => Card::CJ,
                'Q' => Card::CQ,
                'K' => Card::CK,
                'A' => Card::CA,
                _ => panic!("Unknown Card"),
            }
        }
    }

    pub fn card_counts(card_list: Vec<Card>) -> HashMap<Card, u8> {
        let mut result = HashMap::new();
        for c in card_list {
            match result.get(&c) {
                Some(count) => result.insert(c, count + 1),
                None => result.insert(c, 1),
            };
        }
        result
    }

    pub fn cards_from_str(input: &str) -> Vec<Card> {
        input.chars().map(|c| Card::from_char(c)).collect()
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl HandType {
        fn from_cards(cards: Vec<Card>) -> Self {
            let counts = card_counts(cards);
            let mut counts: Vec<(Card, u8)> = counts.into_iter().collect();
            counts.sort_by(|a, b| b.1.cmp(&a.1));
            let (card, count) = &counts[0];
            match count {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => {
                    if counts.len() == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                2 => {
                    if counts.len() == 3 {
                        HandType::TwoPair
                    } else {
                        HandType::Pair
                    }
                }
                1 => HandType::HighCard,
                _ => panic!("Unknown Hand"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Hand {
        hand_type: HandType,
        cards: Vec<Card>,
    }

    impl Hand {
        pub fn from_word(word: &str) -> Self {
            let cards = cards_from_str(word);
            let hand_type = HandType::from_cards(cards.clone());
            Hand { cards, hand_type }
        }
        fn from_cards(cards: Vec<Card>) -> Self {
            let hand_type = HandType::from_cards(cards.clone());
            Hand { cards, hand_type }
        }
    }
}

mod part_two {
    use std::collections::HashMap;
    #[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy, Ord)]
    pub enum Card {
        CJ,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        CT,
        CQ,
        CK,
        CA,
    }

    impl Card {
        pub fn from_char(c: char) -> Self {
            match c {
                'J' => Card::CJ,
                '2' => Card::C2,
                '3' => Card::C3,
                '4' => Card::C4,
                '5' => Card::C5,
                '6' => Card::C6,
                '7' => Card::C7,
                '8' => Card::C8,
                '9' => Card::C9,
                'T' => Card::CT,
                'Q' => Card::CQ,
                'K' => Card::CK,
                'A' => Card::CA,
                _ => panic!("Unknown Card"),
            }
        }
    }

    pub fn card_counts(card_list: Vec<Card>) -> HashMap<Card, u8> {
        let mut result = HashMap::new();
        for c in card_list {
            match result.get(&c) {
                Some(count) => result.insert(c, count + 1),
                None => result.insert(c, 1),
            };
        }
        result
    }

    pub fn cards_from_str(input: &str) -> Vec<Card> {
        input.chars().map(Card::from_char).collect()
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl HandType {
        fn from_cards(cards: Vec<Card>) -> Self {
            // J is wild, so we need to account for this when we work out what
            // the hand type is
            // We do this by simply considering number of jokers alongside the numbers of each card

            let counts = card_counts(cards.clone());
            let num_jokers = *counts.get(&Card::CJ).unwrap_or(&0);
            let mut counts: Vec<(Card, u8)> = counts.into_iter().collect();
            counts.sort_by(|a, b| b.1.cmp(&a.1));
            let (_, count) = &counts[0];
            match count {
                5 => HandType::FiveOfAKind,
                4 => {
                    if num_jokers == 1 || num_jokers == 4 {
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                }
                3 => {
                    if num_jokers == 0 {
                        if counts.len() == 2 {
                            HandType::FullHouse
                        } else {
                            HandType::ThreeOfAKind
                        }
                    } else if num_jokers == 1 {
                        HandType::FourOfAKind
                    } else if num_jokers == 2 {
                        HandType::FiveOfAKind
                    } else {
                        // Jokers must be 3, which must be the 3 of a kind we have here
                        // We can combine that with either 2 or 1 other card to make four or five of a kind
                        match counts.len() {
                            2 => HandType::FiveOfAKind,
                            3 => HandType::FourOfAKind,
                            _ => panic!("Unknown Hand"),
                        }
                    }
                }
                2 => {
                    if num_jokers == 0 {
                        if counts.len() == 3 {
                            HandType::TwoPair
                        } else {
                            HandType::Pair
                        }
                    } else if num_jokers == 1 {
                        if counts.len() == 3 {
                            // Two pair with a joker is a full house
                            HandType::FullHouse
                        } else {
                            HandType::ThreeOfAKind
                        }
                    } else if num_jokers == 2 {
                        if counts.len() == 3 {
                            // This means we have a pair (since the only way we can have 3 counts is with 2,2,1) and a pair of jokers
                            // So we can get 4 of a kind with that
                            HandType::FourOfAKind
                        } else {
                            // This means we have a pair of jokers, so that's 3 of a kind when combined with the other card
                            HandType::ThreeOfAKind
                        }
                    } else {
                        HandType::Pair
                    }
                }
                1 => {
                    if num_jokers == 1 {
                        HandType::Pair
                    } else {
                        HandType::HighCard
                    }
                }
                _ => panic!("Unknown Hand"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Hand {
        pub hand_type: HandType,
        cards: Vec<Card>,
    }

    impl Hand {
        pub fn from_word(word: &str) -> Self {
            let cards = cards_from_str(word);
            Hand::from_cards(cards.clone())
        }
        fn from_cards(cards: Vec<Card>) -> Self {
            let hand_type = HandType::from_cards(cards.clone());
            Hand { cards, hand_type }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    use part_one::Hand;
    // split each line into two words (on whitespace) and collect into a vector
    let mut hands = input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            (Hand::from_word(words[0]), words[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<(Hand, u32)>>();

    // Sort the hands by the first element (the hand) and then by the second element (the score)
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, score))| score * (i + 1) as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    use part_two::Hand;
    // split each line into two words (on whitespace) and collect into a vector
    let mut hands = input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            (Hand::from_word(words[0]), words[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<(Hand, u32)>>();

    // Sort the hands by the first element (the hand) and then by the second element (the score)
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    Some(
        hands
            .iter()
            .enumerate()
            // .inspect(|(i, (hand, bet))| println!("{}: {:?}, bet:{}", i, hand, bet))
            .map(|(i, (_, score))| score * (i + 1) as u32)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[rstest::rstest]
    #[case('2', '3')]
    #[case('2', 'T')]
    #[case('2', 'J')]
    #[case('2', 'Q')]
    #[case('2', 'K')]
    #[case('2', 'A')]
    fn test_card_order(#[case] c1: char, #[case] c2: char) {
        use part_one::Card;
        let c1 = Card::from_char(c1);
        let c2 = Card::from_char(c2);
        assert!(c2 > c1);
    }

    #[rstest::rstest]
    #[case('2', '3')]
    #[case('2', 'T')]
    #[case('J', '2')]
    #[case('2', 'Q')]
    #[case('2', 'K')]
    #[case('2', 'A')]
    fn test_card_order_part_2(#[case] c1: char, #[case] c2: char) {
        use part_two::Card;
        let c1 = Card::from_char(c1);
        let c2 = Card::from_char(c2);
        assert!(c2 > c1);
    }

    #[rstest::rstest]
    #[case('2', '2')]
    #[case('T', 'T')]
    fn test_card_eq(#[case] c1: char, #[case] c2: char) {
        use part_one::Card;
        let c1 = Card::from_char(c1);
        let c2 = Card::from_char(c2);
        assert!(c2 == c1);
    }

    #[test]
    fn test_counts() {
        use part_one::{card_counts, cards_from_str, Card};
        let cards = cards_from_str("234233");
        let counts = card_counts(cards);
        assert_eq!(counts[&Card::from_char('2')], 2);
        assert_eq!(counts[&Card::from_char('3')], 3);
        assert_eq!(counts[&Card::from_char('4')], 1);
    }
    // There were lots of edge cases in the hand type logic, so I've tried to test some of the
    // ones I seemed to be getting wrong answers for here.
    #[rstest::rstest]
    #[case("JJ5AT", part_two::HandType::ThreeOfAKind)]
    #[case("22222", part_two::HandType::FiveOfAKind)]
    #[case("2222J", part_two::HandType::FiveOfAKind)]
    #[case("222JJ", part_two::HandType::FiveOfAKind)]
    #[case("22JJJ", part_two::HandType::FiveOfAKind)]
    #[case("2JJJJ", part_two::HandType::FiveOfAKind)]
    #[case("JJJJJ", part_two::HandType::FiveOfAKind)]
    fn test_hand_type_part_2(#[case] input: &str, #[case] expected: part_two::HandType) {
        use part_two::Hand;
        let hand = Hand::from_word(input);
        assert_eq!(hand.hand_type, expected);
    }
}

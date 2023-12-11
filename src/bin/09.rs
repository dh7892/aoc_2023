advent_of_code::solution!(9);

use chumsky::prelude::*;

// Get an integer, including negative numbers
fn integer_parser() -> impl Parser<char, i32, Error = Simple<char>> {
    // Parser for an optional negative sign
    let sign = just('-').or_not();

    // Parser for one or more digits
    let digits = text::int(10);

    // Combine the sign and digits to create an i32
    sign.then(digits).map(|(sign, digits)| match sign {
        Some(_) => -digits.parse::<i32>().unwrap(),
        None => digits.parse::<i32>().unwrap(),
    })
}

fn integer_list_parser() -> impl Parser<char, Vec<i32>, Error = Simple<char>> {
    integer_parser().separated_by(text::whitespace())
}

// Return the last value and a vector of the differences between each value
fn gradient_and_last(data: Vec<i32>) -> (i32, Vec<i32>) {
    let last = *data.last().unwrap();
    let deltas = data.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    (last, deltas)
}

// Return the first value and a vector of the differences between each value
fn gradient_and_first(data: Vec<i32>) -> (i32, Vec<i32>) {
    let first = *data.first().unwrap();
    let deltas = data.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    (first, deltas)
}
fn extrapolate(data: Vec<i32>) -> i32 {
    let mut last_values = Vec::new();
    let (mut last, mut deltas) = gradient_and_last(data);
    last_values.push(last);

    loop {
        (last, deltas) = gradient_and_last(deltas.clone());
        last_values.push(last);
        if deltas.iter().all(|&d| d == 0) {
            break;
        }
    }
    last_values.iter().sum::<i32>()
}

fn pretrapolate(data: Vec<i32>) -> i32 {
    let mut last_values = Vec::new();
    let (mut first, mut deltas) = gradient_and_first(data);
    last_values.push(first);

    loop {
        (first, deltas) = gradient_and_first(deltas.clone());
        last_values.push(first);
        if deltas.iter().all(|&d| d == 0) {
            break;
        }
    }
    last_values.iter().rev().fold(0, |acc, &x| x - acc)
}

pub fn part_one(input: &str) -> Option<i32> {
    let data = input
        .lines()
        .map(|line| integer_list_parser().parse(line).unwrap())
        .collect::<Vec<_>>();
    Some(data.into_iter().map(extrapolate).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let data = input
        .lines()
        .map(|line| integer_list_parser().parse(line).unwrap())
        .collect::<Vec<_>>();
    Some(data.into_iter().map(pretrapolate).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_extrapolate() {
        let result = extrapolate(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, 18);
    }
    #[rstest::rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], -3)]
    #[case(vec![1, 3, 6, 10, 15, 21], 0)]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    fn test_pretrapolate(#[case] data: Vec<i32>, #[case] expected: i32) {
        assert_eq!(pretrapolate(data), expected);
    }

    #[test]
    fn test_gradient_and_last() {
        let result = gradient_and_last(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, (15, vec![3, 3, 3, 3, 3]));
    }
}

advent_of_code::solution!(12);

use std::str::Chars;

use chumsky::prelude::*;

fn integer_parser() -> impl Parser<char, u32, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<u32>().unwrap())
}

fn integer_list_parser() -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
    integer_parser().separated_by(just(','))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    operational,
    damaged,
}

fn counts_of_damaged(statuses: &Vec<Status>) -> Vec<u32> {
    let mut result = Vec::new();
    let mut count = 0;
    for status in statuses.iter() {
        match status {
            Status::operational => {
                if count > 0 {
                    result.push(count);
                    count = 0;
                }
            }
            Status::damaged => count += 1,
        }
    }
    if count > 0 {
        result.push(count);
    }
    result
}

// Return true if the sequences of damaged statuses matches the counts
fn line_valid(statuses: &Vec<Status>, counts: &Vec<u32>) -> bool {
    counts_of_damaged(statuses) == *counts
}

// Return true if the current line could become valid
fn could_be_valid(statuses: &Vec<Status>, counts: &Vec<u32>, original_string: &str) -> bool {
    let length = original_string.len() as u32;
    let counts_of_damaged_so_far = counts_of_damaged(statuses);
    if counts_of_damaged_so_far.len() > counts.len() {
        return false;
    }

    // Early return if we can't possibly get to enough damaged
    let number_of_damaged = counts.iter().sum::<u32>();
    let damaged_so_far = statuses.iter().filter(|s| **s == Status::damaged).count() as u32;
    let possible_remaining_damaged = original_string
        .chars()
        .skip(statuses.len())
        .filter(|c| *c == '#' || *c == '?')
        .count() as u32;

    if damaged_so_far + possible_remaining_damaged < number_of_damaged {
        return false;
    }

    let num_counts = counts_of_damaged_so_far.len();

    for (i, count) in counts_of_damaged_so_far.iter().enumerate() {
        if i + 1 == num_counts {
            // Last count (either completed or still counting)

            if statuses.last() == Some(&Status::damaged) {
                if count > &counts[i] {
                    // We might still be accumulating a count, so we can only say it's wrong if it's too large
                    return false;
                } else {
                    // See if we have the right number of positions remaining
                }
            }
            if statuses.last() == Some(&Status::operational) {
                if count != &counts[i] {
                    // We can't still be accumulating, so we can check the last count
                    return false;
                } else {
                    // See if it's possible to have the right number of damaged positions remaining
                    // Look at the damage tile groups still remaining and include a space between for an operational tile
                    // (needed to separate each group). If the number of spaces we have left to process
                    // is less than the number of damaged tiles, we can return false

                    let chars_remaining = length - statuses.len() as u32;
                    let num_blocks_remaining = counts.len() - i - 1;
                    if num_blocks_remaining > 0 {
                        let min_values_remaining =
                            number_of_damaged - damaged_so_far + num_blocks_remaining as u32 - 1;
                        if min_values_remaining > chars_remaining {
                            return false;
                        }
                    }
                }
            }
        } else if count != &counts[i] {
            return false;
        }
    }
    true
}

// Recursive function to parse the next char in the input and either:
// Add damaged/operational to each of the current statuses,
// or (if the char is a '?') replace each of the status lists with two lists, with both permutations
// The input is an iterator of chars
fn parse_statuses(
    mut input: Chars,
    current: &Vec<Vec<Status>>,
    counts: &Vec<u32>,
    original_string: &str,
) -> Vec<Vec<Status>> {
    // println!(
    //     "About to do next char, currently processed {} chars of the line, got {} permutations",
    //     current[0].len(),
    //     current.len()
    // );
    if let Some(c) = input.next() {
        match c {
            '.' => {
                let mut result = current.clone();
                for status in result.iter_mut() {
                    status.push(Status::operational);
                }
                parse_statuses(input, &result, counts, original_string)
            }
            '#' => {
                let mut result = current.clone();
                for status in result.iter_mut() {
                    status.push(Status::damaged);
                }
                parse_statuses(input, &result, counts, original_string)
            }
            '?' => {
                let mut result = Vec::new();
                for status in current.iter() {
                    let mut have_added_one = false;
                    let mut damaged = status.clone();
                    damaged.push(Status::damaged);
                    if could_be_valid(&damaged, &counts, original_string) {
                        have_added_one = true;
                        result.push(damaged);
                    }
                    let mut operational = status.clone();
                    operational.push(Status::operational);
                    if could_be_valid(&operational, &counts, original_string) {
                        have_added_one = true;
                        result.push(operational);
                    }
                }
                parse_statuses(input, &result, counts, original_string)
            }
            _ => panic!("Invalid input"),
        }
    } else {
        current.clone()
    }
}

// Parse the line and return the number of valid permutations there were
fn process_line(line: &str) -> u32 {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let counts = integer_list_parser().parse(words[1]).unwrap();
    let statues = parse_statuses(words[0].chars(), &vec![vec![]], &counts, words[0]);
    println!("Number of statues to check: {}", statues.len());
    statues
        .iter()
        .filter(|status| line_valid(status, &counts))
        .count() as u32
}
// Parse the line, increase it and return the number of valid permutations there were
fn process_line_part_2(line: &str) -> u32 {
    let words = line.split_whitespace().collect::<Vec<&str>>();

    let counts = integer_list_parser().parse(words[1]).unwrap();
    // Increase the counts but repeating 5 times
    let count = counts.len() * 5;
    let counts = counts
        .iter()
        .cloned()
        .cycle()
        .take(count)
        .collect::<Vec<u32>>();
    let separator = std::iter::once('?');
    let count = ((words[0].len() + 1) * 5) - 1;
    let padded_statuses = (words[0].chars().chain(separator))
        .cycle()
        .take(count)
        .collect::<String>();
    let statues = parse_statuses(
        padded_statuses.chars(),
        &vec![vec![]],
        &counts,
        &padded_statuses,
    );

    println!("Number of statues to check: {}", statues.len());
    statues
        .iter()
        .filter(|status| line_valid(status, &counts))
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(process_line).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().map(process_line_part_2).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_parse_statuses() {
        use Status::*;
        let input = "?.";
        let counts = vec![1];
        let result = parse_statuses(input.chars(), &vec![vec![]], &counts, input);
        assert_eq!(
            result,
            vec![vec![damaged, operational], vec![operational, operational],]
        );
    }
    #[test]
    fn test_parse_statuses_2() {
        use Status::*;
        let input = "?#?";
        let counts = vec![1];
        let result = parse_statuses(input.chars(), &vec![vec![]], &counts, input);
        assert_eq!(result, vec![vec![operational, damaged, operational],]);
    }

    #[test]
    fn test_line() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        let valid = process_line(line);
        assert_eq!(valid, 1);
    }

    #[test]
    fn test_line_part_2() {
        let line = ".??..??...?##. 1,1,3";
        let valid = process_line_part_2(line);
        assert_eq!(valid, 16384);
    }
    #[test]
    fn test_line_part_2_big() {
        let line = "?????.??.???. 1,1,1";
        let valid = process_line_part_2(line);
        assert_eq!(valid, 16384);
    }
    #[test]
    fn test_line_part_2_big_again() {
        let line = "????.######..#####. 1,6,5";
        let valid = process_line_part_2(line);
        assert_eq!(valid, 2500);
    }
}

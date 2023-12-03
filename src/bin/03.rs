use std::collections::HashMap;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Hash, Eq)]
struct Location {
    row: usize,
    col: usize,
}

struct Number {
    row: usize,
    col: usize,
    length: usize,
    value: usize,
}

fn number_is_valid(number: &Number, symbols: &HashMap<Location, char>) -> bool {
    number
        .symbol_locations()
        .iter()
        .any(|location| symbols.get(location).is_some())
}

impl Number {
    fn symbol_locations(&self) -> Vec<Location> {
        let mut locations = Vec::new();
        let min_col = if self.col > 0 { self.col - 1 } else { 0 };
        let min_row = if self.row > 0 { self.row - 1 } else { 0 };
        for row in min_row..=self.row + 1 {
            for col in min_col..=self.col + self.length {
                if row == self.row && col >= self.col && col < self.col + self.length {
                    // exclude the number itself
                    continue;
                }
                locations.push(Location { row, col });
            }
        }
        locations
    }
}

fn add_number_to_list(acc_number: &mut String, numbers: &mut Vec<Number>, row: usize, col: usize) {
    if !acc_number.is_empty() {
        let number = Number {
            row,
            col: col - acc_number.len(),
            length: acc_number.len(),
            value: acc_number.parse().unwrap(),
        };
        numbers.push(number);
        acc_number.clear();
    }
}

fn parse_input(input: &str) -> (Vec<Number>, HashMap<Location, char>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    let mut acc_number = "".to_owned();
    let mut last_col = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                acc_number.push(c);
                last_col = col;
            } else {
                add_number_to_list(&mut acc_number, &mut numbers, row, col);
                if c != '.' {
                    symbols.insert(Location { row, col }, c);
                }
            }
        }
        // If we were still accumulating a number when we reached the end of the line, we need to add it now
        add_number_to_list(&mut acc_number, &mut numbers, row, last_col + 1);
    }
    (numbers, symbols)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_input(input);
    let result = numbers
        .iter()
        .filter(|number| number_is_valid(number, &symbols))
        .map(|number| number.value)
        .sum::<usize>();
    Some(result as u32)
}

fn numbers_next_to_location<'a>(location: &'a Location, numbers: &'a [Number]) -> Vec<&'a Number> {
    let mut result = Vec::new();
    for number in numbers {
        let locations = number.symbol_locations();
        if locations.contains(location) {
            result.push(number);
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_input(input);
    let total = symbols
        .into_iter()
        .filter(|(_, c)| *c == '*')
        .map(|(location, _)| {
            let numbers = numbers_next_to_location(&location, &numbers);
            if numbers.len() == 2 {
                return numbers[0].value * numbers[1].value;
            }
            0
        })
        .sum::<usize>();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_symbol_locations_1() {
        let number = Number {
            row: 0,
            col: 0,
            length: 1,
            value: 0,
        };
        let locations = number.symbol_locations();
        assert_eq!(locations.len(), 3);
        assert!(locations.contains(&Location { row: 0, col: 1 }));
        assert!(locations.contains(&Location { row: 1, col: 0 }));
        assert!(locations.contains(&Location { row: 1, col: 1 }));
    }

    #[test]
    fn test_symbol_locations_2() {
        let number = Number {
            row: 3,
            col: 4,
            length: 2,
            value: 0,
        };
        let locations = number.symbol_locations();
        assert_eq!(locations.len(), 10);
        assert!(locations.contains(&Location { row: 2, col: 3 }));
        assert!(locations.contains(&Location { row: 3, col: 3 }));
        assert!(locations.contains(&Location { row: 3, col: 6 }));
        assert!(locations.contains(&Location { row: 4, col: 6 }));
    }

    #[test]
    fn test_number_is_valid() {
        let mut symbols = HashMap::new();
        symbols.insert(Location { row: 0, col: 1 }, 'a');
        let number = Number {
            row: 0,
            col: 0,
            length: 1,
            value: 0,
        };
        assert!(number_is_valid(&number, &symbols));
    }

    #[test]
    fn test_number_is_valid_2() {
        let mut symbols = HashMap::new();
        symbols.insert(Location { row: 0, col: 5 }, 'a');
        let number = Number {
            row: 1,
            col: 2,
            length: 3,
            value: 0,
        };
        assert!(number_is_valid(&number, &symbols));
    }

    #[test]
    fn test_number_is_valid_3() {
        let mut symbols = HashMap::new();
        symbols.insert(Location { row: 8, col: 5 }, '*');
        let number = Number {
            row: 7,
            col: 6,
            length: 3,
            value: 755,
        };
        assert!(number_is_valid(&number, &symbols));
    }

    #[test]
    fn test_number_is_not_valid() {
        let mut symbols = HashMap::new();
        symbols.insert(Location { row: 0, col: 6 }, 'a');
        let number = Number {
            row: 1,
            col: 2,
            length: 3,
            value: 0,
        };
        assert!(!number_is_valid(&number, &symbols));
    }
}

advent_of_code::solution!(11);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Galaxy {
    row: u64,
    column: u64,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u64 {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }
}

// Parse the input into a Hashmap of galaxies and also return set of all rows and columns
fn parse_input(input: &str) -> (HashMap<Galaxy, u64>, HashSet<u64>, HashSet<u64>) {
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();
    let mut galaxies = HashMap::new();
    let mut id = 0;
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert(
                    Galaxy {
                        row: row.try_into().unwrap(),
                        column: column.try_into().unwrap(),
                    },
                    id,
                );
                id += 1;
                rows.insert(row.try_into().unwrap());
                columns.insert(column.try_into().unwrap());
            }
        }
    }
    (galaxies, rows, columns)
}

fn find_empty_rows(rows: HashSet<u64>) -> Vec<u64> {
    let max_row = *rows.iter().max().unwrap();
    let mut empty_rows = Vec::new();
    for row in 0..=max_row {
        if !rows.contains(&row) {
            empty_rows.push(row);
        }
    }
    empty_rows
}

fn find_empty_columns(columns: HashSet<u64>) -> Vec<u64> {
    let max_column = *columns.iter().max().unwrap();
    let mut empty_columns = Vec::new();
    for column in 0..=max_column {
        if !columns.contains(&column) {
            empty_columns.push(column);
        }
    }
    empty_columns
}

fn pad_galaxies(
    galaxies: &mut HashMap<Galaxy, u64>,
    rows: HashSet<u64>,
    columns: HashSet<u64>,
    padding: u64,
) -> HashMap<Galaxy, u64> {
    let empty_rows = find_empty_rows(rows);
    // We can loop over the empty rows and increase the index for any galaxy whose row is greater than the empty row
    // We loop in reverse order to avoid spoiling the indices
    let mut galaxies = galaxies.clone();
    let mut new_galaxies = HashMap::new();
    for row in empty_rows.iter().rev() {
        for (galaxy, id) in galaxies.iter_mut() {
            if galaxy.row > *row {
                new_galaxies.insert(
                    Galaxy {
                        row: galaxy.row + padding,
                        column: galaxy.column,
                    },
                    *id,
                );
            } else {
                new_galaxies.insert(*galaxy, *id);
            }
        }
        galaxies = new_galaxies.clone();
        new_galaxies.clear();
    }

    // Now do the same for columns
    let mut new_galaxies = HashMap::new();
    let empty_columns = find_empty_columns(columns);
    for column in empty_columns.iter().rev() {
        for (galaxy, id) in galaxies.iter_mut() {
            if galaxy.column > *column {
                new_galaxies.insert(
                    Galaxy {
                        row: galaxy.row,
                        column: galaxy.column + padding,
                    },
                    *id,
                );
            } else {
                new_galaxies.insert(*galaxy, *id);
            }
        }
        galaxies = new_galaxies.clone();
        new_galaxies.clear();
    }
    galaxies
}

pub fn part_one(input: &str) -> Option<u64> {
    let (galaxies, rows, columns) = parse_input(input);
    let padded_galaxies = pad_galaxies(&mut galaxies.clone(), rows, columns, 1);
    // Loop over all galaxies and sum the distances to all other galaxies
    let total_distance = padded_galaxies
        .keys()
        .combinations(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum::<u64>();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (galaxies, rows, columns) = parse_input(input);
    let padded_galaxies = pad_galaxies(&mut galaxies.clone(), rows, columns, 1000000 - 1);
    // Loop over all galaxies and sum the distances to all other galaxies
    let total_distance = padded_galaxies
        .keys()
        .combinations(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum::<u64>();

    Some(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (galaxies, rows, columns) = parse_input(input);
        let padded_galaxies = pad_galaxies(&mut galaxies.clone(), rows, columns, 9);
        // Loop over all galaxies and sum the distances to all other galaxies
        let total_distance = padded_galaxies
            .keys()
            .combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .sum::<u64>();

        let result = Some(total_distance);
        assert_eq!(result, Some(1030));
    }
    #[test]
    fn test_part_two_b() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (galaxies, rows, columns) = parse_input(input);
        let padded_galaxies = pad_galaxies(&mut galaxies.clone(), rows, columns, 99);
        // Loop over all galaxies and sum the distances to all other galaxies
        let total_distance = padded_galaxies
            .keys()
            .combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .sum::<u64>();

        let result = Some(total_distance);
        assert_eq!(result, Some(8410));
    }

    #[test]
    fn test_distance() {
        let galaxy1 = Galaxy { row: 0, column: 4 };
        let galaxy3 = Galaxy { row: 2, column: 0 };
        let galaxy6 = Galaxy { row: 7, column: 12 };
        let galaxy7 = Galaxy { row: 10, column: 9 };
        let galaxy8 = Galaxy { row: 11, column: 0 };
        let galaxy9 = Galaxy { row: 11, column: 5 };
        assert_eq!(galaxy1.distance(&galaxy7), 15);
        assert_eq!(galaxy3.distance(&galaxy6), 17);
        assert_eq!(galaxy8.distance(&galaxy9), 5);
    }
}

use std::collections::{HashMap, HashSet};

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Visit {
    row: usize,
    col: usize,
    direction: Direction,
}

struct LossMap {
    grid: HashMap<(usize, usize), u32>,
    rows: usize,
    cols: usize,
}

impl LossMap {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn add_value(&mut self, row: usize, col: usize, value: u32) {
        self.grid.insert((row, col), value);
        if self.rows <= row {
            self.rows = row + 1;
        }
        if self.cols <= col {
            self.cols = col + 1;
        }
    }

    fn from_input(input: &str) -> Self {
        let mut map = Self::new();
        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                map.add_value(row, col, value.to_digit(10).unwrap());
            }
        }
        map
    }
}

fn loss_from_path(map: &LossMap, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut loss = 0;
    for i in start.0.min(end.0)..=start.0.max(end.0) {
        for j in start.1.min(end.1)..=start.1.max(end.1) {
            if !(i == start.0 && j == start.1) {
                // Don't count the current position
                // Or we'll get it twice
                loss += map.grid.get(&(i, j)).unwrap();
            }
        }
    }
    loss
}

fn min_loss_to_exit(
    cache: &mut HashMap<(u32, u32, Direction), u32>,
    visits: &HashSet<Visit>,
    map: &LossMap,
    row: usize,
    col: usize,
    direction: Direction,
) -> Option<u32> {
    use Direction::*;

    if (row, col) == (map.rows - 1, map.cols - 1) {
        // We're at the exit
        return Some(0);
    }

    let visit = Visit {
        row,
        col,
        direction,
    };
    // If we've already visited this position, we can't go there again
    if visits.contains(&visit) {
        println!("Already visited: {:?}", visit);
        return None;
    }

    // We're safe to visit here, so add the visit to our set
    let mut visits = visits.clone();
    visits.insert(visit);

    if let Some(&loss) = cache.get(&(row as u32, col as u32, direction)) {
        return Some(loss);
    }

    // Due to how the parent call will have permuted the moves, we can assume that we have to
    // Change direction now (i.e. we don't care how many steps we took in the previous direction)
    let mut next_moves = vec![];
    match direction {
        North | South => {
            for i in 1..=3 {
                if col + i < map.cols {
                    next_moves.push((row, col + i, East));
                }
                if col >= i {
                    next_moves.push((row, col - i, West));
                }
            }
        }
        East | West => {
            for i in 1..=3 {
                if row + i < map.rows {
                    next_moves.push((row + i, col, South));
                }
                if row >= i {
                    next_moves.push((row - i, col, North));
                }
            }
        }
    }
    let min_loss = next_moves
        .iter()
        .filter_map(|&(next_row, next_col, next_direction)| {
            let loss = loss_from_path(map, (row, col), (next_row, next_col));
            min_loss_to_exit(cache, &visits, map, next_row, next_col, next_direction)
                .map(|rec_loss| loss + rec_loss)
        })
        .min();

    // Update our cache with the minimum loss
    if let Some(l) = min_loss {
        cache.insert((row as u32, col as u32, direction), l);
    }
    min_loss
}

pub fn part_one(input: &str) -> Option<u32> {
    let loss_map = LossMap::from_input(input);
    let mut cache = HashMap::new();
    let visits = HashSet::new();
    min_loss_to_exit(&mut cache, &visits, &loss_map, 0, 0, Direction::East)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[rstest::rstest]
    #[case(11, 12, Direction::East, Some(3))]
    // #[case(11, 11, Direction::East, Some(6))]
    // #[case(7, 12, Direction::East, Some(28))]
    // #[case(7, 11, Direction::South, Some(31))]
    // #[case(4, 11, Direction::East, Some(47))]
    #[case(4, 10, Direction::South, Some(50))]
    #[case(2, 10, Direction::East, Some(59))]
    // #[case(2, 8, Direction::South, Some(65))]
    fn test_part_one_cases(
        #[case] row: usize,
        #[case] col: usize,
        #[case] d: Direction,
        #[case] expected: Option<u32>,
    ) {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let loss_map = LossMap::from_input(input);
        let mut cache = HashMap::new();
        let visits = HashSet::new();
        let result = min_loss_to_exit(&mut cache, &visits, &loss_map, row, col, d);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

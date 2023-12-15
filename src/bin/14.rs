advent_of_code::solution!(14);

use core::num;
use std::collections::HashMap;

enum Rock {
    Round,
    Cube,
}

struct RockMap {
    rows: usize,
    cols: usize,
    rocks: HashMap<(usize, usize), Rock>,
}

impl RockMap {
    fn new(rows: usize, cols: usize) -> RockMap {
        RockMap {
            rows,
            cols,
            rocks: HashMap::new(),
        }
    }

    fn add_rock(&mut self, row: usize, col: usize, rock: Rock) {
        self.rocks.insert((row, col), rock);
        if row > self.rows {
            self.rows = row;
        }
        if col > self.cols {
            self.cols = col;
        }
    }

    // Move all rocks up as far as they can go
    fn move_up(&mut self) {
        for col in 0..=self.cols {
            let mut last_empty_row = 0;
            for row in 0..=self.rows {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => {
                            last_empty_row = row + 1;
                        }
                        Rock::Round => {
                            // We can move this rock up to the last empty row
                            if row > last_empty_row {
                                self.rocks.remove(&(row, col));
                                self.rocks.insert((last_empty_row, col), Rock::Round);
                            }
                            last_empty_row += 1;
                        }
                    }
                }
            }
        }
    }

    fn move_down(&mut self) {
        for col in 0..=self.cols {
            let mut last_empty_row = self.rows;
            // Got to loop from the other side
            for row in (0..=self.rows).rev() {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => {
                            if row > 0 {
                                last_empty_row = row - 1;
                            }
                        }
                        Rock::Round => {
                            // We can move this rock down to the last empty row
                            if row < last_empty_row {
                                self.rocks.remove(&(row, col));
                                self.rocks.insert((last_empty_row, col), Rock::Round);
                            }
                            if row > 0 {
                                last_empty_row -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn move_left(&mut self) {
        for row in 0..=self.rows {
            let mut last_empty_col = 0;
            for col in 0..=self.cols {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => {
                            last_empty_col = col + 1;
                        }
                        Rock::Round => {
                            // We can move this rock left to the last empty col
                            if col > last_empty_col {
                                self.rocks.remove(&(row, col));
                                self.rocks.insert((row, last_empty_col), Rock::Round);
                            }
                            last_empty_col += 1;
                        }
                    }
                }
            }
        }
    }

    fn move_right(&mut self) {
        for row in 0..=self.rows {
            let mut last_empty_col = self.cols;
            // Got to loop from the other side
            for col in (0..=self.cols).rev() {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => {
                            if col > 0 {
                                last_empty_col = col - 1;
                            }
                        }
                        Rock::Round => {
                            // We can move this rock right to the last empty col
                            if col < last_empty_col {
                                self.rocks.remove(&(row, col));
                                self.rocks.insert((row, last_empty_col), Rock::Round);
                            }
                            if col > 0 {
                                last_empty_col -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.move_up();
        self.move_left();
        self.move_down();
        self.move_right();
    }

    // Calculate the force load by adding up the force from each round rock
    // The force from a rock is the number of rows it is from the last row
    fn load(&self) -> usize {
        let mut load = 0;
        for ((row, _), rock) in self.rocks.iter() {
            match rock {
                Rock::Cube => {}
                Rock::Round => {
                    load += self.rows + 1 - row;
                }
            }
        }
        load
    }

    fn _print_map(&self) {
        for row in 0..=self.rows {
            for col in 0..=self.cols {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => print!("#"),
                        Rock::Round => print!("O"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn map_hash(&self) -> String {
        let mut map = String::new();
        for row in 0..=self.rows {
            for col in 0..=self.cols {
                if let Some(rock) = self.rocks.get(&(row, col)) {
                    match rock {
                        Rock::Cube => map.push('#'),
                        Rock::Round => map.push('O'),
                    }
                } else {
                    map.push('.');
                }
            }
            map.push('\n');
        }
        map
    }
}

fn rock_map_from_input(input: &str) -> RockMap {
    let mut rock_map = RockMap::new(0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    rock_map.add_rock(row, col, Rock::Round);
                }
                '#' => {
                    rock_map.add_rock(row, col, Rock::Cube);
                }
                _ => {}
            }
        }
    }
    rock_map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rock_map = rock_map_from_input(input);
    rock_map.move_up();
    Some(rock_map.load())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rock_map = rock_map_from_input(input);
    let mut seen_maps = Vec::new();
    let mut loads = Vec::new();
    // let num_cycles = 1_000_000_000;
    let num_cycles = 1_000_000_000;

    // Run until we find a repeating pattern
    for _i in 0..num_cycles {
        rock_map.cycle();
        let h = rock_map.map_hash();
        let index_seen_before = seen_maps
            .iter()
            .enumerate()
            .find(|(_, m)| *m == &h)
            .map(|(i, _)| i);
        match index_seen_before {
            Some(index) => {
                // We've seen this map before so we can calculate the load for the 1 billionth cycle
                let cycle_length = seen_maps.len() - index;
                let cycle = (num_cycles - index - 1) % cycle_length;
                let load = loads[index + cycle];
                return Some(load);
            }
            None => {
                seen_maps.push(h);
                loads.push(rock_map.load());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[rstest::rstest]
    #[case(1, 87)]
    #[case(3, 69)]
    fn test_cycles(#[case] cycles: usize, #[case] expected_load: usize) {
        let input = advent_of_code::template::read_file("examples", DAY);

        let mut rock_map = rock_map_from_input(&input);
        for _ in 0..cycles {
            rock_map.cycle();
        }
        assert_eq!(rock_map.load(), expected_load);
    }
}

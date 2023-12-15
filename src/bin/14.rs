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
    // let num_cycles = 1_000_000_000;
    let num_cycles = 1_00;
    // Rather than try to get all 1000000000 cycles, I run the first 1000
    // print out the results and look for patterns
    // It turns out that you can clearly see a repeating pattern after a few cycles.
    // So I manually worked out the repeating sequence from that and calculated what the 1 billionth cycle would be

    // Run for a few cycles to let the pattern stabilise
    for _ in 0..num_cycles {
        rock_map.cycle();
    }

    // Now run for cycles until we have a repeating pattern
    let mut cycles = Vec::new();
    for _ in num_cycles..num_cycles + 100 {
        rock_map.cycle();
        let load = rock_map.load();
        if cycles.contains(&load) {
            break;
        }
        cycles.push(load);
    }
    // We now have our repeating pattern so we can calculate the load for the 1 billionth cycle
    let cycle = (1_000_000_000 - num_cycles - 1) % cycles.len();
    let load = cycles[cycle];
    println!(
        "Load after {} cycles is {}, (it's number {} in our repeating pattern",
        1_000_000_000, load, cycle
    );
    Some(load)
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

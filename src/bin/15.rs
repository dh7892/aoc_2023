advent_of_code::solution!(15);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation<'a> {
    AddLens(&'a str, u32),
    TakeLens(&'a str),
}

fn word_to_operation(word: &str) -> Operation {
    let mut last_index_of_id = 0;
    let mut focal_power = 0;
    let mut operator_char = ' ';

    for c in word.chars() {
        if c.is_whitespace() {
            continue;
        } else if c == '=' || c == '-' {
            operator_char = c;
        } else if operator_char == ' ' {
            last_index_of_id += 1;
        } else {
            focal_power = c.to_digit(10).unwrap();
        }
    }

    match operator_char {
        '=' => Operation::AddLens(&word[..last_index_of_id], focal_power),
        '-' => Operation::TakeLens(&word[..last_index_of_id]),
        _ => unreachable!(),
    }
}

struct LensBox<'a> {
    lens_ids: Vec<&'a str>,
    lens_focal_powers: Vec<u32>,
}

impl<'a> LensBox<'a> {
    fn new() -> Self {
        Self {
            lens_ids: Vec::new(),
            lens_focal_powers: Vec::new(),
        }
    }

    fn add_lens(&mut self, lens_id: &'a str, lens_focal_power: u32) {
        // Check if we already had a lens with this id in the box
        let lens_index = self.lens_ids.iter().position(|id| id == &lens_id);
        match lens_index {
            Some(index) => {
                // If we did, replace the focal power
                self.lens_focal_powers[index] = lens_focal_power;
            }
            None => {
                // If not, we can just add it
                self.lens_ids.push(lens_id);
                self.lens_focal_powers.push(lens_focal_power);
            }
        }
    }

    fn take_lens(&mut self, lens_id: &'a str) {
        let lens_index = self.lens_ids.iter().position(|id| id == &lens_id);
        if let Some(index) = lens_index {
            self.lens_ids.remove(index);
            self.lens_focal_powers.remove(index);
        }
    }

    fn do_operation(&mut self, operation: Operation<'a>) {
        match operation {
            Operation::AddLens(lens_id, focal_power) => self.add_lens(lens_id, focal_power),
            Operation::TakeLens(lens_id) => self.take_lens(lens_id),
        }
    }

    fn focus_power(&self) -> u32 {
        self.lens_focal_powers
            .iter()
            .enumerate()
            .map(|(i, f)| dbg!((i as u32 + 1) * f))
            .sum()
    }
}

fn make_hash(current_hash: u32, word: &str) -> u32 {
    let mut result = current_hash;
    for c in word.chars() {
        if char::is_whitespace(c) {
            println!("Skipping whitespace");
            continue;
        }
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

fn total_score(lens_boxes: &HashMap<u32, LensBox>) -> u32 {
    lens_boxes
        .iter()
        .map(|(index, lensbox)| dbg!((*index + 1) * lensbox.focus_power()))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hash = 0;
    let result = input
        .split(',')
        .map(|word| {
            hash = make_hash(0, word);
            hash
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lens_boxes = HashMap::new();
    for op in input.split(',') {
        let operation = word_to_operation(op);
        let box_index = match operation {
            Operation::AddLens(lens_id, _) => make_hash(0, lens_id),
            Operation::TakeLens(lens_id) => make_hash(0, lens_id),
        };
        let lens_box = lens_boxes.entry(box_index).or_insert_with(LensBox::new);
        lens_box.do_operation(operation);
    }
    Some(total_score(&lens_boxes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[rstest::rstest]
    #[case("lens-", Operation::TakeLens("lens"))]
    #[case("lens=2", Operation::AddLens("lens", 2))]
    fn test_word_to_operation(#[case] word: &str, #[case] expected: Operation) {
        assert_eq!(word_to_operation(word), expected);
    }
}

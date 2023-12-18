advent_of_code::solution!(18);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    direction: Direction,
}

fn input_to_vertical_lines_part_1(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let (mut x, mut y) = (0, 0);

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction_char = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        // let color = u32::from_str_radix(&color[1..], 16).unwrap();
        let (dx, dy, direction) = match direction_char {
            "U" => (0, -1, Some(Direction::Up)),
            "D" => (0, 1, Some(Direction::Down)),
            "L" => (-1, 0, None),
            "R" => (1, 0, None),
            _ => panic!("Invalid direction"),
        };
        let x1 = x;
        let y1 = y;
        let x2 = x + dx * count as i32;
        let y2 = y + dy * count as i32;
        if let Some(direction) = direction {
            // Only save vertical lines
            lines.push(Line {
                x1,
                y1,
                x2,
                y2,
                direction,
            });
        }
        // Update for the next line
        x = x2;
        y = y2;
    }
    lines
}

// Use ray marching to find the area of the hole
// By looping row by row and considering vertical lines
fn lines_to_area(lines: &[Line]) -> u32 {
    // Only look at starts of line as there will always be a corresponding other vertical line
    // That starts at the end of the line (even if it's not directly after this line)
    let mut rows_of_interest = lines.iter().map(|line| line.y1).collect::<Vec<_>>();
    rows_of_interest.sort_unstable();
    rows_of_interest.dedup();

    rows_of_interest.windows(2).fold(0, |area, row| {
        let (row1, row2) = (row[0], row[1]);
        let lines_of_interest = lines
            .iter()
            .filter(|line| {
                // Only consider lines that are in the range of the current rows
                match line.direction {
                    Direction::Up => line.y1 >= row2 && line.y2 <= row1,
                    Direction::Down => line.y1 <= row1 && line.y2 >= row2,
                }
            })
            .collect::<Vec<_>>();

        let mut cols_of_interest = lines_of_interest
            .iter()
            .flat_map(|line| vec![line.x1, line.x2])
            .collect::<Vec<_>>();
        cols_of_interest.sort_unstable();
        cols_of_interest.dedup();
        cols_of_interest.windows(2).fold(area, |area, col| {
            let (col1, col2) = (col[0], col[1]);
            let mut inside_count = 0;
            for line in lines_of_interest.iter() {
                if line.x1 <= col1 {
                    if line.direction == Direction::Up {
                        inside_count += 1;
                    } else {
                        inside_count -= 1;
                    }
                }
            }
            if inside_count > 0 {
                area + (row2 - row1) as u32 * ((col2 - col1) as u32 + 1)
            } else {
                area
            }
        })
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let vertical_lines = input_to_vertical_lines_part_1(input);
    Some(lines_to_area(&vertical_lines))
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

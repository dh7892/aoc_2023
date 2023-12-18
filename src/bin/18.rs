advent_of_code::solution!(18);

use chumsky::prelude::*;
use std::collections::HashMap;

fn integer_parser() -> impl Parser<char, u32, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<u32>().unwrap())
}

fn integer_list_parser() -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
    integer_parser().separated_by(text::whitespace())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WallType {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    RightDown,
    RightUp,
    DownLeft,
    DownRight,
    LeftUp,
    LeftDown,
    Middle, // This is not technically in a wall, it's a space surrounded by walls
}

// Each line of input is in the form of: D C (#aabbcc)
// Where D is direction (U, D, L, R)
// C is the count of how many steps to take in that direction
// And the part in brackets is hex color is the color to draw the line
fn wall_map_from_input(input: &str) -> (HashMap<(i32, i32), WallType>, i32, i32, i32, i32) {
    use WallType::*;
    let mut wall_map = HashMap::new();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    let mut x = 0;
    let mut y = 0;
    let mut last_type = None;
    let mut first_type = None;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction_char = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        // let color = u32::from_str_radix(&color[1..], 16).unwrap();
        let (dx, dy, wall_type) = match direction_char {
            "U" => (0, -1, Up),
            "D" => (0, 1, Down),
            "L" => (-1, 0, Left),
            "R" => (1, 0, Right),
            _ => panic!("Invalid direction"),
        };
        if first_type.is_none() {
            first_type = Some(wall_type);
        }
        for i in 0..count {
            if i == 0 {
                if let Some(last_type) = last_type {
                    // We are at a corner so update the last point
                    let corner_type = match (last_type, wall_type) {
                        (Left, Up) => LeftUp,
                        (Left, Down) => LeftDown,
                        (Right, Up) => RightUp,
                        (Right, Down) => RightDown,
                        (Up, Left) => UpLeft,
                        (Up, Right) => UpRight,
                        (Down, Left) => DownLeft,
                        (Down, Right) => DownRight,
                        _ => {
                            println!("Last type: {:?}, wall type: {:?}", last_type, wall_type);
                            panic!("Invalid corner")
                        }
                    };
                    wall_map.insert((x, y), corner_type);
                }
            };

            x += dx;
            y += dy;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
            wall_map.insert((x, y), wall_type);
        }
        last_type = Some(wall_type);
    }

    // Fix up the last corner
    if let (Some(wall_type), Some(last_type)) = (first_type, last_type) {
        // We are at a corner so update the last point
        let corner_type = match (last_type, wall_type) {
            (Left, Up) => LeftUp,
            (Left, Down) => LeftDown,
            (Right, Up) => RightUp,
            (Right, Down) => RightDown,
            (Up, Left) => LeftUp,
            (Up, Right) => UpRight,
            (Down, Left) => DownLeft,
            (Down, Right) => DownRight,
            _ => {
                println!("Last type: {:?}, wall type: {:?}", last_type, wall_type);
                panic!("Invalid corner")
            }
        };
        wall_map.insert((x, y), corner_type);
    }
    (wall_map, min_x, max_x, min_y, max_y)
}

fn print_wall_map(
    wall_map: &HashMap<(i32, i32), WallType>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) {
    use WallType::*;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(direction) = wall_map.get(&(x, y)) {
                match direction {
                    Up => print!("|"),
                    Down => print!("|"),
                    Left => print!("-"),
                    Right => print!("-"),
                    UpRight | LeftDown => print!("┌"),
                    RightDown | UpLeft => print!("┐"),
                    DownLeft | RightUp => print!("┘"),
                    LeftUp | DownRight => print!("└"),
                    Middle => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn hole_map(
    wall_map: &HashMap<(i32, i32), WallType>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> HashMap<(i32, i32), WallType> {
    use WallType::*;
    let mut hole_map = HashMap::new();
    for y in min_y..=max_y {
        let mut inside = false;
        for x in min_x..=max_x {
            if let Some(direction) = wall_map.get(&(x, y)) {
                // We are on a wall, so we count that
                hole_map.insert((x, y), *direction);
                match (inside, direction) {
                    (true, Up) => (),
                    (true, Down) => inside = false,
                    (true, Left) => (),
                    (true, Right) => (),
                    (true, UpRight) => (),
                    (true, RightDown) => inside = false,
                    (true, DownLeft) => inside = false,
                    (true, LeftUp) => (),
                    (true, Middle) => (),
                    (false, Up) => inside = true,
                    (false, Down) => (),
                    (false, Left) => (),
                    (false, Right) => (),
                    (false, UpRight) => inside = true,
                    (false, RightDown) => (),
                    (false, DownLeft) => (),
                    (false, LeftUp) => inside = true,
                    (false, Middle) => (),
                    _ => (),
                }
            } else if inside {
                hole_map.insert((x, y), Middle);
            }
        }
    }
    hole_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let (wall_map, min_x, max_x, min_y, max_y) = wall_map_from_input(input);
    print_wall_map(&wall_map, min_x, max_x, min_y, max_y);
    let hole_map = hole_map(&wall_map, min_x, max_x, min_y, max_y);
    println!("Hole map:");
    print_wall_map(&hole_map, min_x, max_x, min_y, max_y);
    Some(hole_map.len() as u32)
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

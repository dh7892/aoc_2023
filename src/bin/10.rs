advent_of_code::solution!(10);

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Position {
    row: i32,
    column: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PipeType {
    EastWest,
    NorthSouth,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
    Start,
}

struct Pipe {
    distance_from_start: u32,
    pipe_type: PipeType,
}

// Given a map of pipes, a current position, and a previous position, return the next position.
// Remember row index increases as you go down, and column index increases as you go right.
fn next_pipe(
    map: &HashMap<Position, Pipe>,
    current: &Position,
    previous: &Position,
) -> Option<Position> {
    let pipe = map.get(current)?;
    let next_pos = match pipe.pipe_type {
        PipeType::EastWest => {
            if current.column > previous.column {
                Position {
                    row: current.row,
                    column: current.column + 1,
                }
            } else {
                Position {
                    row: current.row,
                    column: current.column - 1,
                }
            }
        }
        PipeType::NorthSouth => {
            if current.row > previous.row {
                // We are going south.
                Position {
                    row: current.row + 1,
                    column: current.column,
                }
            } else {
                Position {
                    row: current.row - 1,
                    column: current.column,
                }
            }
        }
        PipeType::SouthEast => {
            if current.column < previous.column {
                // We came from East
                // So we go South
                Position {
                    row: current.row + 1,
                    column: current.column,
                }
            } else {
                Position {
                    row: current.row,
                    column: current.column + 1,
                }
            }
        }
        PipeType::SouthWest => {
            if current.column > previous.column {
                // We came from West
                // So we go South
                Position {
                    row: current.row + 1,
                    column: current.column,
                }
            } else {
                Position {
                    row: current.row,
                    column: current.column - 1,
                }
            }
        }
        PipeType::NorthEast => {
            if current.row > previous.row {
                // We came from the north
                // So we go East
                Position {
                    row: current.row,
                    column: current.column + 1,
                }
            } else {
                Position {
                    row: current.row - 1,
                    column: current.column,
                }
            }
        }
        PipeType::NorthWest => {
            if current.row > previous.row {
                // We came from the north
                // So we go West
                Position {
                    row: current.row,
                    column: current.column - 1,
                }
            } else {
                Position {
                    row: current.row - 1,
                    column: current.column,
                }
            }
        }
        PipeType::Start => return None,
    };
    Some(next_pos)
}

// Convert input into a map of pipes and return the start position.
fn input_to_map(input: &str) -> (HashMap<Position, Pipe>, Position) {
    let mut map = HashMap::new();
    let mut start = Position { row: 0, column: 0 };
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            let pipe_type = match c {
                '-' => PipeType::EastWest,
                '|' => PipeType::NorthSouth,
                'S' => {
                    start = Position {
                        row: row as i32,
                        column: column as i32,
                    };
                    PipeType::Start
                }
                'F' => PipeType::SouthEast,
                '7' => PipeType::SouthWest,
                'L' => PipeType::NorthEast,
                'J' => PipeType::NorthWest,
                _ => continue,
            };
            map.insert(
                Position {
                    row: row as i32,
                    column: column as i32,
                },
                Pipe {
                    distance_from_start: 0,
                    pipe_type,
                },
            );
        }
    }
    (map, start)
}

fn find_path(map: &HashMap<Position, Pipe>, start: &Position) -> Option<Vec<Position>> {
    let try_these = vec![
        Position {
            row: start.row,
            column: start.column + 1,
        },
        Position {
            row: start.row,
            column: start.column - 1,
        },
        Position {
            row: start.row + 1,
            column: start.column,
        },
        Position {
            row: start.row - 1,
            column: start.column,
        },
    ];

    for pos in try_these {
        if let Some(path) = find_path_from_start(map, *start, pos) {
            return Some(path);
        }
    }
    None
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Path {
    path: Vec<Position>,
    map: HashMap<Position, Pipe>,
    min_row: i32,
    max_row: i32,
    min_column: i32,
    max_column: i32,
}

impl Path {
    fn new(path: Vec<Position>, map: HashMap<Position, Pipe>) -> Self {
        let mut min_row = i32::max_value();
        let mut max_row = i32::min_value();
        let mut min_column = i32::max_value();
        let mut max_column = i32::min_value();
        for pos in &path {
            if pos.row < min_row {
                min_row = pos.row;
            }
            if pos.row > max_row {
                max_row = pos.row;
            }
            if pos.column < min_column {
                min_column = pos.column;
            }
            if pos.column > max_column {
                max_column = pos.column;
            }
        }
        Self {
            path,
            map,
            min_row,
            max_row,
            min_column,
            max_column,
        }
    }
    // Return true if the given point is contained within the path
    // We probe in each cardinal direction from the point and count the number of times we cross
    // If the number of crossings in each direction is odd, we are inside the path.
    fn point_contained(&self, pos: &Position) -> bool {
        for direction in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            // If the point is part of the pipe, we can early return false
            if let Some(_) = self.path.iter().find(|p| *p == pos) {
                return false;
            }
            let crossings = self.crossings_from_point(*pos, *direction);
            if crossings % 2 == 0 {
                return false;
            }
        }
        true
    }

    // Check the positon. If there is a pipe segment at that position, and it's
    // in our main pipe loop (i.e. not just a random pipe segment somewhere else),
    // return the pipe type.
    // Else None
    fn pipe_type_at_position(&self, pos: &Position) -> Option<PipeType> {
        let pos_on_path = self.path.iter().find(|p| *p == pos);
        if let Some(_) = pos_on_path {
            return Some(self.map.get(pos)?.pipe_type);
        }
        None
    }

    fn is_bounding_point(&self, pos: &Position, direction: Direction) -> bool {
        let pos_on_path = self.path.iter().find(|p| *p == pos);
        match pos_on_path {
            Some(_) => {
                let path_type = self.map.get(pos).unwrap().pipe_type;
                match direction {
                    Direction::North => {
                        if let PipeType::EastWest = path_type {
                            return true;
                        }
                    }
                    Direction::South => {
                        if let PipeType::EastWest = path_type {
                            return true;
                        }
                    }
                    Direction::East => {
                        if let PipeType::NorthSouth = path_type {
                            return true;
                        }
                    }
                    Direction::West => {
                        if let PipeType::NorthSouth = path_type {
                            return true;
                        }
                    }
                }
            }
            None => {
                return false;
            }
        }
        false
    }

    fn crossings_from_point(&self, position: Position, direction: Direction) -> u32 {
        let mut crossings = Vec::new();
        let mut position = position;

        let (mut row_delta, mut column_delta) = match direction {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };

        while position.row >= self.min_row
            && position.row <= self.max_row
            && position.column >= self.min_column
            && position.column <= self.max_column
        {
            position.row += row_delta;
            position.column += column_delta;
            if let Some(pipe_type) = self.pipe_type_at_position(&position) {
                crossings.push(pipe_type);
            }
        }

        // Now count up the numbers of N, E, S and W connections within the pipes we cross
        let (mut north, mut south, mut east, mut west) = (0, 0, 0, 0);
        for p in crossings {
            match p {
                PipeType::NorthSouth => {
                    north += 1;
                    south += 1;
                }
                PipeType::EastWest => {
                    east += 1;
                    west += 1;
                }
                PipeType::NorthEast => {
                    north += 1;
                    east += 1;
                }
                PipeType::NorthWest => {
                    north += 1;
                    west += 1;
                }
                PipeType::SouthEast => {
                    south += 1;
                    east += 1;
                }
                PipeType::SouthWest => {
                    south += 1;
                    west += 1;
                }
                PipeType::Start => {}
            }
        }
        match direction {
            Direction::North | Direction::South => east.min(west),
            Direction::East | Direction::West => north.min(south),
        }
    }

    fn bounding_points(&self) -> Vec<Position> {
        let mut points = Vec::new();
        for row in self.min_row..=self.max_row {
            for column in self.min_column..=self.max_column {
                let pos = Position { row, column };
                if self.point_contained(&pos) {
                    points.push(pos);
                }
            }
        }
        points
    }
}

fn find_path_from_start(
    map: &HashMap<Position, Pipe>,
    previous: Position,
    current: Position,
) -> Option<Vec<Position>> {
    let mut path = Vec::new();
    let mut previous = previous;
    let mut current = current;
    loop {
        path.push(current);
        if let PipeType::Start = map.get(&current)?.pipe_type {
            break;
        }
        let next = next_pipe(map, &current, &previous)?;
        previous = current;
        current = next;
    }
    Some(path)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = input_to_map(input);
    let path = find_path(&map, &start)?;
    // We want to know the position of the path that is the furthest from the start.
    // This is half the length of the path.
    Some((path.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = input_to_map(input);
    let path = find_path(&map, &start)?;
    let map_path = Path::new(path, map);
    let bounding_points = map_path.bounding_points();
    Some(bounding_points.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = part_one(input);
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_b() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = part_one(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_b() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part_two(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_single_point() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let (map, start) = input_to_map(input);
        let path = find_path(&map, &start).unwrap();
        let map_path = Path::new(path, map);
        let p = Position { row: 6, column: 2 };
        let result = map_path.point_contained(&p);
        assert_eq!(result, true);
    }
}

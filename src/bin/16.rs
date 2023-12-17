advent_of_code::solution!(16);

use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum MirrorSplitter {
    Forward,    // /
    Backward,   // \
    Horizontal, // -
    Vertical,   // |
}

// The head of a beam of light currently as a point in the grid and a direction it's moving it
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct LightBeam {
    row: usize,
    col: usize,
    direction: Direction,
}

struct MirrorMap {
    mirrors: HashMap<(usize, usize), MirrorSplitter>,
    cols: usize,
    rows: usize,
    beams: Vec<LightBeam>,
    lit_points: HashSet<(usize, usize)>,
    // Also count the number of times we've seen a beam at a point in a direction
    // This allows us to detect cycles
    beam_entries: HashSet<LightBeam>,
}

impl MirrorMap {
    fn new(width: usize, height: usize) -> Self {
        let beams = vec![LightBeam {
            row: 0,
            col: 0,
            direction: Direction::East,
        }];
        let mut lit_points = HashSet::new();
        lit_points.insert((0, 0));
        let mut beam_entries = HashSet::new();
        beam_entries.insert(LightBeam {
            row: 0,
            col: 0,
            direction: Direction::East,
        });

        Self {
            mirrors: HashMap::new(),
            cols: width,
            rows: height,
            beams,
            lit_points,
            beam_entries,
        }
    }

    fn initialise_beam(&mut self, start_beam: LightBeam) {
        self.beams = vec![start_beam];
        self.lit_points = HashSet::new();
        self.lit_points.insert((start_beam.row, start_beam.col));
        self.beam_entries = HashSet::new();
        self.beam_entries.insert(start_beam);
    }

    fn add_mirror(&mut self, row: usize, col: usize, mirror: MirrorSplitter) {
        self.mirrors.insert((row, col), mirror);
        if row > self.rows {
            self.rows = row;
        }
        if col > self.cols {
            self.cols = col;
        }
    }

    fn get_mirror(&self, row: usize, col: usize) -> Option<&MirrorSplitter> {
        self.mirrors.get(&(row, col))
    }

    fn bounded_move(&self, row: usize, col: usize, direction: Direction) -> Option<(usize, usize)> {
        let (row, col) = match direction {
            Direction::North => {
                if row == 0 {
                    return None;
                } else {
                    (row - 1, col)
                }
            }
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => {
                if col == 0 {
                    return None;
                } else {
                    (row, col - 1)
                }
            }
        };
        if row <= self.rows && col <= self.cols {
            Some((row, col))
        } else {
            None
        }
    }

    fn move_beams(&mut self) {
        use Direction::*;
        let mut new_beams = Vec::new();

        for beam in self.beams.iter() {
            if let Some(mirror_splitter) = self.get_mirror(beam.row, beam.col) {
                // We hit a mirror or a splitter so we might need to change direction or add more beams
                let next_directions = match (mirror_splitter, beam.direction) {
                    (MirrorSplitter::Forward, Direction::North) => vec![East],
                    (MirrorSplitter::Forward, Direction::East) => vec![North],
                    (MirrorSplitter::Forward, Direction::South) => vec![West],
                    (MirrorSplitter::Forward, Direction::West) => vec![South],
                    (MirrorSplitter::Backward, Direction::North) => vec![West],
                    (MirrorSplitter::Backward, Direction::East) => vec![South],
                    (MirrorSplitter::Backward, Direction::South) => vec![East],
                    (MirrorSplitter::Backward, Direction::West) => vec![North],
                    (MirrorSplitter::Horizontal, Direction::North) => vec![West, East], // Split the beam in two
                    (MirrorSplitter::Horizontal, Direction::East) => vec![East], // Beam unchanged
                    (MirrorSplitter::Horizontal, Direction::South) => vec![West, East], // Split the beam in two
                    (MirrorSplitter::Horizontal, Direction::West) => vec![West], // Beam unchanged
                    (MirrorSplitter::Vertical, Direction::North) => vec![North], // Beam unchanged
                    (MirrorSplitter::Vertical, Direction::East) => vec![North, South], // Split the beam in two
                    (MirrorSplitter::Vertical, Direction::South) => vec![South], // Beam unchanged
                    (MirrorSplitter::Vertical, Direction::West) => vec![North, South], // Split the beam in two
                };
                for d in next_directions {
                    if let Some((row, col)) = self.bounded_move(beam.row, beam.col, d) {
                        if !self.beam_entries.insert(LightBeam {
                            row,
                            col,
                            direction: d,
                        }) {
                            // We've already seen this beam entry, so we're in a cycle
                            continue;
                        }
                        new_beams.push(LightBeam {
                            row,
                            col,
                            direction: d,
                        });
                        self.lit_points.insert((row, col));
                    }
                }
            } else {
                // We are just moving through space as normal
                if let Some((row, col)) = self.bounded_move(beam.row, beam.col, beam.direction) {
                    if !self.beam_entries.insert(LightBeam {
                        row,
                        col,
                        direction: beam.direction,
                    }) {
                        // We've already seen this beam entry, so we're in a cycle
                        continue;
                    }
                    new_beams.push(LightBeam {
                        row,
                        col,
                        direction: beam.direction,
                    });
                    self.lit_points.insert((row, col));
                }
            }
        }
        self.beams = new_beams;
    }

    fn move_all_beams(&mut self) {
        while !self.beams.is_empty() {
            self.move_beams();
        }
    }

    fn num_lit_points(&self) -> usize {
        self.lit_points.len()
    }
}

impl Debug for MirrorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // First, write out the mirrors
        writeln!(f)?;
        for row in 0..=self.rows {
            for col in 0..=self.cols {
                if let Some(mirror) = self.get_mirror(row, col) {
                    match mirror {
                        MirrorSplitter::Forward => write!(f, "/")?,
                        MirrorSplitter::Backward => write!(f, "\\")?,
                        MirrorSplitter::Horizontal => write!(f, "-")?,
                        MirrorSplitter::Vertical => write!(f, "|")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;

        // Now, write out the beams
        for row in 0..=self.rows {
            for col in 0..=self.cols {
                if self.lit_points.contains(&(row, col)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> MirrorMap {
    let mut map = MirrorMap::new(0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '/' => map.add_mirror(row, col, MirrorSplitter::Forward),
                '\\' => map.add_mirror(row, col, MirrorSplitter::Backward),
                '-' => map.add_mirror(row, col, MirrorSplitter::Horizontal),
                '|' => map.add_mirror(row, col, MirrorSplitter::Vertical),
                _ => {}
            }
        }
    }
    map
}

fn permute_start_beams(map: &MirrorMap) -> Vec<LightBeam> {
    let mut result = Vec::new();
    // Start beams going south from top row
    for col in 0..=map.cols {
        result.push(LightBeam {
            row: 0,
            col,
            direction: Direction::South,
        });
    }
    // Start beams going east from left column
    for row in 0..=map.rows {
        result.push(LightBeam {
            row,
            col: 0,
            direction: Direction::East,
        });
    }
    // Start beams going north from bottom row
    for col in 0..=map.cols {
        result.push(LightBeam {
            row: map.rows,
            col,
            direction: Direction::North,
        });
    }
    // Start beams going west from right column
    for row in 0..=map.rows {
        result.push(LightBeam {
            row,
            col: map.cols,
            direction: Direction::West,
        });
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut mirror_map = parse_input(input);
    mirror_map.move_all_beams();
    Some(mirror_map.num_lit_points())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut mirror_map = parse_input(input);
    let result = permute_start_beams(&mirror_map)
        .into_iter()
        .map(|start_beam| {
            mirror_map.initialise_beam(start_beam);
            mirror_map.move_all_beams();
            mirror_map.num_lit_points()
        })
        .max();
    Some(result?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}

advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Pattern {
    num_rows: usize,
    num_columns: usize,
    // Each row is a vector column values where we have a rock (#)
    rows: Vec<Vec<usize>>,
    // Each column is a vector of row values where we have a rock (#)
    // This is a duplicate of the data in `rows` but it makes it easier to
    // iterate over columns.
    columns: Vec<Vec<usize>>,
}

impl Pattern {
    fn new(num_rows: usize, num_columns: usize) -> Self {
        Self {
            num_rows,
            num_columns,
            rows: vec![vec![]; num_rows],
            columns: vec![vec![]; num_columns],
        }
    }

    fn parse(input: &str) -> Self {
        let lines = input.lines();
        let num_rows = lines.clone().count();
        let num_columns = input.lines().next().unwrap().chars().count();
        let mut pattern = Self::new(num_rows, num_columns);

        for (row, line) in lines.enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    pattern.rows[row].push(col);
                    pattern.columns[col].push(row);
                }
            }
        }
        pattern
    }

    // Return true if there is a line of symmetry between the given row and the subsequent row
    fn row_is_symmetry(&self, symmetry_row: usize) -> bool {
        for i in 0..=symmetry_row {
            let row = &self.rows[i];
            let dist_to_symmetry = symmetry_row - i + 1;
            if let Some(reflected_row) = self.rows.get(symmetry_row + dist_to_symmetry) {
                if row != reflected_row {
                    return false;
                }
            }
        }
        true
    }
    fn column_is_symmetry(&self, symmetry_column: usize) -> bool {
        for i in 0..=symmetry_column {
            let column = &self.columns[i];
            let dist_to_symmetry = symmetry_column - i + 1;
            if let Some(reflected_column) = self.columns.get(symmetry_column + dist_to_symmetry) {
                if column != reflected_column {
                    return false;
                }
            }
        }
        true
    }
    // If we provide an exclude_score, we will not return a score that matches it.
    fn find_symmetry_score(&self, exclude_score: Option<usize>) -> usize {
        for row in 0..self.num_rows - 1 {
            if self.row_is_symmetry(row) {
                let score = (row + 1) * 100;
                if let Some(exclude_score) = exclude_score {
                    if score == exclude_score {
                        continue;
                    }
                }
                return score;
            }
        }
        for column in 0..self.num_columns - 1 {
            if self.column_is_symmetry(column) {
                let score = column + 1;
                if let Some(exclude_score) = exclude_score {
                    if score == exclude_score {
                        continue;
                    }
                }
                return score;
            }
        }
        0
    }

    // Try swapping between rock/ash at each location in turn until we find on that creates a valid reflection
    // somewhere in the pattern. Return the score of the first valid reflection we find.
    fn smudge_permutations(&self) -> usize {
        let old_symmetry_score = self.find_symmetry_score(None);
        for row in 0..self.num_rows {
            for col in 0..self.num_columns {
                let mut new_pattern = Pattern::new(self.num_rows, self.num_columns);
                // Fix up our rows to change the smudge
                for (i, old_row) in self.rows.iter().enumerate() {
                    if i == row {
                        let mut new_row = old_row.clone();
                        // If this row contains a rock at this column, change it to ash
                        if let Some(index) = new_row.iter().position(|&x| x == col) {
                            new_row.remove(index);
                        } else {
                            new_row.push(col);
                        }
                        // Also, sort the row so that the columns are in order
                        new_row.sort();
                        new_pattern.rows[i] = new_row;
                    } else {
                        new_pattern.rows[i] = old_row.clone();
                    }
                }
                // Now fix up our columns to match the new rows
                for (i, old_column) in self.columns.iter().enumerate() {
                    if i == col {
                        let mut new_column = old_column.clone();
                        // If this column contains a rock at this row, change it to ash
                        if let Some(index) = new_column.iter().position(|&x| x == row) {
                            new_column.remove(index);
                        } else {
                            new_column.push(row);
                        }
                        // Also, sort the column so that the rows are in order
                        new_column.sort();
                        new_pattern.columns[i] = new_column;
                    } else {
                        new_pattern.columns[i] = old_column.clone();
                    }
                }
                let score = new_pattern.find_symmetry_score(Some(old_symmetry_score));
                if score > 0 {
                    return score;
                }
            }
        }
        0
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .split("\n\n")
        .map(|pattern_str| Pattern::parse(pattern_str).find_symmetry_score(None))
        .sum::<usize>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .split("\n\n")
        .map(|pattern_str| Pattern::parse(pattern_str).smudge_permutations())
        .sum::<usize>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATTERN_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    const PATTERN_2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const PATTERN_3: &str = "#...##..#
#...##..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_parse_pattern() {
        let input = PATTERN_1;
        let pattern = Pattern::parse(input);
        assert_eq!(pattern.num_rows, 7);
        assert_eq!(pattern.num_columns, 9);
        assert_eq!(pattern.rows[0], vec![0, 2, 3, 6, 7]);
        assert_eq!(pattern.rows[6], vec![0, 2, 4, 5, 7]);
        assert_eq!(pattern.columns[0], vec![0, 2, 3, 6]);
        assert_eq!(pattern.columns[2], vec![0, 1, 4, 5, 6]);
        // There should be no symmetry rows in this pattern
        for row in 0..pattern.num_rows - 1 {
            assert!(!pattern.row_is_symmetry(row));
        }
        assert!(!pattern.column_is_symmetry(0));
        assert!(!pattern.column_is_symmetry(1));
        assert!(!pattern.column_is_symmetry(2));
        assert!(!pattern.column_is_symmetry(3));
        // This is our line of symmetry for this pattern
        assert!(pattern.column_is_symmetry(4));
        assert!(!pattern.column_is_symmetry(5));

        assert_eq!(pattern.find_symmetry_score(None), 5);
    }
    #[test]
    fn test_parse_pattern_2() {
        let input = PATTERN_2;
        let pattern = Pattern::parse(input);
        assert_eq!(pattern.find_symmetry_score(None), 400);
    }
    #[test]
    fn test_parse_pattern_3() {
        let input = PATTERN_3;
        let pattern = Pattern::parse(input);
        assert_eq!(pattern.find_symmetry_score(None), 100);
    }
    #[test]
    fn test_parse_pattern_smudge() {
        let input = PATTERN_1;
        let pattern = Pattern::parse(input);
        assert_eq!(pattern.smudge_permutations(), 300);
    }
    #[test]
    fn test_parse_pattern_smudge_2() {
        let input = PATTERN_2;
        let pattern = Pattern::parse(input);
        assert_eq!(pattern.smudge_permutations(), 100);
    }
}

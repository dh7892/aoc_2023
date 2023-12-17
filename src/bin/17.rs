use petgraph::{algo::dijkstra, graph::Node, graph::NodeIndex};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(17);

use petgraph::graph::DiGraph;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    orientation: Orientation,
    row: usize,
    col: usize,
}

fn graph_from_input_part_1(
    input: &str,
) -> (DiGraph<Location, u32>, NodeIndex, NodeIndex, NodeIndex) {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut locations = HashMap::new();
    let mut start: Option<NodeIndex> = None;
    let mut end_1: Option<NodeIndex> = None;
    let mut end_2: Option<NodeIndex> = None;
    for (row, line) in input.lines().enumerate() {
        rows = row + 1;
        for (col, c) in line.chars().enumerate() {
            if col >= cols {
                cols = col + 1;
            }
            let loss = c.to_digit(10).unwrap_or(0);
            locations.insert((row, col), loss);
            let location = Location {
                orientation: Orientation::Horizontal,
                row,
                col,
            };

            let index = graph.add_node(location);
            if (row, col) == (0, 0) {
                start = Some(index);
            }
            if (row, col) == (rows - 1, cols - 1) {
                end_1 = Some(index);
            }
            node_map.insert(location, index);

            let location = Location {
                orientation: Orientation::Vertical,
                row,
                col,
            };
            let index = graph.add_node(location);
            if (row, col) == (rows - 1, cols - 1) {
                end_2 = Some(index);
            }
            node_map.insert(location, index);
        }
    }
    // Now loop over all locations and add edges
    for row in 0..rows {
        for col in 0..cols {
            let min_row = row.saturating_sub(3);
            let max_row = rows.min(row + 4);
            let min_col = col.saturating_sub(3);
            let max_col = cols.min(col + 4);
            // Add horizontal edges to vertical nodes
            let mut loss = 0;
            let from = Location {
                orientation: Orientation::Horizontal,
                row,
                col,
            };
            let from_index = node_map.get(&from).unwrap();

            for r in (min_row..row).rev() {
                loss += locations.get(&(r, col)).unwrap();
                let to = Location {
                    orientation: Orientation::Vertical,
                    row: r,
                    col,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }
            loss = 0;
            for r in row..max_row {
                if r == row {
                    continue;
                }
                loss += locations.get(&(r, col)).unwrap();
                let to = Location {
                    orientation: Orientation::Vertical,
                    row: r,
                    col,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }

            loss = 0;
            let from = Location {
                orientation: Orientation::Vertical,
                row,
                col,
            };
            let from_index = node_map.get(&from).unwrap();
            for c in (min_col..col).rev() {
                loss += locations.get(&(row, c)).unwrap();
                let to = Location {
                    orientation: Orientation::Horizontal,
                    row,
                    col: c,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }

            loss = 0;
            for c in col..max_col {
                if c == col {
                    continue;
                }
                loss += locations.get(&(row, c)).unwrap();
                let to = Location {
                    orientation: Orientation::Horizontal,
                    row,
                    col: c,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }
        }
    }

    (graph, start.unwrap(), end_1.unwrap(), end_2.unwrap())
}

// Minimum move of 4, max of 10
fn graph_from_input_part_2(
    input: &str,
) -> (DiGraph<Location, u32>, NodeIndex, NodeIndex, NodeIndex) {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut locations = HashMap::new();
    let mut start: Option<NodeIndex> = None;
    let mut end_1: Option<NodeIndex> = None;
    let mut end_2: Option<NodeIndex> = None;
    for (row, line) in input.lines().enumerate() {
        rows = row + 1;
        for (col, c) in line.chars().enumerate() {
            if col >= cols {
                cols = col + 1;
            }
            let loss = c.to_digit(10).unwrap_or(0);
            locations.insert((row, col), loss);
            let location = Location {
                orientation: Orientation::Horizontal,
                row,
                col,
            };

            let index = graph.add_node(location);
            if (row, col) == (0, 0) {
                start = Some(index);
            }
            if (row, col) == (rows - 1, cols - 1) {
                end_1 = Some(index);
            }
            node_map.insert(location, index);

            let location = Location {
                orientation: Orientation::Vertical,
                row,
                col,
            };
            let index = graph.add_node(location);
            if (row, col) == (rows - 1, cols - 1) {
                end_2 = Some(index);
            }
            node_map.insert(location, index);
        }
    }
    // Now loop over all locations and add edges
    for row in 0..rows {
        for col in 0..cols {
            if row == 4 && col == 8 {
                println!("here");
            }
            let min_row = row.saturating_sub(10);
            let max_row = row.saturating_sub(3);
            // Add horizontal edges to vertical nodes
            // Even though we can't stop at the first 4 places, we still need to count the loss
            let mut loss = (max_row..row)
                .map(|r| locations.get(&(r, col)).unwrap())
                .sum();
            let from = Location {
                orientation: Orientation::Horizontal,
                row,
                col,
            };
            let from_index = node_map.get(&from).unwrap();

            for r in (min_row..max_row).rev() {
                loss += locations.get(&(r, col)).unwrap();
                let to = Location {
                    orientation: Orientation::Vertical,
                    row: r,
                    col,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }
            let min_row = rows.min(row + 4);
            let max_row = rows.min(row + 11);
            loss = (row..min_row)
                .filter(|r| *r != row)
                .map(|r| locations.get(&(r, col)).unwrap())
                .sum();
            for r in min_row..max_row {
                if r == row {
                    continue;
                }
                loss += locations.get(&(r, col)).unwrap();
                let to = Location {
                    orientation: Orientation::Vertical,
                    row: r,
                    col,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }

            let min_col = col.saturating_sub(10);
            let max_col = col.saturating_sub(3);
            loss = (max_col..col)
                .map(|c| locations.get(&(row, c)).unwrap())
                .sum();
            let from = Location {
                orientation: Orientation::Vertical,
                row,
                col,
            };
            let from_index = node_map.get(&from).unwrap();
            for c in (min_col..max_col).rev() {
                loss += locations.get(&(row, c)).unwrap();
                let to = Location {
                    orientation: Orientation::Horizontal,
                    row,
                    col: c,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }

            let min_col = cols.min(col + 4);
            let max_col = cols.min(col + 11);
            loss = (col..min_col)
                .filter(|c| *c != col)
                .map(|c| locations.get(&(row, c)).unwrap())
                .sum();
            for c in min_col..max_col {
                if c == col {
                    continue;
                }
                loss += locations.get(&(row, c)).unwrap();
                let to = Location {
                    orientation: Orientation::Horizontal,
                    row,
                    col: c,
                };
                let to_index = node_map.get(&to).unwrap();

                graph.add_edge(*from_index, *to_index, loss);
            }
        }
    }

    (graph, start.unwrap(), end_1.unwrap(), end_2.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, start, end_1, end_2) = graph_from_input_part_1(input);
    let node_map_1 = dijkstra(&graph, start, Some(end_1), |e| *e.weight());
    let node_map_2 = dijkstra(&graph, start, Some(end_2), |e| *e.weight());
    let dist_1 = node_map_1.get(&end_1)?;
    let dist_2 = node_map_2.get(&end_2)?;

    Some(*dist_1.min(dist_2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, start_1, end_1, end_2) = graph_from_input_part_2(input);
    let start_2 = find_node_index(
        &graph,
        Location {
            orientation: Orientation::Vertical,
            row: 0,
            col: 0,
        },
    )?;
    let node_map_1_1 = dijkstra(&graph, start_1, Some(end_1), |e| *e.weight());
    let node_map_2_1 = dijkstra(&graph, start_1, Some(end_2), |e| *e.weight());
    let node_map_1_2 = dijkstra(&graph, start_2, Some(end_1), |e| *e.weight());
    let node_map_2_2 = dijkstra(&graph, start_2, Some(end_2), |e| *e.weight());
    let dist_1_1 = node_map_1_1.get(&end_1)?;
    let dist_2_1 = node_map_2_1.get(&end_2)?;
    let dist_1_2 = node_map_1_2.get(&end_1)?;
    let dist_2_2 = node_map_2_2.get(&end_2)?;
    let best = dist_1_1.min(dist_2_1).min(dist_1_2).min(dist_2_2);

    Some(*best)
}

fn find_node_index(graph: &DiGraph<Location, u32>, location: Location) -> Option<NodeIndex> {
    for node in graph.node_indices() {
        if graph[node] == location {
            return Some(node);
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[rstest::rstest]
    #[case(11, 11, Orientation::Horizontal, 6)]
    #[case(11, 12, Orientation::Horizontal, 3)]
    #[case(11, 11, Orientation::Vertical, 8)]
    #[case(7, 12, Orientation::Horizontal, 28)]
    fn test_from_point(
        #[case] row: u32,
        #[case] col: u32,
        #[case] orientation: Orientation,
        #[case] expected: u32,
    ) {
        let (graph, _start, end_1, end_2) =
            graph_from_input_part_1(&advent_of_code::template::read_file("examples", DAY));
        let location = Location {
            orientation,
            row: row as usize,
            col: col as usize,
        };
        let node_index = find_node_index(&graph, location).unwrap();
        let node_map = dijkstra(&graph, node_index, Some(end_1), |e| *e.weight());
        let dist_1 = node_map.get(&end_1).unwrap();
        let node_map = dijkstra(&graph, node_index, Some(end_2), |e| *e.weight());
        let dist_2 = node_map.get(&end_2).unwrap();
        let dist = dist_1.min(dist_2);
        assert_eq!(*dist, expected);
    }

    #[rstest::rstest]
    #[case(4, 12, Orientation::Horizontal, 34)]
    #[case(4, 8, Orientation::Vertical, 55)]
    #[case(0, 8, Orientation::Horizontal, 73)]
    #[case(0, 0, Orientation::Vertical, 94)]
    fn test_from_point_part_2(
        #[case] row: u32,
        #[case] col: u32,
        #[case] orientation: Orientation,
        #[case] expected: u32,
    ) {
        let (graph, _start, end_1, end_2) =
            graph_from_input_part_2(&advent_of_code::template::read_file("examples", DAY));
        let location = Location {
            orientation,
            row: row as usize,
            col: col as usize,
        };
        let node_index = find_node_index(&graph, location).unwrap();
        let node_map = dijkstra(&graph, node_index, Some(end_1), |e| *e.weight());
        let dist_1 = node_map.get(&end_1).unwrap();
        let node_map = dijkstra(&graph, node_index, Some(end_2), |e| *e.weight());
        let dist_2 = node_map.get(&end_2).unwrap();
        let dist = dist_1.min(dist_2);
        assert_eq!(*dist, expected);
    }
}

use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

// bit filthy hard coding these, but it's probably fine and makes
// searching for the start / end points a bit easier
const SOURCE_ID: u32 = 0;
const TARGET_ID: u32 = 9_999;

const SOURCE_HEIGHT: u32 = 0;
const TARGET_HEIGHT: u32 = 25;

#[derive(Clone)]
struct Node {
    id: u32,
    height: u32,
}

fn id(c: char, row: usize, col: usize, line_length: usize) -> u32 {
    match c {
        'S' => SOURCE_ID,
        'E' => TARGET_ID,
        _ => ((row * line_length) + col + 1) as u32,
    }
}

fn height(c: char) -> u32 {
    match c {
        'S' => SOURCE_HEIGHT,
        'E' => TARGET_HEIGHT,
        _ => c as u32 - 'a' as u32,
    }
}

fn line_to_vec(line: &str, row: usize) -> Vec<Node> {
    line.chars()
        .enumerate()
        .map(|(idx, c)| Node {
            id: id(c, row, idx, line.len()),
            height: height(c),
        })
        .collect()
}

fn find_edges_for_coordinate(
    grid: &Vec<Vec<Node>>,
    row_idx: usize,
    col_idx: usize,
) -> Vec<(u32, u32)> {
    let source = &grid[row_idx as usize][col_idx as usize];

    let row = row_idx as i32;
    let col = col_idx as i32;

    let raw_search_coords = vec![
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ];

    // only keep coords that are in bounds
    let filtered_search_coords: Vec<(i32, i32)> = raw_search_coords
        .into_iter()
        .filter(|(r, c)| *r >= 0 && *r < grid.len() as i32 && *c >= 0 && *c < grid[0].len() as i32)
        .collect();

    // get the edges where there is a valid path
    let edges: Vec<(u32, u32)> = filtered_search_coords
        .into_iter()
        .filter_map(|(r, c)| {
            let target = &grid[r as usize][c as usize];
            if (target.height as i32 - source.height as i32) < 2 {
                Some((source.id, target.id))
            } else {
                None
            }
        })
        .collect();

    edges
}

fn build_graph(grid: Vec<Vec<Node>>) -> petgraph::Graph<i32, ()> {
    let mut edges: Vec<(u32, u32)> = Vec::new();
    for (row_idx, rows) in grid.iter().enumerate() {
        for (col_idx, _) in rows.iter().enumerate() {
            edges.append(&mut find_edges_for_coordinate(&grid, row_idx, col_idx));
        }
    }
    let unique_edges: Vec<(u32, u32)> = edges.into_iter().unique().collect();
    let g = DiGraph::<i32, ()>::from_edges(&unique_edges);
    g
}

pub fn part_a(input: &str) -> u32 {
    let grid: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| line_to_vec(l, row))
        .collect();

    let graph = build_graph(grid);

    let node_map = dijkstra(&graph, SOURCE_ID.into(), None, |_| 1);
    *node_map.get(&NodeIndex::new(TARGET_ID as usize)).unwrap()
}

pub fn part_b(input: &str) -> u32 {
    let grid: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| line_to_vec(l, row))
        .collect();

    let graph = build_graph(grid.clone());

    let mut candidate_shortest_distances: Vec<Option<u32>> = Vec::new();

    // brute force djiikstra for each node at height 0
    // this isn't that efficient, but solves the problem
    for rows in grid.iter() {
        for node in rows.iter() {
            if node.height == 0 {
                let target = NodeIndex::new(TARGET_ID as usize);
                let node_map = dijkstra(&graph, node.id.into(), Some(target), |_| 1);
                let distance = node_map
                    .get(&NodeIndex::new(TARGET_ID as usize))
                    .map(|x| *x);
                candidate_shortest_distances.push(distance);
            }
        }
    }

    *candidate_shortest_distances.iter().flatten().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(12);
        assert_eq!(part_a(&input), 31);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(12);
        assert_eq!(part_b(&input), 29);
    }
}

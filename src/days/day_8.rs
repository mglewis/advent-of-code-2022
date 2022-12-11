use advent_of_code_2022::to_u32;

fn line_to_vec(line: &str) -> Vec<u32> {
    line.chars().map(|c| to_u32(&c.to_string())).collect()
}

fn is_hidden_by_neighbours(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    col_idx: usize,
    tree_height: u32,
) -> bool {
    let (mut hidden_left, mut hidden_right, mut hidden_up, mut hidden_down) =
        (false, false, false, false);

    // search rows to the left
    for idx in 0..col_idx {
        if grid[row_idx][idx] >= tree_height {
            hidden_left = true;
        }
    }
    // search rows to the right
    for idx in col_idx + 1..grid[0].len() {
        if grid[row_idx][idx] >= tree_height {
            hidden_right = true;
        }
    }
    // search cols to the top
    for idx in 0..row_idx {
        if grid[idx][col_idx] >= tree_height {
            hidden_up = true;
        }
    }

    // search cols to the bottom
    for idx in row_idx + 1..grid.len() {
        if grid[idx][col_idx] >= tree_height {
            hidden_down = true;
        }
    }

    hidden_left && hidden_right && hidden_down && hidden_up
}

fn scenic_score_for_tree(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    col_idx: usize,
    tree_height: u32,
) -> u32 {
    let (mut visible_left, mut visible_right, mut visible_up, mut visible_down) = (0, 0, 0, 0);

    // search rows to the left
    for idx in (0..col_idx).rev() {
        visible_left += 1;
        if grid[row_idx][idx] >= tree_height {
            break;
        }
    }
    // search rows to the right
    for idx in col_idx + 1..grid[0].len() {
        visible_right += 1;
        if grid[row_idx][idx] >= tree_height {
            break;
        }
    }
    // search cols to the top
    for idx in (0..row_idx).rev() {
        visible_up += 1;
        if grid[idx][col_idx] >= tree_height {
            break;
        }
    }

    // search cols to the bottom
    for idx in row_idx + 1..grid.len() {
        visible_down += 1;
        if grid[idx][col_idx] >= tree_height {
            break;
        }
    }

    visible_left * visible_right * visible_up * visible_down
}

pub fn part_a(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines().map(|l| line_to_vec(l)).collect();

    let mut visible_trees: u32 = 0;

    for (row_idx, rows) in grid.iter().enumerate() {
        for (col_idx, tree_height) in rows.iter().enumerate() {
            if !is_hidden_by_neighbours(&grid, row_idx, col_idx, *tree_height) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

pub fn part_b(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines().map(|l| line_to_vec(l)).collect();

    let mut scenic_scores: Vec<u32> = Vec::new();

    for (row_idx, rows) in grid.iter().enumerate() {
        for (col_idx, tree_height) in rows.iter().enumerate() {
            scenic_scores.push(scenic_score_for_tree(&grid, row_idx, col_idx, *tree_height));
        }
    }
    *scenic_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(8);
        assert_eq!(part_a(&input), 21);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(8);
        assert_eq!(part_b(&input), 8);
    }
}

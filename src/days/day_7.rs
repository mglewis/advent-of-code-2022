use advent_of_code_2022::to_u32;
use std::collections::HashMap;

const MAX_FILE_SIZE: u32 = 100_000;
const REQUIRED_FREE_SPACE: u32 = 30_000_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;

const CD_CMD: &str = "$ cd ";
const CD_UP_CMD: &str = "$ cd ..";
const LS_CMD: &str = "$ ls";
const DIR_MARKER: &str = "dir ";

fn dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut current_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let mut already_visited_current_path = false;

    input.lines().for_each(|line| {
        let dir_str = current_path.join("/");

        if line == CD_UP_CMD {
            current_path.pop();
        } else if line.starts_with(CD_CMD) {
            let current_dir = line.replace(CD_CMD, "");
            current_path.push(current_dir);
        } else if line.starts_with(DIR_MARKER) {
            // do nothing
        } else if line.starts_with(LS_CMD) {
            if dir_sizes.contains_key(&dir_str) {
                already_visited_current_path = true;
            } else {
                already_visited_current_path = false;
            }
        } else if already_visited_current_path == false {
            // update current and all parent directories
            let file_size = to_u32(line.split_once(" ").unwrap().0);
            let mut path = current_path.clone();
            while !path.is_empty() {
                let dir = path.join("/");
                *dir_sizes.entry(dir).or_insert(0) += file_size;
                path.pop();
            }
        }
    });
    dir_sizes
}

pub fn part_a(input: &str) -> u32 {
    let dirs = dir_sizes(input);
    dirs.values().filter(|x| **x < MAX_FILE_SIZE).sum()
}

pub fn part_b(input: &str) -> u32 {
    let dirs = dir_sizes(input);

    let used_space: u32 = *dirs.get("/").unwrap();
    let current_free_space: u32 = TOTAL_DISK_SPACE - used_space;
    let space_to_clear: u32 = REQUIRED_FREE_SPACE - current_free_space;

    *dirs
        .values()
        .filter(|v| **v > space_to_clear)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(7);
        assert_eq!(part_a(&input), 95437);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(7);
        assert_eq!(part_b(&input), 24933642);
    }
}

use itertools::Itertools;

fn find_start_of_packet(input: &str, window_size: usize) -> u32 {
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() - window_size {
        let window = &chars[i..i + window_size].to_vec();
        let unique_elems: Vec<&char> = window.iter().unique().collect();
        if unique_elems.len() == window.len() {
            return (i + window_size) as u32;
        }
    }

    panic!("No sequence of non-reccuring chars found!")
}

pub fn part_a(input: &str) -> u32 {
    find_start_of_packet(input, 4)
}

pub fn part_b(input: &str) -> u32 {
    find_start_of_packet(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(6);
        assert_eq!(part_a(&input), 5);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(6);
        assert_eq!(part_b(&input), 23);
    }
}

fn char_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn get_duplicated_priority_for_rucksack(rucksack: &str) -> u32 {
    let priorities: Vec<u32> = rucksack.chars().map(char_priority).collect();

    let (compartment_a, compartment_b) = priorities.split_at(priorities.len() / 2);

    let mut priority_occurrences: [u32; 52] = [0; 52];

    for priority in compartment_a {
        priority_occurrences[*priority as usize - 1] += 1;
    }

    *compartment_b
        .iter()
        .find(|p| priority_occurrences[**p as usize - 1] != 0)
        .unwrap()
}

fn get_priority_mask_for_rucksack(rucksack: &str) -> [bool; 52] {
    let priorities: Vec<u32> = rucksack.chars().map(char_priority).collect();

    let mut priority_mask: [bool; 52] = [false; 52];

    for priority in priorities {
        priority_mask[priority as usize - 1] = true;
    }

    priority_mask
}

fn get_priority_for_rucksack_group(rucksacks: Vec<&str>) -> u32 {
    let priority_masks: Vec<[bool; 52]> = rucksacks
        .iter()
        .map(|x| get_priority_mask_for_rucksack(x))
        .collect();

    let a = priority_masks[0];
    let b = priority_masks[1];
    let c = priority_masks[2];

    for idx in 0..a.len() {
        if a[idx] && b[idx] && c[idx] {
            return (idx + 1) as u32;
        }
    }
    panic!("Did not find priority that was encountered in all three vectors")
}

pub fn part_a(input: &str) -> u32 {
    let rucksacks: Vec<&str> = input.lines().collect();
    let priorities: Vec<u32> = rucksacks
        .iter()
        .map(|r| get_duplicated_priority_for_rucksack(r))
        .collect();
    priorities.iter().sum()
}

pub fn part_b(input: &str) -> u32 {
    let rucksacks: Vec<&str> = input.lines().collect();

    let grouped_rucksacks: Vec<Vec<&str>> = rucksacks
        .chunks_exact(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let priorities = grouped_rucksacks
        .iter()
        .map(|x| get_priority_for_rucksack_group(x.to_vec()));

    priorities.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(3);
        assert_eq!(part_a(&input), 157);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(3);
        assert_eq!(part_b(&input), 70);
    }
}

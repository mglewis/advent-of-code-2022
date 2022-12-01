fn to_u32(i: &str) -> u32 {
    i.trim().parse::<u32>().unwrap()
}

fn sum_of_snacks(snacks: &str) -> u32 {
    snacks.lines().map(to_u32).sum()
}

pub fn part_a(input: &str) -> u32 {
    let snacks_per_elf = input.split("\n\n").collect::<Vec<&str>>();
    let calories_per_elf = snacks_per_elf
        .iter()
        .map(|x| sum_of_snacks(*x))
        .collect::<Vec<u32>>();
    *calories_per_elf.iter().max().unwrap()
}

pub fn part_b(input: &str) -> u32 {
    let snacks_per_elf = input.split("\n\n").collect::<Vec<&str>>();
    let mut calories_per_elf = snacks_per_elf
        .iter()
        .map(|x| sum_of_snacks(*x))
        .collect::<Vec<u32>>();
    calories_per_elf.sort();
    calories_per_elf.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_to_u32() {
        assert_eq!(to_u32("123"), 123);
    }

    #[test]
    #[should_panic]
    fn test_to_u32_bad_input() {
        to_u32("not_a_u32");
    }

    #[test]
    fn test_sum_of_snacks() {
        let snacks = "1
            2
            100";
        assert_eq!(sum_of_snacks(snacks), 103);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(1, 'a');
        assert_eq!(part_a(&input), 101);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(1, 'b');
        assert_eq!(part_b(&input), 106);
    }
}

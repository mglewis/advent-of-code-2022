use advent_of_code_2022::to_u32;

struct Instruction {
    quantity: u32,
    source: u32,
    destination: u32,
}

fn get_stack_width(stack_labels: &str) -> u32 {
    let last_stack_label = stack_labels.split_whitespace().last().unwrap();
    to_u32(last_stack_label)
}

fn build_stacks(stack_str: &str) -> Vec<Vec<char>> {
    // expected input:
    //     [G]         [P]         [M]
    //     [V]     [M] [W] [S]     [Q]
    //     [N]     [N] [G] [H]     [T] [F]
    //     [J]     [W] [V] [Q] [W] [F] [P]
    // [C] [H]     [T] [T] [G] [B] [Z] [B]
    // [S] [W] [S] [L] [F] [B] [P] [C] [H]
    // [G] [M] [Q] [S] [Z] [T] [J] [D] [S]
    // [B] [T] [M] [B] [J] [C] [T] [G] [N]
    //  1   2   3   4   5   6   7   8   9

    let mut stack_lines: Vec<&str> = stack_str.lines().collect();

    let stack_labels = stack_lines.pop().unwrap();
    let stack_width = get_stack_width(stack_labels);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_width as usize);
    for _ in 0..stack_width {
        stacks.push(Vec::new());
    }

    let start_offset: usize = 1;
    let offset: usize = 4;

    for line in stack_lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        let mut current_stack_idx: usize = 0;

        for line_idx in (start_offset..chars.len()).step_by(offset) {
            let current_char = chars[line_idx];
            if current_char != ' ' {
                stacks[current_stack_idx].push(current_char)
            }
            current_stack_idx += 1;
        }
    }
    stacks
}

fn build_instruction(line: &str) -> Instruction {
    let cleaned_line: String = line
        .replace("move", " ")
        .replace("from", " ")
        .replace("to", " ");

    let line_content: Vec<u32> = cleaned_line.split_whitespace().map(|x| to_u32(x)).collect();

    Instruction {
        quantity: line_content[0],
        source: line_content[1],
        destination: line_content[2],
    }
}

fn build_instructions(instructions: &str) -> Vec<Instruction> {
    // expected input:
    // move 2 from 4 to 2
    // move 6 from 9 to 7
    // move 4 from 7 to 2
    // move 2 from 4 to 1
    // move 2 from 6 to 7
    instructions.lines().map(|l| build_instruction(l)).collect()
}

fn read_top_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().map(|s| s[s.len() - 1]).into_iter().collect()
}

pub fn part_a(input: &str) -> String {
    let (stack_str, instructions_str): (&str, &str) = input.split_once("\n\n").unwrap();

    let mut stacks = build_stacks(stack_str);
    let instructions = build_instructions(instructions_str);

    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let c = stacks[instruction.source as usize - 1].pop().unwrap();
            stacks[instruction.destination as usize - 1].push(c);
        }
    }

    read_top_of_stacks(&stacks)
}

pub fn part_b(input: &str) -> String {
    let (stack_str, instructions_str): (&str, &str) = input.split_once("\n\n").unwrap();

    let mut stacks = build_stacks(stack_str);
    let instructions = build_instructions(instructions_str);

    for instruction in instructions {
        let mut additions: Vec<char> = Vec::new();
        for _ in 0..instruction.quantity {
            let c = stacks[instruction.source as usize - 1].pop().unwrap();
            additions.push(c);
        }
        additions.reverse();
        stacks[instruction.destination as usize - 1].append(&mut additions);
    }

    read_top_of_stacks(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(5);
        assert_eq!(part_a(&input), "BDC");
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(5);
        assert_eq!(part_b(&input), "BDA");
    }
}

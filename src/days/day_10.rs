use std::str::FromStr;
use Instruction::{AddX, Noop};

const ADDX_REPR: &'static str = "addx";
const NOOP_REPR: &'static str = "noop";

const ADDX_CYCLES: i64 = 2;
const NOOP_CYCLES: i64 = 1;

const CYCLE_ANALYSIS_OFFSET: i64 = 20;
const CYCLE_ANALYSIS_PERIOD: i64 = 40;

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 40;

#[derive(PartialEq)]
enum Instruction {
    AddX { val: i64 },
    Noop,
}

impl Instruction {
    fn cycles(&self) -> i64 {
        match self {
            AddX { val: _ } => ADDX_CYCLES,
            Noop => NOOP_CYCLES,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(ADDX_REPR) {
            let val = s[5..].parse::<i64>().unwrap();
            Ok(AddX { val })
        } else if s == NOOP_REPR {
            Ok(Noop)
        } else {
            Err(())
        }
    }
}

struct SystemState {
    x_register: i64,
    clock_cycles: i64,
}

fn execute(instructions: Vec<Instruction>) -> Vec<SystemState> {
    // returns a vector of system states, where each state is a snapshot of the
    // system at a given clock cycle
    let mut x_register: i64 = 1;
    let mut clock_cycles: i64 = 0;

    let starting_state = SystemState {
        x_register: x_register,
        clock_cycles: clock_cycles,
    };

    let mut system_states: Vec<SystemState> = Vec::from([starting_state]);

    for instruction in instructions {
        // account for clock ticks where nothing happens
        for _ in 0..instruction.cycles() {
            clock_cycles += 1;
            let state = SystemState {
                x_register: x_register,
                clock_cycles: clock_cycles,
            };
            system_states.push(state);
        }

        // actually execute the instruction
        match instruction {
            AddX { val } => {
                x_register += val;
            }
            Noop => (),
        }
    }
    system_states
}

pub fn part_a(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    let system_states = execute(instructions);

    let filtered_system_states: Vec<&SystemState> = system_states
        .iter()
        .filter(|s| (s.clock_cycles + CYCLE_ANALYSIS_OFFSET) % CYCLE_ANALYSIS_PERIOD == 0)
        .collect();

    filtered_system_states
        .iter()
        .map(|s| s.x_register * s.clock_cycles)
        .sum()
}

pub fn part_b(input: &str) -> String {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    let system_states = &execute(instructions)[1..];

    let mut screen: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT] = [['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for state in system_states {
        let sprite_coverage_window: Vec<i64> =
            ((state.x_register - 1)..(state.x_register + 2)).collect();
        let row = ((state.clock_cycles as usize - 1) / SCREEN_WIDTH) % SCREEN_HEIGHT;
        let col = (state.clock_cycles - 1) as usize % SCREEN_WIDTH;

        if sprite_coverage_window.contains(&(col as i64)) {
            // row and col of current pixel that is being drawn
            screen[row][col] = '#';
        }
    }
    "\n".to_owned() + &screen.map(|row| row.iter().collect::<String>()).join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(10);
        assert_eq!(part_a(&input), 13140);
    }

    #[test]
    fn test_part_b() {
        let expected_output = "\n".to_owned()
            + &vec![
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
            .join("\n");

        let input = read_test_file(10);
        assert_eq!(part_b(&input), expected_output);
    }
}

use eval::Expr;
use std::num::ParseIntError;
use std::str::FromStr;

type MonkeyId = usize;
type ItemWorry = i64;

const OPERATION_PREFIX: &str = "Operation: new = ";

const RELIEF_DIVISOR: i64 = 3;
const NO_RELIEF_DIVISOR: i64 = 1;

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    conditional_divisor: i64,
    true_target: MonkeyId,
    false_target: MonkeyId,
    total_inspections: i64,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let maybe_items: Result<Vec<ItemWorry>, ParseIntError> = remove_non_numeric_chars(lines[1])
            .split_whitespace()
            .map(|s| s.parse::<ItemWorry>())
            .collect();
        let operation = lines[2].replace(OPERATION_PREFIX, "");
        let conditional_divisor: i64 = remove_non_numeric_chars(lines[3]).parse::<i64>()?;
        let true_target: MonkeyId = remove_non_numeric_chars(lines[4]).parse::<MonkeyId>()?;
        let false_target: MonkeyId = remove_non_numeric_chars(lines[5]).parse::<MonkeyId>()?;

        Ok(Monkey {
            items: maybe_items?,
            operation: operation,
            conditional_divisor: conditional_divisor,
            true_target: true_target,
            false_target: false_target,
            total_inspections: 0,
        })
    }
}

fn remove_non_numeric_chars(input: &str) -> String {
    let filtered_input: String = input
        .chars()
        .filter(|c| c.is_numeric() || c.is_whitespace())
        .collect();
    filtered_input.trim().to_string()
}

fn new_item_worry(item: ItemWorry, operation: &str, relief_divisor: i64) -> ItemWorry {
    let expression = Expr::new(operation).value("old", item);
    let worry_during_inspection = expression.exec().ok().and_then(|v| v.as_i64()).unwrap();
    worry_during_inspection / relief_divisor
}

fn process_round(input: Vec<Monkey>, relief_divisor: i64) -> Vec<Monkey> {
    // documentation on evaluating the operation string can be found here:
    // https://docs.rs/eval/latest/eval/
    let mut monkeys = input.clone();
    for m in 0..monkeys.len() {
        let monkey = &mut monkeys[m];
        let true_target = monkey.true_target;
        let false_target = monkey.false_target;
        let conditional_divisor = monkey.conditional_divisor;

        let items: Vec<ItemWorry> = monkey
            .items
            .drain(..)
            .map(|i| new_item_worry(i, &monkey.operation, relief_divisor))
            .collect();
        monkey.total_inspections += items.len() as i64;

        for item in items {
            let target_monkey = if item % conditional_divisor == 0 {
                true_target
            } else {
                false_target
            };
            // give the item to the target monkey
            monkeys[target_monkey].items.push(item);
        }
    }

    monkeys
}

pub fn part_a(input: &str) -> i64 {
    let monkey_strings: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = monkey_strings
        .iter()
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect();

    for _round in 0..20 {
        monkeys = process_round(monkeys, RELIEF_DIVISOR);
    }

    let mut inspections: Vec<i64> = monkeys.iter().map(|m| m.total_inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product::<i64>()
}

pub fn part_b(input: &str) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_create_monkey_from_str() {
        let input = vec![
            "Monkey 0:",
            "Starting items: 99, 67, 92, 61, 83, 64, 98",
            "Operation: new = old * 17",
            "Test: divisible by 3",
            "  If true: throw to monkey 4",
            "  If false: throw to monkey 2",
        ]
        .join("\n");
        let monkey = input.parse::<Monkey>().unwrap();
        assert_eq!(monkey.items, vec![99, 67, 92, 61, 83, 64, 98]);
        assert_eq!(monkey.conditional_divisor, 3);
        assert_eq!(monkey.true_target, 4);
        assert_eq!(monkey.false_target, 2);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(11);
        assert_eq!(part_a(&input), 10605);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(11);
        assert_eq!(part_b(&input), 2);
    }
}

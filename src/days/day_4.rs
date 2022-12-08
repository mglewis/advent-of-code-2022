use advent_of_code_2022::to_u32;

struct Range {
    lower: u32,
    upper: u32,
}

fn ranges_entirely_overlap(a: &Range, b: &Range) -> bool {
    let a_overlaps_b = a.lower <= b.lower && a.upper >= b.upper;
    let b_overlaps_a = b.lower <= a.lower && b.upper >= a.upper;
    a_overlaps_b || b_overlaps_a
}

fn ranges_intersect(a: &Range, b: &Range) -> bool {
    let a_lower_within_b = a.lower >= b.lower && a.lower <= b.upper;
    let a_upper_within_b = a.upper >= b.lower && a.upper <= b.upper;
    a_lower_within_b || a_upper_within_b || ranges_entirely_overlap(a, b)
}

fn to_ranges(line: &str) -> (Range, Range) {
    let cleaned_line: String = line.replace(",", "-");
    let split_line: Vec<&str> = cleaned_line.split("-").collect();
    let int_vals: Vec<u32> = split_line.iter().map(|x| to_u32(*x)).collect();

    let a = Range {
        lower: int_vals[0],
        upper: int_vals[1],
    };
    let b = Range {
        lower: int_vals[2],
        upper: int_vals[3],
    };

    (a, b)
}

pub fn part_a(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let ranges: Vec<(Range, Range)> = lines.iter().map(|x| to_ranges(x)).collect();
    let ranges_overlap: Vec<bool> = ranges
        .iter()
        .map(|(a, b)| ranges_entirely_overlap(a, b))
        .collect();
    ranges_overlap.iter().map(|x| if *x { 1 } else { 0 }).sum()
}

pub fn part_b(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let ranges: Vec<(Range, Range)> = lines.iter().map(|x| to_ranges(x)).collect();
    let ranges_intersect: Vec<bool> = ranges.iter().map(|(a, b)| ranges_intersect(a, b)).collect();
    ranges_intersect
        .iter()
        .map(|x| if *x { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_ranges_overlap() {
        let a = Range { lower: 1, upper: 2 };
        let b = Range { lower: 2, upper: 5 };
        let c = Range { lower: 3, upper: 4 };
        assert_eq!(ranges_entirely_overlap(&a, &b), false);
        assert_eq!(ranges_entirely_overlap(&c, &b), true);
        assert_eq!(ranges_entirely_overlap(&b, &c), true);
    }

    #[test]
    fn test_ranges_intersect() {
        let a = Range { lower: 1, upper: 2 };
        let b = Range { lower: 2, upper: 5 };
        let c = Range { lower: 6, upper: 6 };
        assert_eq!(ranges_intersect(&a, &b), true);
        assert_eq!(ranges_intersect(&a, &c), false);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(4);
        assert_eq!(part_a(&input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(4);
        assert_eq!(part_b(&input), 4);
    }
}

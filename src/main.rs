extern crate argparse;
use advent_of_code_2022::read_file;
use argparse::{ArgumentParser, Store};
use days::*;
mod days;

fn main() {
    let mut day = 0;
    let mut part = ' ';
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of Code 2022.");
        ap.refer(&mut day)
            .add_option(&["-d", "--day"], Store, "Day to run");
        ap.refer(&mut part)
            .add_option(&["-p", "--part"], Store, "Part to run [a, b]");
        ap.parse_args_or_exit();
    }

    if day < 1 || day > 25 {
        panic!("--day parameter must be specified and be between `1` and `25` inclusive");
    }

    let valid_parts = vec!['a', 'b'];
    if !valid_parts.contains(&part) {
        panic!("--part parameter must be specified and be set either `a` or `b`");
    }

    let input = read_file(day, part);

    let result = match (day, part) {
        (1, 'a') => day_1::part_a(&input),
        (1, 'b') => day_1::part_b(&input),
        (_, _) => panic!("Unrecognised day [{}] part [{}]", day, part),
    };

    println!("Result: {}", result)
}

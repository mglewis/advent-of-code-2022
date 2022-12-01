use std::{env, fs};

pub fn read_file(day: u32, part: char) -> String {
    read_file_path(day, part, vec!["inputs"])
}

pub fn read_test_file(day: u32, part: char) -> String {
    read_file_path(day, part, vec!["inputs", "test"])
}

fn read_file_path(day: u32, part: char, path: Vec<&str>) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = format!("day_{}{}.txt", day, part);
    let filepath = path.iter().fold(cwd, |c, x| c.join(x));
    let f = fs::read_to_string(filepath.join(filename.clone()));
    f.expect(&format!("could not open input file {}", filename))
}

use crate::{core::{get_input_lines, FileLines}, part1::solve_part1};

mod core;
mod part1;

fn main() {
    let lines: FileLines = get_input_lines().unwrap();
    let lines1 = lines.into_iter();

    let sol1 = solve_part1(lines1);

    println!("solution {:?}", sol1);
}

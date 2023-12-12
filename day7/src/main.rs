use crate::core::{get_input_lines, FileLines};

mod core;
mod part1;
mod part2;

fn main() {
    let lines: FileLines = get_input_lines().unwrap();
    let lines1 = lines.into_iter();

    let sol1 = part2::solve(lines1);

    println!("solution {:?}", sol1);
}

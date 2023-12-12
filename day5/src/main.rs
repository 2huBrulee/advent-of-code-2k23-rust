mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

fn get_input_lines() -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open("test")?;
    return Ok(BufReader::new(file).lines());
}

fn main() {
    let lines = get_input_lines().unwrap();
    // let input = part1::parse_input(lines);
    // let solution = part1::solve_p1(&input);

    // println!("{:?}", solution);

    let input2 = part2::parse_input_2(lines);
    let solution2 = part2::solve_p2(&input2);

    println!("{:?}", solution2)
}

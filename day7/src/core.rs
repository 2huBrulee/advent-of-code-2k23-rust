use std::{
  fs::File,
  io::{BufRead, BufReader, Error, Lines},
};

pub type FileLines = Lines<BufReader<File>>;

pub fn get_input_lines() -> Result<FileLines, Error> {
  let file = File::open("input")?;
  return Ok(BufReader::new(file).lines());
}

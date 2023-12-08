use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};


fn get_number_map() -> HashMap<&'static str, i8> {
    HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ])
}


fn get_lines() -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open("input")?;
    return Ok(BufReader::new(file).lines());
}

fn get_line_numbers(line: String) -> Vec<i8> {
    let mut substring = line;
    let mut numbers: Vec<i8> = Vec::new();
    let number_map = get_number_map();

    while substring.len() > 0 {
        let mut number: Option<&i8> = None;
        let mut len = 1;

        while number.is_none() && len <= substring.len() {
            let ssss = &substring[0..len];

            number = number_map.get(ssss);
            len = len + 1;
        }

        if let Some(found_number) = number {
            numbers.push(*found_number)
        }

        substring.remove(0);
    }


    return numbers;
}


fn solve_part_2(lines: Lines<BufReader<File>>) -> Result<i64, Error> {
    let mut acc: i64 = 0;

    for line in lines {
        if let Ok(l) = line {
            let numbers = get_line_numbers(l);

            let size = numbers.len();

            let first = numbers[0];
            let last = numbers[size - 1];

            acc = acc + (first * 10 + last) as i64;
        } else {
            return Err(Error::new(ErrorKind::Other, "Some lines not readable / parseable"));
        }
    }


    return Ok(acc);
}


fn solve_part_1(lines: Lines<BufReader<File>>) -> Result<i64, Error> {
    let mut acc: i64 = 0;

    for line in lines {
        if let Ok(l) = line {
            let numbers = l.split("").filter_map(|c|
                c.parse::<i8>().ok()
            ).collect::<Vec<_>>();

            let size = numbers.len();

            let first = numbers[0];
            let last = numbers[size - 1];

            acc = acc + (first * 10 + last) as i64;
        } else {
            return Err(Error::new(ErrorKind::Other, "Some lines not readable / parseable"));
        }
    }

    return Ok(acc);
}


fn main() {
    let lines = get_lines().unwrap();

    let solution = solve_part_2(lines);

    match solution {
        Ok(n) => println!("{}", n),
        Err(e) => println!("error => {}", e)
    }
}

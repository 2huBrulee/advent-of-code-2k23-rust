use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn solve() -> Result<i64, Error> {
    let file = File::open("input")?;

    let lines = BufReader::new(file).lines();

    let mut acc: i64 = 0;

    for line in lines {

        if let Ok(l) = line {
            let numbers = l.split("").filter_map(|c|
                c.parse::<i8>().ok()
            ).collect::<Vec<_>>();

            let size = numbers.len();

            let first = numbers[0];
            let last  = numbers[size -1 ];

            acc = acc + (first * 10 + last) as i64;
        } else {
            return Err(Error::new(ErrorKind::Other, "Some lines not readable / parseable"))
        }
    }

    return Ok(acc);
}


fn main() {
    let solution = solve();

    match  solution {
        Ok(n) => println!("{}", n),
        Err(e ) => println!("error => {}", e)
    }
}

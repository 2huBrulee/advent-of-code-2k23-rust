use crate::core::FileLines;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn number_of_ways_to_beat_distance(&self) -> i64 {
        let minus_b_halved = self.time as f64 / 2.0;
        let inside_sqrt = (self.time.pow(2) - 4 * self.distance) as f64;

        let m1 = minus_b_halved - (inside_sqrt.sqrt() / 2.0);
        let m2 = minus_b_halved + (inside_sqrt.sqrt() / 2.0);

        let test = (m1.floor() as i64)..(m2.ceil() as i64);

        let l = (test.collect::<Vec<_>>().len() as i64) - 1;

        l
    }
}

#[derive(Debug)]
struct Part1Input {
    races: Vec<Race>,
}

pub fn solve_part1(lines: FileLines) -> i64 {
    let input = parse_input(lines);

    let mut result = 1_i64;

    for race in &input.races {
        result *= race.number_of_ways_to_beat_distance();
    }

    result
}

fn parse_input(lines: FileLines) -> Part1Input {
    let mut input = Part1Input { races: Vec::new() };

    let lines_vec = lines.map(|l| l.unwrap()).collect::<Vec<_>>();

    let times = lines_vec
        .get(0)
        .unwrap()
        .split(" ")
        .filter_map(|c| c.parse::<i64>().ok());

    let distances = lines_vec
        .get(1)
        .unwrap()
        .split(" ")
        .filter_map(|c| c.parse::<i64>().ok());

    let time = times
        .map(|t| t.to_string())
        .fold("".to_string(), |acc, e| acc + e.as_str())
        .parse::<i64>()
        .unwrap();

    let distance = distances
        .map(|t| t.to_string())
        .fold("".to_string(), |acc, e| acc + e.as_str())
        .parse::<i64>()
        .unwrap();

    input.races.push(Race { time, distance });

    // let pairs = times.zip(distances).collect::<Vec<_>>();

    // for (time, distance) in pairs {
    //     input.races.push(Race { time, distance })
    // }

    input
}

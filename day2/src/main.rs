use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

#[derive(Debug)]
struct GameSet {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Bag {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct Input {
    games: Vec<Game>,
}

fn is_set_valid(set: &GameSet, bag: &Bag) -> bool {
    set.red <= bag.red && set.blue <= bag.blue && set.green <= bag.green
}

fn is_game_valid(game: &Game, bag: &Bag) -> bool {
    println!("{:?}", game);
    println!("evaluatuing");

    let mut is_valid = true;
    let mut set_index = 0;

    while is_valid && set_index < (*game).sets.len() {
        let is_set_valid = (*game).sets.get(set_index).map_or_else(|| false, |set| is_set_valid(set, bag));

        println!("seto");
        println!("{:?}", is_set_valid);

        is_valid = is_valid && is_set_valid;
        set_index += 1;
    }

    println!("{:?}", is_valid);
    return is_valid;
}

fn solve_p2(input: Input) -> i64 {
    let mut acc: i64 = 0;

    for game in input.games {
        let mut game_bag = Bag {
            green: 0,
            blue: 0,
            red: 0,
        };


        for set in game.sets {
            if set.red > game_bag.red { game_bag.red = set.red }
            if set.green > game_bag.green { game_bag.green = set.green }
            if set.blue > game_bag.blue { game_bag.blue = set.blue }
        }


        acc = acc + (game_bag.red * game_bag.green * game_bag.blue) as i64
    }


    return acc;
}


fn solve_p1(input: Input, bag: Bag) -> i32 {
    let mut sum: i32 = 0;

    for game in input.games {
        let is_game_valid = is_game_valid(&game, &bag);

        if is_game_valid {
            sum = sum + game.id;
        }
    }


    return sum;
}

fn get_input_lines() -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open("input")?;
    return Ok(BufReader::new(file).lines());
}

fn parse_set(string: String) -> GameSet {
    let mut game_set = GameSet {
        green: 0,
        red: 0,
        blue: 0,
    };


    let colors_config = string.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

    for colors_config in colors_config {
        let parts = colors_config.split(' ').collect::<Vec<&str>>();

        let number = parts.get(0).and_then(|c| c.parse::<i32>().ok()).unwrap_or(0);
        let color = *(parts.get(1).unwrap_or(&""));


        if color == "green" {
            game_set.green = number
        }

        if color == "red" {
            game_set.red = number
        }

        if color == "blue" {
            game_set.blue = number
        }
    }


    return game_set;
}

fn parse_sets(string: String) -> Vec<GameSet> {
    string.split(";").map(|part| parse_set(String::from(part))).collect()
}

fn parse_game_id_from_name(string: String) -> i32 {
    let parts = string.split(" ").collect::<Vec<&str>>();
    parts.get(1).and_then(|id| id.parse::<i32>().ok()).unwrap_or(0)
}


fn parse_input_line(line: String) -> Game {
    let mut game = Game {
        sets: Vec::new(),
        id: 0,
    };

    let parts = line.split(": ").collect::<Vec<&str>>();

    let id = parts.get(0).map(|name| parse_game_id_from_name(String::from(*name))).unwrap();
    let sets = parts.get(1).map(|s| parse_sets(String::from(*s))).unwrap();

    game.id = id;
    game.sets = sets;


    return game;
}

fn parse_input_lines(lines: Lines<BufReader<File>>) -> Input {
    let mut input = Input {
        games: Vec::new(),
    };

    for line in lines {
        if let Ok(l) = line {
            let game = parse_input_line(l);
            input.games.push(game)
        }
    }


    return input;
}


fn main() {
    let lines = get_input_lines().unwrap();
    let input = parse_input_lines(lines);

    // let solution1 = solve_p1(input, Bag {
    //     blue: 14,
    //     red: 12,
    //     green: 13,
    // });

    let solution2 = solve_p2(input);

    // println!("solution 1 => {}", solution1);
    println!("solution 1 => {}", solution2);
}

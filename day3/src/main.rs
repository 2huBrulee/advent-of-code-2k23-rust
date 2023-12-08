use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct NumberPosition {
    number: i32,
    coords: Coord,
}

#[derive(Debug, Clone)]
struct NumberGroup {
    positions: Vec<NumberPosition>,
}

impl NumberGroup {
    fn new(initial_position: NumberPosition) -> Self {
        let mut positions: Vec<NumberPosition> = Vec::new();
        positions.push(initial_position);

        NumberGroup {
            positions,
        }
    }

    fn add(&mut self, number_position: NumberPosition) {
        self.positions.push(number_position)
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: i32,
    adjacent_coords: HashSet<Coord>,
}

#[derive(Debug, Clone)]
struct PartNumberCreator {
    groups: Vec<NumberGroup>,
}

impl PartNumberCreator {
    fn new() -> Self {
        PartNumberCreator {
            groups: Vec::new()
        }
    }


    fn add_number(&mut self, number: i32, coord: Coord) {
        println!("adding number {} at {:?}", number, coord);


        let adjacent_group = self.find_adjacent_group(coord.clone());


        match adjacent_group {
            Some(mut group) => {
                group.add(NumberPosition {
                    coords: coord,
                    number,
                })
            }
            None => {
                self.groups.push(NumberGroup::new(NumberPosition {
                    coords: coord,
                    number,
                }))
            }
        }
    }

    fn find_adjacent_group(&mut self, coord: Coord) -> Option<&mut NumberGroup> {
        let mut found_group: Option<&mut NumberGroup> = None;

        for group in self.groups.iter_mut() {
            for position in &group.positions {
                let is_adjacent = (position.coords.x == coord.x + 1 || position.coords.x == coord.x - 1) && position.coords.y == coord.y;

                if is_adjacent {
                    found_group = Some(group);
                    break;
                }
            }

            if found_group.is_some() {
                break;
            }
        }

        return found_group;
    }

    fn show(self: &Self) {
        for group in &self.groups {
            println!("{:?}", group)
        }
    }

    fn get_parts(self) -> Vec<PartNumber> {
        let mut parts: Vec<PartNumber> = Vec::new();


        for group in self.groups {
            println!("{:?}", group);
            let mut part_number = PartNumber {
                number: 0,
                adjacent_coords: HashSet::new(),
            };

            for position in group.positions {
                part_number.number = part_number.number * 10 + position.number;
                let adjacents = get_adjacent_coords(position.coords);

                for adjacent in adjacents {
                    part_number.adjacent_coords.insert(adjacent);
                }
            }

            println!("{:?}", part_number);

            parts.push(part_number);
        }

        return parts;
    }
}

fn get_adjacent_coords(coord: Coord) -> Vec<Coord> {
    let mut adjacents = Vec::new();

    adjacents.push(Coord { x: coord.x, y: coord.y + 1 });
    adjacents.push(Coord { x: coord.x + 1, y: coord.y + 1 });
    adjacents.push(Coord { x: coord.x + 1, y: coord.y });
    adjacents.push(Coord { x: coord.x + 1, y: coord.y - 1 });
    adjacents.push(Coord { x: coord.x, y: coord.y - 1 });
    adjacents.push(Coord { x: coord.x - 1, y: coord.y - 1 });
    adjacents.push(Coord { x: coord.x - 1, y: coord.y });
    adjacents.push(Coord { x: coord.x - 1, y: coord.y + 1 });


    return adjacents;
}


struct Symbol {
    symbol: String,
    coord: Coord,
}


fn parse_symbols_and_part_numbers(lines: Lines<BufReader<File>>) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_creator = PartNumberCreator::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y_index, line) in lines.enumerate() {
        if let Ok(l) = line {
            let chars = l.split("").enumerate();

            for (x_index, char) in chars {
                if char != "" && char != "." {
                    println!(" x:{}, y:{}, {}", x_index, y_index, char);

                    let number_result = char.parse::<i32>();

                    match number_result {
                        Ok(n) => {
                            part_creator.add_number(n, Coord {
                                x: x_index as i32,
                                y: y_index as i32,
                            })
                        }
                        _ => {
                            symbols.push(Symbol {
                                symbol: String::from(char),
                                coord: Coord {
                                    x: x_index as i32,
                                    y: y_index as i32,
                                },
                            })
                        }
                    }
                }
            }
        } else {
            panic!("wrong input")
        }
    }

    return (part_creator.get_parts(), symbols);
}

fn solve_part_2(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    let (parts, symbols) = parse_symbols_and_part_numbers(lines);

    for symbol in &symbols {
        if symbol.symbol == "*" {
            let mut adjacent_parts: Vec<&PartNumber> = Vec::new();
            let mut gear_ratio = 1;

            println!("* found");

            for part in &parts {
                for adjacent_coord in &part.adjacent_coords {
                    if *adjacent_coord == symbol.coord {
                        adjacent_parts.push(part);
                    }
                }
            }

            if adjacent_parts.len() == 2 {
                for adjacent_part in adjacent_parts {
                    gear_ratio =  gear_ratio * adjacent_part.number
                }

                sum = sum + gear_ratio;
            }
        }
    }


    return sum;
}


fn solve_part_1(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    let (parts, symbols) = parse_symbols_and_part_numbers(lines);

    for part in &parts {
        let mut is_adjacent_to_symbol = false;

        for adjacent_coord in &part.adjacent_coords {
            for symbol in &symbols {
                if symbol.coord == *adjacent_coord {
                    is_adjacent_to_symbol = true
                }

                if is_adjacent_to_symbol { break; }
            }
            if is_adjacent_to_symbol { break; }
        }


        if is_adjacent_to_symbol {
            sum = sum + part.number
        }
    }


    return sum;
}


fn get_input_lines() -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open("input")?;
    return Ok(BufReader::new(file).lines());
}


fn main() {
    let lines = get_input_lines().unwrap();

    let solution = solve_part_2(lines);

    println!("{:?}", solution);
}

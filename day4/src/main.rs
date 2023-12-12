use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    other_numbers: Vec<i32>,
}

impl Card {
    fn get_number_of_winning_numbers(&self) -> u32 {
        let mut count = 0;

        for winning_number in &self.winning_numbers {
            let mut is_winning = false;

            for number in &self.other_numbers {
                if number == winning_number {
                    is_winning = true
                }

                if is_winning {
                    break;
                }
            }

            if is_winning {
                count += 1;
            }
        }

        return count;
    }

    fn get_worth(&self) -> i32 {
        let count = self.get_number_of_winning_numbers();

        return if count != 0 { 2_i32.pow(count - 1) } else { 0 };
    }
}

#[derive(Debug)]
struct Input {
    cards: Vec<Card>,
}

fn parse_input(lines: Lines<BufReader<File>>) -> Input {
    let mut input = Input { cards: Vec::new() };

    for line in lines {
        match line {
            Ok(l) => {
                let mut card = Card {
                    id: 0,
                    winning_numbers: Vec::new(),
                    other_numbers: Vec::new(),
                };

                let parts = l.split(":").collect::<Vec<_>>();
                let id_string = (*parts.get(0).unwrap())
                    .split("Card")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect::<Vec<_>>();
                let id = id_string.get(0).unwrap();

                let numbers_string = parts.get(1).unwrap();
                let numbers_vec = (*numbers_string).split(" |").collect::<Vec<_>>();

                println!("{:?}", numbers_vec);

                let winning_numbers_string = numbers_vec.get(0).unwrap();
                let other_numbers_string = numbers_vec.get(1).unwrap();

                let winning_numbers = (*winning_numbers_string)
                    .split(" ")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect::<Vec<_>>();
                let other_numbers = (*other_numbers_string)
                    .split(" ")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect::<Vec<_>>();

                card.id = id.clone();
                card.winning_numbers = winning_numbers;
                card.other_numbers = other_numbers;

                input.cards.push(card);
            }
            Err(_) => panic!("OWARI DA"),
        }
    }

    return input;
}

fn get_input_lines() -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open("input")?;
    return Ok(BufReader::new(file).lines());
}

struct CardCounter {
    mapu: HashMap<i32, i32>,
}

impl CardCounter {
    fn new() -> Self {
        CardCounter {
            mapu: HashMap::new(),
        }
    }

    fn add_card(&mut self, card_id: i32, times: i32) {
        println!("adding <{}> {} times", card_id, times);

        let copies = self.get_copies(card_id);

        self.mapu.insert(card_id, copies + times);
    }

    fn get_copies(&self, card_id: i32) -> i32 {
        *self.mapu.get(&card_id).unwrap_or(&0)
    }

    fn get_total_number(&self) -> i32 {
        let mut total = 0;

        for (_, copies) in &self.mapu {
            total += copies;
        }

        return total;
    }

}

fn solve_p2(input: Input) -> i32 {

    let mut card_counter = CardCounter::new();

    for card in &input.cards {
        card_counter.add_card(card.id, 1);
        let copies = card_counter.get_copies(card.id);
        let number_of_winning_cards = card.get_number_of_winning_numbers();

        println!("{:?}", number_of_winning_cards);

        if number_of_winning_cards == 0 {
            continue;
        }

        for index in 1..=number_of_winning_cards {
            let extra_winning_card = card.id + index as i32;
            card_counter.add_card(extra_winning_card, copies);
        }

        println!("end of card processing");
    }

    return card_counter.get_total_number();
}

fn solve_p1(input: Input) -> i32 {
    let mut solution = 0;

    for card in &input.cards {
        solution = solution + card.get_worth()
    }

    return solution;
}

fn main() {
    let lines = get_input_lines().unwrap();
    let input = parse_input(lines);
    let solution = solve_p2(input);

    println!("{:?}", solution);
}

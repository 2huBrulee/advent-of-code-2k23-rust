use crate::core::FileLines;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
enum HandType {
    FiveOfAKind = 1,
    FourOfAKind = 2,
    FullHouse = 3,
    ThreeOfAKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

type Card = String;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_map: HashMap<String, i32> = HashMap::new();

        for card in &self.cards {
            let current_value = card_map.get(card).unwrap_or(&0);
            card_map.insert(card.clone(), current_value + 1);
        }

        let kvs = card_map.iter().collect::<Vec<_>>();

        if kvs.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if kvs.len() == 2 {
            if *kvs.first().unwrap().1 == 4 || *kvs.first().unwrap().1 == 1 {
                return HandType::FourOfAKind;
            }

            return HandType::FullHouse;
        }

        if kvs.len() == 3 {
            if *kvs.get(0).unwrap().1 == 3
                || *kvs.get(1).unwrap().1 == 3
                || *kvs.get(2).unwrap().1 == 3
            {
                return HandType::ThreeOfAKind;
            }

            return HandType::TwoPair;
        }

        if kvs.len() == 4 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut result = true;

        if self.cards.len() != other.cards.len() {
            return false;
        }

        for index in 1..=self.cards.len() {
            result = self.cards.get(index - 1).unwrap() == other.cards.get(index - 1).unwrap();
        }

        result
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_cmp = (other.get_type() as i32).cmp(&(self.get_type() as i32));

        if type_cmp != Ordering::Equal {
            return Some(type_cmp);
        }

        let mut card_cmp = Ordering::Equal;

        for index in 1..=self.cards.len() {
            let self_str = get_card_strenght(self.cards.get(index - 1).unwrap());
            let other_str = get_card_strenght(other.cards.get(index - 1).unwrap());

            if self_str > other_str {
                card_cmp = Ordering::Greater;
                break;
            }
            if self_str < other_str {
                card_cmp = Ordering::Less;
                break;
            }
        }

        Some(card_cmp)
    }
}

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: i32,
}

fn get_card_strenght(card: &Card) -> i32 {
    match card.as_str() {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        "9" => 9,
        "8" => 8,
        "7" => 7,
        "6" => 6,
        "5" => 5,
        "4" => 4,
        "3" => 3,
        "2" => 2,
        _ => 0,
    }
}

#[derive(Debug)]
struct Input {
    bids: Vec<HandBid>,
}

impl Input {
    fn sort(&mut self) {
        println!("MUTAITO");
        self.bids
            .sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    }
}

pub fn solve(lines: FileLines) -> i32 {
    let mut input = parse_input(lines);

    let mut result = 0;

    input.sort();

    println!("{:?}", input.bids);
    
    for index in 1..=input.bids.len() {
        let hand_bid = index as i32 * input.bids.get(index-1).unwrap().bid;
        println!("{:?}", hand_bid);
        result = result + hand_bid;
    }

    result
}

fn parse_input(lines: FileLines) -> Input {
    let mut input = Input { bids: Vec::new() };

    for line in lines {
        if let Ok(l) = line {
            let line_split = l.split(" ").collect::<Vec<_>>();
            let hand_str = line_split.get(0).unwrap();
            let bid_str = line_split.get(1).unwrap();

            let mut hand = Hand { cards: Vec::new() };

            hand.cards.append(
                hand_str
                    .split("")
                    .filter(|c| !c.eq(&""))
                    .map(|s| String::from(s))
                    .collect::<Vec<_>>()
                    .as_mut(),
            );

            let hand_bid = HandBid {
                hand,
                bid: bid_str.parse::<i32>().unwrap(),
            };

            input.bids.push(hand_bid);
        } else {
            panic!("bad input")
        }
    }

    input
}

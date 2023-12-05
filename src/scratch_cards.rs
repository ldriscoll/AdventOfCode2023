use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    plays: HashSet<u32>,
}

struct Winner<'a> {
    id: u32,
    winners: Vec<&'a u32>,
    score: u32,
}

impl Clone for Winner<'_> {
    fn clone(&self) -> Self {
        Winner {
            id: self.id,
            winners: self.winners.clone(),
            score: self.score
        }
    }
}
pub(crate) fn calculate_scratch_score(filename: &str) -> u32 {
    let games = load_cards(filename);

    let winners = gather_winners(&games);
    let mut total = 0;
    for winner in winners {
        total += winner.score;
        println!("Card {} has {} ({:?}) intersections with score {}",
                 winner.id,
                 winner.winners.len(),
                 winner.winners,
                 winner.score);
    }
    return total;
}

pub(crate) fn calculate_accumulating_score(filename: &str) -> u32 {
    let games = load_cards(filename);

    let winners = gather_winners(&games);
    let mut accumulator:HashMap<u32, u32> = HashMap::new();
    for cur in 0..winners.len() {
        accumulator.insert(1 + cur as u32, 1);
    }

    let mut total_cards = 0;
    for winner in winners {
        let my_count = accumulator.get(&winner.id).unwrap();
        total_cards += my_count;
        println!("Score of {} is {}, count {}", winner.id, winner.score, my_count);
        for next_cards in 0..winner.winners.len() {
            let next_id = winner.id + next_cards as u32 + 1;
            let new_total = accumulator.get(&next_id).unwrap() + accumulator.get(&winner.id).unwrap();
            accumulator.insert(next_id, new_total);
            println!("Incrementing accumulation of {} to {}", next_id, new_total);
        }

    }

    return total_cards;
}

fn gather_winners(cards: &Vec<Card>) -> Vec<Winner> {
    let winners:Vec<Winner> = cards.iter()
        .map(|card| {
            let winners:Vec<&u32> = card.winning_numbers.intersection(&card.plays).collect();
            let mut score = 0;
            let id = card.id;
            if !winners.is_empty() {
                score = 2u32.pow(winners.len() as u32 - 1);
            }
            Winner {
                id,
                winners,
                score,
            }
        })
        .collect();
    winners.to_vec()
}

fn load_cards(filename: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        cards.push(parse_card(line));
    }
    return cards;
}

fn parse_card(line: &str) -> Card {
    let by_colon = line.split_once(":").unwrap();
    let id = by_colon.0.split_whitespace().last().unwrap().trim().parse::<u32>().unwrap();
    let numbers = by_colon.1.split_once("|").unwrap();
    let winning_numbers = get_numbers(numbers.0);
    let plays = get_numbers(numbers.1);

    Card {
        id,
        winning_numbers,
        plays,
    }
}

fn get_numbers(numbers_as_string: &str) -> HashSet<u32> {
    numbers_as_string.trim().split_whitespace()
        .map(|st| st.parse::<u32>().unwrap())
        .collect()
}


use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    id: usize,
    card_numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        u32::try_from(
            self.card_numbers
                .iter()
                .filter(|number| self.winning_numbers.contains(number))
                .count(),
        )
        .unwrap()
    }
}

fn number_string_to_u32(num_str: &str) -> u32 {
    num_str.parse::<u32>().unwrap()
}

fn input_strings_to_cards(input: &str) -> (HashMap<usize, Card>, HashMap<usize, u32>) {
    let card_iter = input.lines().enumerate().map(|(i, line)| {
        let mut split_numbers = line.split(": ").nth(1).unwrap().split(" | ");

        let winning_numbers = HashSet::from_iter(
            split_numbers
                .next()
                .unwrap()
                .split_whitespace()
                .map(number_string_to_u32),
        );
        let card_numbers = split_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(number_string_to_u32)
            .collect::<Vec<u32>>();

        Card {
            id: i + 1,
            winning_numbers,
            card_numbers,
        }
    });

    let mut cards = HashMap::new();
    let mut copies = HashMap::new();
    for card in card_iter {
        let id = card.id;
        cards.insert(id, card);
        copies.insert(id, 1);
    }

    (cards, copies)
}

fn part2(input: &str) -> u32 {
    let mut overall_score = 0;
    let (cards, ref mut copies) = input_strings_to_cards(input);

    for id in 1..=cards.values().len() {
        let card = cards.get(&id).unwrap();
        let score = usize::try_from(card.get_score()).unwrap();

        if score > 0 {
            // if we have n winning cards we recieve n cards for all cards from id to score
            for next_id in (id + 1)..=(id + score) {
                let num_winning_cards = *copies.get(&id).unwrap();
                *copies.get_mut(&next_id).unwrap() += num_winning_cards;
            }
        }

        overall_score += copies.get(&id).unwrap()
    }

    overall_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part2(example);
        assert_eq!(result, 13);
    }
}

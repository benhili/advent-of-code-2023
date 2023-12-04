use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    card_numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        self.card_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .fold(0, |acc, number| {
                if self.winning_numbers.contains(number) {
                    if acc > 0 {
                        acc * 2
                    } else {
                        1
                    }
                } else {
                    acc
                }
            })
    }
}

fn number_string_to_u32(num_str: &str) -> u32 {
    num_str.parse::<u32>().unwrap()
}

fn input_strings_to_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
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
                winning_numbers,
                card_numbers,
            }
        })
        .collect::<Vec<Card>>()
}

fn part1(input: &str) -> u32 {
    let cards = input_strings_to_cards(input);
    cards.iter().map(|card| card.get_score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part1(example);
        assert_eq!(result, 13);
    }
}

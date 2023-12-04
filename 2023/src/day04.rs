use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

struct Card {
    id: usize,
    winning: HashSet<usize>,
    have: HashSet<usize>
}

impl Card {
    fn score(&self) -> usize {
        let matching_cards = self.count();
        match matching_cards {
            0 => 0,
            _ => 1 << (matching_cards - 1)
        }
    }
    #[inline]
    fn count(&self) -> usize {
         self.have.intersection(&self.winning).collect::<Vec<&usize>>().len()
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    let mut result = vec![];
    for card in input.lines() {
        let (card, numbers) = card.split_once(": ").unwrap();
        let id = card.split_whitespace().last().unwrap();
        let (winning, have) = numbers.split_once(" | ").unwrap();
        let winning = winning.split_whitespace()
            .map(|number| number.parse().unwrap()).collect();
        let have = have.split_whitespace()
            .map(|number| number.parse().unwrap()).collect();

        let card = Card {
            id: id.parse().unwrap(),
            winning,
            have
        };
        result.push(card);
    }
    result
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> usize {
    input.iter()
        .map(|card| card.score())
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> usize {
    let mut copies: HashMap<usize, usize> = HashMap::new();
    for card in input {
        let card_count = card.count();
        let current_count = copies.get(&card.id).unwrap_or(&0) + 1;
        for i in card.id + 1..card.id + 1 + card_count {
            let card_count = copies.entry(i).or_insert(0);
            *card_count += current_count;
        }
    }
    copies.values().sum::<usize>() + input.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 30);
    }
}
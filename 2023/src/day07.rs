use std::cmp::Ordering;
use std::iter::zip;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Eq, PartialEq)]
struct Hand {
    bid: u32,
    cards: [u32; 5],
    jokers: u32
}

impl Hand {
    fn new1(line: &str) -> Self {
        let (chars, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse().unwrap();

        let mut cards = [0; 5];
        let mut chars = chars.chars();
        for i in 0..5 {
            if let Some(card) = chars.next() {
                let value = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    num => num.to_digit(10).unwrap() - 2,
                };
                cards[i] = value;
            }
        }
        Hand{bid, cards, jokers: 0}
    }

    fn new2(line: &str) -> Self {
        let (chars, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse().unwrap();

        let mut cards = [0; 5];
        let mut chars = chars.chars();
        let mut jokers = 0;
        for i in 0..5 {
            if let Some(card) = chars.next() {
                let value = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 0,
                    'T' => 9,
                    num => num.to_digit(10).unwrap() - 1,
                };
                if value == 0 {
                    jokers += 1;
                }
                cards[i] = value;
            }
        }
        Hand{bid, cards, jokers}
    }

    fn hand_type(&self) -> HandType {
        let mut cards = [0;13];
        for card in self.cards {
            cards[card as usize] += 1
        }
        if self.jokers != 0 {
            cards[0] = 0;
        }
        cards.sort_by(|a, b| b.cmp(a));
        match cards[0] + self.jokers {
            5 => HandType::Five,
            4 => HandType::Four,
            3 if cards[1] == 2 => HandType::Full,
            3 => HandType::Three,
            2 if cards[1] == 2 => HandType::Pair,
            2 => HandType::Two,
            1 => HandType::High,
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();
        if self_hand_type == other_hand_type {
            for (self_card, other_card) in zip(self.cards, other.cards) {
                if self_card == other_card {
                    continue;
                }
                return self_card.cmp(&other_card)
            }
        }
        self_hand_type.cmp(&other_hand_type)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    High,
    Two,
    Pair,
    Three,
    Full,
    Four,
    Five,
}

impl From<HandType> for usize {
    fn from(value: HandType) -> Self {
        value as usize
    }
}

#[aoc_generator(day7, part1)]
fn parse1(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new1).collect()
}

#[aoc_generator(day7, part2)]
fn parse2(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new2).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> u32 {
    let hands: Vec<&Hand> = input.iter().sorted().collect();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (((i + 1) * hand.bid as usize) as u32))
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> u32 {
    let hands: Vec<&Hand> = input.iter().sorted().collect();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (((i + 1) * hand.bid as usize) as u32))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(EXAMPLE)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE)), 5905);
    }
}
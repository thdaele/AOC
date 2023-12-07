use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[inline]
fn get_card_value(card: char, part2: bool) -> u8 {
    if !part2 {
        match card {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            num => num.to_digit(10).unwrap() as u8 - 2,
        }
    } else {
        match card {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 0,
            'T' => 9,
            num => num.to_digit(10).unwrap() as u8 - 1,
        }
    }
}

// #[derive(Eq, PartialEq)]
struct Hand {
    bid: u32,
    // First value is the hand_type, the following values are the cards in order
    value: [u8; 6]
}

impl Hand {
    fn new(line: &str, part2: bool) -> Self {
        let (chars, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse().unwrap();

        let mut values = [0; 6];
        let mut chars = chars.chars();
        let mut jokers = 0;
        for value in values.iter_mut().skip(1) {
            let card = chars.next().unwrap();
            let card_value = get_card_value(card, part2);
            if part2 && card_value == 0 {
                jokers += 1;
            }
            *value = card_value;
        }
        values[0] = Hand::hand_type(&values, jokers).into();

        Hand {
            bid,
            value: values
        }
    }

    fn hand_type(value: &[u8; 6], jokers: u8) -> HandType {
        let mut cards = [0;13];
        for card in value.iter().skip(1) {
            cards[*card as usize] += 1
        }
        if jokers != 0 {
            cards[0] = 0;
        }
        cards.sort_unstable();
        cards.reverse();
        match cards[0] + jokers {
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

    #[allow(dead_code)]
    // Some experiments I did to see if the array sort is faster than a fast hashmap
    fn hand_type_hashmap(value: &[u8; 6], jokers: u8) -> HandType {
        let mut cards: FxHashMap<u8, u8> = FxHashMap::default();
        for card in &value[1..6] {
            let count = cards.entry(*card).or_insert(0);
            *count += 1;
        }
        if jokers != 0 {
            cards.remove(&0);
        }
        let (_, max_count) = cards.iter().max_by_key(|entry | entry.1).unwrap_or((&0, &0));
        match max_count + jokers {
            5 => HandType::Five,
            4 => HandType::Four,
            3 if cards.len() == 2 => HandType::Full,
            3 => HandType::Three,
            2 if cards.len() == 3 => HandType::Pair,
            2 => HandType::Two,
            1 => HandType::High,
            _ => unreachable!()
        }
    }
}

enum HandType {
    High,
    Two,
    Pair,
    Three,
    Full,
    Four,
    Five,
}

impl From<HandType> for u8 {
    fn from(value: HandType) -> Self {
        value as u8
    }
}

#[aoc_generator(day7, part1)]
fn parse1(input: &str) -> Vec<Hand> {
    input.lines().map(|line| Hand::new(line, false)).collect()
}

#[aoc_generator(day7, part2)]
fn parse2(input: &str) -> Vec<Hand> {
    input.lines().map(|line| Hand::new(line, true)).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> u32 {
    let hands = input.iter().sorted_unstable_by_key(|hand| hand.value);

    hands
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i as u32 + 1) * hand.bid))
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> u32 {
    let hands = input.iter().sorted_unstable_by_key(|hand| hand.value);

    hands
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i as u32 + 1) * hand.bid))
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
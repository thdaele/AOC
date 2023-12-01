use std::ops::Add;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Item {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<(bool, u32)> {
    let mut boss_stats: [u32; 3] = [0; 3];

    for (index, line) in input.lines().enumerate() {
        if let Some((_stat, value)) = line.split_once(": ") {
            boss_stats[index] = value.parse().unwrap();
        } else {
            unreachable!()
        }
    }

    let [boss_health, boss_damage, boss_armor] = boss_stats;

    let weapons = [
        Item{cost: 8, damage: 4, armor: 0},
        Item{cost: 10, damage: 5, armor: 0},
        Item{cost: 25, damage: 6, armor: 0},
        Item{cost: 40, damage: 7, armor: 0},
        Item{cost: 74, damage: 8, armor: 0},
    ];

    let armors = [
        Item{cost: 0, damage: 0, armor: 0},
        Item{cost: 13, damage: 0, armor: 1},
        Item{cost: 31, damage: 0, armor: 2},
        Item{cost: 53, damage: 0, armor: 3},
        Item{cost: 75, damage: 0, armor: 4},
        Item{cost: 102, damage: 0, armor: 5},
    ];

    let rings = [
        Item{cost: 25, damage: 1, armor: 0},
        Item{cost: 50, damage: 2, armor: 0},
        Item{cost: 100, damage: 3, armor: 0},
        Item{cost: 20, damage: 0, armor: 1},
        Item{cost: 40, damage: 0, armor: 2},
        Item{cost: 80, damage: 0, armor: 3},
    ];

    let mut rings_comb = Vec::with_capacity(22);
    rings_comb.push(Item{cost: 0, damage: 0, armor: 0});

    for i in 0..6 {
        rings_comb.push(rings[i]);
        for j in (i + 1)..6 {
            rings_comb.push(rings[i] + rings[j]);
        }
    }

    let mut results = Vec::with_capacity(660);

    for weapon in weapons {
        for armor in armors {
            for &third in &rings_comb {
                let Item {cost, damage, armor} = weapon + armor + third;

                let damage_to_boss = damage.saturating_sub(boss_armor).max(1);
                let you_turns = (boss_health + damage_to_boss - 1) / damage_to_boss;

                let damage_to_you = boss_damage.saturating_sub(armor).max(1);
                let boss_turns = (100 + damage_to_you - 1) / damage_to_you;

                let win = you_turns <= boss_turns;

                results.push((win, cost));
            }
        }
    }

    results
}

#[aoc(day21, part1)]
fn part1(input: &[(bool, u32)]) -> u32 {
    *input.iter()
        .filter(|(w, _)| *w)
        .map(|(_, c)| c)
        .min().unwrap()
}

#[aoc(day21, part2)]
fn part2(input: &[(bool, u32)]) -> u32 {
    *input.iter()
        .filter(|(w, _)| !*w)
        .map(|(_, c)| c)
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Hit Points: 103
Damage: 9
Armor: 2";
        assert_eq!(part1(&parse(input)), 121);
    }

    #[test]
    fn part2_example() {
        let input = "Hit Points: 103
Damage: 9
Armor: 2";
        assert_eq!(part2(&parse(input)), 201);
    }
}

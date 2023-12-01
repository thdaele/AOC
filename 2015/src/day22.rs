use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct State {
    boss_hp: i32,
    boss_damage: i32,
    player_hp: i32,
    player_mana: i32,
    shield_effect: u8,
    poison_effect: u8,
    recharge_effect: u8,
}

impl State {
    fn new(boss_hp: i32, boss_damage: i32) -> Self {
        State {
            boss_hp,
            boss_damage,
            player_hp: 50,
            player_mana: 500,
            shield_effect: 0,
            poison_effect: 0,
            recharge_effect: 0,
        }
    }

    fn apply_spell_effects(&mut self) {
        if self.shield_effect > 0 {
            self.shield_effect -= 1;
        }
        if self.poison_effect > 0 {
            self.poison_effect -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_effect > 0 {
            self.recharge_effect -= 1;
            self.player_mana += 101;
        }
    }

    fn boss_attack(&mut self) {
        let mut attack = self.boss_damage;
        if self.shield_effect > 0 {
            attack = (self.boss_damage - 7).max(1);
        }

        self.player_hp -= attack;
    }

    fn player_alive(&self) -> bool {
        self.player_hp > 0
    }

    fn boss_dead(&self) -> bool {
        self.boss_hp <= 0
    }

    fn magic_missile(&mut self) -> Option<(i32, bool)> {
        if self.player_mana >= 53 {
            self.boss_hp -= 4;
            self.player_mana -= 53;

            self.apply_spell_effects();
            self.boss_attack();
            return Some((53, self.boss_dead()));
        }
        None
    }

    fn drain(&mut self) -> Option<(i32, bool)> {
        if self.player_mana >= 73 {
            self.boss_hp -= 2;
            self.player_hp += 2;
            self.player_mana -= 73;

            self.apply_spell_effects();
            self.boss_attack();
            return Some((73, self.boss_dead()));
        }
        None
    }

    fn shield(&mut self) -> Option<(i32, bool)> {
        if self.player_mana >= 113 && self.shield_effect == 0 {
            self.shield_effect = 6;
            self.player_mana -= 113;

            self.apply_spell_effects();
            self.boss_attack();
            return Some((113, self.boss_dead()));
        }
        None
    }

    fn poison(&mut self) -> Option<(i32, bool)> {
        if self.player_mana >= 173 && self.poison_effect == 0 {
            self.poison_effect = 6;
            self.player_mana -= 173;

            self.apply_spell_effects();
            self.boss_attack();
            return Some((173, self.boss_dead()));
        }
        None
    }

    fn recharge(&mut self) -> Option<(i32, bool)> {
        if self.player_mana >= 229 && self.recharge_effect == 0 {
            self.recharge_effect = 5;
            self.player_mana -= 229;

            self.apply_spell_effects();
            self.boss_attack();
            return Some((229, self.boss_dead()));
        }
        None
    }
}


#[aoc_generator(day22)]
fn parse(input: &str) -> [i32; 2] {
    let mut boss_stats: [i32; 2] = [0; 2];

    for (index, line) in input.lines().enumerate() {
        if let Some((_stat, value)) = line.split_once(": ") {
            boss_stats[index] = value.parse().unwrap();
        } else {
            unreachable!()
        }
    }

    boss_stats
}

#[aoc(day22, part1)]
fn part1(input: &[i32; 2]) -> i32 {
    let [boss_hp, boss_damage] = input;
    let start = State::new(*boss_hp, *boss_damage);

    play(start, false)
}

#[aoc(day22, part2)]
fn part2(input: &[i32; 2]) -> i32 {
    let [boss_hp, boss_damage] = input;
    let start = State::new(*boss_hp, *boss_damage);

    play(start, true)
}

fn play(start: State, part_2: bool) -> i32 {
    let mut todo = BinaryHeap::new();
    let mut memoization = HashSet::new();

    todo.push(Reverse((0, start)));
    memoization.insert(start);

    while let Some(Reverse((spent, mut state))) = todo.pop() {
        state.apply_spell_effects();
        if state.boss_dead() {
            return spent;
        }

        if part_2 {
            state.player_hp -= 1;
            if state.player_hp <= 0 {
                continue;
            }
        }

        let mut state_copy = state;
        if let Some((spell_cost, boss_dead)) = state_copy.magic_missile() {
            if boss_dead {
                return spent + spell_cost;
            }

            if state_copy.player_alive() && memoization.insert(state_copy) {
                todo.push(Reverse((spent + spell_cost, state_copy)))
            }
        }

        let mut state_copy = state;
        if let Some((spell_cost, boss_dead)) = state_copy.drain() {
            if boss_dead {
                return spent + spell_cost;
            }

            if state_copy.player_alive() && memoization.insert(state_copy) {
                todo.push(Reverse((spent + spell_cost, state_copy)))
            }
        }

        let mut state_copy = state;
        if let Some((spell_cost, boss_dead)) = state_copy.shield() {
            if boss_dead {
                return spent + spell_cost;
            }

            if state_copy.player_alive() && memoization.insert(state_copy) {
                todo.push(Reverse((spent + spell_cost, state_copy)))
            }
        }

        let mut state_copy = state;
        if let Some((spell_cost, boss_dead)) = state_copy.poison() {
            if boss_dead {
                return spent + spell_cost;
            }

            if state_copy.player_alive() && memoization.insert(state_copy) {
                todo.push(Reverse((spent + spell_cost, state_copy)))
            }
        }

        let mut state_copy = state;
        if let Some((spell_cost, boss_dead)) = state_copy.recharge() {
            if boss_dead {
                return spent + spell_cost;
            }

            if state_copy.player_alive() && memoization.insert(state_copy) {
                todo.push(Reverse((spent + spell_cost, state_copy)))
            }
        }
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Hit Points: 71
Damage: 10";
        assert_eq!(part1(&parse(input)), 1824);
    }

    #[test]
    fn part2_example() {
        let input = "Hit Points: 71
Damage: 10";
        assert_eq!(part2(&parse(input)), 1937);
    }
}
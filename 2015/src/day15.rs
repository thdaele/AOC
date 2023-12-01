use std::cmp;
use std::iter::zip;
use itertools::{any, iproduct};

#[derive(Debug)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let value = value.replace(',', "");
        let mut parts = value.split_whitespace().step_by(2).skip(1);
        Ingredient {
            capacity: parts.next().unwrap().parse().unwrap(),
            durability: parts.next().unwrap().parse().unwrap(),
            flavor: parts.next().unwrap().parse().unwrap(),
            texture: parts.next().unwrap().parse().unwrap(),
            calories: parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn find_best(ingredients: &[Ingredient], num: i32, cal: bool) -> i32 {
    let mut max_score = i32::MIN;
    for (a, b, c) in iproduct!(0..num, 0..num, 0..num) {
        let new_num = num - (a + b + c);
        if new_num < 0 {
            continue;
        }
        let amounts = [a, b, c, new_num];
        let score = calc_score(zip(ingredients, amounts), cal);
        max_score = cmp::max(max_score, score);
    }
    max_score
}

fn calc_score<'a>(amounts: impl Iterator<Item = (&'a Ingredient, i32)>, cal: bool) -> i32 {
    let (mut total_cap, mut total_dur, mut total_flav, mut total_text, mut total_cal) = (0, 0, 0, 0, 0);
    let max_calories = 500;

    for (ingredient, num) in amounts {
        total_cap += ingredient.capacity * num;
        total_dur += ingredient.durability * num;
        total_flav += ingredient.flavor * num;
        total_text += ingredient.texture * num;
        total_cal += ingredient.calories * num;
    }
    let all = [total_cap, total_dur, total_flav, total_text];
    if any(all, |x| x < 1) {
        return 0;
    }

    if cal && total_cal != max_calories {
        return 0;
    }
    total_cap * total_dur * total_flav * total_text
}


#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Ingredient> {
    input.lines().map(Ingredient::from).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(ingredients: &[Ingredient]) -> i32 {
    find_best(ingredients, 100, false)
}

#[aoc(day15, part2)]
pub fn solve_part2(ingredients: &[Ingredient]) -> i32 {
    find_best(ingredients, 100, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve_part1(&input_generator(input)), 62842880)
    }

    #[test]
    fn example2() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve_part2(&input_generator(input)), 57600000)
    }
}
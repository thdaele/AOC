use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default)]
struct ColorSet {
    blue: usize,
    red: usize,
    green: usize
}

struct Game {
    color_set: Vec<ColorSet>,
    id: usize
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    let mut games = vec![];
    for line in input.lines() {
        let (game, colors) = line.split_once(": ").unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        let mut game = Game {
            color_set: vec![],
            id: id.parse().unwrap()
        };
        let sets = colors.split("; ");
        for set in sets {
            let mut color_set = ColorSet::default();
            let colors = set.split(", ");
            for color in colors {
                let (amount, kind) = color.split_once(' ').unwrap();
                match kind {
                    "blue" => color_set.blue = amount.parse().unwrap(),
                    "red" => color_set.red = amount.parse().unwrap(),
                    "green" => color_set.green = amount.parse().unwrap(),
                    _ => unreachable!()
                }
            }
            game.color_set.push(color_set);
        }
        games.push(game);
    }
    games
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> usize {
    input.iter()
        .filter(|game| {
            game.color_set.iter().all(|color_set| color_set.red <= 12 && color_set.green <= 13 && color_set.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> usize {
    let mut result = 0;
    for game in input {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color_set in &game.color_set {
            if color_set.red > red {
                red = color_set.red;
            }
            if color_set.green > green {
                green = color_set.green;
            }
            if color_set.blue > blue {
                blue = color_set.blue;
            }
        }
        result += red * blue * green;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 2286);
    }
}
use crate::day16::Match::{Equal, Greater, Less};

#[derive(Default)]
pub struct Info {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
    part_2: bool
}

impl Info {
    pub fn new(children: u32, cats: u32, samoyeds: u32, pomeranians: u32, akitas: u32, vizslas: u32,
               goldfish: u32, trees: u32, cars: u32, perfumes: u32) -> Self {
        Self {
            children: Some(children),
            cats: Some(cats),
            samoyeds: Some(samoyeds),
            pomeranians: Some(pomeranians),
            akitas: Some(akitas),
            vizslas: Some(vizslas),
            goldfish: Some(goldfish),
            trees: Some(trees),
            cars: Some(cars),
            perfumes: Some(perfumes),
            part_2: false
        }
    }
}

impl From<&str> for Info {
    fn from(value: &str) -> Self {
        let value = value.replace(',', ":");
        let mut parts = value.split(':').skip(1);
        let mut info = Info::default();
        while let Some(object) = parts.next() {
            let amount = parts.next().unwrap().trim().parse().unwrap();
            match object.trim() {
                "children" => info.children = Some(amount),
                "cats" => info.cats = Some(amount),
                "samoyeds" => info.samoyeds = Some(amount),
                "pomeranians" => info.pomeranians = Some(amount),
                "akitas" => info.akitas = Some(amount),
                "vizslas" => info.vizslas = Some(amount),
                "goldfish" => info.goldfish = Some(amount),
                "trees" => info.trees = Some(amount),
                "cars" => info.cars = Some(amount),
                "perfumes" => info.perfumes = Some(amount),
                _ => unreachable!()
            }
        }
        info
    }
}

enum Match {
    Equal,
    Greater,
    Less
}

impl PartialEq for Info {
    fn eq(&self, other: &Info) -> bool {
        let part_2 = self.part_2 || other.part_2;
        compare(self.children, other.children, part_2, Equal)
        && compare(self.cats, other.cats, part_2, Greater)
        && compare(self.samoyeds, other.samoyeds, part_2, Equal)
        && compare(self.pomeranians, other.pomeranians, part_2, Less)
        && compare(self.akitas, other.akitas, part_2, Equal)
        && compare(self.vizslas, other.vizslas, part_2, Equal)
        && compare(self.goldfish, other.goldfish, part_2, Less)
        && compare(self.trees, other.trees, part_2, Greater)
        && compare(self.cars, other.cars, part_2, Equal)
        && compare(self.perfumes, other.perfumes, part_2, Equal)
    }
}

fn compare(this: Option<u32>, other: Option<u32>, part_2: bool, matched: Match) -> bool {
    match (this, other) {
        (Some(this), Some(other)) => {
            if part_2 {
                match matched {
                    Equal => this == other,
                    Greater => this > other,
                    Less => this < other
                }
            } else {
                this == other
            }
        },
        (Some(_this), None) => true,
        (None, Some(_other)) => true,
        (None, None) => true
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Info> {
    input.lines().map(Info::from).collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(aunts: &[Info]) -> usize {
    let aunt_sue = Info::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);
    // Input is 1-indexed
    aunts.iter()
        .position(|info| info == &aunt_sue).unwrap() + 1
}

#[aoc(day16, part2)]
pub fn solve_part2(aunts: &[Info]) -> usize {
    let mut aunt_sue = Info::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);
    aunt_sue.part_2 = true;
    // Input is 1-indexed
    aunts.iter()
        .position(|info| info == &aunt_sue).unwrap() + 1
}

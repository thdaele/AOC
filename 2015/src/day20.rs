use aoc_runner_derive::aoc;

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let number: usize = input.parse().unwrap();

    let mut presents = vec![10; number / 10];
    for elf in 2..number / 10 {
        for house in (elf..number / 10).step_by(elf) {
            presents[house] += elf * 10;
        }
        if presents[elf] >= number {
            return elf;
        }
    }
    unreachable!()
    // Takes too damm long
    // for house in 1..number / 10 {
    //     let mut presents = 0;
    //     for elf in 1..=house {
    //         if house % elf == 0 {
    //             presents += 10 * elf;
    //         }
    //     }
    //     if presents >= number {
    //         return house;
    //     }
    // }
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let number: usize = input.parse().unwrap();

    let mut presents = vec![10; number / 10];
    for elf in 2..number / 10 {
        let mut visited = 0;
        for house in (elf..number / 10).step_by(elf) {
            presents[house] += elf * 11;
            visited += 1;
            if visited == 50 {
                break;
            }
        }
        if presents[elf] >= number {
            return elf;
        }
    }
    unreachable!()
}

use aoc_runner_derive::aoc;

fn hash(input: &str) -> u8 {
    let mut result: u8 = 0;
    for ascii in input.as_bytes() {
        result = result.wrapping_add(*ascii);
        result = result.wrapping_mul(17);
    }
    result
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    input.split(',').map(|substr| hash(substr) as u32).sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u32 {
    let mut boxes:Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    for substr in input.split(',') {
        if substr.contains('=') {
            let (label_str, focal) = substr.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let label = hash(label_str);

            // Use hashcode to find the correct box
            let lens_slots = &mut boxes[label as usize];
            // Check box for lens with same label_str
            let pos = lens_slots.iter().position(|lens| lens.0 == label_str);
            if let Some(pos) = pos {
                // replace the old lens with the new lens
                let lens = &mut lens_slots[pos];
                lens.1 = focal;
            } else {
                lens_slots.push((label_str, focal));
            }
        } else {
            // '-' is always the last character, remove it to get the label
            let label_str = &substr[..substr.len() - 1];
            let label = hash(label_str);

            // Use hashcode to find the correct box
            let lens_slots = &mut boxes[label as usize];
            // Check box for lens with same label_str
            let pos = lens_slots.iter().position(|lens| lens.0 == label_str);
            if let Some(pos) = pos {
                // Remove if the lens was found
                lens_slots.remove(pos);
            }
        }
    }
    boxes.iter().enumerate().map(|(box_index, lens_slots)| {
        (box_index as u32 + 1) * lens_slots.iter().enumerate().map(|(slot, (_, lens))| (slot as u32 + 1) * *lens as u32).sum::<u32>()
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 145);
    }
}
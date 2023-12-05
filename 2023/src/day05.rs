use std::cmp::min;
use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};

struct MappingEntry {
    destination: Range<usize>,
    source: Range<usize>
}

struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Vec<MappingEntry>>
}

#[inline]
fn apply_mapping(seed: usize, entry: &MappingEntry) -> usize {
    (seed - entry.source.start) + entry.destination.start
}

#[inline]
fn apply_mapping_rev(location: usize, entry: &MappingEntry) -> usize {
    (location - entry.destination.start) + entry.source.start
}

fn update_seed(mut seed: usize, almanac: &Almanac) -> usize {
    // Mappings are in the correct order so we can just loop over the mappings and update the seed
    for mapping in &almanac.mappings {
        for entry in mapping {
            if entry.source.contains(&seed) {
                seed = apply_mapping(seed, entry);
                break;
            }
        }
    }
    seed
}

fn update_location(mut location: usize, almanac: &Almanac) -> usize {
    for mapping in almanac.mappings.iter().rev() {
        for entry in mapping {
            if entry.destination.contains(&location) {
                location = apply_mapping_rev(location, entry);
                break;
            }
        }
    }
    location
}

fn update_range(mut ranges: Vec<Range<usize>>, almanac: &Almanac) -> Vec<Range<usize>> {
    // Mappings are in the correct order so we can just loop over the mappings and update the ranges
    for mapping in &almanac.mappings {
        let mut new_ranges = vec![];
        for cur_range in ranges {
            let mut range = cur_range.clone();
            // Mapping entries are sorted on start of source and tiebreaker on end of source
            // So we can just loop and update the ranges
            for entry in mapping {
                // Check if we still need to map the range further
                if range.start < range.end && range.end >= entry.source.start {
                    new_ranges.extend(range.consume_and_map(entry));
                } else {
                    break;
                }
            }
            // The case when there are no mappings for the range or for the tail of the range
            if range.start < range.end {
                new_ranges.push(range);
            }
        }
        ranges = new_ranges;
    }
    ranges
}

trait RangeMerge<Rhs=Self> {
    type Output;
    fn overlap(&self, rhs: &Rhs) -> bool;

    fn consume_and_map(&mut self, mapping_entry: &MappingEntry) -> Vec<Self::Output>;
}

impl RangeMerge for Range<usize> {
    type Output = Range<usize>;
    #[inline]
    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn consume_and_map(&mut self, entry: &MappingEntry) -> Vec<Self::Output> {
        let mut result = vec![];
        // Check if we can use this entry to map (part of) the range
        if self.overlap(&entry.source) {
            if self.start < entry.source.start {
                result.push(self.start..entry.source.start);
                self.start = entry.source.start;
            }
            if self.end < entry.source.end {
                result.push(apply_mapping(self.start, entry)..apply_mapping(self.end, entry));
                self.start = self.end;
            } else {
                result.push(apply_mapping(self.start, entry)..apply_mapping(entry.source.end, entry));
                self.start = entry.source.end;
            }
        }
        result
    }
}


#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = seeds.split(':').last().unwrap()
        .split_whitespace().map(|seed| seed.parse().unwrap()).collect();

    let mut mappings = vec![];
    for map in maps.split("\n\n") {
        let (_, map) = map.split_once(':').unwrap();
        let mut mapping = vec![];
        for entry in map.trim().lines() {
            let mut values = entry.split_whitespace().map(|value| value.parse().unwrap());
            let destination_start = values.next().unwrap();
            let source_start = values.next().unwrap();
            let range_length = values.next().unwrap();
            let mapping_entry = MappingEntry {
                destination: destination_start..destination_start+range_length,
                source: source_start..source_start+range_length
            };
            mapping.push(mapping_entry);
        }
        // Needs this sort for part 2
        mapping.sort_by_key(|k| (k.source.start, k.source.end));
        mappings.push(mapping);
    }

    Almanac {
        seeds,
        mappings
    }
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> usize {
    input.seeds
        .iter()
        .map(|&seed| update_seed(seed, input))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> usize {
    let ranges: Vec<Range<usize>> = input.seeds
        .chunks(2)
        .map(|range| range[0]..range[0]+range[1])
        .collect();

    let result = update_range(ranges, input);
    result.iter().min_by_key(|k| k.start).unwrap().start
}

#[aoc(day5, part2, bruteforce)]
fn part2_bruteforce(input: &Almanac) -> usize {
    let ranges: Vec<Range<usize>> = input.seeds
        .chunks(2)
        .map(|range| range[0]..range[0]+range[1])
        .collect();
    let mut result = usize::MAX;
    for range in ranges {
        let range_min = range.into_iter()
            .map(|seed| update_seed(seed, input))
            .min()
            .unwrap();
        result = min(result, range_min);
    }
    result
}

#[aoc(day5, part2, backward_bruteforce)]
fn part2_backward_bruteforce(input: &Almanac) -> usize {
    let ranges: Vec<Range<usize>> = input.seeds
        .chunks(2)
        .map(|range| range[0]..range[0]+range[1])
        .collect();
    for location in 1..usize::MAX {
        let seed = update_location(location, input);

        if ranges.iter().any(|range| range.contains(&seed)) {
            return location;
        }
    }
    unreachable!()
}



#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 46);
    }
}
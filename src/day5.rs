use aoc_runner_derive::{aoc, aoc_generator};

pub type SeedToLocations = (Vec<u64>, Vec<Vec<Vec<u64>>>);

#[aoc_generator(day5)]
pub fn read_input(input: &str) -> SeedToLocations {
    let mut blocks = input.split("\n\n");
    let seeds: Vec<u64> = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (
        seeds,
        blocks
            .map(|s| {
                s.lines()
                    .skip(1)
                    .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
                    .collect()
            })
            .collect(),
    )
}

fn apply_section((mut low, mut ilen): (u64, u64), section: &[Vec<u64>]) -> Vec<(u64, u64)> {
    let mut seed = Vec::new();
    for mapping in section {
        let (destination, origin, length) = (mapping[0], mapping[1], mapping[2]);
        if origin >= low && origin < low + ilen {
            if origin > low {
                seed.push((low, origin - low));
                ilen -= origin - low;
                low = origin;
            }
            let l = length.min(ilen);
            seed.push((destination, l));
            low += l;
            ilen -= l;
        } else if origin < low && origin + length > low {
            let l = (origin + length - low).min(ilen);
            seed.push((low - origin + destination, l));
            low += l;
            ilen -= l;
        }
    }
    if ilen > 0 {
        seed.push((low, ilen));
    }
    seed
}
fn apply_sections(interval: (u64, u64), sections: &[Vec<Vec<u64>>]) -> Vec<(u64, u64)> {
    if sections.is_empty() {
        vec![interval]
    } else {
        apply_section(interval, &sections[0])
            .into_iter()
            .flat_map(|i| apply_sections(i, &sections[1..]))
            .collect()
    }
}

#[aoc(day5, part1)]
pub fn solve_part1((seeds, maps): &SeedToLocations) -> u64 {
    seeds
        .clone()
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |s, map| {
                for m in map {
                    if s >= m[1] && s < m[1] + m[2] {
                        return s - m[1] + m[0];
                    }
                }
                s
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &SeedToLocations) -> u64 {
    let (seeds, mut maps) = input.clone();
    for section in &mut maps {
        section.sort_unstable_by_key(|i| i[1]);
    }
    seeds
        .chunks_exact(2)
        .map(|chunk| {
            apply_sections((chunk[0], chunk[1]), &maps)
                .into_iter()
                .min_by_key(|(l, _)| *l)
                .unwrap()
                .0
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn finds_closest_location_for_test_input() {
        let input = r#"
seeds: 79 14 55 13

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
56 93 4
"#
        .trim();
        let maps = read_input(input);
        assert_eq!(solve_part1(&maps), 35);
    }

    #[test]
    pub fn solves_part1() {
        let input = include_str!("../input/2023/day5.txt").trim();
        let maps = read_input(input);
        assert_eq!(solve_part1(&maps), 251346198);
    }

    #[test]
    pub fn part_2_works_for_test_input() {
        let input = r#"
seeds: 79 14 55 13

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
56 93 4
"#
        .trim();
        assert_eq!(solve_part2(&read_input(input)), 46);
    }

    #[test]
    pub fn solves_part_2() {
        let input = include_str!("../input/2023/day5.txt").trim();
        let maps = read_input(input);

        assert_eq!(solve_part2(&maps), 72263011);
    }
}

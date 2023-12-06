use aoc_runner_derive::{aoc, aoc_generator};

pub struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    pub fn constant_time_wins(&self) -> u64 {
        let d = self.duration as f64 / 2.0;
        let d_2 = d * d;
        let root = (d_2 - self.record as f64).sqrt();
        let from_dur = (d - root) as u64;
        let to_dur = (d + root) as u64;
        to_dur - from_dur
    }
}

#[aoc_generator(day6, part1)]
pub fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            duration: *t,
            record: *d,
        })
        .collect()
}

#[aoc_generator(day6, part2)]
fn parse_input_part2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .fold("".to_string(), |acc, s| acc + s)
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .fold("".to_string(), |acc, s| acc + s)
        .parse()
        .unwrap();
    Race {
        duration: time,
        record: distance,
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &[Race]) -> u64 {
    input
        .iter()
        .fold(1, |acc, race| acc * race.constant_time_wins())
}

#[aoc(day6, part2)]
pub fn part2(input: &Race) -> u64 {
    input.constant_time_wins()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    #[test]
    pub fn can_read_test_input() {
        let races = parse_input(TEST_INPUT);
        assert_eq!(races.len(), 3);
    }

    #[test]
    pub fn can_solve_part1_test_input() {
        let races = parse_input(TEST_INPUT);
        let possibilities = races.iter().fold(1, |acc, race| acc * race.possible_wins());
        assert_eq!(possibilities, 4 * 8 * 9);
    }

    #[test]
    pub fn solves_part1() {
        let races = parse_input(include_str!("../input/2023/day6.txt"));
        assert_eq!(part1(&races), 2612736);
    }

    #[test]
    pub fn can_solve_part2_for_testinput() {
        let race = parse_input_part2(TEST_INPUT);
        assert_eq!(race.possible_wins(), 71503);
    }

    #[test]
    pub fn solves_part2() {
        let race = parse_input_part2(include_str!("../input/2023/day6.txt"));
        assert_eq!(part2(&race), 29891250);
    }
}

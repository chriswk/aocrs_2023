use aoc_runner_derive::{aoc, aoc_generator};
use num::Zero;
pub type Sequence = Vec<isize>;
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn find_next_values(mut sequence: Sequence) -> isize {
    let mut last_numbers = vec![*sequence.last().unwrap()];
    loop {
        let new_sequence: Vec<_> = sequence.windows(2).map(|s| s[1] - s[0]).collect();

        if new_sequence.iter().all(|n| n.is_zero()) {
            break;
        }
        sequence = new_sequence;
        last_numbers.push(*sequence.last().unwrap());
    }
    last_numbers.iter().sum()
}

fn find_previous_values(mut sequence: Sequence) -> isize {
    let mut first_numbers = vec![*sequence.first().unwrap()];
    loop {
        let new_sequence: Vec<_> = sequence.windows(2).map(|s| s[1] - s[0]).collect();
        if new_sequence.iter().all(|n| n.is_zero()) {
            break;
        }
        sequence = new_sequence;
        first_numbers.push(*sequence.first().unwrap());
    }
    first_numbers.iter().rev().fold(0, |acc, n| n - acc)
}

#[aoc(day9, part1)]
fn part1(input: &[Sequence]) -> isize {
    input.iter().map(|s| find_next_values(s.clone())).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Sequence]) -> isize {
    input.iter().map(|s| find_previous_values(s.clone())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TEST: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SMALL_TEST)), 114);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day9.txt"))),
            2005352194
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SMALL_TEST)), 2);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(include_str!("../input/2023/day9.txt"))), 1077);
    }
}

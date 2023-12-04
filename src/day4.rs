use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub winning: HashSet<u32>,
    pub own: HashSet<u32>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Card> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let numbers = line
                .split(':')
                .nth(1)
                .map(|f| f.split('|'))
                .unwrap()
                .collect::<Vec<_>>();
            let winning = numbers
                .get(0)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let own = numbers
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            Card {
                id: id + 1,
                winning,
                own,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|c| {
            let intersect = c.own.intersection(&c.winning).count();
            if intersect == 0 {
                return 0;
            }
            1 << (intersect - 1)
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> u32 {
    let mut copies: HashMap<u32, u32> = HashMap::new();
    let max_id = input.iter().map(|c| c.id).max().unwrap();
    for card in input {
        let winning = card.own.intersection(&card.winning).count();
        let copies_of_me = copies
            .entry(card.id as u32)
            .and_modify(|c| *c += 1)
            .or_insert(1)
            .clone();
        if winning == 0 {
            continue;
        }

        let start_index = card.id + 1;
        let end_index = min(start_index + winning - 1, max_id);
        for i in start_index..=end_index {
            copies
                .entry(i as u32)
                .and_modify(|c| *c += copies_of_me.clone())
                .or_insert(copies_of_me.clone());
        }
    }
    copies.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_handle_test_input_part1() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim();
        let input = generator(input);
        assert_eq!(solve_part1(&input), 13)
    }
    #[test]
    pub fn can_handle_test_input_part2() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim();
        let input = generator(input);
        assert_eq!(solve_part2(&input), 30);
    }
}

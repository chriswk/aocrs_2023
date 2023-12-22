use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

pub type Network = HashMap<String, (String, String)>;

#[aoc_generator(day8)]
fn parse(input: &str) -> (String, Network) {
    let instructions = input.lines().next().unwrap().chars().collect();
    let pattern = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
    let nodes = input
        .lines()
        .skip(2)
        .map(|l| {
            let c = pattern.captures(l).unwrap();
            let start = String::from(c.get(1).unwrap().as_str());
            let left = String::from(c.get(2).unwrap().as_str());
            let right = String::from(c.get(3).unwrap().as_str());
            (start, (left, right))
        })
        .collect();
    (instructions, nodes)
}

fn walk_network(
    instructions: &str,
    network: &Network,
    select_start_nodes: fn(&str) -> bool,
    is_end_node: fn(&str) -> bool,
) -> usize {
    network
        .keys()
        .filter(|node| select_start_nodes(node))
        .map(|current_node| {
            calculate_steps(instructions, network, is_end_node, current_node.as_str())
        })
        .reduce(lcm)
        .unwrap()
}

fn calculate_steps<'a>(
    instructions: &str,
    network: &'a Network,
    is_end_node: fn(&str) -> bool,
    mut current_node: &'a str,
) -> usize {
    let mut steps = 0;
    for step in instructions.chars().cycle() {
        if is_end_node(current_node) {
            return steps;
        }
        steps += 1;
        let (left, right) = network.get(current_node).unwrap();
        if step == 'L' {
            current_node = left.as_str();
        } else if step == 'R' {
            current_node = right.as_str();
        }
        if is_end_node(current_node) {
            return steps;
        }
    }
    panic!("ZZZ is not found");
}

#[aoc(day8, part1)]
fn part1((instructions, network): &(String, Network)) -> usize {
    walk_network(instructions, network, |n| n == "AAA", |n| n == "ZZZ")
}

#[aoc(day8, part2)]
fn part2((instructions, network): &(String, Network)) -> usize {
    walk_network(
        instructions,
        network,
        |n| n.ends_with('A'),
        |n| n.ends_with('Z'),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EASY_EXAMPLE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const LOOPING_EXAMPLE: &str = r#"LLR
    
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const GHOSTING_EXAMPLE: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EASY_EXAMPLE)), 2);
    }

    #[test]
    fn part1_longer_example() {
        assert_eq!(part1(&parse(LOOPING_EXAMPLE)), 6);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(&parse(include_str!("../input/2023/day8.txt"))), 18157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(GHOSTING_EXAMPLE)), 6);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day8.txt"))),
            14299763833181
        );
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let g = input
        .lines()
        .map(|l| {
            let (src, targets) = l.split_once(" -> ").unwrap();
            let destinations = targets.split(", ").collect::<Vec<_>>();
            if src == "broadcaster" {
                (src, (b'b', destinations))
            } else {
                (&src[1..], (src.as_bytes()[0], destinations))
            }
        })
        .collect::<HashMap<_, _>>();
    let mut state = HashSet::new();
    let mut conjunctions = HashMap::<&str, HashMap<&str, bool>>::new();
    for (&node, (_, connections)) in &g {
        for n in connections {
            match g.get(n) {
                Some((node_type, _)) => {
                    if node_type == &b'&' {
                        conjunctions.entry(n).or_default().insert(&node, false);
                    }
                }
                None => continue,
            }
        }
    }
    let mut lows = 0;
    let mut highs = 0;
    for t in 0..1000 {
        let mut q = VecDeque::from_iter([("broadcaster", "button", false)]);
        while let Some((node, previous, high)) = q.pop_front() {
            if high {
                highs += 1;
            } else {
                lows += 1;
            }
            match g.get(node) {
                Some((node_type, connections)) => {
                    let pulse = match node_type {
                        b'b' => false,
                        b'%' => {
                            if high {
                                continue;
                            }
                            let on = state.contains(node);
                            if on {
                                state.remove(node);
                            } else {
                                state.insert(node);
                            }
                            !on
                        }
                        b'&' => {
                            conjunctions.get_mut(node).unwrap().insert(previous, high);
                            !conjunctions[node].values().all(|&b| b)
                        }
                        _ => unreachable!("Invalid node"),
                    };
                    q.extend(connections.iter().map(|n| (*n, node, pulse)));
                }
                None => continue,
            };
        }
    }
    highs * lows
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let g = input
        .lines()
        .map(|l| {
            let (src, targets) = l.split_once(" -> ").unwrap();
            let destinations = targets.split(", ").collect::<Vec<_>>();
            if src == "broadcaster" {
                (src, (b'b', destinations))
            } else {
                (&src[1..], (src.as_bytes()[0], destinations))
            }
        })
        .collect::<HashMap<_, _>>();
    let mut state = HashSet::new();
    let mut conjunctions = HashMap::<&str, HashMap<&str, bool>>::new();
    for (&node, (_, connections)) in &g {
        for n in connections {
            match g.get(n) {
                Some((node_type, _)) => {
                    if node_type == &b'&' {
                        conjunctions.entry(n).or_default().insert(&node, false);
                    }
                }
                None => continue,
            }
        }
    }
    let mut cycles = [None; 4];
    for t in 0.. {
        let mut q = VecDeque::from_iter([("broadcaster", "button", false)]);
        while let Some((node, prev, high)) = q.pop_front() {
            if high && node == "kl" {
                let i = match prev {
                    "mk" => 0,
                    "fp" => 1,
                    "xt" => 2,
                    "zc" => 3,
                    _ => unreachable!(),
                };
                cycles[i] = cycles[i].or(Some(t + 1));
            }
            match g.get(node) {
                Some((node_type, connections)) => {
                    let pulse = match node_type {
                        b'b' => false,
                        b'%' => {
                            if high {
                                continue;
                            }
                            let on = state.contains(node);
                            if on {
                                state.remove(node);
                            } else {
                                state.insert(node);
                            }
                            !on
                        }
                        b'&' => {
                            conjunctions.get_mut(node).unwrap().insert(prev, high);
                            !conjunctions[node].values().all(|&b| b)
                        }
                        _ => unreachable!("Invalid node type"),
                    };
                    q.extend(connections.iter().map(|&n| (n, node, pulse)));
                }
                None => continue,
            }
        }
        if cycles.iter().all(|o| o.is_some()) {
            break;
        }
    }
    cycles.iter().fold(1, |a, o| a.lcm(&o.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    const INPUT: &str = include_str!("../input/2023/day20.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11687500);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(INPUT), 980457412);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(INPUT), 232774988886497);
    }
}

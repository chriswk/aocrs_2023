use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

type Graph = HashMap<u16, HashSet<u16>>;

#[aoc_generator(day25)]
fn parse(input: &str) -> Graph {
    let mut node_ids = HashMap::new();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (start, ends) = line.split_once(": ").unwrap();
        let ends: Vec<_> = ends.split(' ').collect();
        for &end in ends.iter() {
            let id = node_ids.len() as u16;
            node_ids.entry(end).or_insert(id);
        }
        let id = node_ids.len() as u16;
        node_ids.entry(start).or_insert(id);
        let start = node_ids[&start];
        let ends = ends.iter().map(|&e| node_ids[&e]);
        for end in ends.clone() {
            nodes.entry(end).or_insert_with(HashSet::new).insert(start);
        }
        nodes.entry(start).or_insert_with(HashSet::new).extend(ends);
    }
    nodes
}
fn find_path(graph: &Graph, start: u16, end: u16) -> Option<Vec<u16>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parents = HashMap::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        if node == end {
            break;
        }
        for &neighbour in graph[&node].iter() {
            if !visited.contains(&neighbour) {
                parents.insert(neighbour, node);
                queue.push_back(neighbour);
            }
        }
    }
    let mut path = Vec::new();
    let mut node = end;
    while node != start {
        path.push(node);
        if parents.contains_key(&node) {
            node = parents[&node];
        } else {
            return None;
        }
    }
    path.push(start);
    path.reverse();
    Some(path)
}

fn connected_count(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(*graph.keys().next().unwrap());
    while let Some(node) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }
        for &neighbour in graph[&node].iter() {
            queue.push(neighbour);
        }
    }
    visited.len()
}
#[aoc(day25, part1)]
fn part1(input: &Graph) -> usize {
    let mut components = input.clone();
    for i in 1..input.len() {
        let paths = (0..3)
            .map(|_| {
                let path = find_path(&components, 0, i as u16).unwrap();
                path.windows(2).for_each(|e| {
                    components.get_mut(&e[0]).unwrap().remove(&e[1]);
                    components.get_mut(&e[1]).unwrap().remove(&e[0]);
                });
                path
            })
            .collect::<Vec<_>>();
        match find_path(&components, 0, i as u16) {
            Some(_) => (),
            None => {
                let size1 = connected_count(&components);
                let size2 = components.len() - size1;
                return size1 * size2;
            }
        }
        paths.into_iter().for_each(|path| {
            path.windows(2).for_each(|e| {
                components.get_mut(&e[0]).unwrap().insert(e[1]);
                components.get_mut(&e[1]).unwrap().insert(e[0]);
            })
        })
    }
    0
}

#[aoc(day25, part2)]
fn part2(input: &Graph) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    const INPUT: &str = include_str!("../input/2023/day25.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 54);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(INPUT)), 527790);
    }
}

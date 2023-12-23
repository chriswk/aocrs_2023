use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::{Direction, Point};

pub type HikingMap = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[aoc_generator(day23)]
fn parse(input: &str) -> HikingMap {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::Slope(Direction::North),
                    'v' => Tile::Slope(Direction::South),
                    '<' => Tile::Slope(Direction::West),
                    '>' => Tile::Slope(Direction::East),
                    _ => unreachable!("Invalid tile: {}", tile),
                })
                .collect()
        })
        .collect()
}
fn starting_point(hiking_map: &[Vec<Tile>]) -> Option<Point> {
    for (x, point) in hiking_map[0].iter().enumerate() {
        match point {
            Tile::Path => {
                return Some(Point {
                    x: x as isize,
                    y: 0,
                })
            }
            _ => continue,
        }
    }
    None
}

fn finishing_point(hiking_map: &[Vec<Tile>]) -> Option<Point> {
    for (x, point) in hiking_map[hiking_map.len() - 1].iter().enumerate() {
        match point {
            Tile::Path => {
                return Some(Point {
                    x: x as isize,
                    y: (hiking_map.len() - 1) as isize,
                })
            }
            _ => continue,
        }
    }
    None
}
fn bottom_right(hiking_map: &[Vec<Tile>]) -> Point {
    Point {
        x: hiking_map[0].len() as isize - 1,
        y: hiking_map.len() as isize - 1,
    }
}

fn adjacent(point: Point, hiking_map: &[Vec<Tile>], ignore_slopes: bool) -> Vec<Point> {
    let bottom_right = bottom_right(hiking_map);
    enum_iterator::all::<Direction>()
        .filter_map(|dir| {
            let next = point.neighbour(dir);
            if next.x >= 0 && next.x <= bottom_right.x && next.y >= 0 && next.y <= bottom_right.y {
                match hiking_map[next.y as usize][next.x as usize] {
                    Tile::Path => Some(next),
                    Tile::Slope(d) => {
                        if ignore_slopes || d == dir {
                            Some(next)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn get_connected_points(
    point: Point,
    hiking_map: &[Vec<Tile>],
    ignore_slopes: bool,
) -> HashMap<Point, usize> {
    let mut connected_points = HashMap::new();
    if point == finishing_point(hiking_map).unwrap() {
        return connected_points;
    } else {
        for starting in adjacent(point, hiking_map, ignore_slopes) {
            let mut visited = HashSet::from([point]);
            let mut nodes = vec![(starting, 1)];
            while let Some((pos, dist)) = nodes.pop() {
                visited.insert(pos);
                let neighbours = adjacent(pos, hiking_map, ignore_slopes)
                    .into_iter()
                    .filter(|new_pos| !visited.contains(new_pos))
                    .collect::<Vec<_>>();
                if neighbours.len() == 1 {
                    nodes.push((*neighbours.first().unwrap(), dist + 1));
                } else {
                    let prev_dist = connected_points.entry(pos).or_default();
                    *prev_dist = usize::max(*prev_dist, dist);
                }
            }
        }
    }
    connected_points
}

fn topological_sort(
    hiking_map: &[Vec<Tile>],
    current_point: Point,
    sorted: &mut Vec<Point>,
    edges: &mut HashMap<Point, HashMap<Point, usize>>,
    visited: &mut HashSet<Point>,
) {
    let connected = edges
        .entry(current_point)
        .or_insert(get_connected_points(current_point, hiking_map, false))
        .clone();
    for (point, _) in connected {
        if !visited.contains(&point) {
            topological_sort(hiking_map, point, sorted, edges, visited);
        }
    }
    visited.insert(current_point);
    sorted.push(current_point);
}

fn calculate_edges(
    hiking_map: &[Vec<Tile>],
    ignore_slopes: bool,
) -> HashMap<Point, HashMap<Point, usize>> {
    let start = starting_point(hiking_map).unwrap();
    let mut vertices = vec![start];
    let mut edges = HashMap::new();
    while let Some(p) = vertices.pop() {
        let connected = get_connected_points(p, hiking_map, ignore_slopes);
        vertices.extend(
            connected
                .keys()
                .filter(|connected_point| !edges.contains_key(*connected_point)),
        );
        edges.insert(p, connected);
    }
    edges
}

fn longest_path_with_slopes(hiking_map: &[Vec<Tile>]) -> usize {
    let start = starting_point(hiking_map).unwrap();
    let mut distances_from_start = HashMap::from([(start, 0)]);
    let mut sorted_points = Vec::new();
    let mut edges = HashMap::new();
    let mut visited = HashSet::from([start]);

    topological_sort(
        hiking_map,
        start,
        &mut sorted_points,
        &mut edges,
        &mut visited,
    );
    for point in sorted_points.into_iter().rev() {
        let distance_to_point = *distances_from_start.entry(point).or_default();
        for (connected, dist) in edges.get(&point).unwrap() {
            let prev_dist = distances_from_start.entry(*connected).or_default();
            *prev_dist = usize::max(*prev_dist, distance_to_point + *dist);
        }
    }
    *distances_from_start
        .get(&finishing_point(hiking_map).unwrap())
        .unwrap()
}

fn check_all_paths(
    current_point: Point,
    finishing_point: Point,
    path: &mut Vec<Point>,
    path_len: usize,
    edges: &HashMap<Point, HashMap<Point, usize>>,
    max_path_len: &mut usize,
) {
    if path.contains(&current_point) {
        return;
    }

    path.push(current_point);

    if current_point == finishing_point {
        *max_path_len = usize::max(*max_path_len, path_len);
        path.pop();
        return;
    }

    for (connected_vertex, dist) in edges.get(&current_point).unwrap().clone() {
        check_all_paths(
            connected_vertex,
            finishing_point,
            path,
            path_len + dist,
            edges,
            max_path_len,
        );
    }
    path.pop();
}

fn longest_path_without_slopes(hiking_map: &[Vec<Tile>]) -> usize {
    let start = starting_point(hiking_map).unwrap();
    let end = finishing_point(hiking_map).unwrap();
    let edges = calculate_edges(hiking_map, true);
    let mut path = Vec::new();
    let mut max_path_len = 0;
    check_all_paths(start, end, &mut path, 0, &edges, &mut max_path_len);
    max_path_len
}

#[aoc(day23, part1)]
fn part1(input: &HikingMap) -> usize {
    longest_path_with_slopes(input)
}

#[aoc(day23, part2)]
fn part2(input: &HikingMap) -> usize {
    longest_path_without_slopes(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    const INPUT: &str = include_str!("../input/2023/day23.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 94);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(INPUT)), 2362);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 154);
    }
}

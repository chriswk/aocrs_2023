use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
};

use aoc_runner_derive::{aoc, aoc_generator};
type Point = (isize, isize);
type Field = HashMap<Point, char>;
type Beam = (Point, Point);

pub struct Grid {
    field: Field,
    width: usize,
    height: usize,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid {
    Grid {
        field: input
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c))
            })
            .collect::<Field>(),
        height: input.lines().count(),
        width: input.lines().next().unwrap().len(),
    }
}

const EAST: Point = (1, 0);
const WEST: Point = (-1, 0);
const NORTH: Point = (0, -1);
const SOUTH: Point = (0, 1);
const ORIGIN: Point = (0, 0);

fn propagate(beam: &Beam, field: &Field, explore: &mut VecDeque<Beam>) {
    match field.get(&beam.0) {
        Some('/') => match beam.1 {
            NORTH => explore.push_back(dt((beam.0, EAST))),
            EAST => explore.push_back(dt((beam.0, NORTH))),
            SOUTH => explore.push_back(dt((beam.0, WEST))),
            _ => explore.push_back(dt((beam.0, SOUTH))),
        },
        Some('\\') => match beam.1 {
            NORTH => explore.push_back(dt((beam.0, WEST))),
            EAST => explore.push_back(dt((beam.0, SOUTH))),
            SOUTH => explore.push_back(dt((beam.0, EAST))),
            _ => explore.push_back(dt((beam.0, NORTH))),
        },
        Some('|') => match beam.1 {
            EAST | WEST => {
                explore.push_back(dt((beam.0, NORTH)));
                explore.push_back(dt((beam.0, SOUTH)));
            }
            _ => explore.push_back(dt(*beam)),
        },
        Some('-') => match beam.1 {
            NORTH | SOUTH => {
                explore.push_back(dt((beam.0, EAST)));
                explore.push_back(dt((beam.0, WEST)));
            }
            _ => explore.push_back(dt(*beam)),
        },
        Some('.') => explore.push_back(dt(*beam)),
        _ => {}
    }
}

fn energize(field: &Field, start: Beam) -> HashSet<Point> {
    let mut beams = HashSet::new();
    let mut explore = VecDeque::from([start]);
    while !explore.is_empty() {
        let beam = explore.pop_front().unwrap();
        if beams.contains(&beam) || !field.contains_key(&beam.0) {
            continue;
        }
        beams.insert(beam);
        propagate(&beam, field, &mut explore);
    }
    beams.iter().map(|(p, _)| *p).collect::<HashSet<_>>()
}

fn max_energized(grid: &Grid) -> usize {
    let mut m = 0;
    for y in 0..grid.height {
        m = max(m, energize(&grid.field, ((0, y as isize), EAST)).len());
        m = max(
            m,
            energize(&grid.field, ((grid.width as isize - 1, y as isize), WEST)).len(),
        );
    }
    for x in 0..grid.width {
        m = max(m, energize(&grid.field, ((x as isize, 0), SOUTH)).len());
        m = max(
            m,
            energize(&grid.field, ((x as isize, grid.height as isize - 1), NORTH)).len(),
        );
    }
    m
}

fn dt(((x, y), (dx, dy)): Beam) -> Beam {
    ((x + dx, y + dy), (dx, dy))
}
#[aoc(day16, part1)]
fn part1(input: &Grid) -> usize {
    let field = input.field.clone();
    let tiles = energize(&field, (ORIGIN, EAST));
    tiles.len()
}

#[aoc(day16, part2)]
fn part2(input: &Grid) -> usize {
    max_energized(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 46);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(include_str!("../input/2023/day16.txt"))), 7927);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 51);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(include_str!("../input/2023/day16.txt"))), 8246);
    }
}

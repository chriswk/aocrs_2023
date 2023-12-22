use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

pub type Grid = Vec<Vec<char>>;
pub type Point = (usize, usize);

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid {
    let mut grid: Grid = input
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| ".".to_owned() + l + ".")
        .map(|l| l.chars().collect())
        .collect();
    grid.insert(0, std::iter::repeat('.').take(grid[0].len()).collect());
    grid.push(std::iter::repeat('.').take(grid[0].len()).collect());
    grid
}

fn find_starting_pos(grid: &Grid) -> Point {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No starting position found");
}

fn connections(grid: &Grid, point: Point) -> Option<(Point, Point)> {
    if point.1 >= grid.len() || point.0 >= grid[0].len() {
        return None;
    }
    let item = grid[point.1][point.0];
    match item {
        '|' => Some(((point.0, point.1.wrapping_sub(1)), (point.0, point.1 + 1))),
        '-' => Some(((point.0.wrapping_sub(1), point.1), (point.0 + 1, point.1))),
        'L' => Some(((point.0, point.1.wrapping_sub(1)), (point.0 + 1, point.1))),
        'J' => Some((
            (point.0.wrapping_sub(1), point.1),
            (point.0, point.1.wrapping_sub(1)),
        )),
        '7' => Some(((point.0.wrapping_sub(1), point.1), (point.0, point.1 + 1))),
        'F' => Some(((point.0, point.1 + 1), (point.0 + 1, point.1))),
        _ => None,
    }
}

fn cardinal_neighbours(point: Point) -> Vec<Point> {
    vec![
        (point.0.wrapping_sub(1), point.1),
        (point.0 + 1, point.1),
        (point.0, point.1.wrapping_sub(1)),
        (point.0, point.1 + 1),
    ]
}

fn find_pipes(grid: &Grid) -> Vec<Point> {
    let s = find_starting_pos(grid);
    let mut curr = s;
    let neighbours = cardinal_neighbours(curr);
    for n in neighbours {
        if n == s {
            continue;
        }
        let connections = connections(grid, n);
        match connections {
            Some((a, b)) => {
                if a == curr || b == curr {
                    curr = n;
                    break;
                }
            }
            None => {}
        }
    }
    let mut pipes = vec![s];
    while grid[curr.1][curr.0] != 'S' {
        let (c1, c2) = connections(grid, curr).unwrap();
        let next = if c1 == *pipes.last().unwrap() { c2 } else { c1 };
        pipes.push(curr);
        curr = next;
    }
    pipes
}

fn count_symbol_occurrence(grid: &Grid, symbol: char) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|c| **c == symbol).count())
        .sum()
}

fn search_and_mark(grid: Grid, curr: Point, pipes: &HashSet<Point>) -> Grid {
    if curr.1 >= grid.len() || curr.0 >= grid[0].len() {
        return grid;
    }
    if grid[curr.1][curr.0] == 'X' {
        return grid;
    }
    if pipes.contains(&curr) {
        return grid;
    }
    let neighbours = cardinal_neighbours(curr);
    let mut g = grid;
    g[curr.1][curr.0] = 'X';
    for n in neighbours {
        g = search_and_mark(g, n, pipes)
    }
    g
}

fn mark_grid(grid: Grid) -> (Grid, HashSet<Point>) {
    let starting_point = find_starting_pos(&grid);
    let pipes = find_pipes(&grid);
    let pipe_set: HashSet<_> = pipes.iter().cloned().collect();
    let mut marked_grid = grid;
    let mut previous = (starting_point.0 as i64, starting_point.1 as i64);
    let mut points_to_mark = vec![];
    for segment in pipes {
        let curr = (segment.0 as i64, segment.1 as i64);
        match (curr.0 - previous.0, curr.1 - previous.1) {
            (1, 0) => {
                points_to_mark.push((segment.0, segment.1 + 1));
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1 + 1));
            }
            (0, 1) => {
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1.wrapping_sub(1)));
                points_to_mark.push((segment.0.wrapping_sub(1), segment.1));
            }
            (-1, 0) => {
                points_to_mark.push((segment.0, segment.1.wrapping_sub(1)));
                points_to_mark.push((segment.0 + 1, segment.1.wrapping_sub(1)));
            }
            (0, -1) => {
                points_to_mark.push((segment.0 + 1, segment.1));
                points_to_mark.push((segment.0 + 1, segment.1 + 1));
            }
            _ => {}
        }
        previous = curr;
    }
    for p in points_to_mark {
        marked_grid = search_and_mark(marked_grid, p, &pipe_set);
    }
    (marked_grid, pipe_set)
}

#[aoc(day10, part1)]
fn part1(input: &Grid) -> usize {
    let pipes = find_pipes(input);
    pipes.len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &Grid) -> usize {
    let (marked_grid, pipe_set) = mark_grid(input.clone());
    let num_marked = count_symbol_occurrence(&marked_grid, 'X');
    if marked_grid[0][0] == 'X' {
        let total = marked_grid.len() * marked_grid[0].len();
        total - num_marked - pipe_set.len()
    } else {
        num_marked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TEST: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    const PART2_EXAMPLE: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SMALL_TEST)), 8);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(include_str!("../input/2023/day10.txt"))), 6717);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(PART2_EXAMPLE)), 10);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(include_str!("../input/2023/day10.txt"))), 381);
    }
}

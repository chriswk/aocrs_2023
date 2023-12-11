use aoc_runner_derive::{aoc, aoc_generator};
pub type Point = (usize, usize);
#[aoc_generator(day11, part1)]
fn parse(input: &str) -> (Vec<Point>, Vec<usize>) {
    let mut coords = Vec::new();
    let mut empty_cols = Vec::from_iter(0..input.lines().next().unwrap().len());
    let mut row_offset = 0;
    for (i, line) in input.lines().enumerate() {
        let mut empty = true;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                coords.push((i + row_offset, j));
                empty_cols.retain(|&x| x != j);
                empty = false;
            }
        }
        row_offset += empty as usize;
    }
    (coords, empty_cols)
}

#[aoc_generator(day11, part2)]
fn parse_with_ancient_expansion(input: &str) -> (Vec<Point>, Vec<usize>) {
    let mut coords = Vec::new();
    let mut empty_cols = Vec::from_iter(0..input.lines().next().unwrap().len());
    let mut row_offset = 0;
    let expansion: usize = 1_000_000 - 1;
    for (i, line) in input.lines().enumerate() {
        let mut empty = true;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                coords.push((i + row_offset, j));
                empty_cols.retain(|&x| x != j);
                empty = false;
            }
        }
        row_offset += empty as usize * expansion;
    }
    (coords, empty_cols)
}

fn expand(mut coords: Vec<Point>, empty_cols: Vec<usize>, expansion: usize) -> Vec<Point> {
    for empty_col in empty_cols.iter().rev() {
        coords.iter_mut().for_each(|(_, j)| {
            if *j > *empty_col {
                *j += expansion;
            }
        });
    }
    coords
}

fn shortest_distance_sum(coords: Vec<Point>) -> usize {
    let mut n = 0;
    (0..coords.len()).for_each(|left| {
        (left + 1..coords.len()).for_each(|right| {
            let left = coords[left];
            let right = coords[right];

            let max_i = left.0.max(right.0);
            let min_i = left.0.min(right.0);
            let max_j = left.1.max(right.1);
            let min_j = left.1.min(right.1);
            n += max_i - min_i + max_j - min_j
        })
    });
    n
}
#[aoc(day11, part1)]
fn part1(input: &(Vec<Point>, Vec<usize>)) -> usize {
    let coords = expand(input.0.clone(), input.1.clone(), 1);
    shortest_distance_sum(coords)
}

#[aoc(day11, part2)]
fn part2(input: &(Vec<Point>, Vec<usize>)) -> usize {
    let coords = expand(input.0.clone(), input.1.clone(), 999_999);
    shortest_distance_sum(coords)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 374);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day11.txt"))),
            10313550
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_with_ancient_expansion(EXAMPLE)), 82000210);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse_with_ancient_expansion(include_str!(
                "../input/2023/day11.txt"
            ))),
            611998089572
        );
    }
}

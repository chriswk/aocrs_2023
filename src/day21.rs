use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::{Direction, Point};

pub type Grid = Vec<Vec<char>>;

#[aoc_generator(day21)]
fn parse(input: &str) -> (Grid, Point) {
    let grid: Grid = input
        .lines()
        .enumerate()
        .map(move |(_y, line)| {
            line.chars()
                .enumerate()
                .map(move |(_x, c)| c)
                .collect::<Vec<_>>()
        })
        .collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    (grid, start)
}

fn passable(grid: &Grid, p: Point) -> bool {
    grid[p.y as usize][p.x as usize] != '#'
}

fn grid_contains(grid: &Grid, p: Point) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < grid[0].len() as isize && p.y < grid.len() as isize
}

fn visit(grid: &Grid, start: Point, max_steps: usize) -> usize {
    let _max_x = grid[0].len();
    let _max_y = grid.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::<Point>::new();
    visited.insert(start);
    queue.push_back((start, 0));
    let mut reached_count = 0;
    let mut cur_count = 0;
    while let Some((pos, steps)) = queue.pop_front() {
        if steps == max_steps {
            reached_count += 1;
        }

        if steps != cur_count {
            cur_count = steps;
            visited.clear();
        }

        if steps > max_steps {
            continue;
        }
        for d in enum_iterator::all::<Direction>() {
            let neighbour = pos.neighbour(d);
            if grid_contains(grid, neighbour)
                && !visited.contains(&neighbour)
                && passable(grid, neighbour)
            {
                visited.insert(neighbour);
                queue.push_back((neighbour, steps + 1))
            }
        }
    }

    reached_count
}

fn pos_mod<T>(a: T, b: T) -> T
where
    T: num::Integer + num::Signed + Copy,
{
    let c = a % b;
    if c < num::zero() {
        c + b
    } else {
        c
    }
}
fn predict_perimeter(
    cur_count: usize,
    past_perims: &[usize],
    past_perims_offsets: &[usize],
) -> usize {
    let term2 = past_perims_offsets[cur_count % 131];

    ((cur_count / 131) - 1) * (past_perims[131] + term2) + past_perims[cur_count % 131 + 131]
}
fn visit_infinite(grid: &Grid, start: Point, max_steps: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::<Point>::new();
    let mut reached_cache = HashMap::<usize, usize>::new();
    let mut perim_cache = HashMap::<usize, usize>::new();
    let mut past_perims = Vec::new();
    let mut past_perims_offset = Vec::<usize>::new();
    visited.insert(start);
    queue.push_back((start, 0));
    let mut gardens = 0;
    let mut cur_count = 0;
    while let Some((pos, count)) = queue.pop_front() {
        if count != cur_count {
            let mut interior_gardens = 0;
            if cur_count >= 2 {
                interior_gardens = *reached_cache.get(&(cur_count - 2)).unwrap();
            }
            let total_gardens = gardens + interior_gardens;
            if cur_count < 262 {
                past_perims.push(gardens);
            } else if cur_count < 393 {
                let predicted_perimeter =
                    ((cur_count / 131) - 1) * past_perims[131] + past_perims[cur_count % 131 + 131];
                let offset = gardens - predicted_perimeter;
                past_perims_offset.push(offset);
            } else {
                let term2 = past_perims_offset[cur_count % 131];
                let predicted_perim = ((cur_count / 131) - 1) * (past_perims[131] + term2)
                    + past_perims[cur_count % 131 + 131];
                let _offset = gardens as isize - predicted_perim as isize;
                break;
            }
            reached_cache.insert(cur_count, total_gardens);
            perim_cache.insert(cur_count, gardens);
            gardens = 0;
            cur_count = count;
        }
        gardens += 1;
        if count >= max_steps {
            continue;
        }
        for d in enum_iterator::all::<Direction>() {
            let neighbour = pos.neighbour(d);
            let mut neighbour_wrap = neighbour;
            neighbour_wrap.x = pos_mod(neighbour.x, grid[0].len() as isize);
            neighbour_wrap.y = pos_mod(neighbour.y, grid.len() as isize);
            if grid_contains(grid, neighbour_wrap)
                && !visited.contains(&neighbour)
                && passable(grid, neighbour_wrap)
            {
                visited.insert(neighbour);
                queue.push_back((neighbour, count + 1))
            }
        }
    }
    let interior_gardens = *reached_cache.get(&(cur_count - 2)).unwrap();
    let total_gardens = gardens + interior_gardens;
    println!(
        "After {cur_count} steps, perim {gardens} + inter {interior_gardens} = {total_gardens}"
    );
    let mut prev2 = *reached_cache.get(&(cur_count - 3)).unwrap();
    let mut prev = *reached_cache.get(&(cur_count - 2)).unwrap()
        + predict_perimeter(cur_count, &past_perims, &past_perims_offset);
    loop {
        let np2 = prev2 + predict_perimeter(cur_count + 1, &past_perims, &past_perims_offset);
        let np = prev + predict_perimeter(cur_count + 2, &past_perims, &past_perims_offset);
        prev2 = np2;
        prev = np;
        cur_count += 2;
        if cur_count >= max_steps {
            println!("{} - {prev2}, {} - {prev}", cur_count - 1, cur_count);
            return prev;
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &(Grid, Point)) -> usize {
    visit(&input.0, input.1, 64)
}
#[aoc(day21, part2)]
fn part2(input: &(Grid, Point)) -> usize {
    visit_infinite(&input.0, input.1, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    const INPUT: &str = include_str!("../input/2023/day21.txt");
    #[test]
    fn part1_small_example() {
        let (grid, start) = parse(EXAMPLE);
        assert_eq!(visit(&grid, start, 6), 16)
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(INPUT)), 3594)
    }
}

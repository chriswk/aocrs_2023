use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
pub type Board = Vec<Vec<u8>>;

#[aoc_generator(day14)]
fn parse(input: &str) -> Board {
    input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn tilt_north(board: &mut Board) {
    let mut done = false;
    while !done {
        done = true;
        for y in 0..board.len() - 1 {
            for x in 0..board[0].len() {
                if board[y + 1][x] == b'O' && board[y][x] == b'.' {
                    board[y][x] = b'O';
                    board[y + 1][x] = b'.';
                    done = false;
                }
            }
        }
    }
}

fn rotate(board: &Board) -> Board {
    let mut new_board = vec![vec![0; board.len()]; board[0].len()];
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            new_board[c][board.len() - 1 - r] = board[r][c];
        }
    }
    new_board
}

fn load(board: &Board) -> usize {
    let len = board.len();
    board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|&c| if c == b'O' { len - y } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day14, part1)]
fn part1(input: &Board) -> usize {
    let mut mine = input.clone();
    tilt_north(&mut mine);
    assert_ne!(&mine, input);
    load(&mine)
}

#[aoc(day14, part2)]
fn part2(input: &Board) -> usize {
    let mut mine = input.clone();
    let mut seen = HashMap::new();
    for i in 1..1_000_000_000 {
        for _ in 0..4 {
            tilt_north(&mut mine);
            mine = rotate(&mine);
        }
        if let Some(seen_at) = seen.insert(mine.clone(), i) {
            if (1_000_000_000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }
    load(&mine)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 136);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day14.txt"))),
            109385
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 64);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day14.txt"))),
            93102
        );
    }
}

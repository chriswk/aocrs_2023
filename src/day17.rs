use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use std::collections::{BTreeSet, HashMap, HashSet};

type Game = (HashMap<IVec2, u32>, IVec2);

#[aoc_generator(day17)]
fn parse(input: &str) -> Game {
    let y = input.lines().count() as i32;
    let x = input.lines().next().unwrap().len() as i32;
    let board = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, c)| {
                (
                    IVec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    c.to_digit(10).unwrap(),
                )
            })
        })
        .collect::<HashMap<IVec2, u32>>();
    (board, IVec2 { x, y })
}

fn find_path(game: &Game, part2: bool) -> u32 {
    let start = IVec2::ZERO;
    let pos_end = game.1 - IVec2::ONE;
    let mut candidates: BTreeSet<(u32, [i32; 2], [i32; 2])> = BTreeSet::new();
    let mut seen: HashSet<(IVec2, IVec2)> = HashSet::new();
    candidates.insert((0, start.to_array(), [0, 0]));
    while let Some((heat_loss, pos, dir)) = candidates.pop_first() {
        let pos: IVec2 = pos.into();
        let dir: IVec2 = dir.into();
        if pos == pos_end {
            return heat_loss;
        }
        if !seen.insert((pos, dir)) {
            continue;
        }
        let possibles = match dir {
            IVec2::X | IVec2::NEG_X => [IVec2::Y, IVec2::NEG_Y],
            IVec2::Y | IVec2::NEG_Y => [IVec2::X, IVec2::NEG_X],
            IVec2::ZERO => [IVec2::X, IVec2::Y],
            _ => unreachable!("Invalid direction"),
        };
        for dir_new in possibles {
            let mut hl = heat_loss;
            let mut new_pos = pos;
            if part2 {
                const MIN_STEP: i32 = 4;
                const MAX_STEP: i32 = 10;
                for step in 1..=MAX_STEP {
                    new_pos += dir_new;
                    if let Some(tile) = game.0.get(&new_pos) {
                        hl += tile;
                        if step >= MIN_STEP {
                            candidates.insert((hl, new_pos.to_array(), dir_new.to_array()));
                        }
                    } else {
                        break;
                    }
                }
            } else {
                for _ in 0..3 {
                    new_pos += dir_new;
                    if let Some(tile) = game.0.get(&new_pos) {
                        hl += tile;
                        candidates.insert((hl, new_pos.to_array(), dir_new.to_array()));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    unreachable!("No path found");
}

#[aoc(day17, part1)]
fn part1(input: &Game) -> u32 {
    find_path(input, false)
}

#[aoc(day17, part2)]
fn part2(input: &Game) -> u32 {
    find_path(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 102);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(include_str!("../input/2023/day17.txt"))), 967);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 94);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(include_str!("../input/2023/day17.txt"))), 1101);
    }
}

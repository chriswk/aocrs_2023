use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub type Brick = (usize, usize, usize, usize, usize, usize);
pub type Bricks = Vec<Brick>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Bricks {
    let mut bricks: Bricks = input
        .lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) = l
                .split(|c: char| !c.is_ascii_digit())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (x1, y1, z1, x2, y2, z2)
        })
        .collect::<Vec<_>>();
    bricks.sort_by_key(|b| b.2);
    bricks
}

fn disintegrate(
    adjacent: &[(HashSet<usize>, HashSet<usize>)],
    falling: &mut HashSet<usize>,
    brick: usize,
) {
    falling.insert(brick);
    for &above in &adjacent[brick].0 {
        if adjacent[above].1.iter().all(|x| falling.contains(x)) {
            disintegrate(adjacent, falling, above)
        }
    }
}

fn drop_all(mut bricks: Bricks, part2: bool) -> usize {
    let mut adjacent = vec![(HashSet::new(), HashSet::new()); bricks.len()];
    let mut space = HashMap::new();
    for i in 0..bricks.len() {
        let (x1, y1, mut z1, x2, y2, mut z2) = bricks[i];
        while z1 > 1
            && (x1..=x2)
                .cartesian_product(y1..=y2)
                .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
        {
            z2 -= 1;
            z1 -= 1;
        }
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            for z in z1..=z2 {
                space.insert((x, y, z), i);
            }
            if let Some(&j) = space.get(&(x, y, z1 - 1)) {
                adjacent[j].0.insert(i);
                adjacent[i].1.insert(j);
            }
        }
        bricks[i] = (x1, y1, z1, x2, y2, z2);
    }
    let mut falling = HashSet::new();
    let mut ans = 0;
    for b in 0..bricks.len() {
        falling.clear();
        disintegrate(&adjacent, &mut falling, b);
        if part2 {
            ans += falling.len() - 1
        } else {
            ans += (falling.len() == 1) as usize;
        }
    }
    ans
}
#[aoc(day22, part1)]
fn part1(input: &Bricks) -> usize {
    drop_all(input.clone(), false)
}

#[aoc(day22, part2)]
fn part2(input: &Bricks) -> usize {
    drop_all(input.clone(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    const INPUT: &str = include_str!("../input/2023/day22.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(INPUT)), 522);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(INPUT)), 83519);
    }
}

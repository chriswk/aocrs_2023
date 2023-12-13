use aoc_runner_derive::{aoc, aoc_generator};

pub type Grid = (Vec<u64>, Vec<u64>);

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<_> = block.lines().collect();
            let rows: Vec<u64> = lines
                .iter()
                .map(|line| {
                    line.chars().fold(0u64, |mut row, c| {
                        row <<= 1;
                        row |= (c == '#') as u64;
                        row
                    })
                })
                .collect();
            let mut cols = vec![0u64; lines[0].len()];
            for &row in rows.iter() {
                for (j, col) in cols.iter_mut().enumerate() {
                    *col <<= 1;
                    *col |= (row >> (lines[0].len() - j - 1)) & 1;
                }
            }
            (rows, cols)
        })
        .collect()
}

fn find_reflection(values: &[u64]) -> Option<usize> {
    'OUTER: for (i, pair) in values.iter().zip(values.iter().skip(1)).enumerate() {
        if pair.0 != pair.1 {
            continue;
        }
        let mut k = 1;
        while i as i32 - k >= 0 && i as i32 + k + 1 < values.len() as i32 {
            if values[i - k as usize] != values[i + k as usize + 1] {
                continue 'OUTER;
            }
            k += 1;
        }
        return Some(i);
    }
    None
}

fn smudged_reflection(values: &[u64]) -> Option<usize> {
    'OUTER: for (i, pair) in values.iter().zip(values.iter().skip(1)).enumerate() {
        let mut smudged = false;
        if pair.0 != pair.1 && (pair.0 ^ pair.1).count_ones() != 1 {
            continue;
        } else if pair.0 != pair.1 {
            smudged = true;
        }
        let mut k = 1;
        while i as i32 - k >= 0 && i as i32 + k + 1 < values.len() as i32 {
            let left = values[i - k as usize];
            let right = values[i + k as usize + 1];
            if left != right && (left ^ right).count_ones() != 1 {
                continue 'OUTER;
            } else if left != right {
                if smudged {
                    continue 'OUTER;
                }
                smudged = true;
            }
            k += 1;
        }
        if smudged {
            return Some(i);
        }
    }
    None
}

#[aoc(day13, part1)]
fn part1(input: &[Grid]) -> usize {
    input
        .iter()
        .map(|(rows, cols)| {
            if let Some(row_reflection) = find_reflection(rows) {
                100 * (row_reflection + 1)
            } else if let Some(col_reflection) = find_reflection(cols) {
                col_reflection + 1
            } else {
                unreachable!("No reflection found");
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Grid]) -> usize {
    input
        .iter()
        .map(|(rows, cols)| {
            if let Some(row_reflection) = smudged_reflection(rows) {
                100 * (row_reflection + 1)
            } else if let Some(col_reflection) = smudged_reflection(cols) {
                col_reflection + 1
            } else {
                unreachable!("No reflection found");
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

    const SECOND: &str = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(FIRST)), 5);
        assert_eq!(part1(&parse(SECOND)), 400);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day13.txt"))),
            39939
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(FIRST)), 300);
        assert_eq!(part2(&parse(SECOND)), 100);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day13.txt"))),
            32069
        );
    }
}

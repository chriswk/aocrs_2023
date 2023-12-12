use aoc_runner_derive::{aoc, aoc_generator};

pub type Springs = (String, Vec<usize>);
#[aoc_generator(day12)]
fn parse<'a>(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap();
            let counts = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (pattern.into(), counts)
        })
        .collect()
}

fn count_arrangements(pattern: &str, counts: &[usize]) -> usize {
    let line = pattern.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];
    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;
    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }
    dp[0][0][0]
}

#[aoc(day12, part1)]
fn part1(input: &[Springs]) -> usize {
    input
        .iter()
        .map(|(pattern, counts)| count_arrangements(&pattern, counts))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[Springs]) -> usize {
    input
        .iter()
        .map(|(p, c)| (p.as_str(), c))
        .map(|(pattern, counts)| {
            let pattern = [pattern; 5].join("?");
            let counts = counts.repeat(5);
            count_arrangements(&pattern, &counts)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(include_str!("../input/2023/day12.txt"))), 7204);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 525152);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day12.txt"))),
            1672318386674
        )
    }
}

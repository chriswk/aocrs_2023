use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
pub type Point = (usize, usize);

pub struct Gear {
    pub symbol: char,
    pub part_numbers: Vec<u32>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> HashMap<Point, Gear> {
    let mut gears: HashMap<Point, Gear> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol != '.' && !symbol.is_digit(10) {
                gears.entry((x, y)).or_insert(Gear {
                    symbol,
                    part_numbers: vec![],
                });
            }
        }
    }

    for (y, line) in input.lines().enumerate() {
        let mut number_digits = vec![];
        let mut digit_start_x = None;
        for (x, symb) in line.chars().enumerate() {
            if let Some(n) = symb.to_digit(10) {
                number_digits.push(n);
                if digit_start_x.is_none() {
                    digit_start_x = Some(x);
                }
            }
            if symb.to_digit(10).is_none() || x == line.chars().count() - 1 {
                if !number_digits.is_empty() {
                    let number = number_digits.iter().copied().fold(0, |acc, n| acc * 10 + n);
                    if let Some(xs) = digit_start_x {
                        for gx in xs.saturating_sub(1)..=x {
                            for yoff in -1..=1 {
                                let gy = y.saturating_add_signed(yoff);
                                if let Some(g) = gears.get_mut(&(gx, gy)) {
                                    g.part_numbers.push(number)
                                }
                            }
                        }
                    }
                }
                number_digits.clear();
                digit_start_x = None;
            }
        }
    }
    gears
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &HashMap<Point, Gear>) -> u32 {
    input
        .values()
        .into_iter()
        .map(|v| v.part_numbers.iter().sum::<u32>())
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &HashMap<Point, Gear>) -> u64 {
    input
        .values()
        .into_iter()
        .filter(|g| g.symbol == '*' && g.part_numbers.len() == 2)
        .map(|g| g.part_numbers.iter().fold(1, |acc, n| acc * (*n as u64)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_handle_test_input_part1() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .trim();
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 4361);
    }

    #[test]
    pub fn can_handle_test_input_part2() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .trim();
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 467835);
    }
}

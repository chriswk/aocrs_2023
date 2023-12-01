use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
pub fn input_generator_part_1(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator_part_2(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_test_example() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let input = input_generator_part_1(input);
        assert_eq!(solve_part1(&input), 142);
    }

    #[test]
    pub fn part_2_test_example() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        let input = input_generator_part_2(input);
        assert_eq!(solve_part2(&input), 281);
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;

pub type Move = (u8, i32);

#[aoc_generator(day18, part1)]
fn parse_part1(input: &str) -> Vec<Move> {
    input.lines().map(|line| {
        let instructions = line.split_ascii_whitespace().collect::<Vec<_>>();
        (instructions[0].as_bytes()[0], instructions[1].parse::<i32>().unwrap())
    }).collect()
}

#[aoc_generator(day18, part2)]
fn parse_part2(input: &str) -> Vec<Move> {
    input.lines().map(|line| {
        let instructions = line.split_ascii_whitespace().collect::<Vec<_>>();
        let hex = instructions[2];
        let direction = match hex.as_bytes()[7] {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => unreachable!("Invalid direction"),
        };
        let hex_code = &hex[2..hex.len() - 2];
        let amount = i32::from_str_radix(hex_code, 16).unwrap();
        (direction, amount)
    }).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Move]) -> i64 {
    lava(input)
}

#[aoc(day18, part2)]
fn part2(input: &[Move]) -> i64 {
    lava(input)
}

fn from_direction(direction: u8) -> IVec2 {
    match direction {
        b'U' => IVec2 { y: 1, x: 0 },
        b'D' => IVec2 { y: -1, x: 0 },
        b'L' => IVec2 { y: 0, x: -1 },
        b'R' => IVec2 { y: 0, x: 1 },
        _ => panic!("Invalid direction"),
    }
}

fn lava(moves: &[Move]) -> i64 {
    let mut previous;
    let mut position = IVec2::ZERO;
    let mut area: i64 = 0;
    let mut perimeter: i64 = 0;

    for &(direction, amount) in moves {
        previous = position;
        position += from_direction(direction) * amount;
        area += determinant(position, previous);
        perimeter += amount as i64;
    }
    (area / 2).abs() + (perimeter / 2+ 1) 
}

fn determinant(a: IVec2, b: IVec2) -> i64 {
    a.x as i64 * b.y as i64 - a.y as i64 * b.x as i64

}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    const INPUT: &str = include_str!("../input/2023/day18.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE)), 62);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse_part1(INPUT)), 33491);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE)), 952408144115);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse_part2(INPUT)), 87716969654406);
    }
}
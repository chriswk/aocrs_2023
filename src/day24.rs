use crate::point::Point3WithVel;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use z3::ast::{Ast, Int};

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Point3WithVel> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('@');
            let pos = parts
                .next()
                .unwrap()
                .trim()
                .split(',')
                .map(|f| f.trim().parse::<i64>().unwrap())
                .map(|f| f as f64)
                .collect_vec();
            let vel = parts
                .next()
                .unwrap()
                .trim()
                .split(',')
                .map(|f| f.trim().parse::<i64>().unwrap())
                .map(|f| f as f64)
                .collect_vec();
            Point3WithVel::from_pos_and_vel_vec(pos, vel)
        })
        .collect()
}

fn find_intersections(lines: &[Point3WithVel], range_from: f64, range_to: f64) -> usize {
    let range = range_from..=range_to;
    lines
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            if let Some(intersect) = a.intersection_x_y(**b) {
                if a.dx.signum() != (intersect.0 - a.x).signum()
                    || b.dx.signum() != (intersect.0 - b.x).signum()
                {
                    return false;
                }
                range.contains(&intersect.0) && range.contains(&intersect.1)
            } else {
                false
            }
        })
        .count()
}

fn throw_rock(hailstones: &[Point3WithVel]) -> i64 {
    let ctx = z3::Context::new(&z3::Config::default());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));
    let zero = Int::from_i64(&ctx, 0);
    for (i, &point) in hailstones.iter().enumerate() {
        let [x, y, z, dx, dy, dz] = [point.x, point.y, point.z, point.dx, point.dy, point.dz]
            .map(|v| Int::from_i64(&ctx, v as i64));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &[Point3WithVel]) -> usize {
    find_intersections(input, 200000000000000.0, 400000000000000.0)
}

#[aoc(day24, part2)]
fn part2(input: &[Point3WithVel]) -> i64 {
    throw_rock(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    const INPUT: &str = include_str!("../input/2023/day24.txt");
    #[test]
    fn finds_intersection() {
        let lines = parse(EXAMPLE);
        assert_eq!(find_intersections(&lines, 7.0, 27.0), 2);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(INPUT)), 20336);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input.split(',').map(|f| f.to_owned()).collect()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> usize {
    input.iter().map(|f| hash(f.as_str())).sum()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for step in input {
        if step.contains('-') {
            let label = step.replace('-', "");
            let n = hash(&label);
            let new_box = boxes[n]
                .iter()
                .filter(|(l, _)| l != &label)
                .cloned()
                .collect::<Vec<_>>();
            boxes[n] = new_box;
        } else {
            let label_and_focus = step.split('=').collect::<Vec<_>>();
            let label = label_and_focus[0];
            let n = hash(label);
            let focal_length = label_and_focus[1].parse::<usize>().unwrap();
            let existing = boxes[n].iter().any(|(l, _)| l == label);
            if existing {
                let new_box = boxes[n]
                    .iter()
                    .map(|pair| {
                        if pair.0 == label {
                            (label.to_string(), focal_length)
                        } else {
                            pair.clone()
                        }
                    })
                    .collect::<Vec<_>>();
                boxes[n] = new_box;
            } else {
                boxes[n].push((label.to_owned(), focal_length));
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .fold(0_usize, |total, (box_idx, lens_box)| {
            total
                + lens_box
                    .iter()
                    .enumerate()
                    .fold(0_usize, |acc, (slot, (_, focal_length))| {
                        acc + (box_idx + 1) * (slot + 1) * focal_length
                    })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn hasher_works_as_described() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1320);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day15.txt").trim())),
            511215
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 145);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day15.txt").trim())),
            236057
        );
    }
}

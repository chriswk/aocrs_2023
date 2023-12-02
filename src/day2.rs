use std::cmp::max;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Game {
    pub rounds: Vec<GameRound>,
    pub id: usize,
}

#[derive(Debug)]
pub struct GameRound {
    pub red: Option<u32>,
    pub blue: Option<u32>,
    pub green: Option<u32>,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut game = Game {
            rounds: Vec::new(),
            id: 0,
        };
        let game_and_sets = line.trim().split(':').collect::<Vec<_>>();
        let game_id = game_and_sets[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        game.id = game_id;
        let game_sets: Vec<_> = game_and_sets[1].split(';').collect();
        let game_sets = game_sets
            .iter()
            .map(|x| String::from(x.trim()))
            .collect::<Vec<_>>();
        for set in game_sets {
            let mut game_round = GameRound {
                red: None,
                blue: None,
                green: None,
            };
            let bags = set
                .split(',')
                .map(|x| x.trim().split(' ').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for bag in bags {
                match bag[1] {
                    "red" => {
                        game_round.red = bag[0].parse::<u32>().ok();
                    }
                    "green" => {
                        game_round.green = bag[0].parse::<u32>().ok();
                    }
                    "blue" => {
                        game_round.blue = bag[0].parse::<u32>().ok();
                    }
                    _ => {}
                }
            }
            game.rounds.push(game_round);
        }
        games.push(game);
    }
    games
}

fn keep_game(game: &Game, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    let mut keep = true;
    for round in &game.rounds {
        if round.red.unwrap_or(0) > max_red
            || round.green.unwrap_or(0) > max_green
            || round.blue.unwrap_or(0) > max_blue
        {
            keep = false;
            break;
        }
    }
    keep
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> usize {
    input
        .iter()
        .filter(|g| keep_game(g, 12, 13, 14))
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let (min_red, min_green, min_blue) =
                g.rounds
                    .iter()
                    .fold((0, 0, 0), |(red, green, blue), round| {
                        (
                            max(red, round.red.unwrap_or(0)),
                            max(green, round.green.unwrap_or(0)),
                            max(blue, round.blue.unwrap_or(0)),
                        )
                    });
            min_red * min_green * min_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn part_1_test_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games = generator(input);
        assert_eq!(games.len(), 5);
        assert_eq!(games.iter().filter(|g| keep_game(g, 12, 13, 14)).count(), 3);
    }

    #[test]
    pub fn part_2_test_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games = generator(input);
        assert_eq!(solve_part2(&games), 2286)
    }
}

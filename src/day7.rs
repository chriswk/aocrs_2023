use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum CamelCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<&char> for CamelCard {
    fn from(value: &char) -> Self {
        match value {
            '2' => CamelCard::Two,
            '3' => CamelCard::Three,
            '4' => CamelCard::Four,
            '5' => CamelCard::Five,
            '6' => CamelCard::Six,
            '7' => CamelCard::Seven,
            '8' => CamelCard::Eight,
            '9' => CamelCard::Nine,
            'T' => CamelCard::Ten,
            'J' => CamelCard::Jack,
            'Q' => CamelCard::Queen,
            'K' => CamelCard::King,
            'A' => CamelCard::Ace,
            _ => panic!("Invalid card value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum JokerCamelCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<&char> for JokerCamelCard {
    fn from(value: &char) -> Self {
        match value {
            '2' => JokerCamelCard::Two,
            '3' => JokerCamelCard::Three,
            '4' => JokerCamelCard::Four,
            '5' => JokerCamelCard::Five,
            '6' => JokerCamelCard::Six,
            '7' => JokerCamelCard::Seven,
            '8' => JokerCamelCard::Eight,
            '9' => JokerCamelCard::Nine,
            'T' => JokerCamelCard::Ten,
            'J' => JokerCamelCard::Joker,
            'Q' => JokerCamelCard::Queen,
            'K' => JokerCamelCard::King,
            'A' => JokerCamelCard::Ace,
            _ => panic!("Invalid card value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CamelHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hand<C>
where
    C: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash,
{
    cards: [C; 5],
}

impl<C> PartialOrd for Hand<C>
where
    C: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash,
    CamelHandType: From<Self>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for Hand<C>
where
    C: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash,
    CamelHandType: From<Self>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type: CamelHandType = (*self).into();
        let other_type: CamelHandType = (*other).into();
        (self_hand_type, self.cards).cmp(&(other_type, other.cards))
    }
}

impl From<Hand<CamelCard>> for CamelHandType {
    fn from(value: Hand<CamelCard>) -> Self {
        let mut map: HashMap<CamelCard, u8> = HashMap::with_capacity(5);
        for card in value.cards.iter().copied() {
            map.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }
        let highest = map.values().max().unwrap();
        match (map.len(), highest) {
            (5, 1) => CamelHandType::HighCard,
            (4, 2) => CamelHandType::OnePair,
            (3, 2) => CamelHandType::TwoPair,
            (3, 3) => CamelHandType::ThreeOfAKind,
            (2, 3) => CamelHandType::FullHouse,
            (2, 4) => CamelHandType::FourOfAKind,
            (1, 5) => CamelHandType::FiveOfAKind,
            _ => unreachable!("Invalid hand type"),
        }
    }
}

impl From<Hand<JokerCamelCard>> for CamelHandType {
    fn from(value: Hand<JokerCamelCard>) -> Self {
        let mut map: HashMap<JokerCamelCard, u8> = HashMap::with_capacity(5);
        for card in value.cards.iter().copied() {
            map.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }
        let highest_non_joker = map
            .iter()
            .filter_map(|(&k, &v)| match k {
                JokerCamelCard::Joker => None,
                _ => Some(v),
            })
            .max()
            .unwrap_or(0);
        let joker_count = map.get(&JokerCamelCard::Joker).copied().unwrap_or(0);
        let highest = highest_non_joker + joker_count;
        let mut len = map.len();
        if joker_count > 0 && highest_non_joker > 0 {
            len -= 1;
        }
        match (len, highest) {
            (5, 1) => CamelHandType::HighCard,
            (4, 2) => CamelHandType::OnePair,
            (3, 2) => CamelHandType::TwoPair,
            (3, 3) => CamelHandType::ThreeOfAKind,
            (2, 3) => CamelHandType::FullHouse,
            (2, 4) => CamelHandType::FourOfAKind,
            (1, 5) => CamelHandType::FiveOfAKind,
            _ => unreachable!("Invalid hand type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Play<C>
where
    C: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash,
    Hand<C>: PartialOrd + Ord,
{
    hand: Hand<C>,
    bid: u128,
}
#[aoc_generator(day7, part1)]
fn parse(input: &str) -> Vec<Play<CamelCard>> {
    input
        .lines()
        .map(|p| {
            let cards = p
                .split(' ').next()
                .unwrap()
                .chars()
                .map(|c| CamelCard::from(&c))
                .collect::<Vec<CamelCard>>();
            let hand = Hand {
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            };
            let bid = p.split(' ').nth(1).unwrap().parse::<u128>().unwrap();
            Play { hand, bid }
        })
        .collect()
}

#[aoc_generator(day7, part2)]
fn parse_with_jokers(input: &str) -> Vec<Play<JokerCamelCard>> {
    input
        .lines()
        .map(|p| {
            let cards = p
                .split(' ').next()
                .unwrap()
                .chars()
                .map(|c| JokerCamelCard::from(&c))
                .collect::<Vec<JokerCamelCard>>();
            let hand = Hand {
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            };
            let bid = p.split(' ').nth(1).unwrap().parse::<u128>().unwrap();
            Play { hand, bid }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Play<CamelCard>]) -> u128 {
    let mut plays = input.to_vec();
    plays.sort_unstable_by_key(|p| p.hand);
    plays
        .iter()
        .enumerate()
        .fold(0, |acc, (i, p)| acc + ((i as u128 + 1) * p.bid))
}
#[aoc(day7, part2)]
fn part2(input: &[Play<JokerCamelCard>]) -> u128 {
    let mut plays = input.to_vec();
    plays.sort_unstable_by_key(|p| p.hand);
    plays
        .iter()
        .enumerate()
        .fold(0, |acc, (i, p)| acc + ((i as u128 + 1) * p.bid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6440);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day7.txt"))),
            249483956
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_with_jokers(EXAMPLE)), 5905);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            part2(&parse_with_jokers(include_str!("../input/2023/day7.txt"))),
            252137472
        );
    }
}

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            c => panic!("{}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Joker => '🂿',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::T => 'T',
            Self::J => 'J',
            Self::Q => 'Q',
            Self::K => 'K',
            Self::A => 'A',
        }
    }
    fn to_joker(&self) -> Self {
        match self {
            Self::J => Self::Joker,
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(Card::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(Card::to_char).collect::<String>()
        )
    }
}

impl Hand {
    fn to_joker(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(Card::to_joker)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug)]
pub struct CamelCards(Vec<(Hand, usize)>);

impl FromStr for CamelCards {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| -> Result<_, ParseError> {
                    let (hand, bid) = line.split_once(' ').unwrap();
                    Ok((hand.parse()?, bid.parse()?))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<CamelCards> {
    input.parse()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn _from_hand_sort(hand: &Hand) -> Self {
        let mut cards = hand.0.to_owned();
        cards.sort_unstable();
        cards.reverse();
        match cards {
            [a, b, c, d, e] if a == b && b == c && c == d && d == e => Self::FiveKind,
            [a, b, c, d, e] if b == c && c == d && (a == b || d == e) => Self::FourKind,
            [a, b, c, d, e] if a == b && (b == c || c == d) && d == e => Self::FullHouse,
            [a, b, c, d, e] if (a == b || c == d) && (b == c) || (c == d && d == e) => {
                Self::ThreeKind
            },
            [a, b, c, d, e] if (a == b) && (c == d || d == e) || (b == c && d == e) => {
                Self::TwoPair
            },
            [a, b, c, d, e] if a == b || b == c || c == d || d == e => Self::OnePair,
            _ => Self::HighCard,
        }
    }
    fn from_hand_counter(hand: &Hand) -> Self {
        let mut counter = HashMap::<Card, usize>::new();
        for card in &hand.0 {
            counter
                .entry(card.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let count_joker = counter.remove(&Card::Joker).unwrap_or(0);
        let mut shape: Vec<(Card, usize)> = counter.into_iter().collect();
        if shape.len() <= 1 {
            return Self::FiveKind;
        }
        shape.select_nth_unstable_by(1, |(_a_card, a_count), (_b_card, b_count)| {
            b_count.cmp(a_count)
        });
        let top_cards = [shape[0].1 + count_joker, shape[1].1];
        match top_cards {
            [4, _] => Self::FourKind,
            [3, 2] => Self::FullHouse,
            [3, _] => Self::ThreeKind,
            [2, 2] => Self::TwoPair,
            [2, _] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

pub fn part1(cards: &CamelCards) -> PartOutput<usize> {
    let mut hands: Vec<(HandType, Hand, usize)> = cards
        .0
        .iter()
        .map(|(hand, bid)| (HandType::from_hand_counter(hand), hand.clone(), *bid))
        .collect();
    hands.sort_by(
        |(a_hand_type, a_hand, _a_bid), (b_hand_type, b_hand, _b_bid)| {
            a_hand_type
                .cmp(b_hand_type)
                .then(a_hand.cmp(b_hand))
                .then_with(|| panic!())
        },
    );
    log::debug!("hands: {:?}", hands);
    PartOutput {
        answer: hands
            .into_iter()
            .map(|(_hand_type, _hand, bid)| bid)
            .enumerate()
            .map(|(idx, bid)| (idx + 1) * bid)
            .sum(),
    }
}

pub fn part2(cards: &CamelCards) -> PartOutput<usize> {
    let mut hands: Vec<(HandType, Hand, usize)> = cards
        .0
        .iter()
        .map(|(hand, bid)| {
            (
                HandType::from_hand_counter(&hand.to_joker()),
                hand.to_joker(),
                *bid,
            )
        })
        .collect();
    hands.sort_by(
        |(a_hand_type, a_hand, _a_bid), (b_hand_type, b_hand, _b_bid)| {
            a_hand_type
                .cmp(b_hand_type)
                .then(a_hand.cmp(b_hand))
                .then_with(|| panic!())
        },
    );
    log::debug!("hands: {:?}", hands);
    PartOutput {
        answer: hands
            .into_iter()
            .map(|(_hand_type, _hand, bid)| bid)
            .enumerate()
            .map(|(idx, bid)| (idx + 1) * bid)
            .sum(),
    }
}

pub const DAY: Day<CamelCards, usize, 1, 0, 0> = Day {
    title: "Camel Cards",
    display: (
        "The total winnings are {answer}.",
        "The new total winnings with jokers are {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day07.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_card() {
        assert!(Card::A > Card::K);
    }

    #[test]
    fn test_hand() {
        assert!(
            Hand([Card::Six, Card::Six, Card::Six, Card::Six, Card::Six,])
                > Hand([Card::Six, Card::Six, Card::Six, Card::Six, Card::Five,])
        );
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            HandType::from_hand_counter(&Hand([
                Card::Six,
                Card::Six,
                Card::Six,
                Card::Six,
                Card::Six,
            ])),
            HandType::FiveKind
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([
                Card::Two,
                Card::Six,
                Card::Six,
                Card::Six,
                Card::Six,
            ])),
            HandType::FourKind
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([
                Card::Two,
                Card::Two,
                Card::Six,
                Card::Six,
                Card::Six,
            ])),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([
                Card::Two,
                Card::Six,
                Card::Six,
                Card::Six,
                Card::A
            ])),
            HandType::ThreeKind
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([Card::Two, Card::Six, Card::Six, Card::Q, Card::Q])),
            HandType::TwoPair
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([Card::Two, Card::Six, Card::Six, Card::Q, Card::A])),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_hand_counter(&Hand([
                Card::Two,
                Card::Six,
                Card::Seven,
                Card::Eight,
                Card::A
            ])),
            HandType::HighCard
        );
    }
}

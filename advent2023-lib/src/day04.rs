use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::parser::read_vec1;
use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    selected: HashSet<usize>,
}

impl Card {
    fn count_matching(&self) -> usize {
        self.winning.intersection(&self.selected).count()
    }
    fn points(&self) -> usize {
        let won = self.count_matching();
        if won == 0 {
            return 0;
        }
        1 << (won - 1)
    }
}

static RE_CARD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Card +(?<id>\d+):(?<winning>.+)\|(?<selected>.+)").unwrap());

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = RE_CARD.captures(s) else {
            return Err(ParseError::Str(format!("unexpected {}", s)));
        };
        Ok(Card {
            id: captures["id"]
                .parse()
                .or(Err(ParseError::Str(format!("card {}", &captures["id"]))))?,
            winning: captures["winning"]
                .split_whitespace()
                .map(usize::from_str)
                .collect::<Result<HashSet<_>, _>>()
                .or(Err(ParseError::Str(format!(
                    "winning {}",
                    &captures["winning"]
                ))))?,
            selected: captures["selected"]
                .split_whitespace()
                .map(usize::from_str)
                .collect::<Result<HashSet<_>, _>>()
                .or(Err(ParseError::Str(format!(
                    "selected {}",
                    &captures["selected"]
                ))))?,
        })
    }
}

#[derive(Debug)]
pub struct PileOfColourfulCards(Vec<Card>);

pub fn parse(input: &str) -> ParseResult<PileOfColourfulCards> {
    Ok(PileOfColourfulCards(read_vec1::<Card>(input)?))
}

pub fn part1(pile_of_cards: &PileOfColourfulCards) -> PartOutput<usize> {
    PartOutput {
        answer: pile_of_cards.0.iter().map(|c| c.points()).sum(),
    }
}

pub fn part2(pile_of_cards: &PileOfColourfulCards) -> PartOutput<usize> {
    let mut card_counter = HashMap::<usize, usize>::new();
    // We assume pile_of_cards is a sorted list
    for card in &pile_of_cards.0 {
        let additional_count = card_counter.entry(card.id).or_insert(1).to_owned();
        let matching = card.count_matching();
        for idx in 1..=matching {
            card_counter
                .entry(card.id + idx)
                .and_modify(|count| *count += additional_count)
                .or_insert(1 + additional_count);
        }
        log::debug!("state: {:#?}", card_counter);
    }
    PartOutput {
        answer: card_counter.values().sum(),
    }
}

pub const DAY: Day<PileOfColourfulCards, usize, 1, 0, 0> = Day {
    title: "Scratchcards",
    display: (
        "The scratchcards are worth {answer} points.",
        "We end up with {answer} scratchcards.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day04.txt")),
};

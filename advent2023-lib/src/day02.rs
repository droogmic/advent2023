use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::parser::read_vec1;
use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for num_colour in s.split(", ") {
            let (num, col) = num_colour
                .split_once(' ')
                .ok_or(ParseError::Str(format!("unexpected {}", num_colour)))?;
            match col {
                "red" => red = num.parse()?,
                "green" => green = num.parse()?,
                "blue" => blue = num.parse()?,
                _ => return Err(ParseError::Str(format!("unexpected {}", col))),
            }
        }
        Ok(Hand { red, green, blue })
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    hands: Vec<Hand>,
}

static RE_GAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?<id>\d+): (?<hands>.+)").unwrap());

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = RE_GAME.captures(s) else {
            return Err(ParseError::Str(format!("Unexpected {}", s)));
        };
        Ok(Game {
            id: captures["id"].parse()?,
            hands: captures["hands"]
                .split("; ")
                .map(Hand::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct Records(Vec<Game>);

pub fn parse(input: &str) -> ParseResult<Records> {
    Ok(Records(read_vec1::<Game>(input)?))
}

pub fn part1(records: &Records) -> PartOutput<usize> {
    PartOutput {
        answer: records
            .0
            .iter()
            .filter_map(|game| {
                if game
                    .hands
                    .iter()
                    .any(|hand| hand.red > 12 || hand.green > 13 || hand.blue > 14)
                {
                    None
                } else {
                    Some(game.id)
                }
            })
            .sum(),
    }
}

pub fn part2(records: &Records) -> PartOutput<usize> {
    PartOutput {
        answer: records
            .0
            .iter()
            .map(|game| {
                let min_red = game.hands.iter().map(|hand| hand.red).max().unwrap();
                let min_green = game.hands.iter().map(|hand| hand.green).max().unwrap();
                let min_blue = game.hands.iter().map(|hand| hand.blue).max().unwrap();
                min_red * min_green * min_blue
            })
            .sum(),
    }
}

pub const DAY: Day<Records, usize, 1, 0, 0> = Day {
    title: "Cube Conundrum",
    display: (
        "The sum of the IDs of the possible games is {answer}.",
        "The sum of the powers is {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day02.txt")),
};

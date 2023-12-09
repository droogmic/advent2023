use std::collections::HashMap;

use crate::parser::{read_map, FromChar};
use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

enum SchematicCell {
    Blank,
    Digit(u8),
    Symbol(char),
}

impl FromChar for SchematicCell {
    type Err = ParseError;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        Ok(match c {
            '.' => Self::Blank,
            '0'..='9' => Self::Digit(c.to_digit(10).unwrap().try_into().unwrap()),
            c => Self::Symbol(c),
        })
    }
}

#[derive(Debug)]
pub struct Part {
    symbol: char,
    numbers: Vec<usize>,
}

#[derive(Debug)]
pub struct Schematic {
    parts: Vec<Part>,
}

fn scan_horizontal(map: &HashMap<(usize, usize), SchematicCell>, start: (usize, usize)) -> usize {
    let mut pos = start;
    assert!(matches!(map.get(&pos), Some(SchematicCell::Digit(_))));
    loop {
        pos = (pos.0 - 1, pos.1);
        match map.get(&pos) {
            Some(SchematicCell::Digit(_)) => continue,
            Some(SchematicCell::Blank) | None => break,
            Some(SchematicCell::Symbol(_)) => panic!(),
        }
    }
    construct_right(map, (pos.0 + 1, pos.1))
}

fn construct_right(map: &HashMap<(usize, usize), SchematicCell>, start: (usize, usize)) -> usize {
    let mut pos = start;
    let Some(SchematicCell::Digit(first_char)) = map.get(&pos) else {
        panic!()
    };
    let mut digits = vec![*first_char];
    loop {
        pos = (pos.0 + 1, pos.1);
        match map.get(&pos) {
            Some(SchematicCell::Digit(d)) => digits.push(*d),
            Some(SchematicCell::Blank | SchematicCell::Symbol(_)) | None => break,
        }
    }
    digits
        .into_iter()
        .fold(0_usize, |acc, elem| acc * 10 + usize::from(elem))
}

pub fn parse(input: &str) -> ParseResult<Schematic> {
    let map = read_map::<SchematicCell>(input)?;
    let mut parts = Vec::new();
    for (&(x, y), cell) in &map {
        if let SchematicCell::Symbol(symbol) = cell {
            let mut numbers = Vec::new();
            // top and bottom
            for off_y in [y - 1, y + 1] {
                if let Some(SchematicCell::Digit(_)) = map.get(&(x - 1, off_y)) {
                    numbers.push(scan_horizontal(&map, (x - 1, off_y)));
                    // Check if second number, e.g. 123 456
                    //                                 p
                    if let Some(SchematicCell::Blank) = map.get(&(x, off_y)) {
                        if let Some(SchematicCell::Digit(_)) = map.get(&(x + 1, off_y)) {
                            numbers.push(construct_right(&map, (x + 1, off_y)));
                        }
                    }
                } else if let Some(SchematicCell::Digit(_)) = map.get(&(x, off_y)) {
                    numbers.push(construct_right(&map, (x, off_y)));
                } else if let Some(SchematicCell::Digit(_)) = map.get(&(x + 1, off_y)) {
                    numbers.push(construct_right(&map, (x + 1, off_y)));
                }
            }
            // left
            if let Some(SchematicCell::Digit(_)) = map.get(&(x - 1, y)) {
                numbers.push(scan_horizontal(&map, (x - 1, y)));
            }
            // right
            if let Some(SchematicCell::Digit(_)) = map.get(&(x + 1, y)) {
                numbers.push(construct_right(&map, (x + 1, y)));
            }
            parts.push(Part {
                symbol: *symbol,
                numbers,
            });
        }
    }
    Ok(Schematic { parts })
}

pub fn part1(schematic: &Schematic) -> PartOutput<usize> {
    log::info!("Schematic: {:#?}", schematic);
    PartOutput {
        answer: schematic
            .parts
            .iter()
            .map(|part| part.numbers.iter().sum::<usize>())
            .sum(),
    }
}

pub fn part2(schematic: &Schematic) -> PartOutput<usize> {
    PartOutput {
        answer: schematic
            .parts
            .iter()
            .filter_map(|part| {
                if part.symbol != '*' || part.numbers.len() != 2 {
                    None
                } else {
                    Some(part.numbers[0] * part.numbers[1])
                }
            })
            .sum(),
    }
}

pub const DAY: Day<Schematic, usize, 1, 0, 0> = Day {
    title: "Gear Ratios",
    display: (
        "The sum of all of the part numbers in the engine schematic is {answer}.",
        "The sum of all of the gear ratios in the engine schematic is {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day03.txt")),
};

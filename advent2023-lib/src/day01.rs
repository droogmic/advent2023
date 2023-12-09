use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, FromRepr};

use crate::parser::read_vec2;
use crate::{Day, DayCalc, Examples, ParseResult, PartOutput};

pub struct Chars(Vec<Vec<char>>);

pub fn parse(input: &str) -> ParseResult<Chars> {
    Ok(Chars(read_vec2::<char>(input)?))
}

pub fn part1(chars: &Chars) -> PartOutput<usize> {
    PartOutput {
        answer: chars
            .0
            .iter()
            .map(|line| {
                let first = line
                    .iter()
                    .find(|c| c.is_numeric())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let last = line
                    .iter()
                    .rev()
                    .find(|c| c.is_numeric())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                usize::try_from(10 * first + last).unwrap()
            })
            .sum::<usize>(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, EnumIter, FromRepr)]
#[strum(serialize_all = "snake_case")]
enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Digit {
    fn val(&self) -> usize {
        *self as usize
    }

    fn find_first(s: &[char]) -> Option<Self> {
        for index in 0..s.len() {
            if s[index].is_numeric() {
                return Self::from_repr(s[index].to_digit(10).unwrap().try_into().unwrap());
            }
            for color in Self::iter() {
                let color_str = color.to_string();
                let end_index = index + color_str.len();
                if let Some(potential) =
                    s.get(index..end_index).map(|s| String::from_iter(s.iter()))
                {
                    log::trace!("comparing {} to {}", potential, color_str);
                    if potential == color_str {
                        log::trace!("matched {}", color_str);
                        return Some(color);
                    }
                }
            }
        }
        log::error!("no match for {}", String::from_iter(s.iter()));
        None
    }

    fn find_last(s: &[char]) -> Option<Self> {
        for index in (0..s.len()).rev() {
            if s[index].is_numeric() {
                return Self::from_repr(s[index].to_digit(10).unwrap().try_into().unwrap());
            }
            for color in Self::iter() {
                let color_str = color.to_string();
                if let Some(start_index) =
                    index.checked_add(1).unwrap().checked_sub(color_str.len())
                {
                    if let Some(potential) = s
                        .get(start_index..=index)
                        .map(|s| String::from_iter(s.iter()))
                    {
                        log::trace!("comparing {} to {}", potential, color_str);
                        if potential == color_str {
                            log::trace!("matched {}", color_str);
                            return Some(color);
                        }
                    }
                }
            }
        }
        log::error!("no match for {}", String::from_iter(s.iter()));
        None
    }
}

pub fn part2(chars: &Chars) -> PartOutput<usize> {
    PartOutput {
        answer: chars
            .0
            .iter()
            .map(|line| {
                let first = Digit::find_first(line).unwrap();
                let last = Digit::find_last(line).unwrap();
                log::debug!(
                    "On line {:?} the first digit is {} and the last is {}",
                    String::from_iter(line.iter()),
                    first,
                    last,
                );
                10 * first.val() + last.val()
            })
            .sum::<usize>(),
    }
}

pub const DAY: Day<Chars, usize, 0, 1, 1> = Day {
    title: "Trebuchet?!",
    display: (
        "The sum of all the numeric calibration values is {answer}.",
        "The sum of all the alphanumeric calibration values is {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::pair(
        include_str!("../../examples/day01-1.txt"),
        include_str!("../../examples/day01-2.txt"),
    ),
};

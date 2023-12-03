use std::char::ParseCharError;
use std::collections::btree_map::BTreeMap;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::rc::Rc;

use recap::Error as RecapError;
use strum::ParseError as StrumParseError;

mod day01;
pub mod parser;
mod test;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    First,
    Second,
}

#[derive(Debug)]
pub enum ParseError {
    Empty,
    Int(ParseIntError),
    Char(ParseCharError),
    Str(String),
    Strum(StrumParseError),
    Recap(RecapError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::Int(value)
    }
}

impl From<ParseCharError> for ParseError {
    fn from(value: ParseCharError) -> Self {
        Self::Char(value)
    }
}

impl From<StrumParseError> for ParseError {
    fn from(value: StrumParseError) -> Self {
        Self::Strum(value)
    }
}

impl From<RecapError> for ParseError {
    fn from(value: RecapError) -> Self {
        Self::Recap(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid input for day")
    }
}

pub type ParseResult<D> = std::result::Result<D, ParseError>;

#[derive(Debug, Default)]
pub struct PartOutput<O> {
    pub answer: O,
}

pub struct DayCalc<D, O> {
    pub parse: fn(&str) -> ParseResult<D>,
    pub part1: fn(&D) -> PartOutput<O>,
    pub part2: fn(&D) -> PartOutput<O>,
}

pub struct Examples<const C: usize, const F: usize, const S: usize> {
    pub common: [&'static str; C],
    pub part1: [&'static str; F],
    pub part2: [&'static str; S],
}
impl Examples<1, 0, 0> {
    const fn _single(include_str: &'static str) -> Self {
        Examples {
            common: [include_str],
            part1: [],
            part2: [],
        }
    }
}
impl Examples<0, 1, 1> {
    const fn pair(first: &'static str, second: &'static str) -> Self {
        Examples {
            common: [],
            part1: [first],
            part2: [second],
        }
    }
}

pub struct Day<D, O, const C: usize, const F: usize, const S: usize> {
    pub title: &'static str,
    pub display: (&'static str, &'static str),
    pub calc: DayCalc<D, O>,
    pub examples: Examples<C, F, S>,
}

pub enum PrimaryExample {
    Same(&'static str),
    Different([&'static str; 2]),
}

pub trait Printable {
    fn get_display(&self) -> (&'static str, &'static str);
    fn get_title(&self) -> &'static str;
    fn get_examples(&self) -> PrimaryExample;
}

impl<D, O, const C: usize, const F: usize, const S: usize> Printable for Day<D, O, C, F, S> {
    fn get_display(&self) -> (&'static str, &'static str) {
        self.display
    }
    fn get_title(&self) -> &'static str {
        self.title
    }
    fn get_examples(&self) -> PrimaryExample {
        let first = self
            .examples
            .part1
            .get(0)
            .or(self.examples.common.get(0))
            .unwrap();
        let second = self
            .examples
            .part2
            .get(0)
            .or(self.examples.common.get(0))
            .unwrap();
        if first == second {
            PrimaryExample::Same(first)
        } else {
            PrimaryExample::Different([first, second])
        }
    }
}

type DayResult = ParseResult<(String, String)>;

pub trait Calculable {
    fn calc(&self, part: Part, input: &str) -> ParseResult<String>;
    fn both(&self, input: &str) -> DayResult;
    fn get_both_func(&self) -> Rc<dyn Fn(&str) -> DayResult>;
}

impl<
        D: 'static,
        O: 'static + std::fmt::Display,
        const C: usize,
        const F: usize,
        const S: usize,
    > Calculable for Day<D, O, C, F, S>
{
    fn calc(&self, part: Part, input: &str) -> ParseResult<String> {
        let parse = self.calc.parse;
        let input = parse(input)?;
        Ok(match part {
            Part::First => (self.calc.part1)(&input).answer.to_string(),
            Part::Second => (self.calc.part2)(&input).answer.to_string(),
        })
    }
    fn both(&self, input: &str) -> DayResult {
        let parse = self.calc.parse;
        let part1 = self.calc.part1;
        let part2 = self.calc.part2;
        let input = parse(input)?;
        Ok((
            part1(&input).answer.to_string(),
            part2(&input).answer.to_string(),
        ))
    }
    fn get_both_func(&self) -> Rc<dyn Fn(&str) -> DayResult> {
        let parse = self.calc.parse;
        let part1 = self.calc.part1;
        let part2 = self.calc.part2;
        Rc::new(move |input: &str| {
            let input = parse(input)?;
            Ok((
                part1(&input).answer.to_string(),
                part2(&input).answer.to_string(),
            ))
        })
    }
}

pub trait DayTrait: Printable + Calculable + Send {}

impl<
        D: 'static,
        O: 'static + std::fmt::Display,
        const C: usize,
        const F: usize,
        const S: usize,
    > DayTrait for Day<D, O, C, F, S>
{
}

pub fn get_days() -> BTreeMap<usize, Box<dyn DayTrait + 'static>> {
    let mut days: BTreeMap<usize, Box<dyn DayTrait + 'static>> = BTreeMap::new();
    days.insert(1, Box::new(day01::DAY));
    days
}

pub fn get_input(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .or_else(|_| fs::read_to_string(format!("../inputs/day{:02}.txt", day)))
    {
        Err(e) => panic!("Err: {}, inputs/day{:02}.txt", e, day),
        Ok(string) => string,
    }
}

#[macro_export]
macro_rules! regex_once {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

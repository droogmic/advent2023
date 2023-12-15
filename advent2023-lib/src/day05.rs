use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Range {
    destination: usize,
    source: usize,
    length: usize,
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let destination = split.next().unwrap().parse()?;
        let source = split.next().unwrap().parse()?;
        let length = split.next().unwrap().parse()?;
        assert!(split.next().is_none());
        Ok(Self {
            destination,
            source,
            length,
        })
    }
}

impl Range {
    fn convert(&self, from: usize) -> Option<usize> {
        let offset = from.wrapping_sub(self.source);
        if offset <= self.length {
            Some(self.destination + offset)
        } else {
            None
        }
    }
    fn invert_convert(&self, from: usize) -> Option<usize> {
        let offset = from.wrapping_sub(self.destination);
        if offset <= self.length {
            Some(self.source + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (from, to) = {
            let (naming, map) = lines.next().unwrap().split_once(' ').unwrap();
            assert_eq!(map, "map:");
            naming.split_once("-to-").unwrap()
        };
        let ranges = lines
            .map(Range::from_str)
            .collect::<Result<Vec<Range>, ParseError>>()?;
        Ok(Self {
            from: from.to_owned(),
            to: to.to_owned(),
            ranges,
        })
    }
}

impl Map {
    fn convert(&self, from: usize) -> usize {
        for range in &self.ranges {
            if let Some(to) = range.convert(from) {
                return to;
            }
        }
        from
    }
    fn invert_convert(&self, from: usize) -> HashSet<usize> {
        let mut set: HashSet<usize> = self
            .ranges
            .iter()
            .filter_map(|range| range.invert_convert(from))
            .collect();
        if self.convert(from) == from {
            set.insert(from);
        }
        set
    }
}

#[derive(Debug)]
pub struct Almanac {
    start: HashSet<usize>,
    ranges: HashSet<RangeInclusive<usize>>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.trim().split("\n\n");
        let (start, ranges) = {
            let mut parts = sections.next().unwrap().split_whitespace();
            assert_eq!(parts.next(), Some("seeds:"));
            let number = parts
                .map(usize::from_str)
                .collect::<Result<Vec<usize>, ParseIntError>>()?;
            (
                number.iter().cloned().collect(),
                number
                    .chunks(2)
                    .map(|chunk| RangeInclusive::new(chunk[0], chunk[0] + chunk[1]))
                    .collect(),
            )
        };
        let maps = sections
            .map(Map::from_str)
            .collect::<Result<Vec<Map>, ParseError>>()?;
        assert_eq!(maps.first().unwrap().from, "seed");
        for window in maps.windows(2) {
            assert_eq!(window[0].to, window[1].from);
        }
        assert_eq!(maps.last().unwrap().to, "location");
        Ok(Self {
            start,
            ranges,
            maps,
        })
    }
}

impl Almanac {
    fn convert(&self, from: usize) -> usize {
        let mut val = from;
        for map in &self.maps {
            val = map.convert(val);
        }
        val
    }
    fn invert_convert(&self, from: usize) -> HashSet<usize> {
        let mut vals = HashSet::new();
        vals.insert(from);
        for map in self.maps.iter().rev() {
            vals = vals
                .into_iter()
                .map(|val| map.invert_convert(val))
                .flatten()
                .collect();
        }
        vals
    }
}

pub fn parse(input: &str) -> ParseResult<Almanac> {
    input.parse()
}

pub fn part1(almanac: &Almanac) -> PartOutput<usize> {
    let min_location = almanac
        .start
        .iter()
        .map(|&seed| almanac.convert(seed))
        .min()
        .unwrap();
    PartOutput {
        answer: min_location,
    }
}

pub fn part2(almanac: &Almanac) -> PartOutput<usize> {
    let mut location = 1;
    loop {
        log::debug!(
            "location {} checking seeds in ranges {:?}",
            location,
            almanac.ranges
        );
        let seeds = almanac.invert_convert(location);
        if seeds
            .into_iter()
            .any(|seed| almanac.ranges.iter().any(|range| range.contains(&seed)))
        {
            break;
        }
        location += 1;
        if location % 1000000 == 0 {
            log::info!("location {}", location);
        }
    }
    PartOutput { answer: location }
}

pub const DAY: Day<Almanac, usize, 1, 0, 0> = Day {
    title: "If You Give A Seed A Fertilizer",
    display: (
        "The lowest location number is {answer}.",
        "The lowest location number given ranges is {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day05.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_range() {
        let range = Range {
            destination: 10,
            source: 20,
            length: 5,
        };
        assert_eq!(range.convert(19), None);
        assert_eq!(range.convert(20), Some(10));
        assert_eq!(range.convert(21), Some(11));
        assert_eq!(range.invert_convert(9), None);
        assert_eq!(range.invert_convert(10), Some(20));
        assert_eq!(range.invert_convert(11), Some(21));
    }

    #[test]
    fn test_map() {
        let map = Map {
            from: String::from("foo"),
            to: String::from("bar"),
            ranges: vec![
                Range {
                    destination: 10,
                    source: 20,
                    length: 5,
                },
                Range {
                    destination: 10,
                    source: 100,
                    length: 2,
                },
            ],
        };
        assert_eq!(map.convert(19), 19);
        assert_eq!(map.convert(20), 10);
        assert_eq!(map.convert(21), 11);
        assert_eq!(map.invert_convert(9), [9].into());
        assert_eq!(map.invert_convert(10), [10, 20, 100].into());
        assert_eq!(map.invert_convert(11), [11, 21, 101].into());
    }
}

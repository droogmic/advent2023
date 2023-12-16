use std::num::ParseIntError;
use std::ops::{Neg, RangeInclusive};
use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn from_parsed(
        time: Result<usize, ParseIntError>,
        distance: Result<usize, ParseIntError>,
    ) -> Result<Self, ParseIntError> {
        let time = match time {
            Ok(time) => time,
            Err(err) => return Err(err),
        };
        let distance = match distance {
            Ok(distance) => distance,
            Err(err) => return Err(err),
        };
        Ok(Self { time, distance })
    }
    fn winning_range(&self) -> RangeInclusive<usize> {
        let a = 1;
        let b = i32::try_from(self.time).unwrap().neg();
        let c = self.distance as f64;
        let roots = solve_quadratic(a, b, c);
        (roots[0] + 0.000000001).ceil() as usize..=(roots[1] - 0.000000001).floor() as usize
    }
}

#[derive(Debug)]
pub struct Competition {
    races: Vec<Race>,
    kerning_race: Race,
}

impl FromStr for Competition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [times, distances] = {
            let mut lines = s.lines();
            [lines.next().unwrap(), lines.next().unwrap()]
        };
        Ok(Self {
            races: times
                .split_whitespace()
                .skip(1)
                .map(usize::from_str)
                .zip(distances.split_whitespace().skip(1).map(usize::from_str))
                .map(|(time, distance)| Race::from_parsed(time, distance))
                .collect::<Result<Vec<_>, _>>()?,
            kerning_race: Race {
                time: times
                    .split_whitespace()
                    .skip(1)
                    .collect::<String>()
                    .parse()?,
                distance: distances
                    .split_whitespace()
                    .skip(1)
                    .collect::<String>()
                    .parse()?,
            },
        })
    }
}

pub fn parse(input: &str) -> ParseResult<Competition> {
    Competition::from_str(input)
}

fn solve_quadratic<A: Into<f64>, B: Into<f64>, C: Into<f64>>(a: A, b: B, c: C) -> [f64; 2] {
    let a: f64 = a.into();
    let b: f64 = b.into();
    let c: f64 = c.into();
    let discriminant_squared = b.powi(2) - (4.0 * a * c);
    assert!(discriminant_squared > 0.0);
    let discriminant = discriminant_squared.sqrt();
    assert!(discriminant.is_normal());
    let roots = [
        (b.neg() - discriminant) / (2.0 * a),
        (b.neg() + discriminant) / (2.0 * a),
    ];
    log::debug!("{} {} {} roots: {:?}", a, b, c, roots);
    roots
}

/// T: competition time
/// t: hold time
/// s: competition distance
/// D: distance to beat
///
/// v = t: velocity
/// s = v * (T - t)
/// s = t * (T - t)
/// s = Tt - t²
///
/// The goal of the competition is to maximize d for 0 < t < T, t ∈ ℤ
///
/// Part 1 requires us to find the solution pair where:
/// D = s = Tt - t²
/// t² - Tt + D = 0
///
/// This is the quadratic equation where
/// a = 1, b = -T, and c = D
pub fn part1(competition: &Competition) -> PartOutput<usize> {
    log::debug!("competition: {:?}", competition);
    PartOutput {
        answer: competition
            .races
            .iter()
            .map(|race| {
                let range = race.winning_range();
                log::debug!("range: {:?}", range);
                range.count()
            })
            .product(),
    }
}

pub fn part2(competition: &Competition) -> PartOutput<usize> {
    PartOutput {
        answer: competition.kerning_race.winning_range().count(),
    }
}

pub const DAY: Day<Competition, usize, 1, 0, 0> = Day {
    title: "Wait For It",
    display: (
        "The product of the ways to beat the record are {answer}.",
        "There are {answer} ways to beat the race.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day06.txt")),
};

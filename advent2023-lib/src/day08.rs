use std::collections::HashMap;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node([char; 3]);

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect::<Vec<_>>().try_into().unwrap()))
    }
}

impl Node {
    fn start() -> Self {
        Self(['A', 'A', 'A'])
    }
    fn end() -> Self {
        Self(['Z', 'Z', 'Z'])
    }
    fn ghost_start(&self) -> bool {
        self.0[2] == 'A'
    }
    fn ghost_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

#[derive(Debug)]
pub struct Documents {
    instructions: Vec<Direction>,
    nodes: HashMap<Node, [Node; 2]>,
}

static RE_NODE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<from>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap());

impl FromStr for Documents {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from_char)
            .collect();
        assert_eq!(lines.next(), Some(""));
        let nodes = lines
            .map(|s| {
                let Some(captures) = RE_NODE.captures(s) else {
                    return Err(ParseError::Str(format!("unexpected {}", s)));
                };
                Ok((
                    Node::from_str(&captures["from"])?,
                    [
                        Node::from_str(&captures["left"])?,
                        Node::from_str(&captures["right"])?,
                    ],
                ))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self {
            instructions,
            nodes,
        })
    }
}

impl Documents {
    fn get_direction(&self, index: usize) -> Direction {
        *self
            .instructions
            .get(index % self.instructions.len())
            .unwrap()
    }
}

pub fn parse(input: &str) -> ParseResult<Documents> {
    input.parse()
}

pub fn part1(maps: &Documents) -> PartOutput<usize> {
    let mut step = 0;
    let mut node = Node::start();
    while node != Node::end() {
        let next = maps.nodes.get(&node).unwrap();
        match maps.get_direction(step) {
            Direction::Left => node = next[0].clone(),
            Direction::Right => node = next[1].clone(),
        }
        step += 1;
    }
    PartOutput { answer: step }
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a.rem_euclid(b))
    }
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a.checked_mul(b)
        .unwrap()
        .checked_div(greatest_common_divisor(a, b))
        .unwrap()
}

pub fn part2(maps: &Documents) -> PartOutput<usize> {
    let starting_nodes: Vec<Node> = maps
        .nodes
        .keys()
        .filter(|n| n.ghost_start())
        .cloned()
        .collect();
    log::debug!("starting: {:?}", starting_nodes);
    let steps: Vec<usize> = starting_nodes
        .into_iter()
        .map(|mut node| {
            let mut step = 0;
            while !node.ghost_end() {
                let next = maps.nodes.get(&node).unwrap();
                match maps.get_direction(step) {
                    Direction::Left => node = next[0].clone(),
                    Direction::Right => node = next[1].clone(),
                }
                step += 1;
            }
            step
        })
        .collect();
    log::debug!("steps: {:?}", steps);
    PartOutput {
        answer: steps.into_iter().reduce(least_common_multiple).unwrap(),
    }
}

pub const DAY: Day<Documents, usize, 0, 2, 1> = Day {
    title: "Haunted Wasteland",
    display: (
        "{answer} steps are required to reach ZZZ.",
        "{answer} steps are required to reach **Z.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples {
        common: [],
        part1: [
            include_str!("../../examples/day08-1-2.txt"),
            include_str!("../../examples/day08-1-1.txt"),
        ],
        part2: [include_str!("../../examples/day08-2.txt")],
    },
};

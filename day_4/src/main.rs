#![allow(dead_code)]
use anyhow::Context;
use std::{ops::Range, str::FromStr};

#[derive(Debug, Default, Clone)]
struct Group {
    first: Box<Range<usize>>,
    second: Box<Range<usize>>,
}

impl Group {
    fn one_contains_the_other(&self) -> bool {
        fully_contains(&self.first, &self.second) || fully_contains(&self.second, &self.first)
    }
}

fn fully_contains(this: &Range<usize>, other: &Range<usize>) -> bool {
    this.start <= other.start && this.end >= other.end
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s
            .split(',')
            .map(|range| {
                let splitted_range = range
                    .split('-')
                    .map(|n| n.parse::<usize>())
                    .filter_map(Result::ok)
                    .collect::<Vec<usize>>();

                Box::new(splitted_range[0]..splitted_range[1])
            })
            .collect::<Vec<_>>();

        let second = splitted.pop().context("No last range")?;
        let first = splitted.pop().context("No first range")?;

        Ok(Group { first, second })
    }
}

const DEFAULT_INPUT_PATH: &'static str = "input.txt";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let file_path = args.nth(1).unwrap_or(DEFAULT_INPUT_PATH.to_string());
    let buf = std::fs::read_to_string(file_path)?;

    let groups = buf
        .lines()
        .map(Group::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<Group>>();

    println!("{:#?}", groups);

    let count = groups
        .iter()
        .map(|group| group.one_contains_the_other())
        .filter(|&bool| bool)
        .count();

    println!("group: {count}");

    Ok(())
}

#![allow(unused)]

use anyhow::Context;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
struct Backpack {
    part_1: Vec<char>,
    part_2: Vec<char>,
}

impl Backpack {
    fn count_char(&self, c: char) -> usize {
        self.part_1.iter().filter(|e| *e == &c).count()
            + self.part_2.iter().filter(|e| *e == &c).count()
    }
}

impl Display for Backpack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.part_1.iter().collect::<String>(),
            self.part_2.iter().collect::<String>(),
        )
    }
}

#[derive(Debug, Clone)]
struct Group {
    one: Backpack,
    two: Backpack,
    three: Backpack,
    badge: char,
}

impl Group {
    fn new(one: Backpack, two: Backpack, three: Backpack) -> Self {
        let common_one_and_two = one.find_common(&two);

        let badge = three
            .find_uniq(&common_one_and_two)
            .expect("invalid input, no uniq char found");

        Group {
            one,
            two,
            three,
            badge,
        }
    }
}

impl Backpack {
    fn find_common_inside(&self) -> Vec<char> {
        let mut common = Vec::new();

        for c in 'A'..='z' {
            if self.part_1.contains(&c) && self.part_2.contains(&c) {
                common.push(c);
            }
        }
        common
    }

    fn find_common(&self, other: &Self) -> Vec<char> {
        let mut common = Vec::new();

        for c in 'A'..='z' {
            if (self.part_1.contains(&c) || self.part_2.contains(&c))
                && (other.part_1.contains(&c) || other.part_2.contains(&c))
            {
                common.push(c);
            }
        }

        common
    }

    fn find_uniq(&self, list: &[char]) -> Option<char> {
        for c in list {
            if self.part_1.contains(&c) || self.part_2.contains(&c) {
                return Some(*c);
            }
        }

        None
    }
}

impl FromStr for Backpack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let middle = s.len() / 2;
        Ok(Backpack {
            part_1: s[..middle].chars().collect(),
            part_2: s[middle..].chars().collect(),
        })
    }
}

const DEFAULT_INPUT: &'static str = "input.txt";

fn get_value(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}

fn main() -> anyhow::Result<()> {
    let input_file_path = std::env::args().nth(1).unwrap_or(DEFAULT_INPUT.to_owned());
    let buf = std::fs::read_to_string(input_file_path).context("failed to read input file")?;
    let packs = buf
        .lines()
        .map(Backpack::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<Backpack>>();

    let total = packs
        .chunks(3)
        .map(|chunk| Group::new(chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
        .map(|group| get_value(group.badge))
        .sum::<u32>();

    println!("Total: {total}");

    Ok(())
}

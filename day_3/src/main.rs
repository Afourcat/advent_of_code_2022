use anyhow::Context;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Backpack {
    part_1: Vec<char>,
    part_2: Vec<char>,
}

impl Backpack {
    fn find_common(&self) -> Vec<char> {
        let mut common = Vec::new();

        for c in 'A'..='z' {
            if self.part_1.contains(&c) && self.part_2.contains(&c) {
                common.push(c);
            }
        }
        common
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
    let total = buf
        .lines()
        .map(Backpack::from_str)
        .filter_map(Result::ok)
        .map(|b| {
            b.find_common()
                .into_iter()
                .map(|c| get_value(c))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("total => {total}");
    Ok(())
}

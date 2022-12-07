#![allow(dead_code)]
#![feature(iter_next_chunk)]
use std::fmt::{Display, Formatter};

use anyhow::Context;

#[derive(Debug, Clone)]
struct Stack {
    inner: Vec<Vec<Option<char>>>,
}

impl Stack {
    fn new(input: &[&str]) -> anyhow::Result<Self> {
        let height = input.len();
        let len = ((input[0].len() - 1) / 4) + 1;

        println!("creating {len} stack of height {height}");
        let mut inner = (0..)
            .take(len)
            .map(|_| Vec::with_capacity(height))
            .collect::<Vec<Vec<Option<char>>>>();

        for (y, line) in input.iter().rev().enumerate() {
            println!("line: {y}{line}");
            let mut chars = line.chars().next_chunk::<4>().iter().enumerate();

            loop {
                match (a, b, c) {
                    (' ', ' ', ' ') => inner[idx].push(None),
                    ('[', x, ']') => inner[idx].push(Some(*x)),
                    _ => anyhow::bail!("invalid input"),
                }
                chars.skip(1);
            }
        }

        Ok(Self { inner })
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, stack) in self.inner.iter().enumerate() {
            writeln!(f, "{} -> {}", idx, "flex")?;
        }
        Ok(())
    }
}

const DEFAULT_INPUT_PATH: &'static str = "input.txt";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let file_path = args.nth(1).unwrap_or(DEFAULT_INPUT_PATH.to_string());
    let buf = std::fs::read_to_string(file_path)?;

    let lines = buf.lines().collect::<Vec<&str>>();

    let separator = lines
        .iter()
        .position(|line| line.len() == 0)
        .context("invalid input file, an empty line is required")?;

    let stacks_lines = &lines[..separator - 1];
    let stack = Stack::new(stacks_lines).context("invalid stack input")?;
    println!("{}", stack);

    // let movements_lines = &lines[separator + 1..];
    // println!("movements_lines: {:?}", movements_lines);

    Ok(())
}

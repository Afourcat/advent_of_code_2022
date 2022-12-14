#![allow(dead_code)]
#![feature(iter_next_chunk)]
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use anyhow::{bail, Context};

#[derive(Debug, Clone)]
struct Stack {
    height: usize,
    width: usize,
    inner: Vec<Vec<char>>,
}

impl Stack {
    fn new(input: &[&str]) -> anyhow::Result<Self> {
        let max_stack_height = input.len();
        let nb_stack = (input[0].len() + 1) / 4;

        let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(max_stack_height); nb_stack];

        for line in input.iter().rev() {
            let elems: Vec<char> = line.chars().collect();
            for (x, chunk) in elems.chunks(4).enumerate() {
                match chunk {
                    ['[', c, ']', ' '] | ['[', c, ']'] => stacks[x].push(*c),
                    [' ', ' ', ' ', ' '] | [' ', ' ', ' '] => (),
                    _ => anyhow::bail!("Invalid input"),
                };
            }
        }

        Ok(Self {
            inner: stacks,
            height: max_stack_height,
            width: nb_stack,
        })
    }

    fn r#move(&mut self, m: Movement) -> anyhow::Result<()> {
        if m.from == m.to {
            return Ok(());
        } else if self.inner[m.from - 1].len() < m.nb {
            anyhow::bail!(
                "Cannot move {} from {} to {} no enough box",
                m.nb,
                m.from,
                m.to
            );
        }

        let mut from = std::mem::take(&mut self.inner[m.from - 1]);
        self.inner[m.to - 1].extend_from_slice(&from[from.len() - m.nb..]);
        from.truncate(from.len() - m.nb);
        self.inner[m.from - 1] = from;

        Ok(())
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, stack) in self.inner.iter().enumerate() {
            writeln!(f, "{} -> {:?}", idx + 1, stack)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    nb: usize,
    from: usize,
    to: usize,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace().into_iter();
        splitted.next();
        let nb = splitted
            .next()
            .context("invalid movement")?
            .parse::<usize>()
            .context("not a number")?;
        splitted.next();
        let from = splitted
            .next()
            .context("invalid movement")?
            .parse::<usize>()
            .context("not a number")?;
        splitted.next();
        let to = splitted
            .next()
            .context("invalid movement")?
            .parse::<usize>()
            .context("not a number")?;

        Ok(Movement { nb, from, to })
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
    let mut stack = Stack::new(stacks_lines).context("invalid stack input")?;
    println!("{}", stack);

    let movements = &lines[separator + 1..]
        .iter()
        .filter_map(|line| Movement::from_str(line).ok())
        .collect::<Vec<Movement>>();

    for m in movements {
        stack.r#move(*m).context("failed to move")?;
    }

    println!("{}", stack);

    for s in stack.inner.iter() {
        print!("{}", s[s.len() - 1]);
    }

    Ok(())
}

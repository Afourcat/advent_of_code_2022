use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn fight(&self, other: &Move) -> u32 {
        let result = match (self, other) {
            (a, b) if a == b => 3,
            (Move::Rock, Move::Scissors) => 0,
            (Move::Paper, Move::Rock) => 0,
            (Move::Scissors, Move::Paper) => 0,
            _ => 6,
        };

        println!("playing: {self:?} vs {other:?} => {result}");
        result
    }

    fn get_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(anyhow::anyhow!("invalid move")),
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    play: Move,
    response: Move,
}

impl Round {
    fn score(&self) -> u32 {
        let mut score = 0;
        score += self.play.fight(&self.response);
        score += self.response.get_score();
        score
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let play = Move::from_str(splitted.next().context("invalid line")?)?;
        let response = Move::from_str(splitted.next().context("invalid line")?)?;
        Ok(Round { play, response })
    }
}

const DEFAULT_INPUT_FILE: &'static str = "input.txt";

fn main() -> anyhow::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or(DEFAULT_INPUT_FILE.to_owned());
    let buf = std::fs::read_to_string(input_file).context("failed to read input file")?;

    let rounds: Vec<u32> = buf
        .lines()
        .map(Round::from_str)
        .filter_map(Result::ok)
        .map(|round| round.score())
        .collect();

    let total: u32 = rounds.iter().sum();

    println!("Rounds: {rounds:?} => {total}");
    Ok(())
}

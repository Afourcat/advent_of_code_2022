#![allow(dead_code)]
use anyhow::Context;

const WINDOW_SIZE: usize = 4;

fn main() -> anyhow::Result<()> {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("example.input.txt".to_owned());
    let puzzle_input = std::fs::read_to_string(file_path).context("failed to read file")?;

    let solution = puzzle_input
        .chars()
        .collect::<Vec<_>>()
        .windows(WINDOW_SIZE)
        .position(|w| {
            println!("{w:?}");
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        })
        .context("no solution found")?
        + WINDOW_SIZE;

    println!("{}", solution);
    Ok(())
}

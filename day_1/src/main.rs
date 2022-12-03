use anyhow::Context;
use std::io::Read;

#[derive(Default, Debug, Clone, Eq, PartialEq, PartialOrd)]
struct Elf {
    foods: Vec<usize>,
    total_calories: usize,
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();

    let input_file = args.nth(1).context("Missing input file argument")?;

    let mut file = std::fs::File::open(input_file).context("Failed to open input file")?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .context("Failed to read input file")?;

    let mut elves = elves_from_string(&buf)?;
    elves.sort_by(|a, b| a.total_calories.cmp(&b.total_calories));
    elves.reverse();

    println!("Top elf calories: {}", elves[0].total_calories);

    let total = elves[0..3]
        .iter()
        .map(|elf| elf.total_calories)
        .sum::<usize>();

    println!("Total calories for the 3 top elves: {total:?}");

    Ok(())
}

fn elves_from_string(buf: &str) -> anyhow::Result<Vec<Elf>> {
    let mut elves: Vec<Elf> = vec![];

    let mut elf = Elf::default();
    for line in buf.lines() {
        if line == "" {
            elf.total_calories = elf.foods.iter().sum::<usize>();
            elves.push(Elf::default());
            elf = std::mem::replace(elves.last_mut().unwrap(), elf);
            continue;
        }

        let calories = line.parse().context("invalid food calories")?;
        elf.foods.push(calories);
    }

    Ok(elves)
}

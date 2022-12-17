use std::fmt::Display;

use anyhow::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tree(u8);

#[derive(Debug, Clone)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    height: usize,
    width: usize,
}

impl Forest {
    fn new(buf: &str) -> Self {
        buf.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .map(|i| Tree(i as u8))
                    .collect::<Vec<Tree>>()
            })
            .collect::<Forest>()
    }

    fn count_visible_trees(&self) -> usize {
        let mut count = 0;

        for (y, row) in self.trees.iter().enumerate() {
            for (x, tree) in row.iter().enumerate() {
                if self.is_visible(x, y, tree.0) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_visible(&self, x: usize, y: usize, height: u8) -> bool {
        // From left
        let left = !(0..x)
            .map(|x| self.trees[y][x].0)
            .any(|cmp_tree| cmp_tree >= height);

        // From right
        let right = !(x + 1..self.width)
            .map(|x| self.trees[y][x].0)
            .any(|cmp_tree| cmp_tree >= height);

        // Top
        let top = !(0..y)
            .map(|y| self.trees[y][x].0)
            .any(|cmp_tree| cmp_tree >= height);

        let bottom = !(y + 1..self.height)
            .map(|y| self.trees[y][x].0)
            .any(|cmp_tree| cmp_tree >= height);

        left || right || top || bottom
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.trees {
            for tree in row {
                write!(f, "{}", tree.0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromIterator<Vec<Tree>> for Forest {
    fn from_iter<T: IntoIterator<Item = Vec<Tree>>>(iter: T) -> Self {
        let trees: Vec<Vec<Tree>> = iter.into_iter().collect();

        Self {
            height: trees.len(),
            width: trees[0].len(),
            trees,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input_file_path = std::env::args()
        .nth(1)
        .unwrap_or("example.input.txt".to_owned());

    let buf = std::fs::read_to_string(input_file_path).context("Failed to read input file")?;

    let forest = Forest::new(&buf);

    let count = forest.count_visible_trees();

    println!("visible_trees: {}", count);

    Ok(())
}

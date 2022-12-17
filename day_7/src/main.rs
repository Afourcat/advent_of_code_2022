#![allow(dead_code)]
use std::collections::HashMap;

use anyhow::Context;

#[derive(Debug, Clone)]
enum Entry {
    Dir(Directory),
    File(usize),
}

#[derive(Default, Debug, Clone)]
struct Directory {
    content: HashMap<String, Entry>,
    size: usize,
}

fn compute_total_dirsize(dir: &mut Directory) -> usize {
    let out = dir.content.iter_mut().fold(0, |acc, (_, v)| {
        acc + match v {
            Entry::Dir(ref mut inner) => compute_total_dirsize(inner),
            Entry::File(size) => *size,
        }
    });

    println!("computing_total_size of {}", out);
    dir.size = out;

    out
}

fn main() -> anyhow::Result<()> {
    let input_file_path = std::env::args()
        .nth(1)
        .unwrap_or("example.input.txt".to_owned());
    let buf = std::fs::read_to_string(input_file_path).context("Failed to read file to string")?;
    let lines = buf.lines().map(|s| s.to_owned()).collect::<Vec<String>>();

    let mut iter = lines.iter();
    //
    // skipping first "/"
    iter.next();
    let mut root_dir = build_dir("/", &mut iter);

    let root_size = compute_total_dirsize(&mut root_dir);
    println!("root_size: {root_size:#?}");

    // Find folder in root with size < 100000 and sum them.
    let result = root_dir
        .content
        .iter()
        .map(|(_, v)| find_lowest_sizes(v))
        .sum::<usize>();

    println!("result: {}", result);

    Ok(())
}

fn find_lowest_sizes(v: &Entry) -> usize {
    match v {
        Entry::Dir(dir) => {
            let s = if dir.size <= 100000 { dir.size } else { 0 };
            dir.content
                .iter()
                .fold(s, |acc, (_, v)| acc + find_lowest_sizes(v))
        }
        Entry::File(_) => 0,
    }
}

fn build_dir(dirname: &str, lines: &mut std::slice::Iter<String>) -> Directory {
    let mut dir = Directory::default();
    let mut in_cmd = false;

    while let Some(command) = lines.next() {
        let splitted = &command.split_whitespace().collect::<Vec<&str>>();

        match &splitted[..] {
            ["$", "ls"] => in_cmd = true,
            ["$", "cd", ".."] => break,
            ["$", "cd", focus] => {
                let focus = *focus;
                dir.content
                    .insert(focus.to_owned(), Entry::Dir(build_dir(focus, lines)));
            }
            [info, name] if in_cmd == true => {
                let entry = if *info == "dir" {
                    Entry::Dir(Default::default())
                } else {
                    Entry::File(info.parse().context("Invalid file size").unwrap())
                };
                dir.content.insert(name.to_string(), entry);
            }
            rest => panic!("Invalid command {rest:?}"),
        };
    }

    dir
}

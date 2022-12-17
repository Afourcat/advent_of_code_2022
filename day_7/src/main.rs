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

impl Directory {
    fn new(lines: &[String]) -> Self {
        let mut iter = lines.iter();

        // skip first "cd /" because we are already building it.
        iter.next();

        let mut root_dir = Self::build_dir(&mut iter);
        root_dir.compute_total_dirsize();
        root_dir
    }

    fn find_folder_to_remove(
        &self,
        current_folder_name: &str,
        perfect_dir_size: usize,
    ) -> (String, usize) {
        self.content.iter().fold(
            (current_folder_name.to_owned(), self.size),
            |(best_fit_name, best_fit_size), (entry_name, entry)| match entry {
                Entry::Dir(dir) => {
                    let (name, size) =
                        dir.find_folder_to_remove(entry_name.as_str(), perfect_dir_size);

                    if size >= perfect_dir_size && size < best_fit_size {
                        (name, size)
                    } else {
                        (best_fit_name, best_fit_size)
                    }
                }
                Entry::File(_) => (best_fit_name, best_fit_size),
            },
        )
    }

    fn build_dir(lines: &mut std::slice::Iter<String>) -> Directory {
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
                        .insert(focus.to_owned(), Entry::Dir(Self::build_dir(lines)));
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

    fn compute_total_dirsize(&mut self) -> usize {
        let out = self.content.iter_mut().fold(0, |acc, (_, v)| {
            acc + match v {
                Entry::Dir(ref mut inner) => inner.compute_total_dirsize(),
                Entry::File(size) => *size,
            }
        });

        self.size = out;
        out
    }
}

const REQUIRED_SIZE_FOR_UPDATE: usize = 30_000_000;
const TOTAL_SIZE: usize = 70_000_000;

fn main() -> anyhow::Result<()> {
    let input_file_path = std::env::args()
        .nth(1)
        .unwrap_or("example.input.txt".to_owned());

    // Construct an iterator over file lines
    let buf = std::fs::read_to_string(input_file_path).context("Failed to read file to string")?;
    let lines = buf.lines().map(|s| s.to_owned()).collect::<Vec<String>>();

    let root = Directory::new(&lines);

    // Compute ideal size to remove
    let unused_space = TOTAL_SIZE - root.size;
    let perfect_dir_size = REQUIRED_SIZE_FOR_UPDATE - unused_space;

    // Find fodler to remove
    println!("we are looking to delete {} of space.", perfect_dir_size);
    let (name, size) = root.find_folder_to_remove("/", perfect_dir_size);
    println!("removing: {} of size => {}.", name, size);

    Ok(())
}

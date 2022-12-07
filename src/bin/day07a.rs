use std::collections::HashMap;
use std::io;
use std::iter::Peekable;

const SIZE_THRESHOLD: usize = 100000;

#[derive(Debug)]
enum LsEntry {
    Dir { name: String },
    File { size: usize, name: String },
}

#[derive(Debug)]
enum Command {
    Cd { path: String },
    Ls { entries: Vec<LsEntry> },
}

impl Command {
    fn read(output: &mut Peekable<impl Iterator<Item = String>>) -> Option<Self> {
        let line = output.next()?;
        let tokens: Vec<_> = line
            .strip_prefix("$ ")
            .unwrap()
            .split_whitespace()
            .collect();

        match *tokens.as_slice() {
            [cmd, path] if cmd == "cd" => Some(Command::Cd {
                path: path.to_owned(),
            }),

            [cmd] if cmd == "ls" => {
                let mut nodes = vec![];
                while let Some(line) = output.next_if(|l| !l.starts_with('$')) {
                    let tokens: Vec<_> = line.split_whitespace().collect();
                    let node = match *tokens.as_slice() {
                        [dir, name] if dir == "dir" => LsEntry::Dir {
                            name: name.to_owned(),
                        },
                        [size, name] => LsEntry::File {
                            size: size.parse().unwrap(),
                            name: name.to_owned(),
                        },
                        _ => panic!(),
                    };
                    nodes.push(node);
                }

                Some(Command::Ls { entries: nodes })
            }

            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Dir {
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn get_dir_mut(&mut self, path: &[String]) -> Option<&mut Dir> {
        match path {
            [] => Some(self),
            [first, rest @ ..] => self.subdirs.get_mut(first)?.get_dir_mut(rest),
        }
    }

    fn size_and_sum_of_small(&self) -> (usize, usize) {
        let mut size: usize = self.files.values().sum();
        let mut sum_of_small = 0;
        for subdir in self.subdirs.values() {
            let (subdir_size, subdir_sum_of_small) = subdir.size_and_sum_of_small();
            size += subdir_size;
            sum_of_small += subdir_sum_of_small;
        }

        if size <= SIZE_THRESHOLD {
            sum_of_small += size;
        }

        (size, sum_of_small)
    }
}

fn main() {
    let mut output = io::stdin().lines().map(Result::unwrap).peekable();

    let first_command = Command::read(&mut output).unwrap();
    assert!(matches!(first_command, Command::Cd { path } if path == "/"));

    let mut root = Dir::new();

    let mut current_path: Vec<String> = vec![];

    while let Some(command) = Command::read(&mut output) {
        match command {
            Command::Cd { path } if path == ".." => {
                assert!(current_path.pop().is_some());
            }

            Command::Cd { path } => {
                current_path.push(path);
                assert!(root.get_dir_mut(current_path.as_slice()).is_some());
            }

            Command::Ls { entries } => {
                let current_dir = root.get_dir_mut(current_path.as_slice()).unwrap();

                for entry in entries {
                    match entry {
                        LsEntry::Dir { name } => {
                            assert!(current_dir.subdirs.insert(name, Dir::new()).is_none());
                        }
                        LsEntry::File { size, name } => {
                            assert!(current_dir.files.insert(name, size).is_none());
                        }
                    }
                }
            }
        }
    }

    let (_, sum_of_small) = root.size_and_sum_of_small();
    println!("{}", sum_of_small);
}
